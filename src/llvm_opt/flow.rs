use std::collections::{HashMap, HashSet, VecDeque};
use crate::structures::llvm_struct::{BinaryOp, CastOp, Instruction, Block, FuncDef};

/// 流向图数据项，能够获取Item的自身标识和关联标识
/// 若T实现了FlowItem trait，则流向图FlowGraph在一个或多个Vec<T>上生成
pub trait FlowItem {
    fn flow_info(&self) -> (Option<&str>, Vec<&str>);
}

/// 取出BinaryOp的自身标识，并取出操作数关联标识
impl FlowItem for BinaryOp {
    fn flow_info(&self) -> (Option<&str>, Vec<&str>) {
        (
            Some(self.res.as_str()),
            vec![self.op1.as_str(), self.op2.as_str()],
        )
    }
}

/// 取出ConverOp的自身标识，并取出操作数关联标识
impl FlowItem for CastOp {
    fn flow_info(&self) -> (Option<&str>, Vec<&str>) {
        (Some(self.res.as_str()), vec![self.val.as_str()])
    }
}

/// 分类取出Instr的自身标识和关联标识
/// 反映的是自身对关联标识的使用情况
impl FlowItem for Instruction {
    fn flow_info(&self) -> (Option<&str>, Vec<&str>) {
        match &self {
            // res使用candidates的值
            Instruction::Add(bin_op) => bin_op.flow_info(),
            Instruction::Sub(bin_op) => bin_op.flow_info(),
            Instruction::Mul(bin_op) => bin_op.flow_info(),
            Instruction::Sdiv(bin_op) => bin_op.flow_info(),
            Instruction::Srem(bin_op) => bin_op.flow_info(),
            Instruction::Fadd(bin_op) => bin_op.flow_info(),
            Instruction::Fsub(bin_op) => bin_op.flow_info(),
            Instruction::Fmul(bin_op) => bin_op.flow_info(),
            Instruction::Fdiv(bin_op) => bin_op.flow_info(),
            Instruction::Cmp(_, bin_op) => bin_op.flow_info(),
            Instruction::Fcmp(_, bin_op) => bin_op.flow_info(),
            Instruction::ZeroExt(conver_op) => conver_op.flow_info(),
            Instruction::I32ToFloat(conver_op) => conver_op.flow_info(),
            Instruction::FloatToI32(conver_op) => conver_op.flow_info(),
            Instruction::Phi(res, _, candidates) => {
                let src: Vec<&str> = candidates.iter().map(|x| x.0.as_str()).collect();
                (Some(res.as_str()), src)
            }
            // Alloca不使用其他值，只有被其他值使用
            Instruction::Alloca {
                res,
                ty: _,
                len: _
            } => (Some(res.as_str()), vec![]),
            // 将ptr分为全局和局部处理
            // 全局ptr始终保持活跃，因此Store不应当剔除
            // 对于局部ptr，value存到ptr中，也就是ptr使用了value
            Instruction::Store {
                ty: _,
                value,
                ptr,
                len: _,
            } => {
                if ptr.contains("@") {
                    (None, vec![value.as_str(), ptr.as_str()])
                } else {
                    (Some(ptr.as_str()), vec![value.as_str()])
                }
            }
            // result使用了ptr的值
            Instruction::Load {
                res,
                ty: _,
                ptr,
                len: _,
            } => (Some(res.as_str()), vec![ptr.as_str()]),
            // Call必被执行，因此自身为None，使用params中的值
            Instruction::Call(_, _, _, params) => {
                let src: Vec<&str> = params.iter().map(|x| x.0.as_str()).collect();
                (None, src)
            }
            // 由于指针所指的值可能被其他标号指向
            // 因此必须保证GetElemPtr及其下游标号活跃
            // 因此自身为None，使用dst,ptr和idx的值
            Instruction::GetElemPtr(dst, _, ptr, idx) => {
                let mut src = vec![dst.as_str()];
                src.push(ptr.as_str());
                let mut idx_src: Vec<&str> = idx.iter().map(|x| x.as_str()).collect();
                src.append(&mut idx_src);
                (None, src)
            }
            // res使用val的值
            Instruction::BitCast(res, _, val, _) => (Some(res.as_str()), vec![val.as_str()]),
            Instruction::Comment(_) => (None, vec![]),
            // 终结符必被执行，使用所有值
            Instruction::Ret(_, val) => {
                let mut src: Vec<&str> = vec![];
                if let Some(v) = val {
                    src.push(v.as_str());
                }
                (None, src)
            }
            Instruction::Br(cond, _, _) => {
                if let Some(cond_val) = cond {
                    (None, vec![cond_val.as_str()])
                } else {
                    (None, vec![])
                }
            }
        }
    }
}

/// 处理基本块，取出基本块标号，并根据TerInstr类型取出关联标识
impl FlowItem for Block {
    fn flow_info(&self) -> (Option<&str>, Vec<&str>) {
        if let Some(Instruction::Br(_, label1, label2)) = &self.ter_ins {
            let mut src: Vec<&str> = vec![];
            src.push(label1.as_str());
            if let Some(label2_val) = label2 {
                src.push(label2_val.as_str())
            }
            let src = src.into_iter().filter(|x| x != &self.block_label).collect();
            (Some(self.block_label.as_str()), src)
        } else {
            (Some(self.block_label.as_str()), vec![])
        }
    }
}

/// 流图模块
pub mod flow_graph {
    use super::*;
    /// 映射标号的关联标号
    type RelateMap = HashMap<String, HashSet<String>>;
    /// 从FlowItem中建立映射关系<br>
    /// 返回两个HashMap，存储值均为集合，集合中的值为键值的关联类型<br>
    /// 第一个集合包含自身决定其活跃性的标识<br>
    /// 第二个集合包含决定自身活跃性的标识<br>
    pub fn build_map(items: Vec<&impl FlowItem>) -> (RelateMap, RelateMap) {
        let mut succs = HashMap::new();
        let mut preds = HashMap::new();
        for item in items.iter() {
            let (self_label, associate_labels) = item.flow_info();
            let self_id = self_label.unwrap_or("");

            if !succs.contains_key(self_id) {
                succs.insert(String::from(self_id), HashSet::new());
            }
            if !preds.contains_key(self_id) {
                preds.insert(String::from(self_id), HashSet::new());
            }
            for associate in associate_labels.into_iter() {
                succs
                    .get_mut(self_id)
                    .unwrap()
                    .insert(String::from(associate));

                if !preds.contains_key(associate) {
                    preds.insert(String::from(associate), HashSet::new());
                }

                preds
                    .get_mut(associate)
                    .unwrap()
                    .insert(String::from(self_id));
            }
        }
        (succs, preds)
    }

    /// 根据映射关系计算活跃的标识
    pub fn calc_active(succs: &RelateMap, mut preds: RelateMap) -> HashSet<String> {
        let mut deque: VecDeque<String> = preds
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| String::from(k))
            .collect();
        let mut traversed: HashSet<String> = deque.iter().map(|x| String::from(x)).collect();

        while !deque.is_empty() {
            let this_label = deque.pop_front().unwrap();
            let succ_set = succs.get(&this_label);
            if succ_set.is_none() {
                continue;
            }
            let succ_set = succ_set.unwrap();

            for i in succ_set.iter() {
                let pred_set = preds.get_mut(i);
                if pred_set.is_none() {
                    continue;
                }
                let pred_set = pred_set.unwrap();
                pred_set.remove(&this_label);
                if pred_set.is_empty() && !traversed.contains(i) {
                    traversed.insert(String::from(i));
                    deque.push_back(String::from(i));
                }
            }
        }

        preds
            .into_iter()
            .filter(|(_, v)| !v.is_empty())
            .map(|(k, _)| k)
            .collect()
    } // calc_active
}

impl FuncDef {
    pub fn make_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect()
    }

    #[allow(unused)]
    pub fn make_func_instrs(&self) -> Vec<&Instruction> {
        self.blocks
            .iter()
            .map(|b| b.make_block_instrs())
            .flatten()
            .collect()
    }
}

impl Block {
    pub fn make_block_instrs(&self) -> Vec<&Instruction> {
        let mut res = vec![];
        for p in self.phi_ins.iter() {
            res.push(p);
        }

        for i in self.instrs.iter() {
            res.push(i);
        }

        if let Some(t) = &self.ter_ins {
            res.push(t);
        }
        res
    }
}
