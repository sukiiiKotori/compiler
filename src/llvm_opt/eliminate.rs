use std::collections::{HashMap, HashSet};
use crate::llvm_opt::{dead_code_eliminate, unreachable_eliminate};
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::utils::check::is_num_label;

impl LLVMProgram {
    pub fn eliminate_unused_code(&mut self) {
        self.func_def.iter_mut().for_each(|func| {
            let active_bb = unreachable_eliminate::eliminate(func);
            let active_labels = dead_code_eliminate::eliminate(func, &active_bb);
            let instr_cnt = func.count_instr();
            let mut active_instrs = collect_active_instructions(func, &active_labels, instr_cnt);
            let active_allocs = collect_active_allocations(func, &active_labels, instr_cnt);
            active_allocs.into_iter().for_each(|i| { active_instrs.insert(i); });
            let (mut labels, mut label_map) = initialize_label_maps();
            func.rewrite(
                &mut |s| update_label(&mut labels, &mut label_map, s),
                &|s| active_bb.contains(s),
                &|i| active_instrs.contains(&(i as usize)),
            );
        });
    }
}

fn collect_active_instructions(func: &mut FuncDef, active_labels: &HashSet<String>, instr_cnt: usize) -> HashSet<usize> {
    func.blocks.iter().map(|block| {
        let mut res: HashSet<usize> = HashSet::new();
        let mut instr_cnt = block.ins_num.clone();
        for p in block.phi_ins.iter() {
            if p.is_active(&active_labels) { res.insert(instr_cnt.clone()); }
            instr_cnt += 1;
        }
        for i in block.nor_ins.iter() {
            if i.is_active(&active_labels) { res.insert(instr_cnt.clone()); }
            instr_cnt += 1;
        }
        if block.ter_ins.is_some() { res.insert(instr_cnt.clone()); }
        res
    }).fold(HashSet::new(), |mut acc, instrs| {
        for i in instrs.into_iter() { acc.insert(i); }
        acc
    })
}

fn collect_active_allocations(func: &mut FuncDef, active_labels: &HashSet<String>, instr_cnt: usize) -> HashSet<usize> {
    func.local_vars.iter().enumerate().filter(|(_, alloc)| {
        alloc.ins.is_active(&active_labels)
    }
    ).map(|(i, _)| instr_cnt + i).collect::<HashSet<usize>>()
}

fn initialize_label_maps() -> (Labels, HashMap<String, usize>) {
    (Labels::new(), HashMap::new())
}

pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, usize>, old_label: &str) -> String {
    if !old_label.contains("%") {
        return String::from(old_label);
    }
    if !is_num_label(old_label) {
        return label_map.get(old_label)
            .map_or(String::from(old_label), |x| String::from(x.to_string()));
    }
    if let Some(new_label) = label_map.get(old_label) {
        return String::from(new_label.to_string());
    }
    let new_label = labels.pop_num_str();
    label_map.insert(String::from(old_label), String::from(&new_label).parse().unwrap());
    new_label
}