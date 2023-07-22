pub mod eliminate;
pub mod flow;
pub mod flow_graph;

use crate::structures::llvm_struct::*;

impl LLVMProgram {
    pub fn optimise_llvm(&mut self) {
        self.eliminate_unused_code();
    }
}