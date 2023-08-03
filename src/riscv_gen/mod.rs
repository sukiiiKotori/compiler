pub mod stack_slot;
pub mod register_resource;
pub mod linearscan;
pub mod select_utils;
pub mod asm_select;
pub mod rewrite;
pub mod write_text;
pub mod asmfunc_stack;
pub mod push_arguments;
pub mod push_datasection;
pub mod push_asmfunc;
pub mod push_localvars;
pub mod save_registers;
pub mod restore_registers;
pub mod register_alloc;
pub mod register_type;
pub mod handle_call;

use linearscan::*;
use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;

pub fn generate_asm(program: &LLVMProgram) -> RiscV {
    let mut asm = RiscV::new();
    //使用LLVM IR来进行数据段的构造，代码段的构造以及指令选择
    program.push_datasection(&mut asm);
    program.push_textsection(&mut asm);
    program.asm_select(&mut asm);
    //进行寄存器分配
    asm.alloc_regs::<LinearScan>();
    //在函数的入口保存使用过的s0-s11寄存器
    asm.save_registers();
    //在函数出口恢复使用过的s0-s11寄存器
    asm.restore_registers();
    //确定程序栈大小
    asm.deterministic_stack();
    //分配和释放栈空间
    asm.stack_alloc_free();
    asm.map_stack_address();
    asm
}

impl LLVMProgram {
    fn push_textsection(&self, asm: &mut RiscV) {
        self.push_asmfunc(asm);
        self.push_localvars(asm);
        self.push_arguments(asm);
    }
}