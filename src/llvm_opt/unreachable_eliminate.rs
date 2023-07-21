use std::collections::HashSet;
use crate::llvm_opt::flow_graph::{build_map, calc_active};
use crate::llvm_opt::*;


///消除函数中的无法到达的基本块并返回活跃变量的集合。<br>
///参数： func - 要消除的无法到达的基本块的函数定义结构<br>
///返回值： 活跃变量的集合<br>
pub fn eliminate(func: &FuncDef) -> HashSet<String> {
    // 生成函数的代码块
    let items: Vec<_> = func.make_blocks();

    // 构建后继和前驱关系的映射
    let (succs, mut preds) = build_map(items.clone());

    // 如果没有"_entry"作为前驱，则添加之
    if !preds.contains_key("_entry") {
        preds.insert(String::from("_entry"), HashSet::new());
    }

    // 在"_entry"的前驱集合中插入空字符串
    // 在构建前驱关系图时，每个代码块的前驱节点被表示为一个集合。
    // 但是，入口点没有前驱节点，因此需要插入一个空字符串，以确保在计算活跃变量时正确处理入口点。
    preds.get_mut("_entry")
        .unwrap()
        .insert(String::from(""));

    // 计算活跃变量并返回结果
    calc_active(&succs, preds)
}
