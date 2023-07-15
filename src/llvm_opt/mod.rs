pub mod eliminate;
pub mod flow;
pub mod deadcode;
pub mod flow_graph;
pub mod unreachable_eliminate;
pub mod dead_code_eliminate;
pub mod reload;

use crate::structures::llvm_struct::*;
use crate::llvm_opt::eliminate::*;


pub fn optimise_llvm(program: LLVMProgram) -> LLVMProgram {
    let program = eliminate_all(program);
    program
}
