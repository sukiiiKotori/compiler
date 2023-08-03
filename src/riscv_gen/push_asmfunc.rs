use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;

impl LLVMProgram {
    pub fn push_asmfunc(&self, asm: &mut RiscV) {
        self.func_def.iter().for_each(|func| func.push_asmfunc(asm));
    }
}

impl FuncDef {
    pub fn push_asmfunc(&self, asm: &mut RiscV) {
        asm.text.funcs.push(AsmFunc::new(
            &self.func_name.replace("@", ""), 
            self.func_type.width.clone()
        ));
    }
}