pub mod eliminate;
pub mod flow;
pub mod flow_graph;
pub mod unreachable_eliminate;
pub mod dead_code_eliminate;

use crate::structures::llvm_struct::*;

impl LLVMProgram {
    pub fn optimise_llvm(&mut self) {
        self.eliminate_unused_code();
    }
}