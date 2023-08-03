use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;


impl RiscV {
    pub fn save_registers(&mut self) {
        self.text.funcs.iter_mut().for_each(|func| func.save_registers());
    }
}

impl AsmFunc {
    fn save_registers(&mut self) {
        self.used_saved.iter().for_each(|saved_reg| {
            //与恢复现场不同，由于函数的入口只有一个，
            //因此保存现场只需要在函数的入口保存一次即可。
            self.stack.push_normal(saved_reg, 8);
            if FLOAT_SAVED_SET.contains(saved_reg) {
                (&mut self.blocks[0]).instrs.insert(
                    0, 
                    AsmInstruction::make_instr(
                        AsmInstructionType::Store, 
                        vec!(saved_reg, "sp", saved_reg, "f"), 
                        Some(8), 
                        vec!()
                    )
                );
            } else {
                (&mut self.blocks[0]).instrs.insert(
                    0, 
                    AsmInstruction::make_instr(
                        AsmInstructionType::Store, 
                        vec!(saved_reg, "sp", saved_reg), 
                        Some(8), 
                        vec!()
                    )
                );
            }
        });
    }
}
