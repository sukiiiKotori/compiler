use std::collections::{HashMap, HashSet};
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::utils::check::is_num_label;

impl LLVMProgram {
    pub fn eliminate_unused_code(&mut self) {
        // 对每个函数进行遍历
        self.func_def.iter_mut().for_each(|func| {
            // 消除不可达代码
            let active_bb = unreachable_code_eliminate(func);
            // 消除死代码
            let active_labels = dead_code_eliminate(func, &active_bb);
            // 统计指令数量
            let instr_cnt = func.count_instr();
            // 收集活跃指令
            let mut active_instrs = collect_active_instructions(func, &active_labels);
            // 收集活跃分配
            let active_allocs = collect_active_allocations(func, &active_labels, instr_cnt);
            // 将活跃分配插入活跃指令集合
            active_allocs.into_iter().for_each(|i| { active_instrs.insert(i); });
            // 初始化标签映射
            let (mut labels, mut label_map) = initialize_label_maps();
            // 重写函数
            func.rewrite (
                // 更新标签
                &mut |s| update_label(&mut labels, &mut label_map, s),
                // 检查基本块是否活跃
                &|s| active_bb.contains(s),
                // 检查指令是否活跃
                &|i| active_instrs.contains(&(i as usize)),
            );
        });
    }
}

// 收集活跃指令
fn collect_active_instructions(func: &mut FuncDef, active_labels: &HashSet<String>) -> HashSet<usize> {
    func.blocks.iter().map(|block| {
        let mut res: HashSet<usize> = HashSet::new();
        let mut instr_cnt = block.ins_num.clone();
        // 遍历phi指令
        for p in block.phi_ins.iter() {
            if p.is_active(&active_labels) { res.insert(instr_cnt.clone()); }
            instr_cnt += 1;
        }
        // 遍历普通指令
        for i in block.nor_ins.iter() {
            if i.is_active(&active_labels) { res.insert(instr_cnt.clone()); }
            instr_cnt += 1;
        }
        // 如果有终止指令，则插入
        if block.ter_ins.is_some() { res.insert(instr_cnt.clone()); }
        res
    }).fold(HashSet::new(), |mut acc, instrs| {
        for i in instrs.into_iter() { acc.insert(i); }
        acc
    })
}

// 消除死代码
pub fn dead_code_eliminate(func: &FuncDef, active_bb: &HashSet<String>) -> HashSet<String> {
    // 构建指令集合
    let items: Vec<&Instruction> = func.make_blocks()
        .iter()
        .filter(|b| active_bb.contains(&b.block_label))
        .map(|b| b.make_block_instrs())
        .flatten()
        .collect();
    // 构建后继和前驱映射
    let (succs, mut preds) = build_map(items.clone());
    // 过滤前驱映射，只保留活跃的
    preds = preds.into_iter()
        .filter(|(s, _)| s.contains("@") || s.contains("%"))
        .collect();
    // 计算活跃基本块
    calc_active(&succs, preds)
}

// 函数用于消除不可达代码，返回活跃的基本块标签的集合
pub fn unreachable_code_eliminate(func: &FuncDef) -> HashSet<String> {
    // 将函数的基本块转换为向量
    let items: Vec<_> = func.make_blocks();
    // 构建后继和前驱的映射关系
    let (succs, mut preds) = build_map(items.clone());
    // 如果前驱中不包含"_entry"，则将其添加到前驱映射中
    if !preds.contains_key("_entry") {
        preds.insert(String::from("_entry"), HashSet::new());
    }
    // 在"_entry"的前驱中添加一个空字符串
    preds.get_mut("_entry")
        .unwrap()
        .insert(String::from(""));
    // 计算活跃的基本块标签
    calc_active(&succs, preds)
}

// 函数用于收集活跃的分配指令，返回活跃的指令的索引的集合
fn collect_active_allocations(func: &mut FuncDef, active_labels: &HashSet<String>, instr_cnt: usize) -> HashSet<usize> {
    // 遍历函数的局部变量并根据活跃的标签进行过滤
    func.local_vars.iter().enumerate().filter(|(_, alloc)| {
        alloc.ins.is_active(&active_labels)
    }).map(|(i, _)| instr_cnt + i).collect::<HashSet<usize>>()
}

// 函数用于初始化标签和标签映射
fn initialize_label_maps() -> (Labels, HashMap<String, usize>) {
    (Labels::new(), HashMap::new())
}

// 函数用于更新标签
pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, usize>, old_label: &str) -> String {
    // 如果旧标签不包含 "%"，则直接返回旧标签
    if !old_label.contains("%") {
        return String::from(old_label);
    }
    // 如果旧标签不是数字标签，则根据标签映射返回新标签
    if !is_num_label(old_label) {
        return label_map.get(old_label)
            .map_or(String::from(old_label), |x| String::from(x.to_string()));
    }
    // 如果旧标签是数字标签且存在于标签映射中，则返回对应的新标签
    if let Some(new_label) = label_map.get(old_label) {
        return String::from(new_label.to_string());
    }
    // 生成一个新的数字标签，并将旧标签与新标签的映射关系添加到标签映射中
    let new_label = labels.pop_num_str();
    label_map.insert(String::from(old_label), String::from(&new_label).parse().unwrap());
    new_label
}