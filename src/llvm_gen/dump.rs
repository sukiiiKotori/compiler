use std::io;
use std::error::Error;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;

pub trait Dump {
    type Out;
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>>;
}

impl Dump for LLVMProgram {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> { 
        for var in self.global_var.iter() {
            var.dump(output)?;
        }
        if !self.global_var.is_empty() {
            write!(output, "\n")?;
        }

        for func in self.func_decl.iter() {
            func.dump(output)?;
        }
        if !self.func_decl.is_empty() {
            write!(output, "\n")?;
        }

        for func in self.func_def.iter() {
            func.dump(output)?;
        }
        Ok(())
    }
}

impl GlobalVar {
    fn get_range(dims: &Vec<i32>, pos: &Vec<i32>) -> (i32, i32) {
        let mut start = 0;
        let mut left_size = dims.iter().fold(1, |acc, x| acc*x);
        for cnt in 0..pos.len() {
            left_size /= dims[cnt];
            start += pos[cnt] * left_size;
        }
        let end = start+dims.iter().skip(pos.len()).fold(1, |acc, x| acc*x);
        (start, end)
    }

    fn all_is_zero(vals: &Vec<String>, start: i32, end: i32) -> bool {
        if vals.is_empty() {
            return true;
        }
        let start = start as usize;
        let end = end as usize;
        vals[start..end].iter().all(|x| x == "0")
    }

    fn dump_arr_init(output: &mut impl io::Write, dims: &Vec<i32>, ty: &SymbolType, vals: &Vec<String>, pos: &mut Vec<i32>) {
        let (start, end) = GlobalVar::get_range(dims, pos);
        if pos.len() == dims.len() {
            match &ty.width {
                SymbolWidth::Arr{tar, dims: _} => write!(output, "{} {}", tar.get_type(), vals[start as usize]).unwrap(),
                _ => panic!("Should not appear"),
            }
        } else {
            match &ty.width {
                SymbolWidth::Arr{tar, dims} => write!(output, "{} ", dims_name(tar, &dims[pos.len()..])).unwrap(),
                _ => panic!("Should not appear"),
            }
            if vals.is_empty() || GlobalVar::all_is_zero(vals, start, end) {
                write!(output, " zeroinitializer").unwrap();
            } else {
                write!(output, "[").unwrap();
                for cnt in 0..dims[pos.len()] {
                    if cnt != 0 {
                        write!(output, ", ").unwrap();
                    }
                    pos.push(cnt);
                    GlobalVar::dump_arr_init(output, dims, ty, vals, pos);
                    pos.pop();
                }
                write!(output, "]").unwrap();
            }
        }
    }
}

impl Dump for GlobalVar {
    type Out = ();

    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> { 
        write!(output, "@{} = global", self.var_name)?;
        match &self.var_type.width {
            SymbolWidth::I32 => {
                write!(output, " {}", self.var_type.get_type())?;
                if self.init_num.len() < 1 {
                    write!(output, " 0")?;
                } else {
                    write!(output, " {}", self.init_num[0].init_val)?;
                }
                write!(output, ", align 4")?;
            },
            SymbolWidth::Float => {
                write!(output, " {}", self.var_type.get_type())?;
                if self.init_num.len() < 1 {
                    write!(output, " 0.0")?;
                } else {
                    write!(output, " {}", self.init_num[0].init_val)?;
                }
                write!(output, ", align 4")?;
            },
            SymbolWidth::Arr{tar: _, dims} => {
                let mut pos: Vec<i32> = vec!();
                GlobalVar::dump_arr_init(output, dims, &self.var_type, &self.init_num.iter().map(|x| x.init_val.clone()).collect(), &mut pos);
            },
            _ => panic!("Should not appear"),
        }
        write!(output, "\n")?;
        Ok(())
    }  
}

impl Dump for FuncDecl {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "declare {} @{}(", self.func_type.get_type(), self.func_name)?;
        for cnt in 0..self.param_types.len() {
            if cnt != 0 {
                write!(output, ", ")?;
            }
            write!(output, "{} noundef", self.param_types[cnt].get_type())?;
        }
        write!(output, ")\n")?;
        Ok(())
    }
}

impl Dump for FuncDef {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "define")?;
        let type_name = &self.func_type.get_type();
        write!(output, " {}", type_name)?;
        write!(output, " {}(", self.func_name)?;
        for i in 0..self.params.len() {
            if i != 0 {
                write!(output, ", ")?;
            }
            self.params[i].dump(output)?;
        }
        write!(output, ")")?;
        write!(output, " {{\n")?;
        
        let blocks = &self.blocks;
        for cnt in 0..blocks.len() {
            let allocs: Option<&Vec<LocalVar>>;
            if cnt == 0 {
                allocs = Some(&self.local_vars);
            } else {
                allocs = None;
            }

            let block = &blocks[cnt];
            if block.ter_ins.is_some() {
                block.dump(output, allocs)?;
                if cnt != blocks.len() - 1 {
                    write!(output, "\n")?;
                }
            } else {
                if block.block_label.contains("ret_then") { // 去除最后一个返回语句跟随的空基本块
                    continue;
                }
                if self.func_type.width == SymbolWidth::Void {
                    block.dump(output, allocs)?;
                    write!(output, "  ret void\n")?;
                } else {
                    if block.nor_ins.len() != 0 || block.phi_ins.len() != 0 {
                        panic!("A meaningful basic block without terminate instruction");
                    }
                    block.dump(output, allocs)?;
                    if self.func_type.width == SymbolWidth::I32 {
                        write!(output, "  ret i32 0\n")?;
                    } else if self.func_type.width == SymbolWidth::Float {
                        write!(output, "  ret float 0.0\n")?;
                    } else {
                        panic!("{} TODO", self.func_type.get_type());
                    }
                }
            }
        }
        write!(output, "}}\n\n")?;
        Ok(())
    }
}

impl Dump for Param {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{} noundef {}", self.param_type.get_type(), self.param_name)?;
        Ok(())
    }
}

impl Block {
    fn dump(&self, output: &mut impl io::Write, allocs: Option<&Vec<LocalVar>>) -> Result<(), Box<dyn Error>> {
        write!(output, "{}:\n", self.block_label)?;
        for phi in self.phi_ins.iter() {
            phi.dump(output)?;
        }

        if let Some(instrs) = allocs {
            for alloc in instrs.iter() {
                alloc.ins.dump(output)?;
            }
            if !instrs.is_empty() {
                write!(output, "\n")?;
            }
        }

        for instr in self.nor_ins.iter() {
            instr.dump(output)?;
        }
        match &self.ter_ins {
            Some(ter) => ter.dump(output)?,
            None => {},
        }
        Ok(())
    }
}

impl Dump for Instruction {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        match &self {
            Instruction::Add(bin_op) => {
                write!(output, "  {} = add ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Sub(bin_op) => {
                write!(output, "  {} = sub ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Mul(bin_op) => {
                write!(output, "  {} = mul ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Sdiv(bin_op) => {
                write!(output, "  {} = sdiv ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Srem(bin_op) => {
                write!(output, "  {} = srem ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Fadd(bin_op) => {
                write!(output, "  {} = fadd ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Fsub(bin_op) => {
                write!(output, "  {} = fsub ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Fmul(bin_op) => {
                write!(output, "  {} = fmul ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Fdiv(bin_op) => {
                write!(output, "  {} = fdiv ", bin_op.res)?;
                bin_op.dump(output)?;
            },
            Instruction::Cmp(cond, bin_op) => {
                write!(output, "  {} = icmp {} ", bin_op.res, cond)?;
                bin_op.dump(output)?;
            },
            Instruction::Fcmp(cond, bin_op) => {
                write!(output, "  {} = fcmp {} ", bin_op.res, cond)?;
                bin_op.dump(output)?;
            },
            Instruction::ZeroExt(conver_op) => {
                write!(output, "  {} = zext ", conver_op.res)?;   
                conver_op.dump(output)?;
            },
            Instruction::I32ToFloat(conver_op) => {
                write!(output, "  {} = sitofp ", conver_op.res)?;   
                conver_op.dump(output)?;
            },
            Instruction::FloatToI32(conver_op) => {
                write!(output, "  {} = fptosi ", conver_op.res)?;   
                conver_op.dump(output)?;
            },
            Instruction::Phi(res, ty, candidates) => {
                write!(output, "  {} = phi {} ", res, ty.get_type())?;
                for i in 0..candidates.len() {
                    if i != 0 {
                        write!(output, ", ")?;
                    }
                    write!(output, "[{}, %{}]", candidates[i].0, candidates[i].1)?;
                }
                write!(output, "\n")?;
            },
            Instruction::Alloca{res, ty, len} => {
                write!(output, "  {} = alloca {}, align {}\n", res, ty.get_type(), len)?;
            },
            Instruction::Store{ty, value, ptr, len} => {
                write!(
                    output, 
                    "  store {} {}, {}* {}, align {}\n", 
                    ty.get_type(), value, ty.get_type(), ptr, len
                )?;
            },
            Instruction::Load{res, ty, ptr, len} => {
                write!(
                    output, 
                    "  {} = load {}, {}* {}, align {}\n", 
                    res, ty.get_type(), ty.get_type(), ptr, len
                )?;
            },
            Instruction::Call(res, label, ty, params) => {
                if res != "" {
                    write!(output, "  {} = call {} {}(", res, ty.get_type(), label)?;
                } else {
                    write!(output, "  call {} {}(", ty.get_type(), label)?;
                }
                for cnt in 0..params.len() {
                    if cnt != 0 {
                        write!(output, ", ")?;
                    }
                    write!(output, "{} noundef {}", params[cnt].1.get_type(), params[cnt].0)?;
                }
                write!(output, ")\n")?;
            },
            Instruction::GetElemPtr(dst, ty, ptr, idx) => {
                write!(output, "  {} = getelementptr inbounds {}, {}* {}", dst, ty.get_type(), ty.get_type(), ptr)?;
                for cnt in 0..idx.len() {
                    write!(output, ", i32 {}", idx[cnt])?;
                }
                write!(output, "\n")?;
            },
            Instruction::BitCast(res, ty, val, ty2) => {
                write!(output, "  {} = bitcast {}* {} to {}*\n", res, ty.get_type(), val, ty2.get_type())?;
            },
            Instruction::Comment(content) => {
                write!(output, "{}", content)?;
            },
            Instruction::Ret(ty, val) => {
                match &val {
                    Some(v) => write!(output, "  ret {} {}\n", ty.get_type(), v)?,
                    None => write!(output, "  ret {}\n", ty.get_type())?,
                }
            },
            Instruction::Br(cond, label1, label2) => {
                if let Some(false_label) = label2 {
                    let condition = cond.as_ref().unwrap();
                    write!(output, "  br i1 {}, label %{}, label %{}\n", condition, label1, false_label)?;
                } else {
                    write!(output, "  br label %{}\n", label1)?;
                }
            },
        }
        Ok(())
    }
}

impl Dump for BinaryOp {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{} {}, {}\n", self.op_type.get_type(), self.op1, self.op2)?;
        Ok(())
    }
}

impl Dump for CastOp {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{} {} to {}\n", self.type_1.get_type(), self.val, self.type_2.get_type())?;
        Ok(())
    }
}

