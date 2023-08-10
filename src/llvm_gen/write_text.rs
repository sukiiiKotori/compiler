use std::io::*;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::structures::writetext_trait::*;

impl WriteText for LLVMProgram {
    //递归遍历解析LLVM结构体
    fn writetext(&self, output: &mut impl Write) { 
        //输出全局变量的文本
        self.global_var.iter().for_each(|it| it.writetext(output));
        write!(output, "\n").unwrap();

        //输出函数声明的文本
        self.func_decl.iter().for_each(|it| it.writetext(output));
        write!(output, "\n").unwrap();

        //输出函数定义的文本
        self.func_def.iter().for_each(|it| it.writetext(output));
    }
}

impl WriteText for GlobalVar {
    // LLVM的第一个struct：GlobalVar
    // 该struct无需递归调用，在LLVM IR起始处生成即可
    fn writetext(&self, output: &mut impl Write) { 
        write!(output, "@{} = global", self.var_name).unwrap();
        match &(self.var_type.width) {
            SymbolWidth::I32 => {
                write!(output, " {}", self.var_type.get_typename()).unwrap();
                //如果没有赋值，则全局变量默认赋0
                if self.init_values.is_empty() {
                    write!(output, " 0").unwrap();
                } else {
                    write!(output, " {}", self.init_values.first().unwrap()).unwrap();
                }
                write!(output, ", align 4").unwrap();
            },
            SymbolWidth::Float => {
                write!(output, " {}", self.var_type.get_typename()).unwrap();
                if self.init_values.is_empty() {
                    write!(output, " 0.0").unwrap();
                } else {
                    write!(output, " {}", self.init_values.first().unwrap()).unwrap();
                }
                write!(output, ", align 4").unwrap();
            },
            // 特别的，对于数组初始化，需要调用
            SymbolWidth::Arr{tar: _, dims} => {
                let mut pos: Vec<i32> = vec!();
                GlobalVar::write_arr(output, dims, &self.var_type, &self.init_values.iter().map(|x| x.to_string()).collect(), &mut pos);
            },
            _ => panic!("Don't support"),
        }
        write!(output, "\n").unwrap();
    }  
}

impl WriteText for FuncDecl {
    fn writetext(&self, output: &mut impl Write) {
        //打印返回值类型
        write!(output, "declare {} @{}(", self.func_type.get_typename(), self.func_name).unwrap();
        //遍历vec，将函数的形参类型全部打印出来
        self.param_types.iter().enumerate().for_each(|(i, param_type)| {
            if i == 0 {
                write!(output, "{} noundef", param_type.get_typename()).unwrap();
            } else {
                write!(output, ", {} noundef", param_type.get_typename()).unwrap();
            }
        });

        write!(output, ")\n").unwrap();
    }
}

impl WriteText for FuncDef {
    //函数体的打印
    fn writetext(&self, output: &mut impl Write) {
        //打印函数体的头部
        write!(output, "define {} {}(", self.func_type.get_typename(), self.func_name).unwrap();

        self.params.iter().enumerate().for_each(|(i, param)| {
            if i == 0 {
                param.writetext(output);
            } else {
                write!(output, ", ").unwrap();
                param.writetext(output);
            }
        });

        write!(output, ") {{\n").unwrap();
        
        let blocks = &self.blocks;

        blocks.iter().enumerate().for_each(|(i, block)| {
            //获取局部变量
            let localvars = match i {
                0 => Some(&self.local_vars),
                _ => None,
            };
            match block.ter_ins {
                //若有终结指令，则调用block的writetext方法
                Some(_) => {
                    block.writetext(output, localvars);
                    if i != blocks.len() - 1 {
                        write!(output, "\n").unwrap();
                    }
                },
                None => {
                    //如果块中没有终结指令但是有指令，则语法错误
                    if block.nor_ins.len() != 0 {
                        panic!();
                    }
                    // 如果返回语句后为空块，则直接跳过不写入
                    if block.block_label.contains("ret_then") {                         
                        return
                    }
                    // 如果没有返回值，则直接 ret void
                    else if self.func_type.width == SymbolWidth::Void {
                        block.writetext(output, localvars);
                        write!(output, "  ret void\n").unwrap();
                    }
                    // 正常的函数
                    else {
                        block.writetext(output, localvars);
                        if self.func_type.width == SymbolWidth::I32 {
                            write!(output, "  ret i32 0\n").unwrap();
                        } else if self.func_type.width == SymbolWidth::Float {
                            write!(output, "  ret float 0.0\n").unwrap();
                        } else {
                            panic!("Don't support");
                        }
                    }
                }
            }
        });
        
        write!(output, "}}\n\n").unwrap();
    }
}

impl WriteText for Param {
    fn writetext(&self, output: &mut impl Write) {
        write!(output, "{} noundef {}", self.param_type.get_typename(), self.param_name).unwrap();
    }
}

impl Block {
    fn writetext(&self, output: &mut impl Write, localvars: Option<&Vec<LocalVar>>) {
        write!(output, "{}:\n", self.block_label).unwrap();

        //如果需要在块首部
        if let Some(instrs) = localvars {
            for alloc in instrs.iter() {
                alloc.ins.writetext(output);
            }
            if !instrs.is_empty() {
                write!(output, "\n").unwrap();
            }
        }

        for instr in self.nor_ins.iter() {
            instr.writetext(output);
        }
        match &self.ter_ins {
            Some(ter) => ter.writetext(output),
            None => {},
        }
    }
}

impl WriteText for Instruction {
    //对于各个指令的writetext，具体需要调用每种指令的writetext
    fn writetext(&self, output: &mut impl Write) {
        //1、对于二元指令，首先输出每种具体的指令头，例如 result = 'op', 然后调用公共的writetext函数
        fn bin_op_write(output: &mut impl Write, bin_op: &BinaryOp, name: &str) {
            write!(output, "  {} = {} ", bin_op.res, name).unwrap();
            bin_op.writetext(output);
        }
        match self {
            // 1、二元指令
            Instruction::Add(bin_op) => bin_op_write(output, bin_op, "add"),
            Instruction::Sub(bin_op) => bin_op_write(output, bin_op, "sub"),
            Instruction::Mul(bin_op) => bin_op_write(output, bin_op, "mul"),
            Instruction::Sdiv(bin_op) => bin_op_write(output, bin_op, "sdiv"),
            Instruction::Srem(bin_op) => bin_op_write(output, bin_op, "srem"),
            Instruction::Fadd(bin_op) => bin_op_write(output, bin_op, "fadd"),
            Instruction::Fsub(bin_op) => bin_op_write(output, bin_op, "fsub"),
            Instruction::Fmul(bin_op) => bin_op_write(output, bin_op, "fmul"),
            Instruction::Fdiv(bin_op) => bin_op_write(output, bin_op, "fdiv"),
            Instruction::Cmp(cond, bin_op) => {
                write!(output, "  {} = icmp {} ", bin_op.res, cond).unwrap();
                bin_op.writetext(output);
            },
            Instruction::Fcmp(cond, bin_op) => {
                write!(output, "  {} = fcmp {} ", bin_op.res, cond).unwrap();
                bin_op.writetext(output);
            },
            Instruction::ZeroExt(conver_op) => {
                write!(output, "  {} = zext ", conver_op.res).unwrap();   
                conver_op.writetext(output);
            },
            Instruction::I32ToFloat(conver_op) => {
                write!(output, "  {} = sitofp ", conver_op.res).unwrap();   
                conver_op.writetext(output);
            },
            Instruction::FloatToI32(conver_op) => {
                write!(output, "  {} = fptosi ", conver_op.res).unwrap();   
                conver_op.writetext(output);
            },
            Instruction::Phi(..) => {
                todo!()
            },
            //2、alloca指令格式为 res = alloca type, align 对齐值
            Instruction::Alloca{res, ty, len} => {
                write!(output, "  {} = alloca {}, align {}\n", res, ty.get_typename(), len).unwrap();
            },
            //3、store和load两个访存指令，格式类似
            Instruction::Store{ty, value, ptr, len} => {
                write!(output, 
                    "  store {} {}, {}* {}, align {}\n", 
                    ty.get_typename(), value, ty.get_typename(), ptr, len
                ).unwrap();
            },
            Instruction::Load{res, ty, ptr, len} => {
                write!(
                    output, 
                    "  {} = load {}, {}* {}, align {}\n", 
                    res, ty.get_typename(), ty.get_typename(), ptr, len
                ).unwrap();
            },
            //4、Call指令
            Instruction::Call(res, label, ty, params) => {
                let lf = if res.is_empty() {
                    format!("  ")
                } else {
                    format!("  {} = ", res)
                };
                write!(output, "{}call {} {}(", lf, ty.get_typename(), label).unwrap();
                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        write!(output, ", ").unwrap();
                    }
                    write!(output, "{} noundef {}", param.1.get_typename(), param.0).unwrap();
                }
                write!(output, ")\n").unwrap();
            },
            //5、用于访问数组，递归dump
            Instruction::GetElemPtr(dst, ty, ptr, idx) => {
                write!(output, "  {} = getelementptr inbounds {}, {}* {}", dst, ty.get_typename(), ty.get_typename(), ptr).unwrap();
                idx.iter().for_each(|it| write!(output, ", i32 {}", it).unwrap());
                write!(output, "\n").unwrap();
            },
            //6、类型转换
            Instruction::BitCast(res, ty, val, ty2) => {
                write!(output, "  {} = bitcast {}* {} to {}*\n", res, ty.get_typename(), val, ty2.get_typename()).unwrap();
            },
            //7、注释
            Instruction::Comment(content) => {
                write!(output, "{}", content).unwrap();
            },
            //8、函数返回
            Instruction::Ret(ty, val) => {
                match val {
                    Some(v) => write!(output, "  ret {} {}\n", ty.get_typename(), v).unwrap(),
                    None => write!(output, "  ret {}\n", ty.get_typename()).unwrap(),
                }
            },
            //9、分支
            Instruction::Br(cond, label1, label2) => {
                match label2 {
                    Some(label) => write!(output, "  br i1 {}, label %{}, label %{}\n", cond.as_ref().unwrap(), label1, label).unwrap(),
                    None => write!(output, "  br label %{}\n", label1).unwrap(),
                }
            },
        }
    }
}

impl WriteText for BinaryOp {
    //二元操作数指令
    fn writetext(&self, output: &mut impl Write) {
        write!(output, "{} {}, {}\n", self.op_type.get_typename(), self.op1, self.op2).unwrap();
    }
}

impl WriteText for CastOp {
    //类型转换指令
    fn writetext(&self, output: &mut impl Write) {
        write!(output, "{} {} to {}\n", self.type_1.get_typename(), self.val, self.type_2.get_typename()).unwrap();
    }
}

