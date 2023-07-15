use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;

/// 消除无法达到达基本块

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