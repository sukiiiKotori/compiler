pub mod scopes;
pub mod symbol;
pub mod build;
pub mod sysy_gen;
pub mod decl;
pub mod arithmetic_gen;
pub mod logic_gen;
pub mod writetext;

use std::error::Error;
use sysy_gen::*;
use scopes::*;
use crate::structures::llvm_struct::*;
use crate::ast::*;

pub fn generate_llvm(my_ast: &mut SysY) -> Result<LLVMProgram, Box<dyn Error>>{
    let mut program = LLVMProgram::new();
    let mut scopes = Scopes::new();
    let mut labels = Labels::new();
    let res = my_ast.generate(&mut program, &mut scopes, &mut labels);
    if let Err(e) = res {
        panic!("Break due to {:?}", e);
    }
    Ok(program)
}