use crate::structures::symbol::SymbolWidth;
use crate::structures::riscv_struct::*;

impl AsmInstr {
    pub fn fetch_type(&self) -> AsmInstrType {
        match self {
            AsmInstr::Li(_) => AsmInstrType::Li,
            AsmInstr::La(_) => AsmInstrType::La,
            AsmInstr::Mv(_) => AsmInstrType::Mv,
            AsmInstr::Fmv(_, _, _) => AsmInstrType::Fmv,
            AsmInstr::Sextw(_) => AsmInstrType::Sextw,
            AsmInstr::Add(_) => AsmInstrType::Add,
            AsmInstr::Addi(_) => AsmInstrType::Addi,
            AsmInstr::Sub(_) => AsmInstrType::Sub,
            AsmInstr::Mul(_) => AsmInstrType::Mul,
            AsmInstr::Div(_) => AsmInstrType::Div,
            AsmInstr::Rem(_) => AsmInstrType::Rem,
            AsmInstr::Xori(_) => AsmInstrType::Xori,
            AsmInstr::Slli(_) => AsmInstrType::Slli,
            AsmInstr::Srli(_) => AsmInstrType::Srli,
            AsmInstr::Srai(_) => AsmInstrType::Srai,
            AsmInstr::Fadd(_) => AsmInstrType::Fadd,
            AsmInstr::Fsub(_) => AsmInstrType::Fsub,
            AsmInstr::Fmul(_) => AsmInstrType::Fmul,
            AsmInstr::Fdiv(_) => AsmInstrType::Fdiv,
            AsmInstr::Fcvt(_, _, _) => AsmInstrType::Fcvt,
            AsmInstr::Slt(_) => AsmInstrType::Slt,
            AsmInstr::Slti(_) => AsmInstrType::Slti,
            AsmInstr::Sgt(_) => AsmInstrType::Sgt,
            AsmInstr::Seqz(_) => AsmInstrType::Seqz,
            AsmInstr::Snez(_) => AsmInstrType::Snez,
            AsmInstr::Flt(_) => AsmInstrType::Flt,
            AsmInstr::Fle(_) => AsmInstrType::Fle,
            AsmInstr::Feq(_) => AsmInstrType::Feq,
            AsmInstr::Store(_, _) => AsmInstrType::Store,
            AsmInstr::Load(_, _) => AsmInstrType::Load,
            AsmInstr::Branch(_) => AsmInstrType::Branch,
            AsmInstr::Jump(_) => AsmInstrType::Jump,
            AsmInstr::Ret() => AsmInstrType::Ret,
            AsmInstr::Call(_, _, _, _) => AsmInstrType::Call,
        }
    }

    #[allow(unused)]
    pub fn fetch_info(&self) -> (AsmInstrType, Vec<&str>, Vec<isize>, Vec<SymbolWidth>) {
        match self {
            AsmInstr::Li(bin) | AsmInstr::La(bin) | AsmInstr::Mv(bin) | 
            AsmInstr::Sextw(bin) | AsmInstr::Seqz(bin) | AsmInstr::Snez(bin) => {
                let (str_vec, num_vec, ty_vec) = bin.fetch_info();
                (self.fetch_type(), str_vec, num_vec, ty_vec)
            },
            AsmInstr::Fmv(bin, dst, src) => {
                let (str_vec, num_vec, _) = bin.fetch_info();
                let ty_vec = vec!(dst.clone(), src.clone());
                (self.fetch_type(), str_vec, num_vec, ty_vec)
            },
            AsmInstr::Fcvt(bin, dst, src) => {
                let (str_vec, num_vec, _) = bin.fetch_info();
                let ty_vec = vec!(dst.clone(), src.clone());
                (self.fetch_type(), str_vec, num_vec, ty_vec)
            },
            AsmInstr::Addi(tri) | AsmInstr::Add(tri) | AsmInstr::Sub(tri) |
            AsmInstr::Mul(tri) | AsmInstr::Div(tri) | AsmInstr::Rem(tri) |
            AsmInstr::Slli(tri) | AsmInstr::Srli(tri) | AsmInstr::Srai(tri) |
            AsmInstr::Xori(tri) | AsmInstr::Fadd(tri) | AsmInstr::Fsub(tri) |
            AsmInstr::Fmul(tri) | AsmInstr::Fdiv(tri) | AsmInstr::Slt(tri) |
            AsmInstr::Slti(tri) | AsmInstr::Sgt(tri) | AsmInstr::Flt(tri) |
            AsmInstr::Fle(tri) | AsmInstr::Feq(tri) => {
                let (str_vec, num_vec, ty_vec) = tri.fetch_info();
                (self.fetch_type(), str_vec, num_vec, ty_vec)
            },
            AsmInstr::Branch(cond_tri) => {
                let (str_vec, num_vec, ty_vec) = cond_tri.fetch_info();
                (AsmInstrType::Branch, str_vec, num_vec, ty_vec)
            },
            AsmInstr::Store(mem, prefix) => {
                let (mut str_vec, num_vec, ty_vec) = mem.fetch_info();
                str_vec.push(prefix);
                (AsmInstrType::Store, str_vec, num_vec, ty_vec)
            },
            AsmInstr::Load(mem, prefix) => {
                let (mut str_vec, num_vec, ty_vec) = mem.fetch_info();
                str_vec.push(prefix);
                (AsmInstrType::Load, str_vec, num_vec, ty_vec)
            },
            AsmInstr::Jump(dst) => {
                (AsmInstrType::Jump, vec!(dst.as_str()), vec!(), vec!())
            },
            AsmInstr::Ret() => {
                (AsmInstrType::Ret, vec!(), vec!(), vec!())
            }
            AsmInstr::Call(ret_val, func_name, params, ty_vec) => {
                let mut str_vec = vec!(ret_val.as_str(), func_name.as_str());
                let mut param_str = params.iter().map(|s| s.as_str()).collect::<Vec<_>>();
                str_vec.append(&mut param_str);
                (AsmInstrType::Call, str_vec, vec!(), ty_vec.clone())
            }, // AsmInstr::_
        } // match
    } // fn
} // impl

impl BinInstr {
    fn fetch_info(&self) -> (Vec<&str>, Vec<isize>, Vec<SymbolWidth>) {
        (vec!(self.dst.as_str(), self.src.as_str()), vec!(), vec!())
    }
}

impl CondTriInstr {
    fn fetch_info(&self) -> (Vec<&str>, Vec<isize>, Vec<SymbolWidth>) {
        let (mut str_vec, num_vec, ty_vec) = self.tri.fetch_info();
        str_vec.insert(0, self.cond.as_str());
        (str_vec, num_vec, ty_vec)
    }
}

impl TriInstr {
    fn fetch_info(&self) -> (Vec<&str>, Vec<isize>, Vec<SymbolWidth>) {
        if let Some(w) = self.width {
            (vec!(self.dst.as_str(), self.op1.as_str(), self.op2.as_str()), vec!(w), vec!())
        } else {
            (vec!(self.dst.as_str(), self.op1.as_str(), self.op2.as_str()), vec!(), vec!())
        }
    }
}

impl MemInstr {
    fn fetch_info(&self) -> (Vec<&str>, Vec<isize>, Vec<SymbolWidth>) {
        (vec!(self.val.as_str(), self.base.as_str(), self.offset.as_str()), vec!(self.width), vec!())
    }
}



