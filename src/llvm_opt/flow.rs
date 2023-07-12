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

/// 取出CastOp的自身标识，并取出操作数关联标识
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

        for i in self.nor_ins.iter() {
            res.push(i);
        }

        if let Some(t) = &self.ter_ins {
            res.push(t);
        }
        res
    }
}
