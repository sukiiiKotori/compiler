use std::collections::{HashMap, HashSet};
use crate::llvm_gen::scopes::Labels;
use crate::structures::llvm_struct::*;


#[allow(unused)]
pub fn eliminate_all(mut program: LLVMProgram) -> LLVMProgram {
    // 输出消除不可达基本块和死代码的提示信息
    // vprintln!("Eliminating unreachable basic block and deadcode...");

    // 对每个函数进行处理
    // program.func_def = program
    //     .func_def
    //     .into_iter()
    //     .map(|f| {
    //         // 消除不可达基本块，返回活跃的基本块集合
    //         let active_bb = unreachable_eliminate::eliminate(&f);
    //
    //         // 消除死代码，返回活跃的标签集合
    //         let active_labels = dead_code_eliminate::eliminate(&f, &active_bb);
    //
    //         // 统计指令的数量
    //         let instr_cnt = f.count_instr();
    //
    //         // 存储活跃的指令索引的集合
    //         let mut active_instrs: HashSet<usize> = f
    //             .blocks
    //             .iter()
    //             .map(|b| {
    //                 let mut res: HashSet<usize> = HashSet::new();
    //                 let mut instr_cnt = b.instr_cnt;
    //
    //                 // 检查基本块中的 phi 指令，如果其活跃则添加到结果集合中
    //                 for p in b.phi_instr.iter() {
    //                     if p.is_active(&active_labels) {
    //                         res.insert(instr_cnt);
    //                     }
    //                     instr_cnt += 1;
    //                 }
    //
    //                 // 检查基本块中的普通指令，如果其活跃则添加到结果集合中
    //                 for i in b.instrs.iter() {
    //                     if i.is_active(&active_labels) {
    //                         res.insert(instr_cnt);
    //                     }
    //                     instr_cnt += 1;
    //                 }
    //
    //                 // 如果基本块有终结指令，则将其索引添加到结果集合中
    //                 if b.ter_instr.is_some() {
    //                     res.insert(instr_cnt);
    //                 }
    //                 res
    //             })
    //             // 将每个基本块中的活跃指令索引集合合并为一个结果集合
    //             .fold(HashSet::new(), |mut acc, instrs| {
    //                 for i in instrs.into_iter() {
    //                     acc.insert(i);
    //                 }
    //                 acc
    //             });
    //
    //         // 存储活跃的分配（alloc）指令索引的集合
    //         let mut active_allocs: HashSet<usize> = f
    //             .allocs
    //             .iter()
    //             .enumerate()
    //             // 过滤出活跃的分配指令，并将其索引转换为实际的指令索引
    //             .filter(|(_, (a, _))| a.is_active(&active_labels))
    //             .map(|(i, _)| instr_cnt + i)
    //             .collect();
    //
    //         // 将活跃的分配指令索引添加到活跃的指令索引集合中
    //         active_allocs.into_iter().for_each(|i| {
    //             active_instrs.insert(i);
    //         });
    //
    //         // 创建标签和索引的映射关系的结构
    //         let mut labels = Labels::new();
    //         let mut label_map = HashMap::new();
    //
    //         // 重新加载函数的内容，根据活跃的基本块、指令和标签进行更新
    //         f.reload(
    //             &mut |s| update_label(&mut labels, &mut label_map, s),
    //             &|s| active_bb.contains(s),
    //             &|i| active_instrs.contains(&(i as usize)),
    //         )
    //     })
    //     .collect();

    // 输出完成的提示信息
    // vprintln!("Finished\n");

    // 返回更新后的程序
    program
}