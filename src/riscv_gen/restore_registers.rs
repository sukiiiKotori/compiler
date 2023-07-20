use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;

impl RiscV {
    pub fn restore_registers(&mut self) {
        self.text.funcs.iter_mut().for_each(|func| func.restore_registers());
    }
}

impl AsmFunc {
    pub fn restore_registers(&mut self) {
        self.blocks.iter_mut().for_each(|block| {
            //找到这个块的ret指令的下标
            match block.instrs.iter().position(|instr| {
                match instr {
                    AsmInstr::Ret() => true,
                    _ => false,
                }
            }) {
                Some(position) => {
                    self.used_saved.iter().for_each(|saved_reg| {
                        if FLOAT_SAVED_SET.contains(saved_reg) {
                            block.instrs.insert(
                                position, 
                                AsmInstr::make_instr(
                                    AsmInstrType::Load, 
                                    vec!(saved_reg, "sp", saved_reg, "f"), 
                                    Some(8), 
                                    vec!()
                                )
                            );
                        } else {
                            block.instrs.insert(
                                position, 
                                AsmInstr::make_instr(
                                    AsmInstrType::Load, 
                                    vec!(saved_reg, "sp", saved_reg), 
                                    Some(8), 
                                    vec!()
                                )
                            );
                        }
                    })
                }
                //如果没有ret指令不需要恢复
                None => {}
            };
        });
    }
}