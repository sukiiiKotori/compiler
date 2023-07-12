pub mod eliminate;
pub mod flow;
pub mod deadcode;

use crate::structures::llvm_struct::*;
use crate::llvm_opt::eliminate::eliminate_all;

pub fn optimise(program: LLVMProgram) -> LLVMProgram {
    let program = eliminate_all(program);
    program
}
