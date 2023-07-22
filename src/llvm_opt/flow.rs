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
        match self {
            Instruction::Add(bin_op)
            | Instruction::Sub(bin_op)
            | Instruction::Mul(bin_op)
            | Instruction::Sdiv(bin_op)
            | Instruction::Srem(bin_op)
            | Instruction::Fadd(bin_op)
            | Instruction::Fsub(bin_op)
            | Instruction::Fmul(bin_op)
            | Instruction::Fdiv(bin_op) => bin_op.flow_info(),
            Instruction::Cmp(_, bin_op)
            | Instruction::Fcmp(_, bin_op) => bin_op.flow_info(),
            Instruction::ZeroExt(castop)
            | Instruction::I32ToFloat(castop)
            | Instruction::FloatToI32(castop) => castop.flow_info(),
            Instruction::Phi(res, _, candidates) => {
                let src: Vec<&str> = candidates.iter().map(|x| x.0.as_str()).collect();
                (Some(res.as_str()), src)
            }
            Instruction::Alloca { res, .. } => (Some(res.as_str()), vec![]),
            Instruction::Store { value, ptr, .. } => {
                if ptr.contains("@") {
                    (None, vec![value.as_str(), ptr.as_str()])
                } else {
                    (Some(ptr.as_str()), vec![value.as_str()])
                }
            }
            Instruction::Load { res, ptr, .. } => (Some(res.as_str()), vec![ptr.as_str()]),
            Instruction::Call(_, _, _, params) => {
                let src: Vec<&str> = params.iter().map(|x| x.0.as_str()).collect();
                (None, src)
            }
            Instruction::GetElemPtr(dst, _, ptr, idx) => {
                let mut src = vec![dst.as_str(), ptr.as_str()];
                let idx_src: Vec<&str> = idx.iter().map(|x| x.as_str()).collect();
                src.extend(idx_src);
                (None, src)
            }
            Instruction::BitCast(res, _, val, _) => (Some(res.as_str()), vec![val.as_str()]),
            Instruction::Comment(_) => (None, vec![]),
            Instruction::Ret(_, val) => {
                let src: Vec<&str> = val.iter().map(|v| v.as_str()).collect();
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
