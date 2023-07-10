// use std::collections::btree_map::IterMut;
// use std::collections::{BTreeMap, BTreeSet};
// use crate::structures::riscv_struct::*;
// use crate::riscv_gen::reg::*;

// impl AsmFunc {
//     // 深度优先编号
//     fn dfs(&self, block_idx: &BTreeMap<&str, usize>) -> Vec<usize> {
//         // 访问序列结果
//         let mut res = Vec::new();
//         // 访问栈<pre, successor, has visited>
//         let mut stk:Vec<(usize, Vec<&String>, usize)> = Vec::new();
//         // 已访问状态
//         let mut visited: BTreeSet<usize> = BTreeSet::new();
        
//         // 访问初始块
//         visited.insert(0);
//         let block = self.blocks.get(0).unwrap();
//         res.push(0);
        
//         // 放入其后续块
//         let next_block:Vec<&String> = block.successor.iter().collect::<Vec<_>>();
//         if !next_block.is_empty() {
//             stk.push((0, next_block, 0));
//         }
    
//         // 深度优先搜索
//         while let Some(item) = stk.pop() {
//             // 获得当前块
//             let block_id = block_idx.get(item.1[item.2].as_str()).unwrap();
            
//             // 当前块pre的后续块如果没有访问完，重新入栈
//             if item.2 < item.1.len() - 1 {
//                 stk.push((item.0, item.1, item.2 + 1));
//             }
            
//             // 如果未访问，放入其后续块
//             if !visited.contains(block_id) {
//                 visited.insert(*block_id);
//                 res.push(*block_id);
//                 let block = self.blocks.get(*block_id).unwrap();
//                 let next_block:Vec<&String> = block.successor.iter().collect::<Vec<_>>();
//                 if !next_block.is_empty() {
//                     stk.push((0, next_block, 0));
//                 }
//             }
//         }
//         res
//     }

//     pub fn update_order(&mut self, id_to_pos: &BTreeMap<usize, usize>) {
//         for (idx, depth_first_order, _) in self.call_info.iter_mut() {
//             *depth_first_order = Some(*id_to_pos.get(idx).unwrap())
//         }
//     }
// }
