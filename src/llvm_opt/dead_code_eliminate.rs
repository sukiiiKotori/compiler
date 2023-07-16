use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;
use crate::llvm_opt::flow::FlowItem;

/// 死代码消除


fn is_label(s: &str) -> bool {
    s.contains("@") || s.contains("%")
}


///消除函数中的冗余代码块并返回活跃变量的集合。<br>
///参数： func - 要消除冗余代码块的函数定义<br>
///参数： active_bb - 活跃的基本块集合，用于筛选要处理的代码块<br>
///返回值： 活跃变量的集合<br>
#[allow(unused)]
pub fn eliminate(func: &FuncDef, active_bb: &HashSet<String>) -> HashSet<String> {

    // 生成活跃的指令集合
    let items: Vec<&Instruction> = func.make_blocks()
        .iter()
        .filter(|b| active_bb.contains(&b.block_label))
        .map(|b| b.make_block_instrs())
        .flatten()
        .collect();

    // 构建后继和前驱关系的映射
    let (succs, mut preds) = build_map(items.clone());

    // 过滤掉不是标签的前驱关系
    preds = preds.into_iter()
        .filter(|(k, _)| is_label(k.as_str()))
        .collect();

    // 计算活跃变量并返回结果
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