use crate::structures::riscv_struct::*;

impl RiscV {
    pub fn remove_mv(&mut self) {
        self.text.funcs.iter_mut().for_each(|func| func.remove_mv());
    }
}

impl AsmFunc {
    pub fn remove_mv(&mut self) {
        self.blocks.iter_mut().for_each(|block| {//遍历每个函数内部的基本块
            block.instrs.retain(|instr| {//传入一个闭包，当mv或fmv的源和目的寄存器相同时消除
                match instr {
                    AsmInstruction::Fmv(bin, _ , _) | AsmInstruction::Mv(bin) => bin.dst != bin.src,
                    _ => true
                }
            });
        });
    }
}