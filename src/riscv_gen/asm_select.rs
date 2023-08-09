use std::collections::HashSet;

use crate::utils::check::*;
use crate::utils::float::*;
use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;
use crate::structures::riscv_regs::*;
use crate::riscv_gen::select_utils::*;

fn pop_temp_label(cnt: &mut usize, ty: &SymbolWidth, func: &mut AsmFunc) -> String {
    let res = format!("%temp.{}", cnt);
    insert_label_type(&res, ty, func);
    *cnt += 1;
    res
}

impl LLVMProgram {
    pub fn asm_select(&self, asm: &mut RiscV) {
        self.func_def.iter().for_each(|func| func.asm_select(asm));
    }
}

impl FuncDef {
    pub fn asm_select(&self, asm: &mut RiscV) {
        if let Some(func) = asm.text.funcs.iter_mut().find(|func| func.label == self.func_name.replace("@", "")) {
            let mut select_cnt = 0;
            self.blocks.iter().enumerate().for_each(|(idx, block)| {
                if idx < self.blocks.len() - 1 {
                    block.select_asm(Some(&self.blocks[idx+1].block_label),
                        &self.func_name.replace("@", ""), 
                        &mut select_cnt, 
                        func, 
                        &asm.data.labels);
                } else {
                    block.select_asm( None, 
                        &self.func_name.replace("@", ""), 
                        &mut select_cnt, 
                        func, 
                        &asm.data.labels);
                }
            });
        }
    }
}

impl Block {
    pub fn select_asm(&self, next_block: Option<&str>, func_label: &str, select_cnt: &mut usize, func: &mut AsmFunc, global_vars: &HashSet<String>) {
        let this_label: String = func_label.to_string()+"."+&self.block_label;
        push_block(&this_label, self.depth, func);
        self.nor_ins.iter().for_each(|instr| instr.select_asm(select_cnt, func, global_vars));
        if let Some(ter) = &self.ter_ins {
            if let Instruction::Br(cond, label1, label2) = ter {
                if let (Some(cond), Some(label2)) = (cond, label2) { // 有条件跳转
                    if let Some(next_block) = next_block { // 取出下一个block的标签
                        if next_block == label1 || next_block == label2 { // 存在目的地是下一个标号
                            let final_label1 = String::from(func_label)+"."+label1;
                            let final_label2 = String::from(func_label)+"."+label2;
                            let cond_val: String;
                            if is_immediate(cond) {
                                cond_val = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec!(&cond_val, cond), None, vec![], func);
                            } else {
                                cond_val = String::from(cond);
                            }
                            if next_block == label1 {
                                gen_instr(AsmInstructionType::Branch, vec!("eq", &cond_val, "zero", &final_label2), None, vec![], func);
                                // 始终将下一个block设为第一个后续block
                                push_successor(&final_label1, func);
                                push_successor(&final_label2, func);
                            } else {
                                gen_instr(AsmInstructionType::Branch, vec!("ne", &cond_val, "zero", &final_label1), None, vec![], func);
                                // 始终将下一个block设为第一个后续block
                                push_successor(&final_label2, func);
                                push_successor(&final_label1, func);
                            } // 两个目的地必然不同，若相同，则不会设置为有条件跳转
                        } else { // 有条件跳转一般会有一个目的地为下一个block
                            todo!();
                        }
                    } else { // 最后一个基本块不返回，不符合情况
                        panic!("Should not appear");
                    }
                } else { // 无条件跳转
                    if let Some(next_block) = next_block { // 取出下一个block的标签
                        let final_label1 = String::from(func_label)+"."+&label1;
                        if next_block != label1 { // 下一块的标号不是当前跳转目标，进行跳转
                            gen_instr(AsmInstructionType::Jump, vec!(&final_label1), None, vec![], func)
                        }
                        push_successor(&final_label1, func);
                    } else { // 最后一个基本块不返回，不符合情况
                        panic!("Should not appear");
                    }
                }
            } else {
                ter.select_asm(select_cnt, func, global_vars);
            }
        } else if next_block.is_none() { // 最后一个基本块无终结指令，手动添加
            gen_instr(AsmInstructionType::Ret, vec![], None, vec![], func);
        }
    }
}

impl Instruction {
    //将一个浮点立即数load至寄存器中。
    //select_cnt：虚拟寄存器编号
    //op：浮点立即数(以IEEE754 Double的64bit形式保存)
    //返回虚拟寄存器编号，如%2
    fn load_float_imm(select_cnt: &mut usize, op: &str, func: &mut AsmFunc) -> String {
        let imm = double_to_float(&op);
        let imm_reg = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
        let dst_reg = pop_temp_label(select_cnt, &SymbolWidth::Float, func);
        gen_instr(AsmInstructionType::Li, vec!(&imm_reg, &imm), None, vec![], func);
        gen_instr(AsmInstructionType::Fmv, vec!(&dst_reg, &imm_reg), None, vec!(SymbolWidth::Float, SymbolWidth::I32), func);
        dst_reg
    }
    //检查操作数是否为浮点立即数
    fn check_float_op(select_cnt: &mut usize, op: &str, func: &mut AsmFunc) -> String {
        if is_immediate(op) {
            Self::load_float_imm(select_cnt, op, func)
        } else {
            op.to_string()
        }
    }

    pub fn select_asm(&self, select_cnt: &mut usize, func: &mut AsmFunc, global_vars: &HashSet<String>) {
        match self {
            //LLVM IR: Add指令
            Instruction::Add(BinaryOp{res, op_type, op1, op2}) => {
                //将该指令的目的寄存器与类型绑定
                insert_label_type(&res, &op_type.width, func);
                if is_immediate(op1) {
                    if inside_imm_range(op1) {
                        gen_instr(AsmInstructionType::Addi, vec![res, op2, op1], None, vec![], func);
                    } else {
                        let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec![&li_res, op1], None, vec![], func);
                        gen_instr(AsmInstructionType::Add, vec![res, op2, &li_res], None, vec![], func);
                    }
                } else if is_immediate(op2) {
                    if inside_imm_range(op2) {
                        gen_instr(AsmInstructionType::Addi, vec![res, op1, op2], None, vec![], func);
                    } else {
                        let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec![&li_res, op2], None, vec![], func);
                        gen_instr(AsmInstructionType::Add, vec![res, op1, &li_res], None, vec![], func);
                    }
                } else {
                    gen_instr(AsmInstructionType::Add, vec![res, op1, op2], None, vec![], func);
                }
            },
            Instruction::Sub(BinaryOp{res, op_type, op1, op2}) => {
                insert_label_type(res, &op_type.width, func);
                if is_immediate(op2) {
                    //求出op2的相反数
                    let imm_neg = op2.parse::<i32>().map(|num| -num).unwrap().to_string();
                    if inside_imm_range(&imm_neg) {
                        //如果能在2^11之内，则用Addi指令。
                        gen_instr(AsmInstructionType::Addi, vec![res, op1, &imm_neg], None, vec![], func);
                    } else {
                        let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec![&li_dst, &imm_neg], None, vec![], func);
                        gen_instr(AsmInstructionType::Add, vec![res, op1, &li_dst], None, vec![], func);
                    }
                } else if is_immediate(op1){
                    let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                    gen_instr(AsmInstructionType::Li, vec![&li_dst, op1], None, vec![], func);
                    gen_instr(AsmInstructionType::Sub, vec![res, &li_dst, op2], None, vec![], func);
                } else {
                    gen_instr(AsmInstructionType::Sub, vec![res, op1, op2], None, vec![], func)
                }
            },
            //这里完成了优化：强度削弱；对有一个立即数是2的幂次的立即数替换为移位指令。
            //能完成这个优化的前提是，常量折叠已做完。
            Instruction::Mul(BinaryOp{res, op_type, op1, op2}) => {
                insert_label_type(res, &op_type.width, func);
                if is_immediate(op1) { //如果op1是立即数
                    if op1.contains("-") {//如果是负数
                        let op1_positive = op1.replace("-", "");//转成正数
                        match is_poweroftwo(&op1_positive) {//如果是2的幂次
                            Some(pow) => {
                                gen_instr(AsmInstructionType::Slli, vec![res, op2, &pow.to_string()], None, vec![], func);
                                gen_instr(AsmInstructionType::Sub, vec![res, "zero", res], None, vec![], func);
                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op1], None, vec![], func);
                                gen_instr(AsmInstructionType::Mul, vec![res, &li_dst, op2], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    } else { //就是正数
                        match is_poweroftwo(op1) {
                            Some(pow) => {
                                gen_instr(AsmInstructionType::Slli, vec![res, op2, &pow.to_string()], None, vec![], func);
                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op1], None, vec![], func);
                                gen_instr(AsmInstructionType::Mul, vec![res, &li_dst, op2], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    }
                } else if is_immediate(op2) {//op2是立即数
                    if &op2[0..1] == "-" {//如果是负数
                        let op2_positive = op2.replace("-", "");//转成正数
                        match is_poweroftwo(&op2_positive) {//如果是2的幂次
                            Some(pow) => {
                                gen_instr(AsmInstructionType::Slli, vec![res, op1, &pow.to_string()], None, vec![], func);
                                gen_instr(AsmInstructionType::Sub, vec![res, "zero", res], None, vec![], func);
                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op2], None, vec![], func);
                                gen_instr(AsmInstructionType::Mul, vec![res, op1, &li_dst], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    } else { //就是正数
                        match is_poweroftwo(op2) {
                            Some(pow) => {
                                gen_instr(AsmInstructionType::Slli, vec![res, op1, &pow.to_string()], None, vec![], func);

                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op2], None, vec![], func);
                                gen_instr(AsmInstructionType::Mul, vec![res, op1, &li_dst], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    }
                } else {//都不是立即数
                    gen_instr(AsmInstructionType::Mul, vec![res, op1, op2], Some(NORMAL_WIDTH), vec![], func);
                }
            },
            Instruction::Sdiv(BinaryOp{res, op_type, op1, op2}) => {
                insert_label_type(res, &op_type.width, func);
                if is_immediate(op1) {//第一个op是立即数，只能用除法
                    let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                    gen_instr(AsmInstructionType::Li, vec![&li_dst, op1], None, vec![], func);
                    gen_instr(AsmInstructionType::Div, vec![res, &li_dst, op2], Some(NORMAL_WIDTH), vec![], func);
                    
                } else if is_immediate(op2) {
                    if &op2[0..1] == "-" {//如果是负数
                        let op2_positive = op2.replace("-", "");//转成正数
                        match is_poweroftwo(&op2_positive) {//如果是2的幂次
                            Some(pow) => {
                                let temp_reg = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Srai, vec![&temp_reg, op1, "63"], None, vec![], func);//算术右移，得到64个1或0
                                gen_instr(AsmInstructionType::Srli, vec![&temp_reg, &temp_reg, &(64-pow).to_string()], None, vec![], func);//逻辑右移，负数会得到pow个1，正数不变
                                gen_instr(AsmInstructionType::Add, vec![op1, &temp_reg, op1], None, vec![], func);
                                gen_instr(AsmInstructionType::Srai, vec![res, op1, &pow.to_string()], None, vec![], func);
                                gen_instr(AsmInstructionType::Sub, vec![res, "zero", res], None, vec![], func);

                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op2], None, vec![], func);
                                gen_instr(AsmInstructionType::Div, vec![res, op1, &li_dst], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    } else { //就是正数
                        match is_poweroftwo(op2) {
                            Some(pow) => {
                                let temp_reg = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Srai, vec![&temp_reg, op1, "63"], None, vec![], func);//算术右移，得到64个1或0
                                gen_instr(AsmInstructionType::Srli, vec![&temp_reg, &temp_reg, &(64-pow).to_string()], None, vec![], func);//逻辑右移，负数会得到pow个1，正数不变
                                gen_instr(AsmInstructionType::Add, vec![op1, &temp_reg, op1], None, vec![], func);
                                gen_instr(AsmInstructionType::Srai, vec![res, op1, &pow.to_string()], None, vec![], func);

                            },
                            None => {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec![&li_dst, op2], None, vec![], func);
                                gen_instr(AsmInstructionType::Div, vec![res, op1, &li_dst], Some(NORMAL_WIDTH), vec![], func);
                            }
                        }
                    }
                } else {
                    gen_instr(AsmInstructionType::Div, vec![res, op1, op2], Some(NORMAL_WIDTH), vec![], func)
                }
            },
            Instruction::Srem(BinaryOp{res, op_type, op1, op2}) => {
                insert_label_type(res, &op_type.width, func);
                if is_immediate(op1) || is_immediate(op2) {
                    let li_dst: String;
                    if is_immediate(op1) {
                        li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec!(&li_dst, op1), None, vec![], func);
                        gen_instr(AsmInstructionType::Rem, vec!(res, &li_dst, op2), Some(NORMAL_WIDTH), vec![], func);
                    } else {
                        li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                        gen_instr(AsmInstructionType::Rem, vec!(res, op1, &li_dst), Some(NORMAL_WIDTH), vec![], func);
                    }
                } else {
                    gen_instr(AsmInstructionType::Rem, vec!(res, op1, op2), Some(NORMAL_WIDTH), vec![], func);
                }
            },
            Instruction::Cmp(cond, BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::I32, func);
                let op1_is_imm = is_immediate(op1);
                let op2_is_imm = is_immediate(op2);
                let exist_imm = op1_is_imm || op2_is_imm;
                match cond.as_str() {
                    "eq" => {
                        if op1 == "0" {
                            if op2_is_imm {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                                gen_instr(AsmInstructionType::Seqz, vec!(res, &li_dst), None, vec![], func);
                            } else {
                                gen_instr(AsmInstructionType::Seqz, vec!(res, op2), None, vec![], func);
                            }
                            return;
                        }

                        let sub_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        if exist_imm {
                            if op1_is_imm {
                                let imm: String;
                                if &op1[0..1] == "-" {
                                    imm = format!("{}", &op1[1..]);
                                } else {
                                    imm = format!("-{}", op1);
                                }
                                if inside_imm_range(&imm) {
                                    gen_instr(AsmInstructionType::Addi, vec!(&sub_res, op2, &imm), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, &imm), None, vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&sub_res, op2, &li_res), None, vec![], func);
                                }
                            } else {
                                let imm: String;
                                if &op2[0..1] == "-" {
                                    imm = format!("{}", &op2[1..]);
                                } else {
                                    imm = format!("-{}", op2);
                                }
                                if inside_imm_range(&imm) {
                                    gen_instr(AsmInstructionType::Addi, vec!(&sub_res, op1, &imm), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, &imm), None, vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&sub_res, op1, &li_res), None, vec![], func);
                                }
                            }
                            gen_instr(AsmInstructionType::Seqz, vec!(res, &sub_res), None, vec![], func);
                        } else {
                            gen_instr(AsmInstructionType::Sub, vec!(&sub_res, op1, op2), None, vec![], func);
                            gen_instr(AsmInstructionType::Seqz, vec!(res, &sub_res), None, vec![], func);
                        }
                    },
                    "ne" => {
                        if op1 == "0" {
                            if op2_is_imm {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                                gen_instr(AsmInstructionType::Snez, vec!(res, &li_dst), None, vec![], func);
                            } else {
                                gen_instr(AsmInstructionType::Snez, vec!(res, op2), None, vec![], func);
                            }
                            return;
                        }

                        let sub_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        if exist_imm {
                            if op1_is_imm {
                                let imm: String;
                                if &op1[0..1] == "-" {
                                    imm = format!("{}", &op1[1..]);
                                } else {
                                    imm = format!("-{}", op1);
                                }
                                if inside_imm_range(&imm) {
                                    gen_instr(AsmInstructionType::Addi, vec!(&sub_res, op2, &imm), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, &imm), None, vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&sub_res, op2, &li_res), None, vec![], func);
                                }
                            } else {
                                let imm: String;
                                if &op2[0..1] == "-" {
                                    imm = format!("{}", &op2[1..]);
                                } else {
                                    imm = format!("-{}", op2);
                                }
                                if inside_imm_range(&imm) {
                                    gen_instr(AsmInstructionType::Addi, vec!(&sub_res, op1, &imm), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, &imm), None, vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&sub_res, op1, &li_res), None, vec![], func);
                                }
                            }
                            gen_instr(AsmInstructionType::Snez, vec!(res, &sub_res), None, vec![], func);
                        } else {
                            gen_instr(AsmInstructionType::Sub, vec!(&sub_res, op1, op2), None, vec![], func);
                            gen_instr(AsmInstructionType::Snez, vec!(res, &sub_res), None, vec![], func);
                        }
                    },
                    "slt" => {
                        if exist_imm {
                            if op1_is_imm {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op1), None, vec![], func);
                                gen_instr(AsmInstructionType::Slt, vec!(res, &li_dst, op1), None, vec![], func);
                            } else {
                                if inside_imm_range(op2) {
                                    gen_instr(AsmInstructionType::Slti, vec!(res, op1, op2), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, op2), None, vec![], func);
                                    gen_instr(AsmInstructionType::Slt, vec!(res, op1, &li_res), None, vec![], func);
                                }
                            }
                        } else {
                            gen_instr(AsmInstructionType::Slt, vec!(res, op1, op2), None, vec![], func);
                        }
                    }, 
                    "sgt" => {
                        if exist_imm {
                            if op1_is_imm {
                                if inside_imm_range(op1) {
                                    gen_instr(AsmInstructionType::Slti, vec!(res, op2, op1), None, vec![], func);
                                } else {
                                    let li_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&li_res, op1), None, vec![], func);
                                    gen_instr(AsmInstructionType::Slt, vec!(res, op2, &li_res), None, vec![], func);
                                }
                            } else {
                                let li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                                gen_instr(AsmInstructionType::Sgt, vec!(res, op1, &li_dst), None, vec![], func);
                            }
                        } else {
                            gen_instr(AsmInstructionType::Sgt, vec!(res, op1, op2), None, vec![], func);
                        }
                    },
                    "sle" => {
                        let op1_final: &str;
                        let op2_final: &str;
                        let li_dst: String;
                        if exist_imm {
                            li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                            if op1_is_imm {
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op1), None, vec![], func);
                                op1_final = &li_dst;
                                op2_final = op2;
                            } else {
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                                op1_final = op1;
                                op2_final = &li_dst;
                            }
                        } else {
                            op1_final = op1;
                            op2_final = op2;
                        }
                        let gt_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Sgt, vec!(&gt_res, op1_final, op2_final), None, vec![], func);
                        gen_instr(AsmInstructionType::Xori, vec!(res, &gt_res, "1"), None, vec![], func);
                    },
                    "sge" => {
                        let op1_final: &str;
                        let op2_final: &str;
                        let li_dst: String;
                        if exist_imm {
                            li_dst = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                            if op1_is_imm {
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op1), None, vec![], func);
                                op1_final = &li_dst;
                                op2_final = op2;
                            } else {
                                gen_instr(AsmInstructionType::Li, vec!(&li_dst, op2), None, vec![], func);
                                op1_final = op1;
                                op2_final = &li_dst;
                            }
                        } else {
                            op1_final = op1;
                            op2_final = op2;
                        }
                        let lt_res = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Slt, vec!(&lt_res, op1_final, op2_final), None, vec![], func);
                        gen_instr(AsmInstructionType::Xori, vec!(res, &lt_res, "1"), None, vec![], func);
                    },
                    _ => panic!("Do not support other Icmp condition."),
                }
            },
            Instruction::Fadd(BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::Float, func);
                let op1_final = Self::check_float_op(select_cnt, op1, func);
                let op2_final = Self::check_float_op(select_cnt, op2, func);
                gen_instr(AsmInstructionType::Fadd, vec!(res, &op1_final, &op2_final), None, vec![], func);
            },
            Instruction::Fsub(BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::Float, func);
                let op1_final = Self::check_float_op(select_cnt, op1, func);
                let op2_final = Self::check_float_op(select_cnt, op2, func);
                gen_instr(AsmInstructionType::Fsub, vec!(res, &op1_final, &op2_final), None, vec![], func);
            },
            Instruction::Fmul(BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::Float, func);
                let op1_final = Self::check_float_op(select_cnt, op1, func);
                let op2_final = Self::check_float_op(select_cnt, op2, func);
                gen_instr(AsmInstructionType::Fmul, vec!(res, &op1_final, &op2_final), None, vec![], func);
            },
            Instruction::Fdiv(BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::Float, func);
                let op1_final = Self::check_float_op(select_cnt, op1, func);
                let op2_final = Self::check_float_op( select_cnt, op2, func);
                gen_instr(AsmInstructionType::Fdiv, vec!(res, &op1_final, &op2_final), None, vec![], func);
            },
            Instruction::Fcmp(cond, BinaryOp{res, op_type: _, op1, op2}) => {
                insert_label_type(res, &SymbolWidth::I32, func);
                let op1_final = Self::check_float_op( select_cnt, op1, func);
                let op2_final = Self::check_float_op(select_cnt, op2, func);

                match cond.as_str() {
                    "oeq" => {
                        gen_instr(AsmInstructionType::Feq, vec!(res, &op1_final, &op2_final), None, vec![], func);
                    },
                    "one" => {
                        let is_eq = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Feq, vec!(&is_eq, &op1_final, &op2_final), None, vec![], func);
                        gen_instr(AsmInstructionType::Xori, vec!(res, &is_eq, "1"), None, vec![], func);
                    },
                    "olt" => {
                        gen_instr(AsmInstructionType::Flt, vec!(res, &op1_final, &op2_final), None, vec![], func);
                    },
                    "ogt" => {
                        gen_instr(AsmInstructionType::Fle, vec!(res, &op2_final, &op1_final), None, vec![], func);
                    },
                    "ole" => {
                        gen_instr(AsmInstructionType::Fle, vec!(res, &op1_final, &op2_final), None, vec![], func);
                    },
                    "oge" => {
                        gen_instr(AsmInstructionType::Flt, vec!(res, &op2_final, &op1_final), None, vec![], func);
                    },
                    _ => panic!("Do not support other Fcmp condition."),
                }
            },
            Instruction::Store{ty, value, ptr, len: _} => {
                insert_label_type(ptr, &SymbolWidth::I64, func);
                // 如果value为函数参数
                if let Some(idx) = func.params.get(value) { 
                    let idx = *idx;
                    if ty.width == SymbolWidth::Float {
                        //如果参数够参数寄存器的数量，则不需要用栈
                        if idx < FLOAT_FUNC_ARG.len() {
                            gen_instr(AsmInstructionType::Store, vec!(FLOAT_FUNC_ARG[idx], "sp", ptr, "f"), Some(NORMAL_WIDTH), vec![], func);
                        }
                        //参数过多，需要用栈
                        else {
                            //从栈里面pop一个位置
                            let load_dst = pop_temp_label(select_cnt, &ty.width, func);
                            gen_instr(AsmInstructionType::Load, vec!(&load_dst, "sp", value, "f"), Some(NORMAL_WIDTH), vec![], func);
                            gen_instr(AsmInstructionType::Store, vec!(&load_dst, "sp", ptr, "f"), Some(NORMAL_WIDTH), vec![], func);
                        }
                    } else {
                        let width_num = match &ty.width {
                            SymbolWidth::Arr{tar: _, dims} => {
                                //如果是指针
                                if dims[0] == -1 {
                                    Some(PTR_WIDTH)
                                } else {
                                    Some(NORMAL_WIDTH)
                                }
                            },
                            _ => Some(NORMAL_WIDTH)
                        };
                        if idx < FUNC_ARG.len() {
                            gen_instr(AsmInstructionType::Store, vec!(FUNC_ARG[idx], "sp", ptr), width_num, vec![], func);
                        } else {
                            let load_dst = pop_temp_label(select_cnt, &ty.width, func);
                            gen_instr(AsmInstructionType::Load, vec!(&load_dst, "sp", value), width_num, vec![], func);
                            gen_instr(AsmInstructionType::Store, vec!(&load_dst, "sp", ptr), width_num, vec![], func);
                        }
                    }
                    return;
                }

                let final_value: String;
                //如果是立即数，对浮点数需要先在data段声明再移动，整数则直接移动
                if is_immediate(&value) {
                    if ty.width == SymbolWidth::Float {
                        final_value = Self::load_float_imm(select_cnt, value, func);
                    } else {
                        final_value = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                        gen_instr(AsmInstructionType::Li, vec!(&final_value, value), None, vec![], func);
                    }
                } else {
                    final_value = String::from(value);
                }

                let prefix = match &ty.width {
                    SymbolWidth::Float => "f",
                    _ => ""
                };

                let inside_stack = func.stack.pushed.contains(ptr);
                let pure_ptr = &ptr[1..];
                //如果是llvm的临时标号，其实就是store分配出来的那些临时标号，比如%c_0，直接store即可。
                if is_num_label(&ptr) {
                    gen_instr(AsmInstructionType::Store, vec!(&final_value, ptr, "0", prefix), Some(NORMAL_WIDTH), vec![], func);
                } else if inside_stack {
                    // 如果在函数的栈里面，则需要通过栈偏移来store，具体值需要到分配完栈大小才能确定
                    gen_instr(AsmInstructionType::Store, vec!(&final_value, "sp", ptr, prefix), Some(NORMAL_WIDTH), vec![], func);
                } else if global_vars.contains(pure_ptr) {
                    //如果是全局变量，比如说预先声明的一个int A。则需要用La指令来移动。
                    //先加载到临时寄存器，再移动到目标地址
                    let store_addr = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                    gen_instr(AsmInstructionType::La, vec!(&store_addr, pure_ptr), None, vec![], func);
                    gen_instr(AsmInstructionType::Store, vec!(&final_value, &store_addr, "0", prefix), Some(NORMAL_WIDTH), vec![], func);
                }
            },
            Instruction::Load{res, ty, ptr, len: _} => {
                let mut res_width = ty.width.clone();
                let prefix = match &ty.width {
                    SymbolWidth::Float => "f",
                    _ => ""
                };

                let width_num = match &ty.width {
                    SymbolWidth::Arr{tar: _, dims} => {
                        //如果是指针
                        if dims[0] == -1 {
                            res_width = SymbolWidth::I64;
                            Some(PTR_WIDTH)
                        } else {
                            Some(NORMAL_WIDTH)
                        }
                    },
                    _ => Some(NORMAL_WIDTH)
                };

                insert_label_type(ptr, &SymbolWidth::I64, func);
                insert_label_type(res, &res_width, func);

                
                let load_res = pop_temp_label(select_cnt, &ty.width, func);
                let inside_stack = func.stack.pushed.contains(ptr);
                let pure_ptr = &ptr[1..];
                //和store指令的步骤一样
                if is_num_label(ptr) {
                    //如果是llvm的临时标号，其实就是store分配出来的那些临时标号，比如%c_0，直接load即可。
                    gen_instr(AsmInstructionType::Load, vec!(&load_res, ptr, "0", prefix), width_num, vec![], func);
                } else if inside_stack {
                    // 如果在函数的栈里面，则需要通过栈偏移来load，具体值需要到分配完栈大小才能确定
                    gen_instr(AsmInstructionType::Load, vec!(&load_res, "sp", ptr, prefix), width_num, vec![], func);
                } else if global_vars.contains(pure_ptr) {
                    //如果是全局变量，比如说预先声明的一个int A。则需要用La指令来移动。
                    //先加载到临时寄存器，再store到目标地址
                    let load_addr = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                    gen_instr(AsmInstructionType::La, vec!(&load_addr, pure_ptr), None, vec![], func);
                    gen_instr(AsmInstructionType::Load, vec!(&load_res, &load_addr, "0", prefix), width_num, vec![], func);
                }
                if prefix == "f" {
                //是float，用fmv
                    gen_instr(AsmInstructionType::Fmv, vec!(res, &load_res), None, vec!(SymbolWidth::Float, SymbolWidth::Float), func);
                } else {
                //是64位指针或者32位整数，使用mov指令
                    gen_instr(AsmInstructionType::Mv, vec!(res, &load_res), None, vec![], func);
                }
                
            },
            Instruction::Ret(ret_type, ret_val) => {
                 if let Some(ret_val) = ret_val {
                    match ret_type.width {
                        SymbolWidth::I32 => {
                            if is_immediate(ret_val) {
                                gen_instr(AsmInstructionType::Li, vec!(RETURN[0], &ret_val), None, vec![], func);
                            } else {
                                gen_instr(AsmInstructionType::Mv, vec!(RETURN[0], &ret_val), None, vec![], func);
                            }
                        }
                        SymbolWidth::Float => {
                            if is_immediate(ret_val) {
                                let imm = double_to_float(ret_val);
                                let imm_reg = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
                                let dst_reg = pop_temp_label(select_cnt, &SymbolWidth::Float, func);
                                gen_instr(AsmInstructionType::Li, vec!(&imm_reg, &imm), None, vec![], func);
                                gen_instr(AsmInstructionType::Fmv, vec!(&dst_reg, &imm_reg), None, vec!(SymbolWidth::Float, SymbolWidth::I32), func);
                            } else {
                                gen_instr(AsmInstructionType::Fmv, vec!(FLOAT_RETURN[0], &ret_val), None, vec!(SymbolWidth::Float, SymbolWidth::Float), func);
                            }
                        }
                        _ => panic!("Error ret type")
                    }
                }
                gen_instr(AsmInstructionType::Ret, vec![], None, vec![], func); 
            },
            Instruction::ZeroExt(CastOp{res, type_1: _, val, type_2: _}) => {
                insert_label_type(res, &SymbolWidth::I32, func);
                gen_instr(AsmInstructionType::Mv, vec!(res, val), None, vec![], func);
            },
            Instruction::I32ToFloat(CastOp{res, type_1, val, type_2}) | Instruction::FloatToI32(CastOp{res, type_1, val, type_2})=> {
                insert_label_type(res, &type_2.width, func);
                gen_instr(AsmInstructionType::Fcvt, vec!(res, val), None, vec!(type_2.width.clone(), type_1.width.clone()), func);
            },
            Instruction::BitCast(res, _, ptr, _) => {
                insert_label_type(res, &SymbolWidth::I64, func);
                let inside_stack = func.stack.pushed.contains(ptr);
                if inside_stack { // 在栈内
                    let ptr_pos = format!("#{}", ptr);
                    gen_instr(AsmInstructionType::Addi, vec!(res, "sp", &ptr_pos), None, vec![], func);
                } else {
                    if global_vars.contains(ptr) {
                        gen_instr(AsmInstructionType::La, vec!(res, ptr), None, vec![], func);
                    } else {
                        gen_instr(AsmInstructionType::Mv, vec!(res, ptr), None, vec![], func);
                    }
                }
            },
            Instruction::GetElemPtr(dst, SymbolType{width: SymbolWidth::Arr{tar: _, dims}, is_const: _}, ptr, idx) => {
                //把dst和I64类型做map映射
                insert_label_type(dst, &SymbolWidth::I64, func);

                let inside_stack = func.stack.pushed.contains(ptr);
                let start_addr: String;

                //接下来的一个大if，目的是设置开始的地址
                //如果是临时标号
                if is_num_label(ptr) {
                    start_addr = String::from(ptr);
                    if idx.len() == 1 {
                        if idx[0] != "0" {
                            let size = dims.iter().map(|i| *i as usize).product::<usize>() * 4;
                            if is_immediate(&idx[0]) {
	                            let this_idx = idx[0].parse::<usize>().unwrap() * size;
	                            let this_idx = format!("{}", this_idx);
	                            let li_res = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
	                            gen_instr(AsmInstructionType::Li, vec!(&li_res, &this_idx), None, vec![], func);
	                            gen_instr(AsmInstructionType::Add, vec!(dst, &start_addr, &li_res), None, vec![], func);
	                        } else {
	                            
	                            let size = format!("{}", size);
                                let this_idx = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                                match is_poweroftwo(&size) {
                                    Some(pow) => {
                                        gen_instr(AsmInstructionType::Slli, vec![&this_idx, &idx[0], &pow.to_string()], None, vec![], func);
                                        gen_instr(AsmInstructionType::Add, vec!(dst, &start_addr, &this_idx), None, vec![], func);
                                    },
                                    None => {
                                        let size_str = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                                        gen_instr(AsmInstructionType::Li, vec!(&size_str, &size), None, vec![], func);
	                                    gen_instr(AsmInstructionType::Mul, vec!(&this_idx, &idx[0], &size_str), Some(PTR_WIDTH), vec![], func);
	                                    gen_instr(AsmInstructionType::Add, vec!(dst, &start_addr, &this_idx), None, vec![], func);
                                    }
                                }
	                        }
                        } else {
                            gen_instr(AsmInstructionType::Mv, vec!(dst, ptr), None, vec![], func);
                        }
                        return;
                    }
                } else {
                    //设置为栈指针（"sp"）加上ptr的偏移量
                    if inside_stack { // 在栈内
                        let ptr_pos = format!("#{}", ptr);
                        if idx.len() == 1 {
                            gen_instr(AsmInstructionType::Addi, vec!(dst, "sp", &ptr_pos), None, vec![], func);
                            return;
                        }
                        start_addr = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                        gen_instr(AsmInstructionType::Addi, vec!(&start_addr, "sp", &ptr_pos), None, vec![], func);
                    } else {
                        //如果是全局变量的标签，直接La
                        let pure_ptr = &ptr[1..];
                        if global_vars.contains(pure_ptr) {
                            if idx.len() == 1 {
                                gen_instr(AsmInstructionType::La, vec!(dst, pure_ptr), None, vec![], func);
                                return;
                            }
                            start_addr = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                            gen_instr(AsmInstructionType::La, vec!(&start_addr, pure_ptr), None, vec![], func);
                        } else {
                            panic!("Undefined pointer {}", ptr);
                        }
                    }
                } // else

                //根据数组的维度情况和指定的索引生成相应的指令。
                //使用一个循环来处理每个索引。循环的计数变量是cnt，从1到idx.len() - 1。
                
                
                
                //最后，将next_addr赋值给last_addr，继续下一次循环。
                //综上所述，这段代码的作用是根据给定的指令信息生成相应的汇编指令，用于计算多维数组中指定索引的元素的内存地址。
                let mut last_addr = start_addr;
                let mut left_size = dims.iter().map(|d| *d as usize).product::<usize>() * 4;
                let mut next_addr: String;
                for cnt in 1..idx.len() {
                    //首先计算剩余的大小left_size，即当前维度及后续维度的元素个数。
                    left_size /= dims[cnt-1] as usize;

                    if cnt == idx.len() - 1 {
                        next_addr = String::from(dst);
                    } else {
                        next_addr = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                    }
                    //根据索引是否为立即数，生成相应的指令。
                    //如果索引是立即数，代码会计算出对应的偏移量this_idx，并根据偏移量是否为0生成相应的指令。
                    if is_immediate(&idx[cnt]) {
                        let this_idx = idx[cnt].parse::<usize>().unwrap() * left_size;
                        if this_idx == 0 {
                            gen_instr(AsmInstructionType::Mv, vec!(&next_addr, &last_addr), None, vec![], func);
                        } else {
                            if inside_imm_range(&this_idx.to_string()) {
                                gen_instr(AsmInstructionType::Addi, vec!(&next_addr, &last_addr, &this_idx.to_string()), None, vec![], func);
                            } else {
                                let imm_reg = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                                gen_instr(AsmInstructionType::Li, vec!(&imm_reg, &this_idx.to_string()), None, vec![], func);
                                gen_instr(AsmInstructionType::Add, vec!(&next_addr, &last_addr, &imm_reg), None, vec![], func);
                            }
                        }
                    } else {
                        //如果索引不是立即数，则生成一系列指令来计算偏移量。
                        //首先，生成一条Li指令，将left_size加载到一个临时寄存器base中。
                        //然后，生成一条Mul指令，将索引乘以base，结果存储在另一个临时寄存器index中。最后，生成一条Add指令，将last_addr加上index，结果存储在next_addr中。
                        if left_size == 0 {
                            gen_instr(AsmInstructionType::Mv, vec!(&next_addr, &last_addr), None, vec![], func);
                        } else {
                            let left_size_str = format!("{}", left_size);
                            let index = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                            match is_poweroftwo(&left_size_str) {
                                Some(pow) => {
                                    gen_instr(AsmInstructionType::Slli, vec![&index, &idx[cnt], &pow.to_string()], None, vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&next_addr, &last_addr, &index), None, vec![], func);
                                },
                                None => {
                                    let base = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                                    gen_instr(AsmInstructionType::Li, vec!(&base, &left_size_str), None, vec![], func);
                                    gen_instr(AsmInstructionType::Mul, vec!(&index, &idx[cnt], &base), Some(NORMAL_WIDTH), vec![], func);
                                    gen_instr(AsmInstructionType::Add, vec!(&next_addr, &last_addr, &index), None, vec![], func);
                                }
                            }
                        }
                    }
                    last_addr = next_addr;
                }
            },
            Instruction::Comment(_) => {}, // 跳过注释
            Instruction::Call(res, label, ty, params) => {
                if &label[1..] == "llvm.memset.p0i8.i64" {
                    let ptr = &params[0].0;
                    let size_byte: usize = (&params[2].0).parse().unwrap();
                    let mut filled_size: usize = 0;
                    let mut inner_filled_size: usize = 0;
                    let imm_reg = pop_temp_label(select_cnt, &SymbolWidth::I64, func);
                    //保存ptr的值
                    gen_instr(AsmInstructionType::Mv, vec!(&imm_reg, ptr), None, vec![], func);
                    while filled_size < size_byte - 4 {
                        if filled_size < 2040 {
                            gen_instr(AsmInstructionType::Store, vec!("zero", &imm_reg, &filled_size.to_string()), Some(PTR_WIDTH), vec![], func);
                            filled_size += 8;
                        } else if filled_size == 2040 {//第一次达到2040，需要用addi加一下
                            gen_instr(AsmInstructionType::Addi, vec!(&imm_reg, &imm_reg, "2040"), None, vec![], func);
                            gen_instr(AsmInstructionType::Store, vec!("zero", &imm_reg, "0"), Some(PTR_WIDTH), vec![], func);
                            filled_size += 8;
                        } else {
                            inner_filled_size += 8;
                            if inner_filled_size < 2040 {
                                gen_instr(AsmInstructionType::Store, vec!("zero", &imm_reg, &inner_filled_size.to_string()), Some(PTR_WIDTH), vec![], func);
                            } else if inner_filled_size == 2040 {
                                gen_instr(AsmInstructionType::Addi, vec!(&imm_reg, &imm_reg, "2040"), None, vec![], func);
                                gen_instr(AsmInstructionType::Store, vec!("zero", &imm_reg, "0"), Some(PTR_WIDTH), vec![], func);
                                inner_filled_size -= 2040;
                            }
                            filled_size += 8;
                        }
                    }
                    //说明要填的是4的倍数，不是8的倍数，还要补一个word
                    if filled_size != size_byte {
                        if filled_size < 2048 {
                            gen_instr(AsmInstructionType::Store, vec!("zero", ptr, &filled_size.to_string()), Some(NORMAL_WIDTH), vec![], func);
                        } else {
                            gen_instr(AsmInstructionType::Li, vec!(&imm_reg, &filled_size.to_string()), None, vec![], func);
                            gen_instr(AsmInstructionType::Add, vec!(&imm_reg, ptr, &imm_reg), None, vec![], func);
                            gen_instr(AsmInstructionType::Store, vec!("zero", &imm_reg, "0"), Some(NORMAL_WIDTH), vec![], func);
                        }
                    }
                    mark_call(func);
                    gen_instr(AsmInstructionType::Call, vec!["", "memset"], None, vec![], func);
                    
                    return;
                }
                mark_call(func);
                insert_label_type(res, &ty.width, func);
                let mut str_vec:Vec<&str> = vec!(res, &label[1..]);
                let mut new_params:Vec<&str> = params.iter().map(|(s, _)| s.as_str()).collect();
                str_vec.append(&mut new_params);
                let mut ty_vec = vec!(ty.width.clone());
                let mut param_ty = params.iter().map(|(_, t)| t.width.clone()).collect::<Vec<_>>();
                ty_vec.append(&mut param_ty);
                gen_instr(AsmInstructionType::Call, str_vec, None, ty_vec, func);
            },
            _ => {
                panic!("没这个指令啊。。。")
            },
        }
    }
}

