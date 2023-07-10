use std::collections::btree_map::IterMut;
use std::collections::{BTreeMap, BTreeSet};
use crate::structures::riscv_struct::*;
use crate::riscv_gen::reg::*;

impl AsmFunc {
    // 将该块的后继块放入栈
    fn push_sux_in_stack(&self, stk: &Vec<(usize, Vec<&String>, usize)>, block: &AsmBlock) {
        let next_block = block.sux.iter().collect::<Vec<_>>();
        if !next_block.is_empty() {
            stk.push((0, next_block, 0));
        }
    }

    // 深度优先编号
    fn dfs(&self, block_idx: &BTreeMap<&str, usize>) -> Vec<usize> {
        let mut res = vec!();
        // vec<pre_id, now_id, has visited>
        let mut stk: Vec<(usize, Vec<&String>, usize)> = vec!();
        // block_state表示该块已搜索
        let mut visited: BTreeSet<usize> = BTreeSet::new();

        // 搜索起始块，然后存入其后继块
        visited.insert(0);
        let block = self.blocks.get(0).unwrap();
        res.push(0);
        
        self.push_sux_in_stack(&stk, block);

        // 深度优先搜索
        while !stk.is_empty() {
            let item = stk.pop().unwrap();
            let block_id = block_idx.get(item.1[item.2].as_str()).unwrap();

            // 该块栈中还有块没有遍历，放回去，计数指向下一个块
            if item.2 < item.1.len() - 1 {
                stk.push((item.0, item.1, item.2 + 1));
            }

            // 如果该块没有遍历，将该块加入遍历顺序结果，加入该块的后续块
            if visited.contains(block_id) == false {
                visited.insert(*block_id);
                res.push(*block_id);
                let block = self.blocks.get(*block_id).unwrap();

            }
        }
        res
    }
}