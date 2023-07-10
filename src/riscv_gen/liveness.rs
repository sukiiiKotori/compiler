use std::collections::{BTreeMap, BTreeSet};
use std::cmp::Ordering;
use std::io::Write;

use crate::riscv_gen::reg::*;
use crate::structures::riscv_struct::*;
use crate::utils::check::{is_num_label, is_temp_opr};

impl AsmFunc {
    fn depth_first_order(&self, block_idx: &BTreeMap<&str, usize>) -> Vec<usize> {
        // 存储深度优先的各基本块在blocks中的顺序
        let mut res = Vec::new();

        // 搜索用vector，当作栈来用
        // 元组内各元素含义依次为：
        // 0: 出发基本块id，
        // 1: Branch指令的标号(若无终结符或终结符为Return，则设为vec!())，
        // 2: 当前从0出发遍历的第一个基本块
        let mut que: Vec<(usize, Vec<&String>, usize)> = Vec::new();

        // 根据基本块id判断是否遍历过
        let mut traversed: BTreeSet<usize> = BTreeSet::new();

        traversed.insert(0);
        let first_bb = self.blocks.get(0).unwrap();
        res.push(0);
        let dsts = first_bb.sux.iter().collect::<Vec<_>>(); 
        if !dsts.is_empty() {
            que.push((0, dsts, 0));
        }

        while !que.is_empty() {
            let this_info = que.pop().unwrap();
            let this_id = block_idx.get(this_info.1[this_info.2].as_str()).unwrap();

            // this_info.1还可遍历
            if this_info.2 < this_info.1.len()-1 {
                que.push((this_info.0, this_info.1, this_info.2+1));
            }

            if !traversed.contains(this_id) {
                traversed.insert(*this_id);
                let this_bb = self.blocks.get(*this_id).unwrap();
                res.push(*this_id);
                let dsts = this_bb.sux.iter().collect::<Vec<_>>();
                if !dsts.is_empty() {
                    que.push((*this_id, dsts, 0));
                }
            }
        } // while !que.is_empty()
        res
    }

    pub fn update_call_pos(&mut self, old2new: &BTreeMap<usize, usize>) {
        for (idx, depth_first_pos, _) in self.call_info.iter_mut() {
            *depth_first_pos = Some(*old2new.get(idx).unwrap())
        }
    }
}

struct BlockLiveInfo<'asm>{
    pub live_gen: BTreeSet<&'asm str>,
    pub live_kill: BTreeSet<&'asm str>,
    pub live_out: BTreeSet<&'asm str>,
    pub live_in: BTreeSet<&'asm str>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IntervalRange {
    pub from: usize,
    pub to: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Interval {
    pub ranges: Vec<IntervalRange>,
}

pub struct Liveness<'asm> {
    target: &'asm AsmFunc,
    block_idx: BTreeMap<&'asm str, usize>, // 根据基本块标号查询其在blocks的序号
    depth_first_order: Vec<usize>, // 按深度优先顺序存储各基本块在blocks中的序号
    depth_first_instr_cnt: Vec<usize>, // 存储各基本块按深度优先顺序后的instr_cnt
    pub old2new: BTreeMap<usize, usize>, // 将指令在函数定义中的位置转换为深度优先顺序编号
    new2old: BTreeMap<usize, usize>, // 与old2new反向
    block_info: BTreeMap<usize, BlockLiveInfo<'asm>>,
    intervals: BTreeMap<&'asm str, Interval>,
}

impl BlockLiveInfo<'_> {
    pub fn new<'asm>() -> BlockLiveInfo<'asm> {
        BlockLiveInfo{
            live_gen: BTreeSet::new(),
            live_kill: BTreeSet::new(),
            live_out: BTreeSet::new(),
            live_in: BTreeSet::new(),
        }
    } // fn
} // impl

impl Ord for IntervalRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
            .then_with(|| self.to.cmp(&other.to))
    }
}

impl PartialOrd for IntervalRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl IntervalRange {
    pub fn is_intersect(&self, other: &Self) -> bool {
        let max_from = std::cmp::max(self.from, other.from);
        let min_from = std::cmp::min(self.from, other.from);
        let max_to = std::cmp::max(self.to, other.to);
        let min_to = std::cmp::min(self.to, other.to);
        max_from >= min_to || (min_from == self.from && max_to == self.to) || (min_from == other.from && max_to == other.to)
    }
}

impl Interval {
    pub fn new() -> Self {
        Interval{
            ranges: Vec::new(),
        }
    }

    pub fn set_first_from(&mut self, from: usize) {
        if self.ranges.is_empty() {
            self.ranges.push(IntervalRange{from: from, to: from});
        } else {
            self.ranges.first_mut().unwrap().from = from;
        }
    }

    pub fn push_from(&mut self, from: usize) {
        if !self.ranges.is_empty() {
            self.ranges.last_mut().unwrap().from = from;
        }
    }

    pub fn push_to(&mut self, to: usize) {
        self.ranges.push(IntervalRange{from: 0, to: to});
    }

    pub fn add_range(&mut self, from: usize, to: usize) {
        let new_range = IntervalRange{from: from, to: to};
        for range in self.ranges.iter_mut() {
            if range.is_intersect(&new_range) {
                range.from = std::cmp::min(range.from, new_range.from);
                range.to = std::cmp::max(range.to, new_range.to);
                return;
            }
        } // for range in 
        self.ranges.push(IntervalRange{from: from, to: to});
    } // fn add_range

    pub fn sort_ranges(&mut self) {
        self.ranges.sort_by(|range0, range1| range0.from.cmp(&range1.from));
    }

    pub fn cmp_earlier(&self, other: &Self) -> Ordering {
        self.ranges.first().unwrap().from.cmp(&other.ranges.first().unwrap().from)
    }

    pub fn cmp_farther(&self, other: &Self) -> Ordering {
        self.ranges.last().unwrap().to.cmp(&other.ranges.last().unwrap().to)
    }

    pub fn is_inactive(&self, now: &Self) -> bool {
        self.ranges.last().unwrap().to <= now.ranges.first().unwrap().from
    }

    pub fn is_interfer(&self, other: &Self) -> bool {
        self.ranges.iter()
            .any(|range0| other.ranges.iter()
                .any(|range1| range0.is_intersect(range1)))
    }
}

impl Liveness<'_> {
    pub fn new<'asm>(target: &'asm AsmFunc) -> Liveness<'asm> {
        let block_idx = target.blocks.iter()
            .enumerate()
            .map(|(i, b)| (b.label.as_str(), i))
            .collect();
        let depth_first_order = target.depth_first_order(&block_idx);
        let mut depth_first_instr_cnt = Vec::new();
        depth_first_instr_cnt.resize(depth_first_order.len(), 0);
        for i in 1..depth_first_order.len() {
            let last_bb = target.blocks.get(depth_first_order[i-1]).unwrap();
            depth_first_instr_cnt[i] = depth_first_instr_cnt[i-1] + last_bb.instrs.len();
        }

        Liveness {
            target: target,
            block_idx: block_idx,
            depth_first_order: depth_first_order,
            depth_first_instr_cnt: depth_first_instr_cnt,
            old2new: BTreeMap::new(),
            new2old: BTreeMap::new(),
            block_info: BTreeMap::new(),
            intervals: BTreeMap::new(),
        }
    }

    /// 获取block内指令的新编号
    fn new_ids(cnt: &mut usize, this_bb: &AsmBlock) -> Vec<(usize, usize)> {
        let instr_len = this_bb.instrs.len();
        (0..instr_len).map(|i|{
            let res = (this_bb.instr_cnt+i, *cnt);
            *cnt += 1;
            res
        })
        .collect()
    }

    /// 计算深度优先顺序下各指令的编号，建立指令新旧编号的映射关系
    pub fn compute_depth_first_instr(&mut self) {
        let mut cnt = 0;
        self.depth_first_order.iter()
            .for_each(|i| { 
                let this_bb = self.target.blocks.get(*i).unwrap();
                let new_ids = Self::new_ids(&mut cnt, this_bb);
                new_ids.into_iter()
                    .for_each(|(i0, i1)| {
                        self.old2new.insert(i0, i1);
                        self.new2old.insert(i1, i0);
                    });
            });
    }
    
    #[allow(unused)]
    pub fn compute_dpeth_first_instr_debug(asm: &RiscV) {
        for func in asm.text.funcs.iter() {
            let mut liveness = Self::new(func);
            liveness.compute_depth_first_instr();
            println!("[{}]", func.label);
            func.blocks.iter()
                .for_each(|b|
                    println!("{}: instr_cnt={}, self_instr={}",
                        b.label,
                        b.instr_cnt,
                        b.instrs.len()
                    )
                );
            liveness.old2new.iter()
                .for_each(|(i0, i1)| {
                    if i0 != i1 {
                        println!("{} -> {}", i0, i1);
                    }
                });
            println!();
        } // for
    } // fn

    /// 在单个block范围内计算虚拟/物理寄存器的liveness
    pub fn compute_local_live_sets(&mut self) {
        self.target.blocks.iter()
            .enumerate()
            .for_each(|(idx, b)| {
                self.block_info.insert(idx, BlockLiveInfo::new());
                let info = self.block_info.get_mut(&idx).unwrap();
                b.instrs.iter()
                    .for_each(|instr| {
                        let (output, inputs) = instr.get_regs();
                        inputs.into_iter()
                            .for_each(|input|{
                                if !info.live_kill.contains(input) {
                                    info.live_gen.insert(input);
                                }
                            });
                        if let Some(out) = output {
                            info.live_kill.insert(out);
                        } // if let Some
                    }); // b.make_block_instrs: for_each
            }); // blocks: for_each
    } // fn

    #[allow(unused)]
    pub fn compute_local_live_sets_debug(asm: &RiscV) {
        println!("== compute_local_live_sets_debug  ==");
        for func in asm.text.funcs.iter() {
            let mut liveness = Self::new(func);
            liveness.compute_local_live_sets();
            liveness.block_info.iter()
                .for_each(|(b, info)| {
                    println!("{}:\nlive_gen={:#?}\nlive_kill={:#?}\n", 
                        func.blocks[*b].label, 
                        info.live_gen, 
                        info.live_kill
                    );
                });
        } // for
    }

    /// 按照深度优先逆序计算block间的虚拟/物理寄存器的liveness
    pub fn compute_global_live_sets(&mut self) {
        self.depth_first_order.iter()
            .rev()
            .for_each(|idx| {
                let this_bb = self.target.blocks.get(*idx).unwrap();
                let new_live_out = this_bb.sux.iter().collect::<Vec<_>>().into_iter()
                    .fold(self.block_info.get_mut(&idx).unwrap().live_out.clone(),
                        |acc, s| {
                            let idx = self.block_idx.get(s.as_str()).unwrap();
                            let sux_info = self.block_info.get(idx).unwrap();
                            &acc | &sux_info.live_in
                        });
                let info = self.block_info.get_mut(&idx).unwrap();
                info.live_out = new_live_out;
                info.live_in = &(&info.live_out - &info.live_kill) | &info.live_gen;
            });
    }

    #[allow(unused)]
    pub fn compute_global_live_sets_debug(asm: &RiscV) {
        println!("== compute_global_live_sets_debug  ==");
        for func in asm.text.funcs.iter() {
            let mut liveness = Self::new(func);
            liveness.compute_local_live_sets();
            liveness.compute_global_live_sets();
            liveness.block_info.iter()
                .for_each(|(b, info)| {
                    println!("{}:\nlive_gen={:#?}\nlive_kill={:#?}\nlive_out={:#?}\nlive_in={:#?}\n", 
                        func.blocks[*b].label, 
                        info.live_gen, 
                        info.live_kill,
                        info.live_in,
                        info.live_out,
                    );
                });
        } // for
    }

    pub fn build_intervals(&mut self) {
        self.depth_first_order.iter()
            .enumerate()
            .rev()
            .for_each(|(depth_idx, x)|{ // 深度优先逆序遍历block
                let this_bb = self.target.blocks.get(*x).unwrap();
                let this_info = self.block_info.get(x).unwrap();

                // block的范围为[block_from, block_to)
                let block_from = self.depth_first_instr_cnt.get(depth_idx).unwrap();
                let block_to = block_from+this_bb.instrs.len();

                // 处理live_out，全部添加当前block的范围
                this_info.live_out.iter()
                    .for_each(|x| {
                        if !self.intervals.contains_key(x) {
                            self.intervals.insert(x, Interval::new());
                        }
                        self.intervals.get_mut(x).unwrap().add_range(*block_from, block_to);
                    });

                this_bb.instrs.iter()
                    .enumerate()
                    .rev()
                    .for_each(|(idx, instr)| { // 逆序遍历当前block的指令
                        let (output, inputs) = instr.get_regs();                
                        if let Some(out) = output {
                            if !self.intervals.contains_key(out) {
                                self.intervals.insert(out, Interval::new());
                            }
                            if ALL_REGS.contains(out) {
                                self.intervals.get_mut(out).unwrap().push_from(block_from+idx);
                            } else {
                                self.intervals.get_mut(out).unwrap().set_first_from(block_from+idx);
                            }
                        }
                        inputs.iter()
                            .for_each(|input|{
                                if !self.intervals.contains_key(input) {
                                    self.intervals.insert(input, Interval::new());
                                }
                                if ALL_REGS.contains(input) {
                                    self.intervals.get_mut(input).unwrap().push_to(block_from+idx);
                                } else {
                                    self.intervals.get_mut(input).unwrap().add_range(*block_from, block_from+idx);
                                }
                            });
                    })
            });
    }

    #[allow(unused)]
    pub fn build_intervals_debug(asm: &RiscV) {
        let mut out = std::fs::File::create("interval.log").unwrap();
        write!(out, "{}\n", asm.text.funcs.len()).unwrap();
        asm.text.funcs.iter()
            .for_each(|f| {
                let mut liveness = Self::new(f);
                liveness.compute_local_live_sets();
                liveness.compute_global_live_sets();
                liveness.build_intervals();
                liveness.dump_intervals();
                write!(out, "{}\n", liveness.intervals.len()).unwrap();
                let mut interval_vec: Vec<(&&str, &mut Interval)> = liveness.intervals.iter_mut().collect();
                interval_vec.sort_by(|(k1, _), (k2, _)| {
                    if is_num_label(k1) && is_num_label(k2) {
                        let k1_num: usize = k1[1..].parse().unwrap();
                        let k2_num: usize = k2[1..].parse().unwrap();
                        k1_num.cmp(&k2_num)
                    } else if is_num_label(k1) && is_temp_opr(k2) {
                        let k1_num: usize = k1[1..].parse().unwrap();
                        let k2_num: usize = k2[6..].parse().unwrap();
                        k1_num.cmp(&k2_num)
                    } else if is_temp_opr(k1) && is_num_label(k2) {
                        let k1_num: usize = k1[6..].parse().unwrap();
                        let k2_num: usize = k2[1..].parse().unwrap();
                        k1_num.cmp(&k2_num)
                    } else if is_temp_opr(k1) && is_temp_opr(k2) {
                        let k1_num: usize = k1[6..].parse().unwrap();
                        let k2_num: usize = k2[6..].parse().unwrap();
                        k1_num.cmp(&k2_num)
                    } else {
                        k1.cmp(&k2)
                    }
                });
                interval_vec.into_iter()
                    .for_each(|(k, v)|{
                        write!(out, "{} {}\n", k, v.ranges.len());
                        v.ranges.sort();
                        v.ranges.iter()
                            .for_each(|range| {
                                write!(out, "{} {}\n", range.from, range.to).unwrap();
                            })
                    });
            });
    }

    #[allow(unused)]
    pub fn function_debug(asm: &RiscV) {
        //Self::compute_dpeth_first_instr_debug(asm);
        //Self::compute_local_live_sets_debug(asm);
        //Self::compute_global_live_sets_debug(asm);
        Self::build_intervals_debug(asm);
    }

    pub fn dump_intervals(&mut self) -> (Vec<(&str, Interval)>, Vec<(&str, Interval)>){
        let mut virt_intervals = Vec::new();
        let mut phy_intervals = Vec::new();

        for (k, v) in self.intervals.iter_mut() {
            v.sort_ranges();
            if ALL_REGS.contains(k) {
                phy_intervals.push((*k, v.clone()));
            } else {
                virt_intervals.push((*k, v.clone()));
            }
        }
        (virt_intervals, phy_intervals)
    }
}

