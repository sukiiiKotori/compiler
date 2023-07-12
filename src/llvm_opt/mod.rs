mod eliminate;
mod flow;

use crate::structures::llvm_struct::*;
use crate::llvm_opt::eliminate::*;


pub fn optimise(program: LLVMProgram) -> LLVMProgram {
    let program = eliminate_all(program);
    program
}
