use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::structures::symbol::SymbolWidth;
use crate::riscv_gen::register_resource::*;
use crate::riscv_gen::register_alloc::*;
use crate::riscv_gen::register_type::*;

// 块活跃性信息
#[derive(Eq, PartialEq, Clone, Debug)]
struct BlockLiveInfo<'asm>{
    pub live_gen: BTreeSet<&'asm str>,      // 生成的活跃变量
    pub live_kill: BTreeSet<&'asm str>,     // 终结的活跃变量
    pub live_out: BTreeSet<&'asm str>,      // 出口活跃变量
    pub live_in: BTreeSet<&'asm str>,       // 入口活跃变量
}

// 活跃区间具体范围
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IntervalRange {
    pub left: usize,
    pub right: usize,
}

// 活跃区间
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Interval {
    pub intervals: Vec<IntervalRange>,
}

// 
pub struct LiveInterval<'asm> {
    target: &'asm AsmFunc,                  // 目标
    block_idx: BTreeMap<&'asm str, usize>,  // 根据基本块标号查询其在blocks的序号   id -> number
    depth_first_order: Vec<usize>,          // 按深度优先顺序存储各基本块在blocks中的序号   // depth_first_search_id -> number
    depth_first_pre_instr_cnt: Vec<usize>,      // 存储各基本块按深度优先顺序后的pre_instr_cnt      // 指令数
    pub old2new: BTreeMap<usize, usize>,    // 将指令在函数定义中的位置转换为深度优先顺序编号 // id -> depth_first_search_id
    new2old: BTreeMap<usize, usize>,        // 与old2new反向            // depth_first_search_id -> id
    block_info: BTreeMap<usize, BlockLiveInfo<'asm>>,   // 各块的活跃性信息
    intervals: BTreeMap<&'asm str, Interval>,           // 各块的活跃区间
}

impl AsmFunc {
    // 深度优先编号
    fn depth_first_search(&self, block_idx: &BTreeMap<&str, usize>) -> Vec<usize> {
        // 访问序列结果
        let mut res = Vec::new();
        // 访问栈<pre, successor, has visited>
        let mut stk:Vec<(usize, Vec<&String>, usize)> = Vec::new();
        // 已访问状态
        let mut visited: BTreeSet<usize> = BTreeSet::new();
        
        // 访问初始块
        visited.insert(0);
        let block = self.blocks.get(0).unwrap();
        res.push(0);
        
        // 放入其后续块
        let next_block:Vec<&String> = block.successor.iter().collect::<Vec<_>>();
        if !next_block.is_empty() {
            stk.push((0, next_block, 0));
        }
    
        // 深度优先搜索
        while let Some(item) = stk.pop() {
            // 获得当前块
            let block_id = block_idx.get(item.1[item.2].as_str()).unwrap();
            
            // 当前块pre的后续块如果没有访问完，重新入栈
            if item.2 < item.1.len() - 1 {
                stk.push((item.0, item.1, item.2 + 1));
            }
            
            // 如果未访问，放入其后续块
            if !visited.contains(block_id) {
                visited.insert(*block_id);
                res.push(*block_id);
                let block = self.blocks.get(*block_id).unwrap();
                let next_block:Vec<&String> = block.successor.iter().collect::<Vec<_>>();
                if !next_block.is_empty() {
                    stk.push((0, next_block, 0));
                }
            }
        }
        res
    }

    pub fn update_order(&mut self, old2new: &BTreeMap<usize, usize>) {
        for (idx, depth_first_order, _) in self.call_info.iter_mut() {
            *depth_first_order = Some(*old2new.get(idx).unwrap())
        }
    }

    // 在给定的寄存器活跃期内，是否调用了其他函数(表明该寄存器是需要保存的)
    pub fn is_reg_saved(&mut self, interval: &Interval, vir: &str) -> bool {
        let mut res = false;
        for (_, depth_first_pos, cross_virs) in self.call_info.iter_mut() {
            let cross = interval.intervals.iter().any(|range| range.left < *depth_first_pos.as_ref().unwrap() && range.right > *depth_first_pos.as_ref().unwrap());
            if cross {
                res = true;
                cross_virs.insert(String::from(vir));
            }
        }
        res
    }

    // 判断变量是否为float
    pub fn is_reg_float(&self, label: &str) -> bool {
        *self.label_type.get(label).expect(&format!("Type of {} havent added", label)) == SymbolWidth::Float
    }

    // 表明使用了被保存寄存器
    pub fn used_saved(&mut self, phy: &'static str) {
        self.used_saved.insert(phy);
    }
}

impl BlockLiveInfo<'_> {
    pub fn new<'asm>() -> BlockLiveInfo<'asm> {
        BlockLiveInfo{
            live_gen: BTreeSet::new(),
            live_kill: BTreeSet::new(),
            live_out: BTreeSet::new(),
            live_in: BTreeSet::new(),
        }
    }
}

impl Ord for IntervalRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.left.cmp(&other.left) {
            Ordering::Equal => self.right.cmp(&other.right),
            _ => self.left.cmp(&other.left)
        }
    }
}

impl PartialOrd for IntervalRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 实现判断两个活跃性区间是否相交的功能
impl IntervalRange {
    pub fn judge_overlap(&self, other: &Self) -> bool {
        !(self.left > other.right || self.right < other.left)
    }
}

impl Interval {
    pub fn new() -> Self {
        Interval { intervals: vec!() }
    }

    pub fn set_first_left(&mut self, left: usize) {
        if !self.intervals.is_empty() {
            self.intervals.first_mut().unwrap().left = left;
        } else {
            self.intervals.push(IntervalRange { left: left, right: left });
        }
    }

    pub fn push_left(&mut self, left: usize) {
        if !self.intervals.is_empty() {
            self.intervals.last_mut().unwrap().left = left;
        } else {
            self.set_first_left(left);
        }
    }

    pub fn push_right(&mut self, right: usize) {
        self.intervals.push(IntervalRange { left: 0, right: right });
    }

    // 和某区间重叠则合并，否则加入末尾
    pub fn push(&mut self, left: usize, right: usize) {
        let push_interval = IntervalRange { left: left, right: right };
        for interval in self.intervals.iter_mut() {
            if interval.judge_overlap(&push_interval) {
                interval.left = std::cmp::min(interval.left, left);
                interval.right = std::cmp::max(interval.right, right);
                return;
            }
        }
        self.intervals.push(push_interval);
    }
    
    pub fn sort_interval(&mut self) {
        self.intervals.sort_by(|a, b| a.cmp(b));
    }

    pub fn is_inactive(&self, now: &Self) -> bool {
        self.intervals.last().unwrap().right <= now.intervals.first().unwrap().left
    }
}

impl LiveInterval<'_> {
    pub fn new<'asm>(target: &'asm AsmFunc) -> LiveInterval<'asm> {
        let block_idx = target.blocks.iter().enumerate().map(|(i, b)| (b.label.as_str(), i)).collect();
        let depth_first_order = target.depth_first_search(&block_idx);
        let mut depth_first_pre_instr_cnt = vec![0; depth_first_order.len()];
        let mut pre_block = target.blocks.get(depth_first_order[0]).unwrap();
        for i in 1..depth_first_pre_instr_cnt.len() {
            depth_first_pre_instr_cnt[i] = depth_first_pre_instr_cnt[i-1] + pre_block.instrs.len();
            pre_block = target.blocks.get(depth_first_order[i]).unwrap();
        }

        LiveInterval {
            target: target,
            block_idx: block_idx,
            depth_first_order: depth_first_order,
            depth_first_pre_instr_cnt: depth_first_pre_instr_cnt,
            old2new: BTreeMap::new(),
            new2old: BTreeMap::new(),
            block_info: BTreeMap::new(),
            intervals: BTreeMap::new(),
        }
    }

    // 更新块指令id
    fn update_instr_id(&self, cnt: &mut usize, block: &AsmBlock) -> Vec<(usize, usize)> {
        let mut res = vec!();
        for i in 0..block.instrs.len() {
            res.push((block.pre_instr_cnt + i, *cnt));
            *cnt += 1;
        }
        res
    }

    // 更新depth first order下的指令序号，建立相互映射关系
    pub fn update_instr_id_dfo(&mut self) {
        let mut cnt: usize = 0;
        for idx in self.depth_first_order.iter() {
            let block = self.target.blocks.get(*idx).unwrap();
            let id_dfo = self.update_instr_id(&mut cnt, block);
            for (a, b) in id_dfo.iter() {
                self.old2new.insert(*a, *b);
                self.new2old.insert(*b, *a);
            }
        }
    }

    // 计算局部的虚拟/物理寄存器活跃区间
    pub fn cul_local_liveinterval(&mut self) {
        for (idx, block) in self.target.blocks.iter().enumerate() {
            let info = self.block_info.entry(idx).or_insert_with(BlockLiveInfo::new);
        
            for instr in &block.instrs {
                let (output, inputs) = instr.get_regs();

                for input in inputs {
                    if !info.live_kill.contains(input) {
                        info.live_gen.insert(input);
                    }
                }
                
                if let Some(out) = output {
                    info.live_kill.insert(out);
                }
            }
        }
    }

    // 计算全局（块间）的虚拟/物理寄存器的活跃区间
    pub fn cul_global_liveinterval(&mut self) {
        for &idx in self.depth_first_order.iter().rev() {
            let block = self.target.blocks.get(idx).unwrap();
            let mut live_out = BTreeSet::new();             //当前块的活跃出口变量
            
            for sur in &block.successor {
                let sur_id = self.block_idx.get(sur.as_str()).unwrap();
                let sur_info = self.block_info.get(sur_id).unwrap();
                live_out.extend(&sur_info.live_in);
            }
            let info = self.block_info.get_mut(&idx).unwrap();
            // 计算出口和入口的活跃变量
            info.live_out = live_out;
            info.live_in = &(&info.live_out - &info.live_kill) | &info.live_gen;
        }
    }

    // 建立活跃区间
    pub fn build_liveinterval(&mut self) {
        for (depth_idx, idx) in self.depth_first_order.iter().enumerate().rev() {
            let block = self.target.blocks.get(*idx).unwrap();
            let info = self.block_info.get(idx).unwrap();

            // 计算开始和结束位置(基本块范围)
            let left = self.depth_first_pre_instr_cnt.get(depth_idx).unwrap();
            let right = left + block.instrs.len();
            
            // 获取该块出口活跃变量
            // 如果不在intervals中，将其插入
            // 更新其对应活跃区间
            for live_out in &info.live_out {
                let interval = self.intervals.entry(live_out).or_insert_with(Interval::new);
                interval.push(*left, right);
            }
            
            // 逆序遍历该块中的指令
            for (idx, instr) in block.instrs.iter().enumerate().rev() {
                let (output, inputs) = instr.get_regs();
                if let Some(out) = output {
                    let interval = self.intervals.entry(out).or_insert_with(Interval::new);
                    if ALL_REGS.contains(out) {
                        interval.push_left(left + idx);
                    } else {
                        interval.set_first_left(left + idx);
                    }
                }
                for input in inputs {
                    let interval = self.intervals.entry(input).or_insert_with(Interval::new);
                    if ALL_REGS.contains(input) {
                        interval.push_right(left + idx);
                    } else {
                        interval.push(*left, left + idx);
                    }
                }
            }
        }
    }

    pub fn dump_intervals(&mut self) -> (Vec<(&str, Interval)>, Vec<(&str, Interval)>) {
        let mut vir = vec!();
        let mut phy = vec!();

        for (idx, intervals) in self.intervals.iter_mut() {
            intervals.sort_interval();
            if ALL_REGS.contains(idx) {
                phy.push((*idx, intervals.clone()));
            } else {
                vir.push((*idx, intervals.clone()));
            }
        }

        (vir, phy)
    }
}

// 活跃节点
#[derive(Clone, Eq, PartialEq)]
pub struct ActiveNode {
    // 都可以不存在
    pub vir: Option<String>,    // 虚拟寄存器
    pub interval: Option<Interval>, // 活跃区间
    pub phy: Option<&'static str>,  // 物理寄存器
}

// 线性扫描结构体
pub struct LinearScan {
    pub var_interval: Vec<(String, Interval)>,
    pub reg_res: RegisterResource,
    pub activenodes: Vec<ActiveNode>,
    pub inactivemap: HashMap<String, &'static str>,
    pub spilled: HashSet<String>,
}

impl Ord for ActiveNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.as_ref().unwrap().intervals.last().unwrap().right.cmp(
            &other.interval.as_ref().unwrap().intervals.last().unwrap().right
        )
    }
}

impl PartialOrd for ActiveNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ActiveNode {
    pub fn new(vir: String, interval: Interval, phy: &'static str) -> Self {
        ActiveNode {
            vir: Some(vir),
            interval: Some(interval),
            phy: Some(phy),
        }
    }

    pub fn take_reg(&mut self) -> (String, Interval, &'static str) {
        (self.vir.take().unwrap(), self.interval.take().unwrap(), self.phy.take().unwrap())
    }
}

impl LinearScan {
    // 加载空闲寄存器，调用寄存器模块方法，并排除不可用的寄存器
    fn load_free_regs(&mut self) {
        self.reg_res.load_free_regs();
        self.reg_res.remove_regs(|reg| PRESERVED_SET.contains(reg) || FLOAT_PRESERVED_SET.contains(reg));
    }

    // 将当前活跃节点中已经处于非活跃状态的节点更改为非活跃节点
    fn disactive(&mut self, now: &Interval, func: &mut AsmFunc) {
        // 获取当前活跃节点中已经处于非活跃状态的节点
        let inactive = self.activenodes.iter().enumerate().filter_map(|(idx, node)|{
            if node.interval.as_ref().unwrap().is_inactive(now) {
                Some(idx)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        // 从活跃节点中移除这些节点
        let inactive_regs: Vec<_> = inactive.into_iter().rev().map(
            |idx| self.activenodes.remove(idx).take_reg()
        ).collect();
        
        for (vir, _, phy) in inactive_regs.into_iter() {
            // 使用被保存寄存器
            if SAVED_SET.contains(phy) || FLOAT_SAVED_SET.contains(phy) {
                func.used_saved(phy);
            }
            // 释放物理寄存器资源
            self.reg_res.free_register(phy);
            // 添加到非活跃节点映射表
            self.inactivemap.insert(vir, phy);
        }
    }

    fn regtype_filter(reg: &str, ty: &RegType) -> bool {
        ty.regtype_filter(reg)
    }

    // 溢出某个虚拟寄存器(vir)到内存，腾出寄存器空间
    // 根据当前虚拟寄存器的活跃区间和类型，在活跃节点列表中选择最大活跃区间的节点进行驱逐（溢出）操作。
    // 如果当前虚拟寄存器的活跃区间较大，则将其标记为溢出；否则，将最大节点驱逐出去，并将当前虚拟寄存器放入活跃节点列表中。
    fn spill_var_reg(&mut self, spilled_vir: String, interval: Interval, regty: RegType) {
        // 找到满足条件的节点中，活跃区间最大的节点
        if let Some((max_idx, max_interval)) = self.activenodes.iter().enumerate().filter(
            |(_, node)| Self::regtype_filter(node.phy.as_ref().unwrap(), &regty)
        ).max_by(
            |(_, node0), (_, node1)| node0.cmp(node1)
        ).map(
            |(idx, node)| (idx, node.interval.as_ref().unwrap())
        ) {
            if interval.intervals.last().unwrap().right.cmp(
                &max_interval.intervals.first().unwrap().left
            ) == Ordering::Greater {
                // 当前虚拟寄存器的活跃区间比最大节点的活跃区间更大。
                // 将当前虚拟寄存器的名称插入到 self.spilled 集合中，表示发生了溢出。
                self.spilled.insert(spilled_vir);
            } else {
                // 当前虚拟寄存器的活跃区间较小，需要将最大节点驱逐出去，并将当前虚拟寄存器放入活跃节点列表中。
                let (vir, _, phy) = self.activenodes.remove(max_idx).take_reg();
                self.spilled.insert(vir);
                self.activenodes.push(ActiveNode::new(spilled_vir, interval, phy));
            }
        }
    }
}

impl RegisterAllocator for LinearScan {
    fn new() -> Self {
        LinearScan {
            var_interval: vec!(), 
            reg_res: RegisterResource::new(), 
            activenodes: vec!(), 
            inactivemap: HashMap::new(), 
            spilled: HashSet::new()
        }
    }
    
    // 分配寄存器
    fn alloc_regs(&mut self, func: &mut AsmFunc) {
        // 创建活跃区间
        let mut liveinterval = LiveInterval::new(func);

        // 计算局部/全局虚拟寄存器的活跃区间，并且建立活跃区间信息
        liveinterval.cul_local_liveinterval();
        liveinterval.cul_global_liveinterval();
        liveinterval.build_liveinterval();
        let (vir_interval, _) = liveinterval.dump_intervals();

        // 将虚拟寄存器和活跃区间存入self.var_interval
        self.var_interval = vir_interval.into_iter().map(
            |(vir, interval)| (vir.to_string(), interval)
        ).collect();
        // 根据depth first order方法排序
        self.var_interval.sort_by(
            |(_, interval1), (_, interval2)|
            interval2.intervals.first().unwrap().left.cmp(&interval1.intervals.first().unwrap().left)
        );

        // 调用 update_instr_id_dfo() 计算指令的深度优先顺序。
        liveinterval.update_instr_id_dfo();
        // 更新调用位置，将旧的位置映射到新的位置。
        func.update_order(&liveinterval.old2new);

        // 加载空闲寄存器
        self.load_free_regs();

        // 处理所有虚拟寄存器的活跃区间，分配物理寄存器或者溢出
        while !self.var_interval.is_empty() {
            let (vir, interval) = self.var_interval.pop().unwrap();
            // 根据当前活跃区间将非活跃节点从活跃节点列表中移除。
            self.disactive(&interval, func);

            // 尝试分配一个新的物理寄存器，如果成功分配到新的物理寄存器，将虚拟寄存器、活跃区间和物理寄存器组成一个新的活跃节点
            // 如果无法分配到新的物理寄存器，则进行溢出

            // 获取寄存器类型(TempInt,SavedInt,TempFloat,SavedFloat)
            let regty = RegType::get_regtype(
                func.is_reg_float(vir.as_str()),
                // 根据活跃间隔判断该寄存器是否是需要保存的
                func.is_reg_saved(&interval, vir.as_str())
            );
            let reg = self.reg_res.get_register(&regty, |reg| Self::regtype_filter(reg, &regty));
            if let Some(phy) = reg {
                self.activenodes.push(ActiveNode::new(vir, interval, phy));
            } else {
                self.spill_var_reg(vir, interval, regty);
            }
        }

        // 遍历活跃节点，如果节点的物理寄存器在 SAVED_SET 或 FLOAT_SAVED_SET 中，表示使用了被保存寄存器
        for ActiveNode{vir, interval: _, phy} in self.activenodes.iter() {
            if SAVED_SET.contains(phy.as_ref().unwrap()) || FLOAT_SAVED_SET.contains(phy.as_ref().unwrap()) {
                func.used_saved(phy.as_ref().unwrap());
            }
            // 添加虚拟寄存器和物理寄存器的映射关系
            self.inactivemap.insert(vir.as_ref().unwrap().to_string(), phy.as_ref().unwrap());
        }
    }

    fn get_alloc_res(&self) -> &HashMap<String, &'static str> {
        &self.inactivemap
    }

    fn get_spilled(&self) -> &HashSet<String> {
        &self.spilled
    }
}