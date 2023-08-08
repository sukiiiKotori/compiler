use crate::ast::Decl;
use crate::ast::ConstDecl;
use crate::ast::VarDecl;
use crate::get_settings;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::llvm_gen::sysy_gen::*;
use crate::llvm_gen::type_utils::*;
use crate::llvm_gen::array_declaration::decl_arr;

// 对声明实现generate
// 分为常量声明和变量声明分别实现
impl Generate for Decl {
    type Out = ();
    fn generate(
            &self,
            program: &mut LLVMProgram,
            scopes: &mut Scopes,
            labels: &mut Labels
        ) -> Self::Out {
        match self {
            Decl::ConstDecl(constdecl) => constdecl.generate(program, scopes, labels),
            Decl::VarDecl(vardecl) => vardecl.generate(program, scopes, labels),
        }
    }
}

// 常量声明
impl Generate for ConstDecl {
    type Out = ();
    fn generate(
            &self,
            program: &mut LLVMProgram,
            scopes: &mut Scopes,
            labels: &mut Labels
        ) -> Self::Out {
            let mut ty = self.ty.generate();
            ty.is_const = true;
            
            // 解析常量定义
            for def in &self.defs {
                let (id, init_ty, init_val, dims) = def.generate(program, scopes, labels);
                let is_arr = !dims.is_empty();
                
                if is_arr {
                    // 数组，调用数组声明方法
                    let ty1 = SymbolType::new(ty.width.clone(), false);
                    decl_arr(program, scopes, labels, &id, &ty1, &dims, &init_ty, &init_val);
                    if !scopes.is_global_scope() {
                        program.push_comment("\n");
                    }
                    continue;
                }
                
                let res = type_conver(program, labels, init_val[0].clone(), &init_ty[0], &ty);
                let symbol_val = make_symbol_val(&ty, &res);
                if scopes.push(labels, &id, &ty, &symbol_val, None).is_none() {
                    panic!("{} has been defined", id);
                }
            }
            
            
    }
}

// 变量声明
impl Generate for VarDecl {
    type Out = ();

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        let mut ty = self.ty.generate();
        ty.is_const = false;

        // 遍历定义
        for def in &self.defs {
            let (id, init_ty, init_val, dims) = def.generate(program, scopes, labels);

            // 数组
            if !dims.is_empty() {
                // 声明数组
                decl_arr(program, scopes, labels, &id, &ty, &dims, &init_ty, &init_val);
                // 局部
                if !scopes.is_global_scope() {
                    program.push_comment("\n");
                }
                continue;
            }
            
            // 是否初始化，如果初始化，转换成当前类型，否则为Void
            let symbol_val = if !init_val.is_empty() {
                let res = type_conver(program, labels, init_val[0].clone(), &init_ty[0], &ty);
                make_symbol_val(&ty, &res)
            } else {
                SymbolVal::Void
            };


            if let Some(label) = scopes.push(labels, id.as_str(), &ty, &symbol_val, None) {
                // 判断是全局变量还是局部变量
                if scopes.is_global_scope() {
                    // 判断是否初始化
                    if init_val.is_empty() {
                        program.push_global_var(&id, &ty, Vec::new());
                    } else {
                        let (_, init) = match &symbol_val {
                            SymbolVal::I32(init) => (SymbolWidth::I32, init),
                            SymbolVal::Float(init) => (SymbolWidth::Float, init),
                            _ => panic!("{:?} is not supported!", symbol_val),
                        };
                        program.push_global_var(&id, &ty,vec!(init));
                    }
                } else {
                    let str_vec = vec!(label.as_str(), "4");
                    let ty_vec = vec!(&ty);
                    let settings = get_settings();

                    if scopes.is_in_while() || settings.all_allocs_in_entry {
                        let block_label = program.get_block_label();
                        program.insert_alloc(
                            Instruction::make_instruction(InstructionType::Alloca, str_vec, ty_vec.clone()),
                            block_label.as_str()
                        );
                    } else {
                        program.push_instr(InstructionType::Alloca, str_vec, ty_vec.clone());
                    }

                    // 如果初始化
                    if !init_val.is_empty() {
                        let val = match &symbol_val {
                            SymbolVal::I32(val) => val,
                            SymbolVal::Float(val) => val,
                            _ => panic!("{:?} is not supported", symbol_val),
                        };
                        let str_vec = vec![val.as_str(), label.as_str(), "4"];
                        program.push_instr(InstructionType::Store, str_vec, ty_vec.clone());
                        program.push_comment("\n");
                    }
                }
            } else {
                panic!("{} has been defined!", id);
            }
        }
        
    }
}