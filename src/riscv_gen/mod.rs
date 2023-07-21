pub mod stack_slot;
pub mod register_resource;
pub mod linearscan;
pub mod build;
pub mod asm_select;
pub mod rewrite;
pub mod write_text;
pub mod asmfunc_stack;
pub mod push_datasection;
pub mod save_registers;
pub mod restore_registers;
pub mod register_alloc;
pub mod register_type;

use linearscan::*;
use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;

pub fn generate_asm(program: &LLVMProgram) -> RiscV {
    let mut asm = RiscV::new();
    program.push_datasection(&mut asm);
    program.asm_select(&mut asm);
    asm.alloc_regs::<LinearScan>();
    asm.save_registers();
    asm.restore_registers();
    asm.deterministic_stack();
    asm.stack_alloc_free();
    asm.map_stack_address();
    asm
}
