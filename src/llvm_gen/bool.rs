use std::error::Error;
use crate::get_settings;
use crate::ast::*;
use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::llvm_gen::arithmetic::*;
use crate::llvm_gen::scopes::*;
use crate::llvm_gen::generate::*;
use crate::llvm_gen::symbol::*;
use crate::utils::check::*;

impl RelExpBody {

    /// 相对运算的运算主体，是相对运算表达式RelExp的抽象结果<br>
    /// 从RelExp接受算子，对运算数进行常量检查、类型比较，最终计算出结果或者生成对应指令
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: String) -> Result<(SymbolType, String), Box<dyn Error>> {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels)?;
        let (ty2, op2) = self.exp2.generate(program, scopes, labels)?;

        if all_is_const(&ty1, &ty2) {
            if op_ty == "slt" {
                return operate(&ty1, &op1, &ty2, &op2, "<");
            } else if op_ty == "sgt"{
                return operate(&ty1, &op1, &ty2, &op2, ">");
            } else if op_ty == "sle"{
                return operate(&ty1, &op1, &ty2, &op2, "<=");
            } else {
                return operate(&ty1, &op1, &ty2, &op2, ">=");
            }
        }

        let (ty1, op1, op2) = type_cmpare(program, labels, ty1, op1, ty2.clone(), op2);
        let result = labels.pop_num_str();

        let str_vec = vec!(
            op_ty.as_str(),
            result.as_str(),
            op1.as_str(),
            op2.as_str(),
        );
        let ty_vec = vec!(&ty1);
        let is_float = ty1.width == SymbolWidth::Float;
    
        if is_float {
            program.push_instr(
                InstructionType::Fcmp,
                str_vec,
                ty_vec,
            );
        } else {
            program.push_instr(
                InstructionType::Cmp,
                str_vec,
                ty_vec,
            );
        }
        Ok((SymbolType::new(SymbolWidth::I1, false), result))
    }
}

/// 根据自身的枚举类型，生成对应算子，传入计算主体，返回其结果
impl Generate for RelExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            RelExp::AddExp(exp) => exp.generate(program, scopes, labels),
            RelExp::Lt(body) => body.gen(program, scopes, labels, String::from("slt")),
            RelExp::Gt(body) => body.gen(program, scopes, labels, String::from("sgt")),
            RelExp::Le(body) => body.gen(program, scopes, labels, String::from("sle")),
            RelExp::Ge(body) => body.gen(program, scopes, labels, String::from("sge")),
        }
    }
}

impl EqExpBody {

    /// 等于运算的运算主体，是等于/不等表达式EqExp的抽象结果<br>
    /// 从EqExp接受算子，对运算数进行常量检查、类型比较，最终计算出结果或者生成对应指令
    fn gen(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels, op_ty: String) -> Result<(SymbolType, String), Box<dyn Error>> {
        let (ty1, op1) = self.exp1.generate(program, scopes, labels)?;
        let (ty2, op2) = self.exp2.generate(program, scopes, labels)?;

        if all_is_const(&ty1, &ty2) {
            if op_ty == "ne" {
                return operate(&ty1, &op1, &ty2, &op2, "!=");
            } else {
                return operate(&ty1, &op1, &ty2, &op2, "==");
            }
        }

        let (ty1, op1, op2) = type_cmpare(program, labels, ty1, op1, ty2.clone(), op2);
        let result = labels.pop_num_str();

        let str_vec = vec!(op_ty.as_str(), result.as_str(), op1.as_str(), op2.as_str());
        let ty_vec = vec!(&ty1);
        let is_float = ty1.width == SymbolWidth::Float;

        if is_float {
            program.push_instr(
                InstructionType::Fcmp,
                str_vec,
                ty_vec,
            );
        } else {
            program.push_instr(
                InstructionType::Cmp,
                str_vec,
                ty_vec,
            );
        }
        Ok((SymbolType::new(SymbolWidth::I1, false), result))
    }
}

/// 根据自身的枚举类型，生成对应算子，传入计算主体，返回其结果
impl Generate for EqExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            EqExp::RelExp(exp) => exp.generate(program, scopes, labels),
            EqExp::EQ(body) => body.gen(program, scopes, labels, String::from("eq")),
            EqExp::NE(body) => body.gen(program, scopes, labels, String::from("ne")),
        }
    }
}

/// 与布尔表达式的计算<br>
/// 若类型为EqExp，则直接调用generate生成结果即可<br>
/// 若类型为And，则首先计算exp1，根据exp1是否为常量进行分类<br>
/// 若exp1为常量，则检查其值，若为0，直接返回0作为结果即可<br>
/// 若不为0，则计算exp2，表达式的结果就是exp2的结果<br>
/// 若exp1为变量，则生成and_false、and_end两个基本块的标号，并获取当前基本块的标号this_bb<br>
/// 生成Branch指令，根据exp1的值跳转到and_false或and_end，然后插入and_false基本块<br>
/// 然后计算exp2，根据其是否为常量获取其值，生成Branch指令，跳转到and_end块，然后插入and_end基本块<br>
/// 最后插入Phi指令，根据结果来源于this_bb还是and_false选择结果是exp1的结果还是exp2的结果
impl Generate for LAndExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            LAndExp::EqExp(exp) => exp.generate(program, scopes, labels),
            LAndExp::And(exp1, exp2) => {
                // 计算LHS
                let boolean = SymbolWidth::I1;
                let (ty1, mut op1) = exp1.generate(program, scopes, labels)?;

                if ty1.is_const {
                    let is_zero = num_is_zero(&ty1, &op1);
                    let res: String;
                    let is_const: bool;
                    if is_zero { // 结果为真
                        res = String::from("0");
                        is_const = true;
                    } else { // 结果为假
                        // 计算RHS
                        let (ty2, mut op2) = exp2.generate(program, scopes, labels)?;
                        if ty2.is_const { 
                            let is_zero2 = num_is_zero(&ty2, &op2);
                            if is_zero2 {
                                res = String::from("0");
                                is_const = true;
                            } else {
                                res = String::from("1");
                                is_const = true;
                            } // not_zero2 else
                        } else {
                            if ty2.width != boolean {
                                let new_op2 = labels.pop_num_str(); 
                                let str_vec = vec!("ne", new_op2.as_str(), "0", op2.as_str());
                                let type_vec = vec!(&ty2);
                                program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                                op2 = new_op2;
                            }
                            res = op2.clone();
                            is_const = false;
                        } // ty2 const else
                    }
                    Ok((SymbolType::new(SymbolWidth::I1, is_const), res))
                } else {
                    let and_true = labels.pop_block("and_true");
                    let and_end = labels.pop_block("and_end");
                    let this_bb = program.get_bb_label();

                    // this_bb     
                    if ty1.width != boolean {
                        let new_op1 = labels.pop_num_str(); 
                        let str_vec = vec!("ne", new_op1.as_str(), "0", op1.as_str());
                        let type_vec = vec!(&ty1);
                        program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                        op1 = new_op1;
                    }

                    let config = get_settings();
                    let use_phi = config.use_phi;
                    let i1_ty = SymbolType::new(SymbolWidth::I1, false);
                    if !use_phi {
                        let type_vec = vec!(&i1_ty);
                        let str_vec = vec!(op1.as_str(), "%replace_phi_0", "1");
                        program.push_instr(InstructionType::Store, str_vec, type_vec);
                    }

                    let ty_vec = vec!();
                    let str_vec = vec!(op1.as_str(), and_true.as_str(), and_end.as_str());
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                    // or_false
                    program.push_bb(and_true.as_str(), scopes);

                    // 计算RHS
                    let (ty2, mut op2) = exp2.generate(program, scopes, labels)?;
                    if ty2.is_const {
                        let is_zero = num_is_zero(&ty2, &op2);
                        if is_zero {
                            op2 = String::from("0");
                        } else {
                            op2 = String::from("1");
                        }
                    } else if ty2.width != boolean {
                        let new_op2 = labels.pop_num_str(); 
                        let str_vec = vec!("ne", new_op2.as_str(), "0", op2.as_str());
                        let type_vec = vec!(&ty2);
                        program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                        op2 = new_op2;
                    }

                    let type_vec = vec!(&i1_ty);
                    let str_vec = vec!(op2.as_str(), "%replace_phi_0", "1");
                    program.push_instr(InstructionType::Store, str_vec, type_vec);

                    let second_bb = program.get_bb_label();
                    let ty_vec = vec!();
                    let str_vec = vec!("", and_end.as_str(), "");
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                    // and_end
                    program.push_bb(and_end.as_str(), scopes);

                    let result = labels.pop_num_str();
                    let i1_ty = SymbolType::new(SymbolWidth::I1, false);
                    if use_phi {
                        let ty_vec = vec!(&i1_ty);
                        let str_vec = vec!(result.as_str(), "0", this_bb.as_str(), op2.as_str(), second_bb.as_str());
                        program.push_phi(str_vec, ty_vec);
                    } else {
                        let ty_vec = vec!(&i1_ty);
                        let str_vec = vec!(result.as_str(), "%replace_phi_0", "1");
                        program.push_instr(InstructionType::Load, str_vec, ty_vec); 
                    }
                    Ok((i1_ty, result))
                } // if const else
            }, // LOrExp::Or
        } // match self
    } // fn{{
}

/// 实现方式同与布尔表达式LAndExp一致，差异之在于标号命名和计算结果的处理上
impl Generate for LOrExp {
    type Out = (SymbolType, String);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Result<Self::Out, Box<dyn Error>> {
        match self {
            LOrExp::LAndExp(exp) => exp.generate(program, scopes, labels),
            LOrExp::Or(exp1, exp2) => {
                // 计算LHS
                let boolean = SymbolWidth::I1;
                let (ty1, mut op1) = exp1.generate(program, scopes, labels)?;

                if ty1.is_const {
                    let not_zero = !num_is_zero(&ty1, &op1);
                    let res: String;
                    let is_const: bool;
                    if not_zero { // 结果为真
                        res = String::from("1");
                        is_const = true;
                    } else { // 结果为假
                        // 计算RHS
                        let (ty2, mut op2) = exp2.generate(program, scopes, labels)?;
                        if ty2.is_const { 
                            let not_zero2 = !num_is_zero(&ty2, &op2);
                            if not_zero2 {
                                res = String::from("1");
                                is_const = true;
                            } else {
                                res = String::from("0");
                                is_const = true;
                            } // not_zero2 else
                        } else {
                            if ty2.width != boolean {
                                let new_op2 = labels.pop_num_str(); 
                                let str_vec = vec!("ne", new_op2.as_str(), "0", op2.as_str());
                                let type_vec = vec!(&ty2);
                                program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                                op2 = new_op2;
                            }
                            res = op2.clone();
                            is_const = false;
                        } // ty2 const else
                    }
                    Ok((SymbolType::new(SymbolWidth::I1, is_const), res))
                } else {
                    let or_false = labels.pop_block("or_false");
                    let or_end = labels.pop_block("or_end");
                    let this_bb = program.get_bb_label();

                    // this_bb     
                    if ty1.width != boolean {
                        let new_op1 = labels.pop_num_str(); 
                        let str_vec = vec!("ne", new_op1.as_str(), "0", op1.as_str());
                        let type_vec = vec!(&ty1);
                        program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                        op1 = new_op1;
                    }

                    let config = get_settings();
                    let use_phi = config.use_phi;
                    let i1_ty = SymbolType::new(SymbolWidth::I1, false);
                    if !use_phi {
                        let type_vec = vec!(&i1_ty);
                        let str_vec = vec!(op1.as_str(), "%replace_phi_0", "1");
                        program.push_instr(InstructionType::Store, str_vec, type_vec);
                    }

                    let ty_vec = vec!();
                    let str_vec = vec!(op1.as_str(), or_end.as_str(), or_false.as_str());
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);

                    // or_false
                    program.push_bb(or_false.as_str(), scopes);

                    // 计算RHS
                    let (ty2, mut op2) = exp2.generate(program, scopes, labels)?;
                    
                    if ty2.is_const {
                        let not_zero = !num_is_zero(&ty2, &op2);
                        if not_zero {
                            op2 = String::from("1");
                        } else {
                            op2 = String::from("0");
                        }
                    } if ty2.width != SymbolWidth::I1 {
                        let new_op2 = labels.pop_num_str(); 
                        let str_vec = vec!("ne", new_op2.as_str(), "0", op2.as_str());
                        let type_vec = vec!(&ty2);
                        program.push_instr(InstructionType::Cmp, str_vec, type_vec);
                        op2 = new_op2;
                    }
                    
                    if !use_phi {
                        let type_vec = vec!(&i1_ty);
                        let str_vec = vec!(op2.as_str(), "%replace_phi_0", "1");
                        program.push_instr(InstructionType::Store, str_vec, type_vec);
                    }

                    let second_bb = program.get_bb_label();
                    let ty_vec = vec!();
                    let str_vec = vec!("", &or_end.as_str(), "");
                    program.push_ter_instr(InstructionType::Br, str_vec, ty_vec);
                    // or_end
                    program.push_bb(or_end.as_str(), scopes);

                    let result = labels.pop_num_str();
                    let i1_ty = SymbolType::new(SymbolWidth::I1, false);
                    if use_phi { 
                        let ty_vec = vec!(&i1_ty);
                        let str_vec = vec!(result.as_str(), "1", this_bb.as_str(), op2.as_str(), second_bb.as_str());
                        program.push_phi(str_vec, ty_vec);
                    } else {
                        let ty_vec = vec!(&i1_ty);
                        let str_vec = vec!(result.as_str(), "%replace_phi_0", "1");
                        program.push_instr(InstructionType::Load, str_vec, ty_vec);
                    }
                    Ok((i1_ty, result))
                } // if const else
            }, // LOrExp::Or
        } // match self
    } // fn{
}

