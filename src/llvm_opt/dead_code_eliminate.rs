use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;
use crate::llvm_opt::flow::FlowItem;

/// 死代码消除


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