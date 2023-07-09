use std::error::Error;
use crate::ast::*;
use crate::structures::llvm_struct::{LLVMProgram, Instruction, InstructionType};
use crate::structures::symbol::*;
use crate::llvm_gen::scopes::{Scopes, Labels};
use crate::llvm_gen::symbol::*;

pub trait Generate {
    type Out;
    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>>;
}

/// 首先添加库函数的声明
/// 然后遍历units，调用对应的generate函数
impl Generate for SysY {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        let ty_void = SymbolType::new(SymbolWidth::Void, false);
        let ty_i32 = SymbolType::new(SymbolWidth::I32, false);
        let ty_i32_clone = ty_i32.clone();
        let ty_i32_ptr = SymbolType::new(
            SymbolWidth::Arr{tar: Box::new(ty_i32_clone),dims: vec!(-1)},
            false
        );
        let ty_i8 = SymbolType::new(SymbolWidth::I8, false);
        let ty_i8_clone = ty_i8.clone();
        let ty_i8_ptr = SymbolType::new(
            SymbolWidth::Arr{tar: Box::new(ty_i8_clone), dims: vec!(-1)},
            false
        );
        let ty_i64 = SymbolType::new(SymbolWidth::I64, false);
        let ty_i1 = SymbolType::new(SymbolWidth::I1, false);
        let ty_float = SymbolType::new(SymbolWidth::Float, false);
        let ty_float_clone = ty_float.clone();
        let ty_float_ptr = SymbolType::new(
            SymbolWidth::Arr{tar: Box::new(ty_float_clone),dims: vec!(-1)},
            false
        );

        program.push_func_decl(&ty_i32, "getint", vec!());
        scopes.push(
            labels,
            "getint",
            &ty_i32,
            &SymbolVal::Func(ty_i32.clone(), vec!()),
            None
        );

        program.push_func_decl(&ty_i32, "getch", vec!());
        scopes.push(
            labels,
            "getch",
            &ty_i32,
            &SymbolVal::Func(ty_i32.clone(), vec!()),
            None
        );

        program.push_func_decl(&ty_i32, "getarray", vec!(&ty_i32_ptr));
        scopes.push(
            labels,
            "getarray",
            &ty_i32,
            &SymbolVal::Func(ty_i32.clone(),vec!(ty_i32_ptr.clone())),
            None
        );

        program.push_func_decl(&ty_float, "getfloat", vec!());
        scopes.push(
            labels,
            "getfloat",
            &ty_float,
            &SymbolVal::Func(ty_float.clone(), vec!()),
            None
        );

        program.push_func_decl(&ty_i32, "getfarray", vec!(&ty_float_ptr));
        scopes.push(
            labels,
            "getfarray",
            &ty_i32,
            &SymbolVal::Func(ty_i32.clone(), vec!(ty_float_ptr.clone())),
            None
        );

        program.push_func_decl(&ty_void, "putint", vec!(&ty_i32));                              
        scopes.push(
            labels,
            "putint",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!(ty_i32.clone())),
            None
        );

        program.push_func_decl(&ty_void, "putch", vec!(&ty_i32));                              
        scopes.push(
            labels,
            "putch",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!(ty_i32.clone())),
            None
        );

        program.push_func_decl(&ty_void, "putarray", vec!(&ty_i32, &ty_i32_ptr));
        scopes.push(
            labels,
            "putarray",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(),
            vec!(ty_i32.clone(), ty_i32_ptr.clone())),
            None
        );

        program.push_func_decl(&ty_void, "putfloat", vec!(&ty_float));                              
        scopes.push(
            labels,
            "putfloat",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!(ty_float.clone())),
            None
        );

        program.push_func_decl(&ty_void, "putfarray", vec!(&ty_i32, &ty_float_ptr));
        scopes.push(
            labels,
            "putfarray",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!(ty_i32.clone(), ty_float_ptr.clone())),
            None
        );

        program.push_func_decl(&ty_void, "starttime", vec!());
        scopes.push(
            labels,
            "starttime",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!()),
            None
        );

        program.push_func_decl(&ty_void, "stoptime", vec!());
        scopes.push(
            labels,
            "stoptime",
            &ty_void,
            &SymbolVal::Func(ty_void.clone(), vec!()),
            None
        );

        program.push_func_decl(
            &ty_void,
            "llvm.memset.p018.i64",
            vec!(&ty_i8_ptr, &ty_i8, &ty_i64, &ty_i1)
        );
        scopes.push(
            labels,
            "llvm.memset.p018.i64",
            &ty_void,
            &SymbolVal::Func(
                ty_void.clone(),
                vec!(ty_i8_ptr.clone(),ty_i8.clone(), ty_i64.clone(), ty_i1.clone())
            ),
            None
        );

        for unit in self.units.iter() {
            unit.generate(program, scopes, labels)?;
        }
        Ok(())
    }
}

/// 进行函数定义FuncDef或声明Decl的generate
impl Generate for CompUnit {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            CompUnit::FuncDef(func_def) => func_def.generate(program, scopes, labels),
            CompUnit::Decl(decl) => decl.generate(program, scopes, labels),
        }
    }
}

/// 解析函数的信息，添加到作用域，并加入到LLVM程序
impl Generate for FuncDef {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        let func_type = self.func_type.generate()?;
        let mut all_info: Vec<(SymbolType, String, String)> = vec!();
        let mut label_info: Vec<(String, SymbolType)> = vec!();
        let mut types: Vec<SymbolType> = vec!();
        if let Some(params) = &self.params {
            for param in params.iter() {
                let (ty, id) = param.generate(program, scopes, labels)?;
                let label = labels.pop_local(&id);
                all_info.push((ty.clone(), String::from(&id), String::from(&label)));
                label_info.push((String::from(&label), ty.clone()));
                types.push(ty.clone());
            }
        }

        if let Some(label) = scopes.push(
            labels,
            self.id.as_str(),
            &func_type,
            &SymbolVal::Func(func_type.clone(), types),
            None
        ) {
            scopes.enter_function(&func_type);
            program.push_func(&func_type, label.as_str(), label_info);

            let i1_ty = SymbolType::new(SymbolWidth::I1, false);
            if let Some(replace_phi) = scopes.push(
                labels,
                "replace_phi",
                &i1_ty,
                &SymbolVal::Void,
                None
            ) {
                let ty_vec = vec!(&i1_ty);
                let str_vec = vec!(replace_phi.as_str(), "1");
                program.insert_alloc(
                    Instruction::make_instr(InstructionType::Alloca, str_vec, ty_vec),
                    "_entry"
                );
            } else {
                panic!("alloc replace_phi failed");
            }

            for (ty, id, last_label) in all_info.iter() {
                if let Some(label) = scopes.push(
                    labels,
                    id.as_str(),
                    &ty, &SymbolVal::Void,
                    None
                ) {
                    let str_vec = vec!(label.as_str(), "4");
                    let type_vec = vec!(ty);
                    program.insert_alloc(
                        Instruction::make_instr(InstructionType::Alloca, str_vec, type_vec),
                        "_entry"
                    );

                    let str_vec = vec!(last_label.as_str(), label.as_str(), "4");
                    let type_vec = vec!(ty);
                    program.push_instr(InstructionType::Store, str_vec, type_vec);
                } else {
                    panic!("Global or local scope error");
                }
            }
            self.block.generate(program, scopes, labels)?;
            scopes.exit_current_scope();
            labels.clear();
        } else {
            panic!("Multi definition of {}", self.id);
        }
        Ok(())
    }
}

/// 返回类型的克隆
impl Type {
    pub fn generate(&self) -> Result<SymbolType, Box<dyn Error>> {
        Ok(self.ty.clone())
    }
}

/// 函数声明参数，有表达式和指针的类型，分类进行操作
impl Generate for FuncFParam {
    type Out = (SymbolType, String);

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        let ty = self.ty.generate()?;
        if self.idx.is_empty() {
            Ok((ty, String::from(&self.id)))
        } else {
            let mut dims: Vec<i32> = vec!();
            for index in self.idx.iter() {
                match index {
                    Index::Exp(exp) => {
                        let (_, val) = exp.generate(program, scopes, labels)?;
                        let dim: i32 = val.parse().expect(&format!("{} is not integer", val));
                        dims.push(dim);
                    },
                    Index::Ptr(dim) => dims.push(dim.clone()),
                }
            }
            let arr_ty = SymbolType::new(
                SymbolWidth::Arr{tar: Box::new(ty), dims: dims},
                false
            );
            Ok((arr_ty, String::from(&self.id)))
        }
    }
}

/// 对每个item进行generate
impl Generate for Block {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        for item in self.items.iter() {
            item.generate(program, scopes, labels)?;
        }
        Ok(())
    }
}

/// Item分为Decl和Stmt
impl Generate for BlockItem {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            BlockItem::Decl(decl) => decl.generate(program, scopes, labels),
            BlockItem::Stmt(stmt) => stmt.generate(program, scopes, labels),
        }
    }
}

/// 根据语句的分类进行generate
/// 执行其不同的功能
impl Generate for Stmt {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            Stmt::Assign(assign) => assign.generate(program, scopes, labels),
            Stmt::Exp(exp_option) => {
                if let Some(exp) = exp_option {
                    exp.generate(program, scopes, labels)?;
                }
                Ok(())
            },
            Stmt::Block(block) => {
                scopes.enter_basis_block();
                block.generate(program, scopes, labels)?;
                scopes.exit_current_scope();
                Ok(())
            },
            Stmt::Return(ret) => {
                let ret_then = labels.pop_block("ret_then");
                ret.generate(program, scopes, labels)?;
                program.push_bb(ret_then.as_str(), scopes);
                Ok(())
            }
            Stmt::Break => {
                if let Some(end) = scopes.get_while_end() {
                    let str_vec = vec!("", end.as_str(), "");
                    let ty_vec = vec!();
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                    let break_then = labels.pop_block("break_then");
                    program.push_bb(break_then.as_str(), scopes);
                    Ok(())
                } else {
                    panic!("Break appears in non-while scope");
                }
            },
            Stmt::Continue => {
                if let Some(entry) = scopes.get_while_entry() {
                    let str_vec = vec!("", entry.as_str(), "");
                    let ty_vec = vec!();
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                    let continue_then = labels.pop_block("continue_then");
                    program.push_bb(continue_then.as_str(), scopes);
                    Ok(())
                } else {
                    panic!("Continue appears in non-while scope");
                }
            },
            Stmt::If{exp, stmt1, stmt2} => {
                let (sym_type, res) = exp.generate(program, scopes, labels)?;
                let res = type_conver(
                    program,
                    labels,
                    res,
                    &sym_type,
                    &SymbolType::new(SymbolWidth::I1, false)
                );

                if let Some(false_stmt) = stmt2 {
                    let if_then = labels.pop_block("if_then");
                    let if_else = labels.pop_block("if_else");
                    let if_end = labels.pop_block("if_end");

                    let ty_vec = vec!();
                    let str_vec = vec!(res.as_str(), if_then.as_str(), if_else.as_str());
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    
                    scopes.enter_if_scope();
                    program.push_bb(if_then.as_str(), scopes);
                    stmt1.generate(program, scopes, labels)?;
                    let ty_vec = vec!();
                    let str_vec = vec!("", if_end.as_str(), "");
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    scopes.exit_current_scope();

                    scopes.enter_if_scope();
                    program.push_bb(if_else.as_str(), scopes);
                    false_stmt.generate(program, scopes, labels)?;
                    let ty_vec = vec!();
                    let str_vec = vec!("", if_end.as_str(), "");
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    scopes.exit_current_scope();

                    program.push_bb(if_end.as_str(), scopes);
                } else {
                    let if_then = labels.pop_block("if_then");
                    let if_end = labels.pop_block("if_end");

                    let ty_vec = vec!();
                    let str_vec = vec!(res.as_str(), if_then.as_str(), if_end.as_str());
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    
                    scopes.enter_if_scope();
                    program.push_bb(if_then.as_str(), scopes);
                    stmt1.generate(program, scopes, labels)?;
                    let ty_vec = vec!();
                    let str_vec = vec!("", if_end.as_str(), "");
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    scopes.exit_current_scope();
                    program.push_bb(if_end.as_str(), scopes);
                }
                Ok(())
            },
            Stmt::While{exp, stmt} => {
                let while_entry = labels.pop_block("while_entry");
                let while_body = labels.pop_block("while_body");
                let while_end = labels.pop_block("while_end");
                
                let ty_vec = vec!();
                let str_vec = vec!("", while_entry.as_str(), "");
                program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                // while_entry
                program.push_bb(while_entry.as_str(), scopes);
                let (ty, res) = exp.generate(program, scopes, labels)?;
                let cond = type_conver(
                    program,
                    labels,
                    res,
                    &ty,
                    &SymbolType::new(SymbolWidth::I1, false)
                );
                
                let ty_vec = vec!();
                let str_vec = vec!(cond.as_str(), while_body.as_str(), while_end.as_str());
                program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                // while_body
                scopes.enter_while_scope(&while_entry, &while_end);
                program.push_bb(while_body.as_str(), scopes);
                stmt.generate(program, scopes, labels)?;
                
                let ty_vec = vec!();
                let str_vec = vec!("", while_entry.as_str(), "");
                program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                scopes.exit_current_scope();

                // while_end
                program.push_bb(while_end.as_str(), scopes);
                Ok(())
            },
        }
    }
}

/// Assign语句获取赋值的标号和值，然后进行类型转换，最后生成Store指令
impl Generate for Assign {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        let (ty1, label1) = self.val.generate(program ,scopes, labels)?;
        let (ty2, label2) = self.exp.generate(program, scopes, labels)?;
        let label2 = type_conver(program, labels, label2, &ty2, &ty1);
        let str_vec = vec!(label2.as_str(), label1.as_str(), "4");
        let type_vec = vec!(&ty1);
        program.push_instr(
            InstructionType::Store,
            str_vec,
            type_vec,
        );
        Ok(())
    }
}

/// Return语句对返回值进行类型转换，并插入Terminate指令
impl Generate for Return {
    type Out = ();

    fn generate(
        &self,
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels
    ) -> Result<Self::Out, Box<dyn Error>> {
        let func_type = scopes.get_current_function_type().expect(
            "Should not appear @ llvm_gen/generate.rs impl Generate for Return "
        );
        if let Some(exp) = &self.val {
            let (ty, exp_val) = exp.generate(program, scopes, labels)?;
            let ret_val: String;
            ret_val = type_conver(program, labels, exp_val, &ty, &func_type);

            let str_vec = vec!(ret_val.as_str());
            let ty_vec = vec!(&func_type);
            program.push_ter_instr(InstructionType::Ret, str_vec, ty_vec);
            Ok(())
        } else {
            let str_vec = vec!();
            let ty_vec = vec!(&func_type);
            program.push_ter_instr(InstructionType::Ret, str_vec, ty_vec);
            Ok(())
        }
    }
}

