use std::error::Error;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
};
use std::cmp::{
    PartialEq,
    PartialOrd,
};
use std::convert::From;
use crate::ast::*;
use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::llvm_gen::scopes::*;
use crate::llvm_gen::generate::*;
use crate::llvm_gen::symbol::*;
use crate::utils::check::*;
use crate::utils::float::*;

/// 计算常量T类型的num1和num2关于op算子的计算结果<br>
/// T类型需要满足基本的算术Trait和比较Trait
pub fn operate_num<T>(num1: T, num2: T, op: &str) -> (SymbolWidth, T) 
where
    T: Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Div<Output=T>+Rem<Output=T>+PartialEq+PartialOrd+From<i32>,
{
    let res: T;
    if op == "+" {
        res = num1 + num2;
    } else if op == "-" {
        res = num1 - num2;
    } else if op == "*" {
        res = num1 * num2;
    } else if op == "/" {
        res = num1 / num2;
    } else if op == "%" {
        res = num1 % num2;
    } else { // Boolean expression
        let res: bool;
        let zero: i32 = 0;
        let zero_t = T::from(zero);
        if op == "==" {
            res = num1 == num2;
        } else if op == "!=" {
            res = num1 != num2; 
        } else if op == "<" {
            res = num1 < num2;
        } else if op == ">" {
            res = num1 > num2;
        } else if op == "<=" {
            res = num1 <= num2; 
        } else if op == ">=" {
            res = num1 >= num2; 
        } else if op == "&&" {
            res = num1 != zero_t && num2 != zero_t;
        } else if op == "||" {
            res = num1 != zero_t || num2 != zero_t;
        } else {
            res = false;
        }
        let res = i32::from(res);
        return (SymbolWidth::I1, T::from(res));
    }
    (SymbolWidth::Void, res)
}

/// 计算常量<br>
/// 首先检查是否有浮点数<br>
/// 然后进行解析<br>
/// 最后调用operate_num计算结果<br>
pub fn operate(ty1: &SymbolType, op1: &String, ty2: &SymbolType, op2: &String, op: &str) -> Result<(SymbolType, String), Box<dyn Error>> {
    if all_is_int(ty1, ty2) {
        let num1:i32 = op1.parse().expect(&format!("Parse i32 {} failed", op1));
        let num2:i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
        let (width, res) = operate_num(num1, num2, op);
        if width == SymbolWidth::Void { // 由ty1和ty2决定结果类型
            if ty1.width > ty2.width {
                Ok((SymbolType::new(ty1.width.clone(), true), res.to_string()))
            } else {
                Ok((SymbolType::new(ty2.width.clone(), true), res.to_string()))
            }
        } else {
            Ok((SymbolType::new(width, true), res.to_string()))
        }
    } else {
        let num1 = parse_float(op1.as_str()) as f64;
        let num2 = parse_float(op2.as_str()) as f64;
        let (width, res) = operate_num(num1, num2, op);
        if width == SymbolWidth::Void {
            if ty1.width > ty2.width {
                Ok((SymbolType::new(ty1.width.clone(), true), format_double(res as f32)))
            } else {
                Ok((SymbolType::new(ty2.width.clone(), true), format_double(res as f32)))
            }
        } else {
            Ok((SymbolType::new(width, true), res.to_string()))
        }
    }
}

impl Generate for Exp {
    type Out = (SymbolType, String);
    
    /// 调用exp成员进行generate
    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        self.exp.generate(program, scopes, labels)
    }
}

/// 对于整型数，直接将其从i32转换为String<br>
/// 对于浮点数，调用parse_float从String获取f32，然后调用format_double将其打印为符合LLVM的16进制64位浮点数格式<br>
impl Generate for Number {
    type Out = (SymbolType, String);

    fn generate(&self, _program: &mut LLVMProgram, _scopes: &mut Scopes, _labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            Number::Int(num) => Ok((SymbolType::new(SymbolWidth::I32, true), num.to_string())),
            Number::Float(num) => Ok(
                (
                    SymbolType::new(SymbolWidth::Float, true), 
                    format_double(parse_float(num.as_str()))
                )
            ),
        }
    }
}

impl LVal {
    /// 从维度dims中跳过skip_num的元素，然后将剩余元素收集作为剩下的维度
    fn get_left_dims(dims: &Vec<i32>, skip_num: usize) -> Vec<i32> {
        dims.iter()
            .skip(skip_num)
            .map(|x| x.clone())
            .collect()
    }

    /// 根据depth从dims中获取剩余维度，与tar组合为剩余数组
    /// 根据idx为剩余数组生成GetElemPtr指令
    fn gen_getelemptr(
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
        let mut str_vec = vec!(new_ptr.as_str(), last_ptr.as_str());
        for i in idx.iter() {
            str_vec.push(i.as_str());
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

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        let val = scopes.get(self.id.as_str()).expect(&format!("Undefined {}", self.id)).clone();
        if val.sym_type.is_const {
            let fetch_val = get_symbol_val(&val.sym_val);
            Ok((val.sym_type.clone(), fetch_val))
        } else {
            match &val.sym_type.width {
                SymbolWidth::I32 => Ok((val.sym_type.clone(), val.label.clone())),
                SymbolWidth::Float => Ok((val.sym_type.clone(), val.label.clone())),
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
                            last_ptr = LVal::gen_getelemptr(program, labels, idx, 0, &tar, &dims, last_ptr);
                        }
                        let left_dims = LVal::get_left_dims(&dims, 1);
                        let left_arr = SymbolType::new(SymbolWidth::Arr{tar: tar.clone(), dims: left_dims.clone()}, true);
                        return Ok((left_arr, last_ptr))
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
                        let (exp_ty, exp_val) = self.idx[cnt].generate(program, scopes, labels)?;
                            let dst_ty = SymbolType::new(SymbolWidth::I32, false);
                            let this_idx = type_conver(program, labels, exp_val, &exp_ty, &dst_ty);
                            idx.push(this_idx);
                            last_ptr = LVal::gen_getelemptr(program, labels, idx, depth, &tar, &dims, last_ptr);
                        }
                    }
                    
                    if self.idx.len() < dims.len() {
                        let mut left_dims = LVal::get_left_dims(&dims, self.idx.len()); 
                        left_dims[0] = -1;
                        let idx = vec!(zero.clone(), zero.clone());
                        last_ptr = LVal::gen_getelemptr(program, labels, idx, self.idx.len(), &tar, &dims, last_ptr);
                        let left_arr = SymbolType::new(SymbolWidth::Arr{tar: tar.clone(), dims: left_dims},false);
                        Ok((left_arr, last_ptr))
                    } else {
                        let mut new_ty = *tar.clone();
                        new_ty.is_const = false;
                        Ok((new_ty, last_ptr))
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

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            PrimaryExp::Exp(exp) => exp.generate(program, scopes, labels),
            PrimaryExp::Number(num) => num.generate(program, scopes, labels),
            PrimaryExp::LVal(val) => {
                let (sym_type, val) = val.generate(program, scopes, labels)?;
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
                    Ok((sym_type, res))
                } else {
                    Ok((sym_type, val))
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

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            UnaryExp::PrimExp(prim_exp) => prim_exp.generate(program, scopes, labels),
            UnaryExp::Pos(unary_exp) => unary_exp.generate(program, scopes, labels),
            UnaryExp::Neg(unary_exp) => {
                let (ty, op2) = unary_exp.generate(program, scopes, labels)?;
                if ty.is_const {
                    let res: String;
                    match ty.width {
                        SymbolWidth::I32 => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = -num;
                            res = num.to_string();
                        },
                        SymbolWidth::Float => {
                            let num: f32 = parse_float(op2.as_str());
                            let num = -num;
                            res = format_double(num);
                        },
                        SymbolWidth::I1 => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = -num;
                            res = num.to_string();
                        },
                        _ => panic!("TODO"),
                    }
                    return Ok((ty, res));
                }
                let result = labels.pop_num_str();
                let is_float = ty.width == SymbolWidth::Float;
                if is_float {
                    program.push_instr(
                        InstructionType::Fsub, 
                        vec!(
                            &result, 
                            &String::from("0.0"), 
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
                Ok((ty, result))
            },
            UnaryExp::Not(unary_exp) => {
                let (ty, op2) = unary_exp.generate(program, scopes, labels)?;
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
                        SymbolWidth::I1 => {
                            let num: i32 = op2.parse().expect(&format!("Parse i32 {} failed", op2));
                            let num = (num == 0) as i32;
                            res = num.to_string();
                        },
                        _ => panic!("TODO"),
                    }
                    return Ok((ty, res));
                }
                let result = labels.pop_num_str();
                let is_float = ty.width == SymbolWidth::Float;
                if is_float {
                    program.push_instr(InstructionType::Fcmp, vec!(&String::from("eq"), &result, &String::from("0.0"), &op2), vec!(&ty));
                } else {
                    program.push_instr(InstructionType::Cmp, vec!(&String::from("eq"), &result, &String::from("0"), &op2), vec!(&ty));
                }
                Ok((SymbolType::new(SymbolWidth::I1, false), result))
            },
            UnaryExp::Call{id, params} => {
                let func = scopes.get_function(id.as_str()).expect(format!("Undefined function {}", id).as_str());
                let ret_type = func.sym_type.clone();
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
                    return Ok((ret_type, res));
                }
                let params = params.as_ref().unwrap();

                if let SymbolVal::Func(_, param_list) = &func.sym_val {
                    let func_param = param_list.clone();
                    let func_label = func.label.clone();
                    assert!(func_param.len() == params.len(), "Params length of function call is not correct, found {}, expected {}", params.len(), func_param.len());

                    // 计算参数
                    let mut param_info: Vec<(String, SymbolType)> = vec!();
                    for cnt in 0..params.len() {
                        let (ty, res) = params[cnt].generate(program, scopes, labels)?;
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
                    Ok((ret_type, res))
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
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: String) -> Result<(SymbolType, String), Box<dyn Error>> {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels)?;
        let (ty2, op2) = self.exp2.generate(program, scopes, labels)?;

        if all_is_const(&ty1, &ty2) {
            return operate(&ty1, &op1, &ty2, &op2, op_ty.as_str());
        }

        let (mut ty1, op1, op2) = type_cmpare(program, labels, ty1, op1, ty2, op2);
        let result = labels.pop_num_str();
        let str_vec = vec!(
            result.as_str(),
            op1.as_str(),
            op2.as_str(),
        );
        let ty_vec = vec!(&ty1);
        let is_float = ty1.width == SymbolWidth::Float;
        if is_float {
            match op_ty.as_str() {
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
            match op_ty.as_str() {
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
        Ok((ty1, result))
    }
}

/// 根据自身的枚举类型，生成对应算子，传入计算主体，返回其结果
impl Generate for MulExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            MulExp::UnaryExp(exp) => exp.generate(program, scopes, labels),
            MulExp::Mul(body) => body.gen(program, scopes, labels, String::from("*")),
            MulExp::Div(body) => body.gen(program, scopes, labels, String::from("/")),
            MulExp::Mod(body) => body.gen(program, scopes, labels, String::from("%")),
        }
    }
}

impl AddExpBody {

    /// 加减运算的运算主体，是加减表达式AddExp的抽象结果<br>
    /// 从AddExp接受算子，对运算数进行常量检查、类型比较，最终计算出结果或者生成对应指令
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: String) -> Result<(SymbolType, String), Box<dyn Error>> {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels)?;
        let (ty2, op2) = self.exp2.generate(program, scopes, labels)?;

        if all_is_const(&ty1, &ty2) {
            return operate(&ty1, &op1, &ty2, &op2, op_ty.as_str());
        }

        let (mut ty1, op1, op2) = type_cmpare(program, labels, ty1, op1, ty2, op2);
        let result = labels.pop_num_str();
        let str_vec = vec!(
            result.as_str(),
            op1.as_str(),
            op2.as_str(),
        );
        let ty_vec = vec!(&ty1);
        let is_float = ty1.width == SymbolWidth::Float;
        if is_float {
            match op_ty.as_str() {
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
            match op_ty.as_str() {
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
        Ok((ty1, result))
    }
}

/// 根据自身的枚举类型，生成对应算子，传入计算主体，返回其结果
impl Generate for AddExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            AddExp::MulExp(exp) => exp.generate(program, scopes, labels),
            AddExp::Add(body) => body.gen(program, scopes, labels, String::from("+")),
            AddExp::Sub(body) => body.gen(program, scopes, labels, String::from("-")),
        }
    }
}

