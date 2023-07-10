pub mod scopes;
pub mod symbol;
pub mod build;
pub mod generate;
pub mod decl;
pub mod arithmetic;
pub mod bool;
pub mod dump;

use std::error::Error;
use generate::*;
use scopes::*;
use crate::structures::llvm_struct::*;
use crate::ast::*;


pub fn generate_program(my_ast: &mut SysY) -> Result<LLVMProgram, Box<dyn Error>>{
    let mut program = LLVMProgram::new();
    let mut scopes = Scopes::new();
    let mut labels = Labels::new();
    let res = my_ast.generate(&mut program, &mut scopes, &mut labels);
    if let Err(e) = res {
        panic!("Break due to {:?}", e);
    }
    Ok(program)
}