use std::collections::{HashMap, HashSet};
use crate::llvm_opt::{dead_code_eliminate, unreachable_eliminate};
use crate::llvm_opt::deadcode::update_label;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;

impl LLVMProgram {
    pub fn eliminate_all(&mut self) {
        // 对每个函数进行处理
        self.func_def.iter_mut().for_each(|func| {
            // 消除不可达基本块，返回活跃的基本块集合
            let active_bb = unreachable_eliminate::eliminate(func);
    
            // 消除死代码，返回活跃的标签集合
            let active_labels = dead_code_eliminate::eliminate(func, &active_bb);
    
            // 统计指令的数量
            let instr_cnt = func.count_instr();
    
            // 存储活跃的指令索引的集合
            let mut active_instrs: HashSet<usize> = func.blocks.iter().map(|b| {
                    let mut res: HashSet<usize> = HashSet::new();
                    let mut instr_cnt = b.ins_num.clone();
    
                    // 检查基本块中的 phi 指令，如果其活跃则添加到结果集合中
                    for p in b.phi_ins.iter() {
                        if p.is_active(&active_labels) {
                            res.insert(instr_cnt.clone());
                        }
                        instr_cnt += 1;
                    }
    
                    // 检查基本块中的普通指令，如果其活跃则添加到结果集合中
                    for i in b.nor_ins.iter() {
                        if i.is_active(&active_labels) {
                            res.insert(instr_cnt.clone());
                        }
                        instr_cnt += 1;
                    }
    
                    // 如果基本块有终结指令，则将其索引添加到结果集合中
                    if b.ter_ins.is_some() {
                        res.insert(instr_cnt.clone());
                    }
                    res
                })
                // 将每个基本块中的活跃指令索引集合合并为一个结果集合
                .fold(HashSet::new(), |mut acc, instrs| {
                    for i in instrs.into_iter() {
                        acc.insert(i);
                    }
                    acc
                });
    
            // 存储活跃的分配（alloc）指令索引的集合
            let active_allocs = func.local_vars.iter().enumerate().filter(|(_, a)| {
                a.ins.is_active(&active_labels)}
            ).map(|(i, _)| instr_cnt + i).collect::<HashSet<usize>>();
    
            // 将活跃的分配指令索引添加到活跃的指令索引集合中
            active_allocs.into_iter().for_each(|i| {
                active_instrs.insert(i);
            });
    
            // 创建标签和索引的映射关系的结构
            let mut labels = Labels::new();
            let mut label_map = HashMap::new();
    
            // 重新加载函数的内容，根据活跃的基本块、指令和标签进行更新
            func.rewrite(
                &mut |s| update_label(&mut labels, &mut label_map, s),
                &|s| active_bb.contains(s),
                &|i| active_instrs.contains(&(i as usize)),
            );
        });
    }
}