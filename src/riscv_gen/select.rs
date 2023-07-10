use crate::utils::check::*;
use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;
use crate::riscv_gen::build::{NORMAL_WIDTH, PTR_WIDTH};
use crate::riscv_gen::reg::{FUNC_ARG, FLOAT_FUNC_ARG};

pub const FLOAT_PREFIX: &str = "f";

fn imm_width(imm: &str) -> SymbolWidth {
    if is_hex(imm) {
        SymbolWidth::Float
    } else {
        SymbolWidth::I32
    }
}

fn incre_cnt(cnt: &mut usize) {
    *cnt += 1;
}

fn gen_temp_label(cnt: &usize) -> String {
    let res = format!("%temp.{}", cnt);
    res
}

fn pop_temp_label(cnt: &mut usize, asm: &mut RiscV, ty: SymbolWidth) -> String {
    let res = gen_temp_label(cnt);
    asm.insert_label_type(res.as_str(), ty);
    incre_cnt(cnt);
    res
}

impl LLVMProgram {
    pub fn select_asm(&self, asm: &mut RiscV) {
        self.global_var.iter()
            .for_each(|v| { v.select_asm(asm); });
        self.func_def.iter()
            .for_each(|f| { f.select_asm(asm); });
    }
}

impl GlobalVar {
    pub fn select_asm(&self, asm: &mut RiscV) {
        if let SymbolType{width: SymbolWidth::Arr{tar, dims: _}, is_const: _} = &self.var_type {
            if tar.is_const {
                asm.push_global_const(self.var_name.as_str(), &self.var_type, self.init_vals.iter().map(|v| v.as_str()).collect());
            } else {
                asm.push_global_var(self.var_name.as_str(), &self.var_type, self.init_vals.iter().map(|v| v.as_str()).collect());
            }
        } else {
            match self.var_type {
                SymbolType{width: SymbolWidth::I32, is_const: _} | SymbolType{width: SymbolWidth::Float, is_const: _} => {
                    asm.push_global_var(self.var_name.as_str(), &self.var_type, self.init_vals.iter().map(|v| v.as_str()).collect());
                },
                _ => {
                    eprintln!("{:#?}", self);
                    todo!();
                },
            }
        }
    }
}

impl FuncDef {
    pub fn select_asm(&self, asm: &mut RiscV) {
        asm.push_func(&self.func_name.as_str()[1..], self.func_type.width.clone());
        let curr_func = asm.text.curr_func();
        let stack = &mut curr_func.stack;
        let label_type = &mut curr_func.label_type;
        let mut select_cnt = 0;

        self.allocs.iter()
            .for_each(|a|{
                match &a.0 {
                    Instruction::Alloca{result, ty, len: _} => {
                        label_type.insert(String::from(result), ty.width.clone());
                        if let SymbolWidth::Arr{tar, dims} = &ty.width {
                            if dims[0] == -1 {
                                stack.push_normal(result.as_str(), 8);
                            } else {
                                let len = tar.get_width() * dims.iter().map(|d| *d as usize).product::<usize>();
                                stack.push_normal(result.as_str(), len as isize);
                            }
                        } else {
                            stack.push_normal(result.as_str(), ty.get_width() as isize);
                        }
                    },
                    _ => panic!("Found {:?} in allocs", a),
                }
            });

        let mut int_cnt = 0;
        let mut float_cnt = 0;
        for fparam in self.fparams.iter() {
            if fparam.param_type.width == SymbolWidth::Float {
                if float_cnt >= FUNC_ARG.len() {
                    stack.push_param(fparam.param_name.as_str(), fparam.param_type.get_width() as isize);
                }
                curr_func.params.insert(String::from(&fparam.param_name), float_cnt);
                float_cnt += 1;
            } else {
                if int_cnt >= FUNC_ARG.len() {
                    stack.push_param(fparam.param_name.as_str(), fparam.param_type.get_width() as isize);
                }
                curr_func.params.insert(String::from(&fparam.param_name), int_cnt);
                int_cnt += 1;
            }
        }

        self.blocks.iter()
            .enumerate()
            .for_each(|(idx, b)| {
                if idx != self.blocks.len() - 1 {
                    b.select_asm(asm, Some(self.blocks[idx+1].block_label.as_str()), &self.func_name.as_str()[1..], &mut select_cnt);
                } else {
                    b.select_asm(asm, None, &self.func_name.as_str()[1..], &mut select_cnt);
                }
            });
    }
}

impl BasicBlock {
    pub fn select_asm(&self, asm: &mut RiscV, next_block: Option<&str>, func_label: &str, select_cnt: &mut usize) {
        let this_label = String::from(func_label)+"."+self.block_label.as_str();
        asm.push_block(this_label.as_str(), self.depth);

        if !self.phi_instr.is_empty() {
            panic!("Do not support emitting LLVM IR with phi instr");
        }

        for instr in self.instrs.iter() {
            instr.select_asm(asm, select_cnt);
        }
        
        if let Some(ter) = &self.ter_instr {
            if let Instruction::Br(cond, label1, label2) = ter {
                if let (Some(cond), Some(label2)) = (cond, label2) { // 有条件跳转
                    if let Some(next_block) = next_block { // 取出下一个block的标签
                        if next_block == label1 || next_block == label2 { // 存在目的地是下一个标号
                            let final_label1 = String::from(func_label)+"."+label1.as_str();
                            let final_label2 = String::from(func_label)+"."+label2.as_str();
                            let cond_val: String;
                            if is_immediate(cond.as_str()) {
                                cond_val = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                asm.gen_instr(AsmInstrType::Li, vec!(cond_val.as_str(), cond.as_str()), vec!(), vec!());
                            } else {
                                cond_val = String::from(cond);
                            }
                            if next_block == label1 {
                                asm.gen_instr(AsmInstrType::Branch, vec!("eq", cond_val.as_str(), "zero", final_label2.as_str()), vec!(), vec!());
                                // 始终将下一个block设为第一个后续block
                                asm.push_sux(final_label1.as_str());
                                asm.push_sux(final_label2.as_str());
                            } else {
                                asm.gen_instr(AsmInstrType::Branch, vec!("ne", cond_val.as_str(), "zero", final_label1.as_str()), vec!(), vec!());
                                // 始终将下一个block设为第一个后续block
                                asm.push_sux(final_label2.as_str());
                                asm.push_sux(final_label1.as_str());
                            } // 两个目的地必然不同，若相同，则不会设置为有条件跳转
                        } else { // 有条件跳转一般会有一个目的地为下一个block
                            todo!();
                        }
                    } else { // 最后一个基本块不返回，不符合情况
                        panic!("Should not appear");
                    }
                } else { // 无条件跳转
                    if let Some(next_block) = next_block { // 取出下一个block的标签
                        let final_label1 = String::from(func_label)+"."+label1.as_str();
                        if next_block != label1 { // 下一块的标号不是当前跳转目标，进行跳转
                            asm.gen_instr(AsmInstrType::Jump, vec!(final_label1.as_str()), vec!(), vec!())
                        }
                        asm.push_sux(final_label1.as_str());
                    } else { // 最后一个基本块不返回，不符合情况
                        panic!("Should not appear");
                    }
                }
            } else {
                ter.select_asm(asm, select_cnt);
            }
        } else if next_block.is_none() { // 最后一个基本块无终结指令，手动添加
            asm.gen_instr(AsmInstrType::Ret, vec!(), vec!(), vec!());
        }
    }
}

impl Instruction {
    fn load_float_imm(asm: &mut RiscV, select_cnt: &mut usize, op: &String) -> String {
        use crate::utils::float::double_to_float;
        let imm = double_to_float(op.as_str());
        let imm_id = asm.rodata.push_float_imm(imm.as_str());
        let imm_label = RoDataSection::format_float_imm(imm_id);
        let imm_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
        let final_op = pop_temp_label(select_cnt, asm, SymbolWidth::Float);
        asm.gen_instr(AsmInstrType::La, vec!(imm_addr.as_str(), imm_label.as_str()), vec!(), vec!());
        asm.gen_instr(AsmInstrType::Load, vec!(final_op.as_str(), imm_addr.as_str(), "0", FLOAT_PREFIX), vec!(NORMAL_WIDTH), vec!());
        final_op
    }

    fn check_float_op(asm: &mut RiscV, select_cnt: &mut usize, op: &String) -> String {
        if is_immediate(op) {
            Self::load_float_imm(asm, select_cnt, op)
        } else {
            String::from(op)
        }
    }

    pub fn select_asm(&self, asm: &mut RiscV, select_cnt: &mut usize) {
        match self {
            Instruction::Add(BinaryOp{result, ty, op1, op2}) => {
                asm.insert_label_type(result.as_str(), ty.width.clone());
                if is_immediate(op1) || is_immediate(op2) {
                    let op: &str;
                    let imm: &str;
                    if is_immediate(op1) {
                        imm = op1.as_str();
                        op = op2.as_str();
                    } else {
                        imm = op2.as_str();
                        op = op1.as_str();
                    }
                    if inside_imm_range(imm) {
                        asm.gen_instr(AsmInstrType::Addi, vec!(result.as_str(), op, imm), vec!(), vec!());
                    } else {
                        let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Add, vec!(result.as_str(), op, li_res.as_str()), vec!(), vec!());
                    }
                } else {
                    asm.gen_instr(AsmInstrType::Add, vec!(result.as_str(), op1, op2), vec!(), vec!());
                }
            },
            Instruction::Sub(BinaryOp{result, ty, op1, op2}) => {
                asm.insert_label_type(result.as_str(), ty.width.clone());
                if is_immediate(op2) {
                    let imm: String;
                    if &op2[0..1] == "-" {
                        imm = format!("{}", &op2[1..]);
                    } else {
                        imm = format!("-{}", op2);
                    }

                    if inside_imm_range(imm.as_str()) {
                        asm.gen_instr(AsmInstrType::Addi, vec!(result.as_str(), op1.as_str(), imm.as_str()), vec!(), vec!())
                    } else {
                        let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Add, vec!(result.as_str(), op1.as_str(), li_res.as_str()), vec!(), vec!());
                    }
                } else {
                    let op: &str;
                    let li_dst = gen_temp_label(select_cnt);
                    if is_immediate(op1) {
                        incre_cnt(select_cnt);
                        asm.insert_label_type(li_dst.as_str(), imm_width(op1.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                        op = li_dst.as_str();
                    } else {
                        op = op1.as_str();
                    }
                    asm.gen_instr(AsmInstrType::Sub, vec!(result.as_str(), op, op2), vec!(), vec!())
                }
            },
            Instruction::Mul(BinaryOp{result, ty, op1, op2}) => {
                asm.insert_label_type(result.as_str(), ty.width.clone());
                if is_immediate(op1) || is_immediate(op2) {
                    let op: &str;
                    let li_dst: String;
                    if is_immediate(op1) {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op1.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                        op = op2.as_str();
                    } else {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                        op = op1.as_str();
                    }
                    asm.gen_instr(AsmInstrType::Mul, vec!(result.as_str(), op, li_dst.as_str()), vec!(NORMAL_WIDTH), vec!());
                } else {
                    asm.gen_instr(AsmInstrType::Mul, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(NORMAL_WIDTH), vec!())
                }
            },
            Instruction::Sdiv(BinaryOp{result, ty, op1, op2}) => {
                asm.insert_label_type(result.as_str(), ty.width.clone());
                if is_immediate(op1) || is_immediate(op2) {
                    let li_dst: String;
                    if is_immediate(op1) {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op1.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Div, vec!(result.as_str(), li_dst.as_str(), op2.as_str()), vec!(NORMAL_WIDTH), vec!());
                    } else {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Div, vec!(result.as_str(), op1.as_str(), li_dst.as_str()), vec!(NORMAL_WIDTH), vec!());
                    }
                } else {
                    asm.gen_instr(AsmInstrType::Div, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(NORMAL_WIDTH), vec!())
                }
            },
            Instruction::Srem(BinaryOp{result, ty, op1, op2}) => {
                asm.insert_label_type(result.as_str(), ty.width.clone());
                if is_immediate(op1) || is_immediate(op2) {
                    let li_dst: String;
                    if is_immediate(op1) {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op1.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Rem, vec!(result.as_str(), li_dst.as_str(), op2.as_str()), vec!(NORMAL_WIDTH), vec!());
                    } else {
                        li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Rem, vec!(result.as_str(), op1.as_str(), li_dst.as_str()), vec!(NORMAL_WIDTH), vec!());
                    }
                } else {
                    asm.gen_instr(AsmInstrType::Rem, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(NORMAL_WIDTH), vec!());
                }
            },
            Instruction::Cmp(cond, BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::I32);
                let op1_is_imm = is_immediate(op1);
                let op2_is_imm = is_immediate(op2);
                let exist_imm = op1_is_imm || op2_is_imm;
                match cond.as_str() {
                    "eq" => {
                        if op1 == "0" {
                            if op2_is_imm {
                                let li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                                asm.gen_instr(AsmInstrType::Seqz, vec!(result.as_str(), li_dst.as_str()), vec!(), vec!());
                            } else {
                                asm.gen_instr(AsmInstrType::Seqz, vec!(result.as_str(), op2.as_str()), vec!(), vec!());
                            }
                            return;
                        }

                        let sub_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        if exist_imm {
                            if op1_is_imm {
                                let imm: String;
                                if &op1[0..1] == "-" {
                                    imm = format!("{}", &op1[1..]);
                                } else {
                                    imm = format!("-{}", op1);
                                }
                                if inside_imm_range(imm.as_str()) {
                                    asm.gen_instr(AsmInstrType::Addi, vec!(sub_res.as_str(), op2.as_str(), imm.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Add, vec!(sub_res.as_str(), op2.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            } else {
                                let imm: String;
                                if &op2[0..1] == "-" {
                                    imm = format!("{}", &op2[1..]);
                                } else {
                                    imm = format!("-{}", op2);
                                }
                                if inside_imm_range(imm.as_str()) {
                                    asm.gen_instr(AsmInstrType::Addi, vec!(sub_res.as_str(), op1.as_str(), imm.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Add, vec!(sub_res.as_str(), op1.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            }
                            asm.gen_instr(AsmInstrType::Seqz, vec!(result.as_str(), sub_res.as_str()), vec!(), vec!());
                        } else {
                            asm.gen_instr(AsmInstrType::Sub, vec!(sub_res.as_str(), op1.as_str(), op2.as_str()), vec!(), vec!());
                            asm.gen_instr(AsmInstrType::Seqz, vec!(result.as_str(), sub_res.as_str()), vec!(), vec!());
                        }
                    },
                    "ne" => {
                        if op1 == "0" {
                            if op2_is_imm {
                                let li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                                asm.gen_instr(AsmInstrType::Snez, vec!(result.as_str(), li_dst.as_str()), vec!(), vec!());
                            } else {
                                asm.gen_instr(AsmInstrType::Snez, vec!(result.as_str(), op2.as_str()), vec!(), vec!());
                            }
                            return;
                        }

                        let sub_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        if exist_imm {
                            if op1_is_imm {
                                let imm: String;
                                if &op1[0..1] == "-" {
                                    imm = format!("{}", &op1[1..]);
                                } else {
                                    imm = format!("-{}", op1);
                                }
                                if inside_imm_range(imm.as_str()) {
                                    asm.gen_instr(AsmInstrType::Addi, vec!(sub_res.as_str(), op2.as_str(), imm.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Add, vec!(sub_res.as_str(), op2.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            } else {
                                let imm: String;
                                if &op2[0..1] == "-" {
                                    imm = format!("{}", &op2[1..]);
                                } else {
                                    imm = format!("-{}", op2);
                                }
                                if inside_imm_range(imm.as_str()) {
                                    asm.gen_instr(AsmInstrType::Addi, vec!(sub_res.as_str(), op1.as_str(), imm.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), imm.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Add, vec!(sub_res.as_str(), op1.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            }
                            asm.gen_instr(AsmInstrType::Snez, vec!(result.as_str(), sub_res.as_str()), vec!(), vec!());
                        } else {
                            asm.gen_instr(AsmInstrType::Sub, vec!(sub_res.as_str(), op1.as_str(), op2.as_str()), vec!(), vec!());
                            asm.gen_instr(AsmInstrType::Snez, vec!(result.as_str(), sub_res.as_str()), vec!(), vec!());
                        }
                    },
                    "slt" => {
                        if exist_imm {
                            if op1_is_imm {
                                let li_dst = pop_temp_label(select_cnt, asm, imm_width(op1.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                                asm.gen_instr(AsmInstrType::Slt, vec!(result.as_str(), li_dst.as_str(), op1.as_str()), vec!(), vec!());
                            } else {
                                if inside_imm_range(op2.as_str()) {
                                    asm.gen_instr(AsmInstrType::Slti, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), op2.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Slt, vec!(result.as_str(), op1.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            }
                        } else {
                            asm.gen_instr(AsmInstrType::Slt, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(), vec!());
                        }
                    }, 
                    "sgt" => {
                        if exist_imm {
                            if op1_is_imm {
                                if inside_imm_range(op1.as_str()) {
                                    asm.gen_instr(AsmInstrType::Slti, vec!(result.as_str(), op2.as_str(), op1.as_str()), vec!(), vec!());
                                } else {
                                    let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                                    asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), op1.as_str()), vec!(), vec!());
                                    asm.gen_instr(AsmInstrType::Slt, vec!(result.as_str(), op2.as_str(), li_res.as_str()), vec!(), vec!());
                                }
                            } else {
                                let li_dst = pop_temp_label(select_cnt, asm, imm_width(op2.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                                asm.gen_instr(AsmInstrType::Sgt, vec!(result.as_str(), op1.as_str(), li_dst.as_str()), vec!(), vec!());
                            }
                        } else {
                            asm.gen_instr(AsmInstrType::Sgt, vec!(result.as_str(), op1.as_str(), op2.as_str()), vec!(), vec!());
                        }
                    },
                    "sle" => {
                        let op1_final: &str;
                        let op2_final: &str;
                        let li_dst = gen_temp_label(select_cnt);
                        if exist_imm {
                            incre_cnt(select_cnt);
                            if op1_is_imm {
                                asm.insert_label_type(li_dst.as_str(), imm_width(op1.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                                op1_final = li_dst.as_str();
                                op2_final = op2.as_str();
                            } else {
                                asm.insert_label_type(li_dst.as_str(), imm_width(op2.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                                op1_final = op1.as_str();
                                op2_final = li_dst.as_str();
                            }
                        } else {
                            op1_final = op1.as_str();
                            op2_final = op2.as_str();
                        }
                        let gt_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        asm.gen_instr(AsmInstrType::Sgt, vec!(gt_res.as_str(), op1_final, op2_final), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Xori, vec!(result.as_str(), gt_res.as_str(), "1"), vec!(), vec!());
                    },
                    "sge" => {
                        let op1_final: &str;
                        let op2_final: &str;
                        let li_dst = gen_temp_label(select_cnt);
                        if exist_imm {
                            incre_cnt(select_cnt);
                            if op1_is_imm {
                                asm.insert_label_type(li_dst.as_str(), imm_width(op1.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op1.as_str()), vec!(), vec!());
                                op1_final = li_dst.as_str();
                                op2_final = op2.as_str();
                            } else {
                                asm.insert_label_type(li_dst.as_str(), imm_width(op2.as_str()));
                                asm.gen_instr(AsmInstrType::Li, vec!(li_dst.as_str(), op2.as_str()), vec!(), vec!());
                                op1_final = op1.as_str();
                                op2_final = li_dst.as_str();
                            }
                        } else {
                            op1_final = op1.as_str();
                            op2_final = op2.as_str();
                        }
                        let lt_res = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        asm.gen_instr(AsmInstrType::Slt, vec!(lt_res.as_str(), op1_final, op2_final), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Xori, vec!(result.as_str(), lt_res.as_str(), "1"), vec!(), vec!());
                    },
                    _ => panic!("Do not support other Icmp condition."),
                }
            },
            Instruction::Fadd(BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::Float);
                let op1_final = Self::check_float_op(asm, select_cnt, op1);
                let op2_final = Self::check_float_op(asm, select_cnt, op2);
                asm.gen_instr(AsmInstrType::Fadd, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
            },
            Instruction::Fsub(BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::Float);
                let op1_final = Self::check_float_op(asm, select_cnt, op1);
                let op2_final = Self::check_float_op(asm, select_cnt, op2);
                asm.gen_instr(AsmInstrType::Fsub, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
            },
            Instruction::Fmul(BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::Float);
                let op1_final = Self::check_float_op(asm, select_cnt, op1);
                let op2_final = Self::check_float_op(asm, select_cnt, op2);
                asm.gen_instr(AsmInstrType::Fmul, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
            },
            Instruction::Fdiv(BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::Float);
                let op1_final = Self::check_float_op(asm, select_cnt, op1);
                let op2_final = Self::check_float_op(asm, select_cnt, op2);
                asm.gen_instr(AsmInstrType::Fdiv, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
            },
            Instruction::Fcmp(cond, BinaryOp{result, ty: _, op1, op2}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::I32);
                let op1_final = Self::check_float_op(asm, select_cnt, op1);
                let op2_final = Self::check_float_op(asm, select_cnt, op2);

                match cond.as_str() {
                    "oeq" => {
                        asm.gen_instr(AsmInstrType::Feq, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
                    },
                    "one" => {
                        let is_eq = pop_temp_label(select_cnt, asm, SymbolWidth::I32);
                        asm.gen_instr(AsmInstrType::Feq, vec!(is_eq.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
                        asm.gen_instr(AsmInstrType::Xori, vec!(result.as_str(), is_eq.as_str(), "1"), vec!(), vec!());
                    },
                    "olt" => {
                        asm.gen_instr(AsmInstrType::Flt, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
                    },
                    "ogt" => {
                        asm.gen_instr(AsmInstrType::Fle, vec!(result.as_str(), op2_final.as_str(), op1_final.as_str()), vec!(), vec!());
                    },
                    "ole" => {
                        asm.gen_instr(AsmInstrType::Fle, vec!(result.as_str(), op1_final.as_str(), op2_final.as_str()), vec!(), vec!());
                    },
                    "oge" => {
                        asm.gen_instr(AsmInstrType::Flt, vec!(result.as_str(), op2_final.as_str(), op1_final.as_str()), vec!(), vec!());
                    },
                    _ => panic!("Do not support other Fcmp condition."),
                }
            },
            Instruction::Store{ty, value, ptr, len: _} => {
                asm.insert_label_type(ptr.as_str(), SymbolWidth::I64);
                let curr_func = asm.text.curr_func();
                if let Some(idx) = curr_func.params.get(value) { // value为函数参数
                    let idx = *idx;
                    if ty.width == SymbolWidth::Float {
                        if idx < FLOAT_FUNC_ARG.len() {
                            asm.gen_instr(AsmInstrType::Store, vec!(FLOAT_FUNC_ARG[idx], "sp", ptr.as_str(), FLOAT_PREFIX), vec!(NORMAL_WIDTH), vec!());
                        } else {
                            let load_dst = pop_temp_label(select_cnt, asm, ty.width.clone());
                            asm.gen_instr(AsmInstrType::Load, vec!(load_dst.as_str(), "sp", value.as_str(), FLOAT_PREFIX), vec!(NORMAL_WIDTH), vec!());
                            asm.gen_instr(AsmInstrType::Store, vec!(load_dst.as_str(), "sp", ptr.as_str(), FLOAT_PREFIX), vec!(NORMAL_WIDTH), vec!());
                        }
                    } else {
                        let mut size = 4;
                        if let SymbolWidth::Arr{tar: _, dims} = &ty.width {
                            if dims[0] == -1 {
                                size = 8;
                            }
                        }
                        if idx < FUNC_ARG.len() {
                            asm.gen_instr(AsmInstrType::Store, vec!(FUNC_ARG[idx], "sp", ptr.as_str()), vec!(size), vec!());
                        } else {
                            let load_dst = pop_temp_label(select_cnt, asm, ty.width.clone());
                            asm.gen_instr(AsmInstrType::Load, vec!(load_dst.as_str(), "sp", value.as_str()), vec!(size), vec!());
                            asm.gen_instr(AsmInstrType::Store, vec!(load_dst.as_str(), "sp", ptr.as_str()), vec!(size), vec!());
                        }
                    }
                    return;
                }

                let final_value: String;
                if is_immediate(value.as_str()) {
                    if ty.width == SymbolWidth::Float {
                        final_value = Self::load_float_imm(asm, select_cnt, value);
                    } else {
                        final_value = pop_temp_label(select_cnt, asm, imm_width(value.as_str()));
                        asm.gen_instr(AsmInstrType::Li, vec!(final_value.as_str(), value.as_str()), vec!(), vec!());
                    }
                } else {
                    final_value = String::from(value);
                }

                let mut prefix = "";
                if ty.width == SymbolWidth::Float {
                    prefix = FLOAT_PREFIX;
                }

                let inside_stack = asm.text.curr_func().stack.pushed.contains(ptr.as_str());
                let pure_ptr = &ptr.as_str()[1..];
                if is_num_label(ptr.as_str()) {
                    asm.gen_instr(AsmInstrType::Store, vec!(final_value.as_str(), ptr.as_str(), "0", prefix), vec!(NORMAL_WIDTH), vec!());
                } else if inside_stack {
                    // ptr的最终位置在dump时确定
                    asm.gen_instr(AsmInstrType::Store, vec!(final_value.as_str(), "sp", ptr.as_str(), prefix), vec!(NORMAL_WIDTH), vec!());
                } else if asm.data.labels.contains(pure_ptr) {
                    let store_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                    asm.gen_instr(AsmInstrType::La, vec!(store_addr.as_str(), pure_ptr), vec!(), vec!());
                    asm.gen_instr(AsmInstrType::Store, vec!(final_value.as_str(), store_addr.as_str(), "0", prefix), vec!(NORMAL_WIDTH), vec!());
                } else {
                    panic!("declaration of {} can not be found", ptr);
                }
            },
            Instruction::Load{result, ty, ptr, len: _} => {
                let mut res_width = ty.width.clone();
                let mut prefix = "";
                if res_width == SymbolWidth::Float {
                    prefix = FLOAT_PREFIX;
                }
                let mut load_width = 4;
                if let SymbolWidth::Arr{tar: _, dims} = &ty.width {
                    if dims[0] == -1 {
                        res_width = SymbolWidth::I64;
                        load_width = 8;
                    }
                }
                asm.insert_label_type(ptr.as_str(), SymbolWidth::I64);
                asm.insert_label_type(result.as_str(), res_width);

                // ptr的最终位置在dump时确定
                let load_res = pop_temp_label(select_cnt, asm, ty.width.clone());
                let inside_stack = asm.text.curr_func().stack.pushed.contains(ptr.as_str());
                let pure_ptr = &ptr.as_str()[1..];
                if is_num_label(ptr.as_str()) {
                    asm.gen_instr(AsmInstrType::Load, vec!(load_res.as_str(), ptr.as_str(), "0", prefix), vec!(load_width), vec!());
                } else if inside_stack {
                    asm.gen_instr(AsmInstrType::Load, vec!(load_res.as_str(), "sp", ptr.as_str(), prefix), vec!(load_width), vec!());
                } else if asm.data.labels.contains(pure_ptr) || asm.rodata.labels.contains(pure_ptr) {
                    let load_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                    asm.gen_instr(AsmInstrType::La, vec!(load_addr.as_str(), pure_ptr), vec!(), vec!());
                    asm.gen_instr(AsmInstrType::Load, vec!(load_res.as_str(), load_addr.as_str(), "0", prefix), vec!(load_width), vec!());
                } else {
                    panic!("declaration of {} can not be found", ptr);
                }
                if load_width < 8 && prefix == "" {
                    asm.gen_instr(AsmInstrType::Sextw, vec!(result.as_str(), load_res.as_str()), vec!(), vec!());
                } else {
                    if prefix == FLOAT_PREFIX {
                        asm.gen_instr(AsmInstrType::Fmv, vec!(result.as_str(), load_res.as_str()), vec!(), vec!(SymbolWidth::Float, SymbolWidth::Float));
                    } else {
                        asm.gen_instr(AsmInstrType::Mv, vec!(result.as_str(), load_res.as_str()), vec!(), vec!());
                    }
                }
            },
            Instruction::Ret(_, ret_val) => {
                if let Some(ret_val) = ret_val {
                    asm.gen_instr(AsmInstrType::Ret, vec!(ret_val.as_str()), vec!(), vec!());
                } else {
                    asm.gen_instr(AsmInstrType::Ret, vec!(), vec!(), vec!());
                }
            },
            Instruction::ZeroExt(ConverOp{result, ty: _, value, ty2: _}) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::I32);
                asm.gen_instr(AsmInstrType::Mv, vec!(result.as_str(), value.as_str()), vec!(), vec!());
            },
            Instruction::I32ToFloat(ConverOp{result, ty, value, ty2}) | Instruction::Fptosi(ConverOp{result, ty, value, ty2})=> {
                asm.insert_label_type(result.as_str(), ty2.width.clone());
                asm.gen_instr(AsmInstrType::Fcvt, vec!(result.as_str(), value.as_str()), vec!(), vec!(ty2.width.clone(), ty.width.clone()));
            },
            Instruction::BitCast(result, _, ptr, _) => {
                asm.insert_label_type(result.as_str(), SymbolWidth::I64);
                let inside_stack = asm.text.curr_func().stack.pushed.contains(ptr.as_str());
                if inside_stack { // 在栈内
                    let ptr_pos = format!("#{}", ptr);
                    asm.gen_instr(AsmInstrType::Addi, vec!(result.as_str(), "sp", ptr_pos.as_str()), vec!(), vec!());
                } else {
                    if asm.rodata.labels.contains(ptr.as_str()) || asm.data.labels.contains(ptr.as_str()) {
                        asm.gen_instr(AsmInstrType::La, vec!(result.as_str(), ptr.as_str()), vec!(), vec!());
                    } else {
                        asm.gen_instr(AsmInstrType::Mv, vec!(result.as_str(), ptr.as_str()), vec!(), vec!());
                    }
                }
            },
            Instruction::GetElemPtr(dst, SymbolType{width: SymbolWidth::Arr{tar: _, dims}, is_const: _}, ptr, idx) => {
                asm.insert_label_type(dst.as_str(), SymbolWidth::I64);

                let inside_stack = asm.text.curr_func().stack.pushed.contains(ptr.as_str());
                let start_addr: String;
                if is_num_label(ptr) {
                    start_addr = String::from(ptr);
                    if idx.len() == 1 {
                        if idx[0] != "0" {
                            let size = dims.iter().map(|i| *i as usize).product::<usize>() * 4;
                            if is_immediate(idx[0].as_str()) {
	                            let this_idx = idx[0].parse::<usize>().unwrap() * size;
	                            let this_idx = format!("{}", this_idx);
	                            let li_res = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
	                            asm.gen_instr(AsmInstrType::Li, vec!(li_res.as_str(), this_idx.as_str()), vec!(), vec!());
	                            asm.gen_instr(AsmInstrType::Add, vec!(dst.as_str(), start_addr.as_str(), li_res.as_str()), vec!(), vec!());
	                        } else {
	                            let size_str = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
	                            let this_idx = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
	                            let size = format!("{}", size);
	                            asm.gen_instr(AsmInstrType::Li, vec!(size_str.as_str(), size.as_str()), vec!(), vec!());
	                            asm.gen_instr(AsmInstrType::Mul, vec!(this_idx.as_str(), idx[0].as_str(), size_str.as_str()), vec!(PTR_WIDTH), vec!());
	                            asm.gen_instr(AsmInstrType::Add, vec!(dst.as_str(), start_addr.as_str(), this_idx.as_str()), vec!(), vec!());
	                        }
                        } else {
                            asm.gen_instr(AsmInstrType::Mv, vec!(dst.as_str(), ptr.as_str()), vec!(), vec!());
                        }
                        return;
                    }
                } else {
                    if inside_stack { // 在栈内
                        let ptr_pos = format!("#{}", ptr);
                        if idx.len() == 1 {
                            asm.gen_instr(AsmInstrType::Addi, vec!(dst.as_str(), "sp", ptr_pos.as_str()), vec!(), vec!());
                            return;
                        }
                        start_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                        asm.gen_instr(AsmInstrType::Addi, vec!(start_addr.as_str(), "sp", ptr_pos.as_str()), vec!(), vec!());
                    } else {
                        let pure_ptr = &ptr[1..];
                        if asm.rodata.labels.contains(pure_ptr) || asm.data.labels.contains(pure_ptr) {
                            if idx.len() == 1 {
                                asm.gen_instr(AsmInstrType::La, vec!(dst.as_str(), pure_ptr), vec!(), vec!());
                                return;
                            }
                            start_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                            asm.gen_instr(AsmInstrType::La, vec!(start_addr.as_str(), pure_ptr), vec!(), vec!());
                        } else {
                            panic!("Undefined pointer {}", ptr);
                        }
                    }
                } // else

                let mut last_addr = start_addr;
                let mut left_size = dims.iter().map(|d| *d as usize).product::<usize>() * 4;
                let mut next_addr: String;
                for cnt in 1..idx.len() {
                    left_size /= dims[cnt-1] as usize;

                    if cnt == idx.len() - 1 {
                        next_addr = String::from(dst);
                    } else {
                        next_addr = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                    }
                    if is_immediate(idx[cnt].as_str()) {
                        let this_idx = idx[cnt].parse::<usize>().unwrap() * left_size;
                        if this_idx == 0 {
                            asm.gen_instr(AsmInstrType::Mv, vec!(next_addr.as_str(), last_addr.as_str()), vec!(), vec!());
                        } else {
                            let this_idx = format!("{}", this_idx);
                            let index = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                            asm.gen_instr(AsmInstrType::Li, vec!(index.as_str(), this_idx.as_str()), vec!(), vec!());
                            asm.gen_instr(AsmInstrType::Add, vec!(next_addr.as_str(), last_addr.as_str(), index.as_str()), vec!(), vec!());
                        }
                    } else {
                        if left_size == 0 {
                            asm.gen_instr(AsmInstrType::Mv, vec!(next_addr.as_str(), last_addr.as_str()), vec!(), vec!());
                        } else {
                            let left_size_str = format!("{}", left_size);
                            let base = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                            let index = pop_temp_label(select_cnt, asm, SymbolWidth::I64);
                            asm.gen_instr(AsmInstrType::Li, vec!(base.as_str(), left_size_str.as_str()), vec!(), vec!());
                            asm.gen_instr(AsmInstrType::Mul, vec!(index.as_str(), idx[cnt].as_str(), base.as_str()), vec!(NORMAL_WIDTH), vec!());
                            asm.gen_instr(AsmInstrType::Add, vec!(next_addr.as_str(), last_addr.as_str(), index.as_str()), vec!(), vec!());
                        }
                    }
                    last_addr = next_addr;
                }
            },
            Instruction::Comment(_) => {}, // 跳过注释
            Instruction::Call(res, label, ty, params) => {
                asm.mark_call();
                if &label[1..] == "llvm.memset.p018.i64" {
                    let str_vec = vec!(res.as_str(), "memset", params[0].0.as_str(), params[1].0.as_str(), params[2].0.as_str());
                    let ty_vec = vec!(ty.width.clone(), params[0].1.width.clone(), params[1].1.width.clone(), params[2].1.width.clone());
                    asm.gen_instr(AsmInstrType::Call, str_vec, vec!(), ty_vec);
                    return;
                }
                asm.insert_label_type(res.as_str(), ty.width.clone());
                let mut str_vec = vec!(res.as_str(), &label[1..]);
                let mut new_params = params.iter().map(|(s, _)| s.as_str()).collect::<Vec<_>>();
                str_vec.append(&mut new_params);
                let mut ty_vec = vec!(ty.width.clone());
                let mut param_ty = params.iter().map(|(_, t)| t.width.clone()).collect::<Vec<_>>();
                ty_vec.append(&mut param_ty);
                asm.gen_instr(AsmInstrType::Call, str_vec, vec!(), ty_vec);
            },
            _ => {
                eprintln!("{:?}", self);
                todo!()
            },
        }
    }
}

