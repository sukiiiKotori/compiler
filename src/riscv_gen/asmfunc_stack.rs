use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::utils::check::*;

impl AsmFunc {
    // 确定函数栈大小
    pub fn deterministic_stack(&mut self) {
        self.stack.deterministic();
    }

    /// 在栈上进行分配和释放操作
    pub fn stack_alloc_free(&mut self) {
        // 获取释放的大小和分配的大小
        let free_size = self.stack.frame_size.to_string();
        let alloc_size = format!("-{}", free_size);

        // 遍历所有的代码块
        for (idx, block) in self.blocks.iter_mut().enumerate() {
            // 如果是第一个代码块且堆栈帧大小大于0
            if idx == 0 && self.stack.frame_size > 0 {
                // 根据堆栈帧大小的不同，插入不同的指令
                if self.stack.frame_size < 2040 {
                    block.instrs.insert(0, AsmInstr::make_instr(
                        AsmInstrType::Addi,
                        vec!("sp", "sp", alloc_size.as_str()),
                        None,
                        vec!()
                    ));
                } else {
                    block.instrs.insert(0, AsmInstr::make_instr(
                        AsmInstrType::Add,
                        vec!("sp", "sp", TEMPORARY[0]),
                        None,
                        vec!()
                    ));
                    block.instrs.insert(0, AsmInstr::make_instr(
                        AsmInstrType::Li,
                        vec!(TEMPORARY[0], alloc_size.as_str()),
                        None,
                        vec!()
                    ));
                }
            }

            // 获取代码块中的最后一条指令
            let last_instr = block.instrs.last();
            if let Some(AsmInstr::Ret(ret_val)) = last_instr {
                // 如果堆栈帧大小大于0
                if self.stack.frame_size > 0 {
                    let before_last = block.instrs.len() - 1;
                    // 根据堆栈帧大小的不同，插入不同的指令
                    // 12位立即数寻找范围为-2048到+2047，这里预留部分，与2040比较
                    if self.stack.frame_size < 2040 {
                        block.instrs.insert(before_last, AsmInstr::make_instr(
                            AsmInstrType::Addi,
                            vec!("sp", "sp", free_size.as_str()),
                            None,
                            vec!()
                        ));
                    } else {
                        // 看返回值使用了哪个寄存器
                        if ret_val == FUNC_ARG[0] {
                            block.instrs.insert(before_last, AsmInstr::make_instr(
                                AsmInstrType::Add,
                                vec!("sp", "sp", TEMPORARY[0]),
                                None,
                                vec!()
                            ));
                            block.instrs.insert(before_last, AsmInstr::make_instr(
                                AsmInstrType::Li,
                                vec!(TEMPORARY[0], free_size.as_str()),
                                None,
                                vec!()
                            ));
                        } else {
                            block.instrs.insert(before_last, AsmInstr::make_instr(
                                AsmInstrType::Add,
                                vec!("sp", "sp", FUNC_ARG[0]),
                                None,
                                vec!()
                            ));
                            block.instrs.insert(before_last, AsmInstr::make_instr(
                                AsmInstrType::Li,
                                vec!(FUNC_ARG[0], free_size.as_str()),
                                None,
                                vec!()
                            ));
                        }
                    }
                }
            }
        }
    }
    /// 适配栈sp相关的立即数寻址范围
    pub fn adapt_stack_address(&mut self) {
        // 遍历所有的代码块
        for block in self.blocks.iter_mut() {
            let len = block.instrs.len();
            // 从后向前遍历所有的指令
            for cnt in (0..len).rev() {
                // 根据指令类型进行匹配
                match block.instrs.get_mut(cnt).unwrap() {
                    AsmInstr::Store(MemInstr{width: _, val, base, offset}, _)
                    | AsmInstr::Load(MemInstr{width: _, val, base, offset}, _) => {
                        // 查找一个不等于 val 的保留寄存器
                        let preserved_reg = PRESERVED.iter().map(|r| *r).find(|r| *r != val.as_str()).unwrap();
                        if base == "sp" {
                            // 获取栈偏移并转换为字符串
                            let stack_pos =  self.stack.get_pos(offset.as_str()).to_string();
                            // 如果栈偏移在立即数范围内，直接赋值给 offset
                            if inside_imm_range(stack_pos.as_str()) {
                                *offset = String::from(&stack_pos);
                            } else {
                                // 否则，记录指令的 base 字段
                                let instr_base = Some(String::from(base.as_str()));
                                // 设置base为保留寄存器，偏移为零
                                *base = String::from(preserved_reg);
                                *offset = String::from("0");
                                // 插入 Li Add 指令
                                block.instrs.insert(cnt, AsmInstr::make_instr(
                                    AsmInstrType::Add,
                                    vec!(preserved_reg, preserved_reg, instr_base.unwrap().as_str()),
                                    None,
                                    vec!()
                                ));
                                block.instrs.insert(cnt, AsmInstr::make_instr(
                                    AsmInstrType::Li,
                                    vec!(preserved_reg, stack_pos.as_str()),
                                    None,
                                    vec!()
                                ));
                            }
                        }
                    },
                    AsmInstr::Addi(TriInstr{width: _, dst, op1, op2}) => {
                        // 查找一个不等于 dst 的保留寄存器
                        let preserved_reg = PRESERVED.iter().map(|r| *r).find(|r| *r != dst.as_str()).unwrap();
                        // op1为sp,op2为#开头表示立即数
                        if op1 == "sp" && &op2[0..1] == "#" {
                            // 获取栈偏移并转换为字符串
                            let stack_pos = self.stack.get_pos(&op2[1..]).to_string();
                            if inside_imm_range(stack_pos.as_str()) {
                                // 如果栈偏移在立即数范围内，直接赋值给 op2
                                *op2 = String::from(&stack_pos);
                            } else {
                                // 否则，记录指令的 dst 字段
                                let instr_dst = Some(String::from(dst.as_str()));
                                // 将addi指令替换为li add
                                block.instrs.insert(cnt, AsmInstr::make_instr(
                                    AsmInstrType::Add,
                                    vec!(instr_dst.unwrap().as_str(),"sp", preserved_reg),
                                    None,
                                    vec!()
                                ));
                                block.instrs.insert(cnt, AsmInstr::make_instr(
                                    AsmInstrType::Li,
                                    vec!(preserved_reg, stack_pos.as_str()),
                                    None,
                                    vec!()
                                ));
                                // 移除原先的addi指令
                                block.instrs.remove(cnt+2);
                            }
                        }
                    }
                    _ => {},
                }
            } // for cnt 遍历指令
        } // for block 遍历代码块
    }
}
