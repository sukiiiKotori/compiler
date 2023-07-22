use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;

pub fn eliminate(func: &FuncDef) -> HashSet<String> {
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
