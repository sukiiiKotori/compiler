use std::error::Error;
use crate::ast::ConstDecl;
use crate::ast::ConstDef;
use crate::ast::Decl;
use crate::ast::VarDecl;
use crate::ast::VarDef;
use crate::ast::InitVal;
use crate::get_settings;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::llvm_gen::scopes::*;
use crate::llvm_gen::build::*;
use crate::llvm_gen::generate::*;
use crate::llvm_gen::symbol::*;

use super::generate;

// 获取某元素在多维数组中，实际偏移位置量
fn get_pos(dims: &Vec<i32>, pos: &Vec<i32>) -> i32 {
    let mut sub_size = dims.iter().skip(1).fold(1, | res, &a | res * a);

    let mut res = pos.first().unwrap().clone();
    res *= sub_size;
    for i in 1..pos.len() {
        sub_size /= dims[i];
        res += pos[i] * sub_size;
    }
    res
}

// 获取数值
fn get_val(vals: &Vec<String>, pos: i32) -> String {
    vals[pos as usize].to_string()
}

// 获取数据类型
fn get_type(types: &Vec<SymbolType>, pos: i32) -> SymbolType {
    types[pos as usize]
}

// 遍历多维数组，使用GetElemPtr或者Store指令初始化，成功返回true
fn traverse_array(
    program: &mut LLVMProgram,  // 程序
    labels: &mut Labels,        // 管理和生成指针标号的结构体的引用。
    ty: &SymbolType,            // 类型
    label: &String,             // 当前维度的指针标号
    types: &Vec<SymbolType>,    // 数组内类型
    vals: &Vec<String>,         // 值
    ins: &mut Vec<Instruction>, // 指令
    pos: &mut Vec<i32>,         // 索引
) -> bool {
    // 数组内类型为空或者值为空
    if types.is_empty() || vals.is_empty() {
        return false;
    }

    match &ty.width {
        SymbolWidth::Arr { tar, dims } => {
            let mut flag = true;
            // 长度相等，表示遍历到底层
            if pos.len() == dims.len() {
                let elem_pos = get_pos(dims, pos);          // 元素偏移位置
                let elem_ty = get_type(types, elem_pos);    // 元素类型

                if elem_ty.width != SymbolWidth::Void {     // 不是空
                    let tar_ty = tar.as_ref().clone();
                    let ty_vec = vec![&tar_ty];
                    let arr_val = get_val(vals, elem_pos);
                    let str_vec = vec![arr_val.as_str(), label.as_str(), "4"];
                    // 生成store指令
                    ins.push(Instruction::make_instruction(InstructionType::Store, str_vec, ty_vec));
                } else {
                    flag = false;
                }
                return flag;
            } else {
                // 长度不等，表明在中间层次
                let mut flag = false;
                let dim_range = dims[pos.len()];    // 当前维度大小

                for cnt in 0..dim_range {           // 遍历当前维度
                    let mut sub_dims: Vec<i32> = vec![];    // 剩余维度
                    let sub_tar: SymbolType;

                    match &ty.width {
                        SymbolWidth::Arr { tar, dims } => {
                            sub_tar = tar.as_ref().clone();
                            sub_dims.extend_from_slice(&dims[pos.len() + 1..]); // 保存剩余维度
                        }
                        _ => panic!("Error!"),   // 不是数组（前面已经判断过是数组）
                    }
                    
                    // 生成低维度子数组
                    let left_arr = SymbolType::new(SymbolWidth::Arr {
                        tar: Box::new(sub_tar),
                        dims: sub_dims,
                    }, ty.is_const);

                    let ty_vec = vec![&left_arr];
                    let ptr = labels.pop_num_str();
                    let mut str_vec = vec![ptr.as_str(), label.as_str()];
                    let idx = cnt.to_string();      // 当前维度的下标
                    str_vec.push("0");
                    str_vec.push(idx.as_str());

                    pos.push(cnt);                          // 放入当前维度下标

                    // 递归继续遍历
                    if traverse_array(program, labels, ty, &ptr, types, vals, ins, pos) {
                        flag = true;
                        ins.push(
                            Instruction::make_instruction(InstructionType::GetElemPtr, str_vec, ty_vec),
                        );
                    } else {
                        labels.recover_num();
                    }
                    
                    // 递归完毕，弹出当前维度下标，循环遍历下一个下标
                    pos.pop();
                }

                return flag;
            }
        }
        _ => panic!("{} is not an array, ty = {:?}", label, ty),
    }
}

// 数组声明
fn decl_arr(
    program: &mut LLVMProgram,
    scopes: &mut Scopes,
    labels: &mut Labels,
    id: &String,
    ty: &SymbolType,
    dims: &Vec<i32>,
    types: &Vec<SymbolType>,
    vals: &Vec<String>,
) {
    let val_len = vals.len(); // 初始化值长度
    // 生成数组类型
    let ty_arr = SymbolType::new(SymbolWidth::Arr { tar: Box::new(ty.clone()), dims: dims.clone()}, false);
    let mut val_init: Vec<String> = vals.iter().enumerate().map(|(cnt, val)| {
        if types[cnt].width != SymbolWidth::Void {
            type_conver(program, labels, val.clone(), &types[cnt], &ty)
        } else {
            val.clone()
        }
    }).collect();

    // 作用域更新
    if let Some(label) = scopes.push(labels, id.as_str(), &ty_arr, &SymbolVal::Void, None) {
        // 如果是全局作用域，放到Program中
        if scopes.is_global_scope() {
            let init_types: Vec<&SymbolType> = types.iter().collect();
            let init_vals: Vec<&String> = val_init.iter().collect();
            program.push_global_var(&id, &ty_arr, init_types, init_vals);
            return;
        }

        // 然后更新作用域
        let str_vec = vec!(label.as_str(), "16");
        let ty_vec = vec!(&ty_arr);
        let settings = crate::get_settings();
        if scopes.is_in_while() || settings.all_allocs_in_entry {
            program.insert_alloc(
                Instruction::make_instruction(InstructionType::Alloca, str_vec, ty_vec), 
                program.get_bb_label().as_str(),
            )
        } else {
            program.push_instr(InstructionType::Alloca, str_vec, ty_vec);
        }
        
        // 做类型转换，使用BitCase将label转换为一个数组指针类型
        let res = labels.pop_num_str();
        let ty_i8 = SymbolType::new(SymbolWidth::I8, false);    // label类型
        let binary_ty = vec!(&ty_arr, &ty_i8);
        let str_vec = vec!(res.as_str(), label.as_str());
        program.push_instr(InstructionType::BitCast, str_vec, binary_ty);

        // 数组初始化
        let memset_size = dims.iter().fold(1, |acc, x| acc * x) * 4;
        let memset_funcname = "@llvm.memset.p018.i64".to_string();
        let str_vec = vec!("", memset_funcname.as_str(), res.as_str(), "0", memset_size.to_string().as_str(), "false");
        // type向量是对应上面数据向量的类型
        let ty_void = SymbolType::new(SymbolWidth::Void, false);
        let ty_i1 = SymbolType::new(SymbolWidth::I1, false);
        let ty_i64 = SymbolType::new(SymbolWidth::I64, false);
        let ty_ptr_i8 = SymbolType::new(SymbolWidth::Arr{tar: Box::new(ty_i8.clone()), dims: vec!(-1)}, false);
        let ty_vec = vec!(&ty_void, &ty_ptr_i8, &ty_i8, &ty_i64, &ty_i1);
        program.push_instr(InstructionType::Call, str_vec, ty_vec);

        let mut ins = vec!();
        let mut pos = vec!();
        // 遍历数组
        let tra_flag = traverse_array(program, labels, &ty_arr, &label, types, &val_init, &mut ins, &mut pos);
        if tra_flag {
            for i in ins.into_iter() {
                program.insert_instr(i);
            }
        }
    } else {
        panic!("{} has been declared!", id);
    }
}

// 对声明实现generate
// 分为常量声明和变量声明分别实现
impl Generate for Decl {
    type Out = ();
    fn generate(
            &self,
            program: &mut LLVMProgram,
            scopes: &mut Scopes,
            labels: &mut Labels
        ) -> Result<Self::Out, Box<dyn Error>> {
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
        ) -> Result<Self::Out, Box<dyn Error>> {
            let mut ty = self.ty.generate()?;
            ty.is_const = true;
            
            // 解析常量定义
            for def in &self.defs {
                let (id, init_ty, init_val, dims) = def.generate(program, scopes, labels)?;
                let is_arr = !dims.is_empty();
                
                if is_arr {
                    let ty1 = SymbolType::new(ty.width.clone(), false);
                    decl_arr(program, scopes, labels, &id, &ty1, &dims, &init_ty, &init_val);
                    if !scopes.is_global_scope() {
                        program.push_comment("\n");
                    }
                    continue;
                }
                
                let res = type_conver(program, labels, init_val[0].clone(), &init_ty[0], &ty);
                let sym_val = make_symbol_val(&ty, &res);
                if scopes.push(labels, &id, &ty, &sym_val, None).is_none() {
                    panic!("{} has been defined", id);
                }
            }
            
            Ok(())
    }
}

impl Generate for ConstDef {
    type Out = (String, Vec<SymbolType>, Vec<String>, Vec<i32>);

    fn generate(
            &self,
            program: &mut LLVMProgram,
            scopes: &mut Scopes,
            labels: &mut Labels
        ) -> Result<Self::Out, Box<dyn Error>> {
            if !self.dims.is_empty() {
                if !scopes.is_global_scope() {
                    program.push_comment(&format!("; init {}\n", self.id));
                }
            }
            
            let dims: Vec<i32> = self.dims
                .iter()
                .map(|dim| dim.generate(program, scopes, labels).map(|(_, val)| val.parse().expect("Not an integer")))
                .collect::<Result<Vec<i32>, Box<dyn Error>>>()?;
            
            let (ty, val) = self.init.generate(program, scopes, labels, &dims)?;
            
            Ok((String::from(self.id.as_str()), ty, val, dims))
    }
}

impl InitVal {
    // 
    fn Init_align(dims: &Vec<i32>, mut fill: i32, now_depth: usize) -> usize {
        if fill < 1 {
            return now_depth + 1;
        } else {
            let mut sub_size: i32 = dims.iter().skip(now_depth).product();
            let mut pos: Vec<i32> = vec!();     // 确定索引
            for i in (now_depth as usize)..(dims.len() - 1) {
                sub_size /= dims[i];
                pos.push(fill/sub_size);
                fill %= sub_size;
            }
            for i in (0..pos.len()).rev() {
                if pos[i] != 0 {
                    return now_depth + i + 1;
                }
            }
            panic!("Error");
        }
    }

    // 填充初始值，缺省值设置为0，类型为Void
    fn Init_padding(
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels,
        items: &Vec<InitVal>,
        dims: &Vec<i32>,
        tys: &mut Vec<SymbolType>,
        vals: &mut Vec<String>, 
        now_depth: usize
    ) -> i32 {
        let sub_size = dims.iter().skip(now_depth).product();
        let mut fill = 0;
    
        for item in items {
            match item {
                InitVal::Exp(exp) => {
                    let (ty, val) = exp.generate(program, scopes, labels).unwrap();
                    tys.push(ty);
                    vals.push(val);
                    fill += 1;
                }
                InitVal::Arr(arr) => {
                    if fill % dims.last().unwrap() != 0 {
                        panic!("Wrong format of init array");
                    }
                    let next_depth = InitVal::Init_align(dims, fill, now_depth);
                    fill += InitVal::Init_padding(program, scopes, labels, arr, dims, tys, vals, next_depth);
                }
            }
        }
    
        tys.extend((0..sub_size - fill).map(|_| SymbolType::new(SymbolWidth::Void, true)));
        vals.extend((0..sub_size - fill).map(|_| String::from("0")));
    
        sub_size
    }

    fn generate(
        &self, 
        program: &mut LLVMProgram, 
        scopes: &mut Scopes, 
        labels: &mut Labels, 
        dims: &Vec<i32>
    ) -> Result<(Vec<SymbolType>, Vec<String>), Box<dyn Error>> {
        match self {
            InitVal::Exp(exp) => {
                let (ty, val) = exp.generate(program, scopes, labels)?;
                Ok((vec!(ty), vec!(val)))
            },
            InitVal::Arr(arr) => {
                let mut ty: Vec<SymbolType> = vec!();
                let mut val: Vec<String> = vec!();
                if !arr.is_empty() {
                    InitVal::Init_padding(program, scopes, labels, &arr, dims, &mut ty, &mut val, 0);
                }
                Ok((ty, val))
            },
        }
    }
}

impl Generate for VarDecl {
    type Out = ();

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        let mut ty = self.ty.generate()?;
        ty.is_const = false;

        for def in self.defs.iter() {
            let (id, init_ty, init_val, dims) = def.generate(program, scopes, labels)?;
            let is_arr = !dims.is_empty();
            let has_init = !init_val.is_empty();

            let sym_val: SymbolVal;
            let init_len = init_val.len();

            if is_arr {
                decl_arr(program, scopes, labels, &id, &ty, &dims, &init_ty, &init_val);
                if !scopes.is_global_scope() {
                    program.push_comment(String::from("\n").as_str());
                }
                continue;
            } else {
                if has_init {
                    let res = type_conver(program, labels, init_val[0].clone(), &init_ty[0], &ty);
                    sym_val = make_symbol_val(&ty, &res);
                } else {
                    sym_val = SymbolVal::Void;
                }
            }

            if let Some(label) = scopes.push(labels, id.as_str(), &ty, &sym_val, None) {
                if scopes.is_global_scope() { // 全局变量
                    if init_len == 0 {
                        program.push_global_var(&id, &ty, vec!(), vec!());
                    } else {
                        match &sym_val {
                            SymbolVal::I32(init) => program.push_global_var(&id, &ty, vec!(&SymbolType::new(SymbolWidth::I32, false)), vec!(init)),
                            SymbolVal::Float(init) => program.push_global_var(&id, &ty, vec!(&SymbolType::new(SymbolWidth::Float, false)), vec!(init)),
                            _ => panic!("{:?} TODO", sym_val),
                        }
                    }
                } else { // 局部变量
                    let str_vec = vec!(label.as_str(), "4");
                    let ty_vec = vec!(&ty);
                    let config = get_settings();
                    if scopes.is_in_while() || config.all_allocs_in_entry {
                        let bb_label = program.get_bb_label();
                        program.insert_alloc(Instruction::make_instruction(InstructionType::Alloca, str_vec, ty_vec), bb_label.as_str());
                    } else {
                        program.push_instr(InstructionType::Alloca, str_vec, ty_vec);
                    }

                    if has_init { // 带初值的一般变量
                        let val = match &sym_val {
                            SymbolVal::I32(val) => val,
                            SymbolVal::Float(val) => val,
                            _ => panic!("{:?} TODO", sym_val),
                        };
                        let str_vec = vec!(val.as_str(), label.as_str(), "4");
                        let type_vec = vec!(&ty);
                        program.push_instr(InstructionType::Store, str_vec, type_vec);
                        program.push_comment(String::from("\n").as_str());
                    } // has_init
                } // is_global
            } else {
                panic!("Multi definition of {}", id);
            } // let Some(label)
        } // for def
        return Ok(());
    }
}

/// 变量定义，与常量定义基本一致
impl Generate for VarDef {
    type Out = (String, Vec<SymbolType>, Vec<String>, Vec<i32>);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        let mut dims: Vec<i32> = vec!();
        for dim in self.dims.iter() {
            let (_, val) = dim.generate(program, scopes, labels)?;
            dims.push(val.parse().expect(&format!("{} is not integer", val)));
        }

        if let Some(init) = &self.init {
            if !scopes.is_global_scope() {
                program.push_comment(format!("; init {}\n", self.id).as_str());
            }
            let (ty, val) = init.generate(program, scopes, labels, &dims)?;
            Ok((String::from(self.id.as_str()), ty, val, dims))
        } else {
            if !scopes.is_global_scope() && !dims.is_empty() {
                program.push_comment(format!("; init {}\n", self.id).as_str());
            }
            Ok((String::from(self.id.as_str()), vec!(), vec!(), dims))
        }
    }
}