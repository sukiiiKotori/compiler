pub mod symbol;
pub mod build;
pub mod sysy_gen;
pub mod decl;
pub mod arithmetic_gen;
pub mod logic_gen;
pub mod writetext;

use crate::llvm_gen::sysy_gen::*;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::ast::*;

pub fn generate_llvm(ast: &mut SysY) -> LLVMProgram {
    let mut program = LLVMProgram::new();
    let mut scopes = Scopes::new();
    let mut labels = Labels::new();
    ast.generate(&mut program, &mut scopes, &mut labels);
    program
}