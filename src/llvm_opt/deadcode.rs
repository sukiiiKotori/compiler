use std::collections::{HashMap, HashSet};
use crate::llvm_opt::flow::*;
use crate::llvm_opt::flow_graph::*;

use crate::utils::check::*;
use crate::structures::llvm_struct::*;
use crate::llvm_gen::scopes::Labels;

pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, String>, old_label: &str) -> String {
    if !old_label.contains("%") { // 跳过非局部标号
        String::from(old_label)
    } else if !is_num_label(old_label) {// 局部非数字标号
        label_map.get(old_label).map_or(String::from(old_label), |x| String::from(x))
    } else { // 局部数字标号
        if let Some(new_label) = label_map.get(old_label) {
            String::from(new_label)
        } else {
            let new_label = labels.pop_num_str();
            label_map.insert(String::from(old_label), String::from(&new_label));
            new_label
        }
    }
}

/// 消除无法达到达基本块
pub mod unreachable_eliminate {
    use super::*;
    pub fn eliminate(func: &FuncDef) -> HashSet<String>{
        let items: Vec<_> = func.make_blocks();
        let (succs, mut preds) = build_map(items.clone());
        if !preds.contains_key("_entry") {
            preds.insert(String::from("_entry"), HashSet::new());
        }
        preds.get_mut("_entry")
            .unwrap()
            .insert(String::from(""));

        calc_active(&succs, preds)
    }

    #[allow(unused)]
    pub fn debug(func: &FuncDef) {
        let actives = eliminate(func);

        println!("[Origin] size: {}", func.blocks.len());
        for bb in func.blocks.iter() {
            println!("{}", bb.block_label);
        }
        println!("");

        println!("[Active] size: {}", actives.len());
        for active in actives.iter() {
            println!("{}", active);
        }
        println!("");

        let eliminated: Vec<_> = func.blocks.iter()
            .map(|x| x.block_label.as_str())
            .filter(|&x| !actives.contains(x))
            .collect();
        println!("[Eliminated] size: {}", eliminated.len());
        for e in eliminated.iter() {
            println!("{}", e);
        }
        println!("");
    }
}

pub mod dead_code_eliminate {
    use super::*;

    fn is_label(s: &str) -> bool {
        s.contains("@") || s.contains("%")
    }

    #[allow(unused)]
    pub fn eliminate(func: &FuncDef, active_bb: &HashSet<String>) -> HashSet<String> {
        let items: Vec<&Instruction> = func.make_blocks()
            .iter()
            .filter(|b| active_bb.contains(&b.block_label))
            .map(|b| b.make_block_instrs())
            .flatten()
            .collect();
        let (succs, mut preds) = build_map(items.clone());
        preds = preds.into_iter()
            .filter(|(k, _)| is_label(k.as_str()))
            .collect();

        calc_active(&succs, preds)
    }

    #[allow(unused)]
    pub fn debug(func: &FuncDef, active_bb: &HashSet<String>) {
        let actives = eliminate(func, active_bb);
        let items: Vec<&Instruction> = func.make_blocks()
            .iter()
            .filter(|b| active_bb.contains(&b.block_label))
            .map(|b| b.make_block_instrs())
            .flatten()
            .collect();

        let eliminated: Vec<_> = items.iter()
            .filter_map(|x| x.flow_info().0)
            .filter(|&x| !actives.contains(x))
            .collect();
        println!("[Eliminated] size: {}", eliminated.len());
        for e in eliminated.iter() {
            println!("{}", e);
        }
        println!("");
    }
}

impl Instruction {
    #[allow(unused)]
    pub fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        match self {
            Instruction::Add(bin_op) => bin_op.is_active(active_labels),
            Instruction::Sub(bin_op) => bin_op.is_active(active_labels),
            Instruction::Mul(bin_op) => bin_op.is_active(active_labels),
            Instruction::Sdiv(bin_op) => bin_op.is_active(active_labels),
            Instruction::Srem(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fadd(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fsub(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fmul(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fdiv(bin_op) => bin_op.is_active(active_labels),
            Instruction::Cmp(_, bin_op) => bin_op.is_active(active_labels),
            Instruction::Fcmp(_, bin_op) => bin_op.is_active(active_labels),
            Instruction::ZeroExt(conver_op) => conver_op.is_active(active_labels),
            Instruction::I32ToFloat(conver_op) => conver_op.is_active(active_labels),
            Instruction::FloatToI32(conver_op) => conver_op.is_active(active_labels),
            Instruction::Phi(res, _, _) => active_labels.contains(res.as_str()),
            Instruction::Alloca{res, ty: _, len: _} => active_labels.contains(res.as_str()),
            Instruction::Store{ty: _, value: _, ptr, len: _} => active_labels.contains(ptr.as_str()),
            Instruction::Load{res, ty: _, ptr: _, len: _} => active_labels.contains(res.as_str()),
            Instruction::Call(_, _, _, _) => true, // Call指令总是执行
            Instruction::GetElemPtr(_, _, _, _) => true, // GetElemPtr总是执行
            Instruction::BitCast(res, _, _, _) => active_labels.contains(res.as_str()),
            // 由于删除死代码后，代码结构发生变化，注释不再打印
            Instruction::Comment(_) => false,
            // 终结指令总是执行
            Instruction::Ret(_, _) => true,
            Instruction::Br(_, _, _) => true, 
        } // match   
    } // fn
} // impl

impl BinaryOp {
    #[allow(unused)]
    fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        active_labels.contains(self.res.as_str())
    }
}

impl CastOp {
    #[allow(unused)]
    fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        active_labels.contains(self.res.as_str())
    }
}

