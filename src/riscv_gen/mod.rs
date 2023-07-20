pub mod stack_slot;
pub mod reg;
pub mod linearscan;
pub mod build;
pub mod select;
pub mod rewrite;
pub mod writetext;
pub mod asmfunc_stack;
pub mod globalvar;

use linearscan::*;
use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;

pub fn emit_asm(program: &LLVMProgram) -> RiscV {
    let mut asm = RiscV::new();
    program.push_globalvars(&mut asm);
    program.select_asm(&mut asm);
    asm.alloc_regs::<LinearScan>();
    asm.save_registers();
    asm.deterministic_stack();
    asm.stack_alloc_free();
    asm.map_stack_address();
    asm
}
