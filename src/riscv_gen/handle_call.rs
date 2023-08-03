use std::collections::{HashMap, HashSet, BTreeSet};

use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::riscv_gen::stack_slot::StackSlot;
use crate::riscv_gen::asm_select::FLOAT_PREFIX;
use crate::structures::symbol::*;
use crate::utils::check::*;
use crate::utils::float::*;

/// 用于处理函数调用的上下文结构体
pub struct HandelCallContext<'a> {
    /// 整型计数器
    pub int_cnt: usize,
    /// 浮点型计数器
    pub float_cnt: usize,
    /// 栈长度
    pub stack_len: isize,
    /// 栈槽的可变引用
    pub stack: &'a mut StackSlot,
    /// 无效寄存器的可变引用
    pub invalid_regs: &'a mut HashSet<&'static str>,
    /// 被存储的寄存器的可变引用
    pub stored_regs: &'a mut BTreeSet<&'a str>,
}

impl<'a> HandelCallContext<'a> {
    pub fn new(
        int_cnt: usize,
        float_cnt: usize,
        stack_len: isize,
        stack: &'a mut StackSlot,
        invalid_regs: &'a mut HashSet<&'static str>,
        stored_regs: &'a mut BTreeSet<&'a str>,
    ) -> Self {
        HandelCallContext{int_cnt, float_cnt, stack_len, stack, invalid_regs, stored_regs}
    }
}

impl AsmFunc {
    /// 展开调用信息
    pub fn handel_call(&mut self, alloc_res: &HashMap<String, &'static str>) {
        let mut call_info_ref = self.call_info.iter().collect::<Vec<_>>();
        // 对当前函数中的代码块从后往前遍历
        for block in self.blocks.iter_mut().rev() {
            loop {
                // 如果还存在待展开的函数调用信息
                if let Some(this_call_info) = call_info_ref.last() {
                    let this_idx = this_call_info.0;
                    // 如果待展开的函数调用信息的索引大于等于当前代码块的前驱指令数
                    if this_idx >= block.pre_instr_cnt {
                        let position = this_idx - block.pre_instr_cnt;
                        // 在当前代码块中展开函数调用
                        block.handel_call(&mut self.stack, this_call_info, alloc_res,  position);
                        call_info_ref.pop();
                    } else {
                        break;
                    }
                } else {
                    // 不存在待展开的调用信息，返回
                    return;
                }
            }
        }
    }
}

impl AsmBlock {
    /// 处理函数调用call指令<br>
    /// 1. 解析调用指令，获取返回值、参数和参数类型。<br>
    /// 2. 恢复在调用指令中被存储的临时寄存器。<br>
    /// 3. 存储调用的返回值。<br>
    /// 4. 获取用于存储参数的寄存器，并将其加入无效寄存器集合中。<br>
    /// 5. 将参数值加载到指定的寄存器或栈槽位置。<br>
    /// 6. 保存参数冲突的寄存器和穿越生命周期的寄存器。<br>
    fn handel_call(
        &mut self,
        stack: &mut StackSlot,
        this_call_info: &(usize, Option<usize>, HashSet<String>),
        alloc_res: &HashMap<String, &'static str>,
        position: usize
    ) {
        let (ret_val, params, types) = match self.instrs.get(position).unwrap() {
            AsmInstruction::Call(r, name, p, t) => {
                if name == "memset" {
                    return;
                }
                (
                String::from(r),
                p.iter().map(|p| String::from(p)).collect::<Vec<_>>(),
                t.iter().cloned().collect::<Vec<_>>()
                )
            },
            _ => panic!("Position error"),
        };
    
        // 获取穿过当前Call指令的Temporary寄存器
        let mut stored_regs = this_call_info.2.iter()
            .filter_map(|r| alloc_res.get(r)
                .filter(|phy| TEMP_SET.contains(*phy) || FLOAT_TEMP_SET.contains(*phy))
            )
            .cloned()
            .collect::<BTreeSet<_>>();
    
        // 恢复穿过当前Call指令的Temporary寄存器
        for temp in stored_regs.iter() {
            let stored_pos = format!("stored.{}", temp);
            stack.push_normal(stored_pos.as_str(), 8);
            let mut prefix = "";
            if FLOAT_TEMP_SET.contains(temp) {
                prefix = FLOAT_PREFIX;
            }
            self.instrs.insert(position+1, AsmInstruction::make_instr(
                AsmInstructionType::Load,
                vec!(temp, "sp", stored_pos.as_str(), prefix),
                Some(PTR_WIDTH),
                vec!()
            ));
        }
    
        // 存储调用返回值
        if !ret_val.is_empty() {
            if types[0] == SymbolWidth::Float {
                self.instrs.insert(position+1, AsmInstruction::make_instr(
                    AsmInstructionType::Fmv,
                    vec!(ret_val.as_str(), "fa0"),
                    None,
                    vec!(SymbolWidth::Float, SymbolWidth::Float)
                ));
            } else {
                self.instrs.insert(position+1, AsmInstruction::make_instr(
                    AsmInstructionType::Mv,
                    vec!(ret_val.as_str(), "a0"),
                    None,
                    vec!()
                ));
            }
        }
    
        // 获取存储参数的寄存器，并把它加入invalid_regs的集合中
        let mut int_pos = 0;
        let mut float_pos = 0;
        let mut invalid_regs = HashSet::new();
        for ty in types.iter().skip(1) {
            if *ty == SymbolWidth::Float {
                if float_pos < FLOAT_FUNC_ARG.len() {
                    invalid_regs.insert(FLOAT_FUNC_ARG[float_pos]);
                }
                float_pos += 1;
            } else {
                if int_pos < FUNC_ARG.len() {
                    invalid_regs.insert(FUNC_ARG[int_pos]);
                }
                int_pos += 1;
            }
            if int_pos >= FUNC_ARG.len() && float_pos >= FLOAT_FUNC_ARG.len() {
                break;
            }
        }

        // 将参数值装载到指定寄存器或者栈槽位置
        let mut context = HandelCallContext::new(
            0,
            0,
            0,
            stack,
            &mut invalid_regs,
            &mut stored_regs
        );
        for (_, (param, ty)) in params.iter().zip(types.iter().skip(1)).enumerate() {
            if *ty == SymbolWidth::Float {
                self.load_float_param(param, position, &mut context);
            } else if let SymbolWidth::Arr{tar: _, dims: _} = ty {
                self.load_int_param(param, position, 8, &mut context);
            } else {
                self.load_int_param(param, position, 4, &mut context);
            }
        }

        // 保存参数冲突的寄存器和穿越生命周期的寄存器
        for reg in context.stored_regs.iter() {
            let stored_reg = format!("stored.{}", reg);
            self.instrs.insert(position, AsmInstruction::make_instr(
                AsmInstructionType::Store,
                vec![reg, "sp", stored_reg.as_str()],
                Some(PTR_WIDTH),
                vec![]
            ));
        }
    }
    
    pub fn load_float_param<'a>(
        &mut self, 
        param: &'a String, 
        position: usize, 
        context: &mut HandelCallContext<'a>,
    ) {
        // 检查浮点数计数器是否超过了浮点型函数参数寄存器的数量
        if context.float_cnt >= FLOAT_FUNC_ARG.len() {
            // 如果超过了数量，则需要将参数存储在栈上
            context.stack_len += 4;
            let stack_pos = format!("-{}", context.stack_len);
            // 如果参数是一个立即数
            if is_immediate(param.as_str()) {
                let imm = double_to_float(param.as_str());
                // 在指定位置插入存储立即数的指令
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Store,
                    vec!(PRESERVED[1], "sp",stack_pos.as_str(), ),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
                // 在指定位置插入加载立即数的指令
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Li,
                    vec!(PRESERVED[1], imm.as_str()),
                    None,
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                // 如果参数是一个无效的寄存器，则抛出错误
                panic!("riscv_gen/build.rs:load_float_param,invalid regs");
            } else {
                // 正常参数，在指定位置插入存储参数的指令
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Store,
                    vec!(param.as_str(), "sp", stack_pos.as_str(), FLOAT_PREFIX),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
            }
        } else {
            // 如果浮点型计数器未超过浮点型函数参数的数量
            context.invalid_regs.remove(FLOAT_FUNC_ARG[context.float_cnt]);
            if is_immediate(param.as_str()) {
                let imm = double_to_float(param.as_str());
                self.instrs.insert(
                    position, 
                    AsmInstruction::make_instr(
                        AsmInstructionType::Fmv, 
                        vec!(FLOAT_FUNC_ARG[context.float_cnt], PRESERVED[1]), 
                        None, 
                        vec!(SymbolWidth::Float, SymbolWidth::I32)
                    )
                );
                self.instrs.insert(
                    position, 
                    AsmInstruction::make_instr(
                        AsmInstructionType::Li, 
                        vec!(PRESERVED[1], &imm), 
                        None, 
                        vec!()
                    )
                );
            } else if context.invalid_regs.contains(param.as_str()) {
                let stored_pos = format!("stored.{}", param);
                context.stored_regs.insert(param.as_str());
                context.stack.push_normal(stored_pos.as_str(), 8);
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Load,
                    vec!(FLOAT_FUNC_ARG[context.float_cnt], "sp", stored_pos.as_str(), FLOAT_PREFIX),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
            } else {
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Fmv,
                    vec!(FLOAT_FUNC_ARG[context.float_cnt], param.as_str()),
                    Some(NORMAL_WIDTH),
                    vec!(SymbolWidth::Float, SymbolWidth::Float)
                ));
            }
        }
        context.float_cnt += 1;
    }

    pub fn load_int_param<'a>(
        &mut self, 
        param: &'a String, 
        position: usize, 
        param_size: isize, 
        context: &mut HandelCallContext<'a>,
    ) {
        if context.int_cnt >= FUNC_ARG.len() {
            context.stack_len += param_size;
            let stack_pos = format!("-{}", context.stack_len);
            if is_immediate(param.as_str()) {
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Store,
                    vec!(PRESERVED[1], "sp", stack_pos.as_str()),
                    Some(param_size),
                    vec!()
                ));
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Li,
                    vec!(PRESERVED[1], param.as_str()),
                    None,
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                panic!("Should not appear");
            } else {
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Store,
                    vec!(param.as_str(), "sp", stack_pos.as_str()),
                    Some(param_size),
                    vec!()
                ));
            }
        } else {
            context.invalid_regs.remove(FUNC_ARG[context.int_cnt]);
            if is_immediate(param.as_str()) {
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Li,
                    vec!(FUNC_ARG[context.int_cnt], param.as_str()),
                    Some(param_size),
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                let stored_pos = format!("stored.{}", param);
                context.stored_regs.insert(param.as_str());
                context.stack.push_normal(stored_pos.as_str(), 8);
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Load,
                    vec!(FUNC_ARG[context.int_cnt], stored_pos.as_str()),
                    Some(PTR_WIDTH),
                    vec!()
                ));
            } else {
                self.instrs.insert(position, AsmInstruction::make_instr(
                    AsmInstructionType::Mv,
                    vec!(FUNC_ARG[context.int_cnt], param.as_str()),
                    None,
                    vec!()
                ));
            }
        }
        context.int_cnt += 1;
    }
}