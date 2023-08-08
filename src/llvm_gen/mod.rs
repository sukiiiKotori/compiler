pub mod type_utils;
pub mod program_gen;
pub mod sysy_gen;
pub mod declaration;
pub mod arithmetic_gen;
pub mod logic_gen;
pub mod write_text;
pub mod instruction_gen;
pub mod define;
pub mod array_declaration;
pub mod initval;

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