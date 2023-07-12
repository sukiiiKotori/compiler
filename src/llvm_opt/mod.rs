pub mod eliminate;
pub mod flow;
pub mod deadcode;
pub mod flow_graph;

use crate::structures::llvm_struct::*;
use crate::llvm_opt::eliminate::*;


pub fn optimise(program: LLVMProgram) -> LLVMProgram {
    let program = eliminate_all(program);
    program
}
