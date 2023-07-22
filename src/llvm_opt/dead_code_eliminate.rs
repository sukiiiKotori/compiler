use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;


pub fn eliminate(func: &FuncDef, active_bb: &HashSet<String>) -> HashSet<String> {
    let items: Vec<&Instruction> = func.make_blocks()
        .iter()
        .filter(|b| active_bb.contains(&b.block_label))
        .map(|b| b.make_block_instrs())
        .flatten()
        .collect();
    let (succs, mut preds) = build_map(items.clone());
    preds = preds.into_iter()
        .filter(|(s, _)| s.contains("@") || s.contains("%"))
        .collect();

    calc_active(&succs, preds)
}
