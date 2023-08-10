use std::convert::From;
use crate::ast::*;
use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::structures::scopes::*;
use crate::llvm_gen::sysy_gen::*;
use crate::llvm_gen::type_utils::*;
use crate::utils::check::*;
use crate::utils::float::*;

/// 完成优化：常量折叠
fn arithetic_operate(ty1: &SymbolType, op1: &str, ty2: &SymbolType, op2: &str, op: &str) -> (SymbolType, String) {
    if both_is_int(ty1, ty2) {
        //操作数均为i32，无需进行类型提升。
        let num1 = op1.parse::<i32>().unwrap();
        let num2 = op2.parse::<i32>().unwrap();
        let res = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            "%" => num1 % num2,
            _ => panic!("Don't support!"),
        };
        //均为i32，无需隐式类型转换，任选一个的类型传入即可
        (SymbolType::new(ty1.width.clone(), true), res.to_string())  
    } else {
        //parse_float函数可以同时解析i32和f32，并且把类型全都提升为f32。
        //若想获取原始数据类型，直接判断`width`即可。
        let num1 = parse_float(op1) as f64;
        let num2 = parse_float(op2) as f64;
        let res = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            "%" => num1 % num2,
            _ => panic!("Don't support!"),
        };
        //实现：隐式类型转换——类型提升。提升顺序：int->float
        if ty1.width > ty2.width {
            (SymbolType::new(ty1.width.clone(), true), format_double(res as f32))
        } else {
            (SymbolType::new(ty2.width.clone(), true), format_double(res as f32))
        }
    }
}

/// i32直接调用库函数解析
/// float则需要将其规格化为IEEE754 Double，长度变为64bit
impl Generate for Number {
    type Out = (SymbolType, String);

    fn generate(&self, _program: &mut LLVMProgram, _scopes: &mut Scopes, _labels: &mut Labels) -> Self::Out {
        match self {
            Number::Int(num) => (SymbolType::new(SymbolWidth::I32, true), num.to_string()),
            Number::Float(num) => (SymbolType::new(SymbolWidth::Float, true), format_double(parse_float(num))),
        }
    }
}

impl LVal {
    /// 从维度dims中跳过skip_num个元素，然后将剩余元素收集作为剩下的维度
    fn get_left_dims(dims: &Vec<i32>, skip_num: usize) -> Vec<i32> {
        dims.iter()
            .skip(skip_num)
            .map(|x| *x)
            .collect()
    }

    /// 根据depth从dims中获取剩余维度，与tar组合为剩余数组
    /// 根据idx为剩余数组生成GetElemPtr指令
    fn gen_get_elem_ptr(
        program: &mut LLVMProgram, 
        labels: &mut Labels, 
        idx: Vec<String>, 
        depth: usize, 
        tar: &Box<SymbolType>, 
        dims: &Vec<i32>, 
        last_ptr: String) -> String
    {
        let left_dims = LVal::get_left_dims(dims, depth);
        let left_arr = SymbolType::new(SymbolWidth::Arr{tar: tar.clone(), dims: left_dims},false);
        let ty_vec = vec!(&left_arr); 
        let new_ptr = labels.pop_num_str();
        let mut str_vec: Vec<&str> = vec!(&new_ptr, &last_ptr);
        for i in idx.iter() {
            str_vec.push(i);
        }
        program.push_instr(InstructionType::GetElemPtr, str_vec, ty_vec);
        new_ptr 
    }
}

/// 首先判断是否为常量，若为常量，则直接从SymVal中取出常量值返回<br>
/// 然后判断是否为数组/指针，若不是，则返回标号<br>
/// 对于数组，首先检查索引长度，根据索引是否为空分类处理<br>
/// 若索引为空，表明直接使用数组/指针的整体作为结果，检查LVal为指针还是数组，分别选择Load和GetElemPtr进行读取<br>
/// 特别地，由于直接使用其整体作为结果，将is_const设为true，以表示暗藏一层指针，在需要时需要加入<br>
/// 若索引不为空，则遍历索引，根据每一层是指针还是数组，分别选择Load和GetElemPtr进行读取<br>
/// 完成遍历后，检查索引长度是否小于维度长度，若小于，还要使用一个GetElemPtr 0, 0指令进行读取<br>
/// 最后，根据剩余维度，选择返回数组/指针类型还是目标类型
impl Generate for LVal {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        let val = scopes.get(self.id.as_str()).expect(&format!("Undefined {}", self.id)).clone();
        if val.ty.is_const {
            (val.ty.clone(), get_symbol_val(&val.value))
        } else {
            match &val.ty.width {
                SymbolWidth::I32 => (val.ty.clone(), val.label.clone()),
                SymbolWidth::Float => (val.ty.clone(), val.label.clone()),
                SymbolWidth::Arr{tar, dims} => {
                    let zero = String::from("0");
                    let mut last_ptr = val.label.clone();

                    if self.idx.is_empty() {
                        if dims[0] == -1 {
                            let left_dims = LVal::get_left_dims(&dims, 0);
                            let left_arr = SymbolType::new(
                                SymbolWidth::Arr{
                                    tar: tar.clone(),
                                    dims: left_dims.clone()
                                },
                                false
                            );
                            let ty_vec = vec!(&left_arr);
                            let len = String::from("8");
                            let new_ptr = labels.pop_num_str();
                            let str_vec = vec!(new_ptr.as_str(), last_ptr.as_str(), len.as_str());
                            program.push_instr(InstructionType::Load, str_vec, ty_vec);
                            last_ptr = new_ptr;
                        } else {
                            let idx = vec!(zero.clone(), zero);
                            last_ptr = LVal::gen_get_elem_ptr(program, labels, idx, 0, &tar, &dims, last_ptr);
                        }
                        let left_dims = LVal::get_left_dims(&dims, 1);
                        let left_arr = SymbolType::new(SymbolWidth::Arr{tar: tar.clone(), dims: left_dims.clone()}, true);
                        return (left_arr, last_ptr)
                    }

                    for cnt in 0..self.idx.len() {
                        let depth: usize;
                        let mut idx: Vec<String> = vec!();
                        if dims[cnt] == -1 {
                            let left_dims = LVal::get_left_dims(&dims, 0);
                            let left_arr = SymbolType::new(
                                SymbolWidth::Arr{
                                    tar: tar.clone(),
                                    dims: left_dims.clone()
                                },
                                false
                            );
                            let ty_vec = vec!(&left_arr);
                            let len = String::from("8");
                            let new_ptr = labels.pop_num_str();
                            let str_vec = vec!(new_ptr.as_str(), last_ptr.as_str(), len.as_str());
                            program.push_instr(InstructionType::Load, str_vec, ty_vec);
                            last_ptr = new_ptr;
                            depth = cnt+1;
                        } else {
                            depth = cnt;
                            idx.push(zero.clone());
                        }
                        if !self.idx.is_empty() {
                            let (exp_ty, exp_val) = self.idx[cnt].generate(program, scopes, labels);
                            let dst_ty = SymbolType::new(SymbolWidth::I32, false);
                            let this_idx = type_conver(program, labels, exp_val, &exp_ty, &dst_ty);
                            idx.push(this_idx);
                            last_ptr = LVal::gen_get_elem_ptr(program, labels, idx, depth, &tar, &dims, last_ptr);
                        }
                    }
                    
                    if self.idx.len() < dims.len() {
                        let mut left_dims = LVal::get_left_dims(&dims, self.idx.len()); 
                        left_dims[0] = -1;
                        let idx = vec!(zero.clone(), zero.clone());
                        last_ptr = LVal::gen_get_elem_ptr(program, labels, idx, self.idx.len(), &tar, &dims, last_ptr);
                        let left_arr = SymbolType::new(SymbolWidth::Arr{tar: tar.clone(), dims: left_dims},false);
                        (left_arr, last_ptr)
                    } else {
                        let mut new_ty = *tar.clone();
                        new_ty.is_const = false;
                        (new_ty, last_ptr)
                    }
                },
                _ => panic!("Should not appear"),
            }
        }
    }
}

/// 对于表达式和数字，直接调用generate获取结果即可<br>
/// 对于LVal，调用generate后，对非数组/指针的变量使用Load指令进行读取
impl Generate for PrimaryExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        match self {
            PrimaryExp::Exp(exp) => exp.generate(program, scopes, labels),
            PrimaryExp::Number(num) => num.generate(program, scopes, labels),
            PrimaryExp::LVal(val) => {
                let (sym_type, val) = val.generate(program, scopes, labels);
                let flag: bool;
                if let SymbolWidth::Arr{tar: _, dims: _} = sym_type.width {
                    flag = false;
                } else if sym_type.is_const {
                    flag = false;
                } else {
                    flag = true;
                }

                if flag {
                    let len = String::from("4");
                    let res = labels.pop_num_str();
                    let type_vec = vec!(&sym_type);
                    let str_vec = vec!(res.as_str(), val.as_str(), len.as_str());
                    program.push_instr(InstructionType::Load, str_vec, type_vec);
                    (sym_type, res)
                } else {
                    (sym_type, val)
                }
            }
        }
    }
}

/// 对于PrimaryExp和取正，直接调用exp的generate函数即可<br>
/// 对于取负，检查是否为常量，对于常量直接计算结果即可；对于变量，则使用Sub或Fsub减去0进行计算<br>
/// 对于取非，流程与取负基本一致，指令使用Icmp和Fcmp<br>
/// 对于函数调用，首先获取函数的签名信息，对于无参数的函数，直接生成Call指令即可<br>
/// 对于有参数的函数，首先计算传入参数的值，然后将传入参数的类型转换为对应的函数参数类型<br>
/// 最后，将类型转换的结果填入Vec，生成Call指令即可
impl Generate for UnaryExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        match self {
            UnaryExp::PrimExp(prim_exp) => prim_exp.generate(program, scopes, labels),
            UnaryExp::Pos(unary_exp) => unary_exp.generate(program, scopes, labels),
            UnaryExp::Neg(unary_exp) => {
                let (ty, op2) = unary_exp.generate(program, scopes, labels);
                if ty.is_const {
                    let res: String;
                    match ty.width {
                        SymbolWidth::I32 => {
                            let num = - op2.parse::<i32>().unwrap();
                            res = num.to_string();
                        },
                        SymbolWidth::Float => {
                            let num: f32 = parse_float(&op2);
                            let num = -num;
                            res = format_double(num);
                        },
                        SymbolWidth::Bool => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = -num;
                            res = num.to_string();
                        },
                        _ => panic!("TODO"),
                    }
                    return (ty, res)
                }
                let result = labels.pop_num_str();
                let is_float = ty.width == SymbolWidth::Float;
                if is_float {
                    program.push_instr(
                        InstructionType::Fsub, 
                        vec!(
                            &result, 
                            "0.0", 
                            &op2
                        ),
                        vec!(&ty), 
                    );
                } else {
                    program.push_instr(
                        InstructionType::Sub, 
                        vec!(
                            &result, 
                            &String::from("0"), 
                            &op2
                        ),
                        vec!(&ty), 
                    );
                }
                (ty, result)
            },
            UnaryExp::Not(unary_exp) => {
                let (ty, op2) = unary_exp.generate(program, scopes, labels);
                if ty.is_const {
                    let res: String;
                    match ty.width {
                        SymbolWidth::I32 => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = (num == 0) as i32;
                            res = num.to_string();
                        },
                        SymbolWidth::Float => {
                            let num: f32 = parse_float(op2.as_str());
                            let num = (num == 0.0) as i32;
                            res = format_double(num as f32);
                        },
                        SymbolWidth::Bool => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = (num == 0) as i32;
                            res = num.to_string();
                        },
                        _ => panic!("Don't support!"),
                    }
                    return (ty, res)
                }
                let result = labels.pop_num_str();
                let is_float = ty.width == SymbolWidth::Float;
                if is_float {
                    program.push_instr(InstructionType::Fcmp, vec!(&String::from("eq"), &result, &String::from("0.0"), &op2), vec!(&ty));
                } else {
                    program.push_instr(InstructionType::Cmp, vec!(&String::from("eq"), &result, &String::from("0"), &op2), vec!(&ty));
                }
                (SymbolType::new(SymbolWidth::Bool, false), result)
            },
            UnaryExp::Call{id, params} => {
                let func = scopes.get_function(id.as_str()).expect(format!("Undefined function {}", id).as_str());
                let ret_type = func.ty.clone();
                if params.is_none() {
                    let res: String;
                    if ret_type.width == SymbolWidth::Void {
                        res = String::from("");
                    } else {
                        res = labels.pop_num_str();
                    }
                    let str_vec = vec!(res.as_str(), func.label.as_str());
                    let type_vec: Vec<&SymbolType> = vec!(&ret_type);
                    program.push_instr(
                        InstructionType::Call,
                        str_vec,
                        type_vec,
                    );
                    return (ret_type, res)
                }
                let params = params.as_ref().unwrap();

                if let SymbolVal::Func(_, param_list) = &func.value {
                    let func_param = param_list.clone();
                    let func_label = func.label.clone();
                    assert!(func_param.len() == params.len(), "Params length of function call is not correct, found {}, expected {}", params.len(), func_param.len());

                    // 计算参数
                    let mut param_info: Vec<(String, SymbolType)> = vec!();
                    for cnt in 0..params.len() {
                        let (ty, res) = params[cnt].generate(program, scopes, labels);
                        param_info.push((res, ty));
                    }
                    
                    // 类型转换
                    let mut dsts: Vec<String> = vec!();
                    for cnt in 0..params.len() {
                        dsts.push(type_conver(program, labels, String::from(&param_info[cnt].0), &param_info[cnt].1, &func_param[cnt]));
                    }

                    let res: String;
                    if ret_type.width == SymbolWidth::Void {
                        res = String::from("");
                    } else {
                        res = labels.pop_num_str();
                    }
                    let mut str_vec = vec!(res.as_str(), func_label.as_str());
                    let mut type_vec: Vec<&SymbolType> = vec!(&ret_type);
                    for cnt in 0..dsts.len() {
                        str_vec.push(dsts[cnt].as_str());
                        type_vec.push(&func_param[cnt]);
                    }
                    program.push_instr(
                        InstructionType::Call,
                        str_vec,
                        type_vec,
                    );
                    (ret_type, res)
                } else {
                    panic!("Should not appear!")
                } // if SymVal::Func
            }, // UnaryExp::Call 
        }
    }
}

impl MulExpBody {

    /// 乘除运算的运算主体，是乘除表达式MulExp的抽象结果<br>
    /// 从MulExp接受算子，对运算数进行常量检查、类型比较，最终计算出结果或者生成对应指令
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: &str) -> (SymbolType, String) {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels);
        let (ty2, op2) = self.exp2.generate(program, scopes, labels);

        if ty1.is_const && ty2.is_const {
            return arithetic_operate(&ty1, &op1, &ty2, &op2, op_ty);
        }

        let (mut ty1, op1, op2) = type_compare(program, labels, ty1, op1, ty2, op2);
        let result = labels.pop_num_str();
        let str_vec = vec!(
            result.as_str(),
            op1.as_str(),
            op2.as_str(),
        );
        let ty_vec = vec!(&ty1);
        if ty1.width == SymbolWidth::Float {
            match op_ty {
                "*" => program.push_instr(
                    InstructionType::Fmul,
                    str_vec,
                    ty_vec,
                ),
                "/" => program.push_instr(
                    InstructionType::Fdiv,
                    str_vec,
                    ty_vec,
                ),
                _ => panic!("Wrong op type {}", op_ty),
            }
        } else {
            match op_ty {
                "*" => program.push_instr(
                    InstructionType::Mul,
                    str_vec,
                    ty_vec,
                ),
                "/" => program.push_instr(
                    InstructionType::Sdiv,
                    str_vec,
                    ty_vec,
                ),
                "%" => program.push_instr(
                    InstructionType::Srem,
                    str_vec,
                    ty_vec,
                ),
                _ => panic!("Wrong op type {}", op_ty),
            }
        }
        ty1.is_const = false;
        (ty1, result)
    }
}

/// 根据自身的枚举类型，生成对应算子，传入计算主体，返回其结果
impl Generate for MulExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        match self {
            MulExp::UnaryExp(exp) => exp.generate(program, scopes, labels),
            MulExp::Mul(body) => body.gen(program, scopes, labels, "*"),
            MulExp::Div(body) => body.gen(program, scopes, labels, "/"),
            MulExp::Mod(body) => body.gen(program, scopes, labels, "%"),
        }
    }
}

impl AddExpBody {

    /// 加减运算的运算主体，是加减表达式AddExp的抽象结果<br>
    /// 从AddExp接受算子，对运算数进行常量检查、类型比较，最终计算出结果或者生成对应指令
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: &str) -> (SymbolType, String) {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels);
        let (ty2, op2) = self.exp2.generate(program, scopes, labels);

        if ty1.is_const && ty2.is_const {
            return arithetic_operate(&ty1, &op1, &ty2, &op2, op_ty);
        }

        let (mut ty1, op1, op2) = type_compare(program, labels, ty1, op1, ty2, op2);
        let result = labels.pop_num_str();
        let str_vec = vec!(
            result.as_str(),
            op1.as_str(),
            op2.as_str(),
        );
        let ty_vec = vec!(&ty1);
        let is_float = ty1.width == SymbolWidth::Float;
        if is_float {
            match op_ty {
                "+" => program.push_instr(
                    InstructionType::Fadd,
                    str_vec,
                    ty_vec,
                ),
                "-" => program.push_instr(
                    InstructionType::Fsub,
                    str_vec,
                    ty_vec,
                ),
                _ => panic!("Wrong op type {}", op_ty),
            }
        } else {
            match op_ty {
                "+" => program.push_instr(
                    InstructionType::Add,
                    str_vec,
                    ty_vec,
                ),
                "-" => program.push_instr(
                    InstructionType::Sub,
                    str_vec,
                    ty_vec,
                ),
                _ => panic!("Wrong op type {}", op_ty),
            }
        }
        ty1.is_const = false;
        (ty1, result)
    }
}

/// for Add
impl Generate for AddExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        match self {
            AddExp::MulExp(exp) => exp.generate(program, scopes, labels),
            AddExp::Add(body) => body.gen(program, scopes, labels, "+"),
            AddExp::Sub(body) => body.gen(program, scopes, labels, "-"),
        }
    }
}

