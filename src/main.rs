mod utils;
mod structures;
mod ast;
mod llvm_gen;
mod riscv_gen;
mod llvm_opt;

use std::fs;
use std::env::args;
use std::fs::read_to_string;
use llvm_gen::generate_llvm;
use riscv_gen::emit_asm;
use crate::structures::writetext_trait::*;

/*
编译器设置选项
*/
pub struct Settings {
    pub use_phi: bool,              // 使用phi指令
    pub optimise: bool,             // 开启优化
    pub debug: bool,                // 调试模式
    pub log: bool,                  // 打印日志
    pub all_allocs_in_entry: bool,  // 在入口处全部分配
}
static SETTINGS: Settings = Settings {
    use_phi: false,
    optimise: true,
    debug: false,
    log: false,
    all_allocs_in_entry: true,
};
pub fn get_settings() -> &'static Settings {
    &SETTINGS
}

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(parser);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    args.next();
    let input_file = args.next().unwrap();
    let input = read_to_string(&input_file).unwrap();
    let mut ast = parser::SysYParser::new().parse(&input).unwrap();
    let mut program = generate_llvm(&mut ast).unwrap();
    args.next();
    let split_output = input_file.split('.').collect::<Vec<_>>();
    let default_output = String::from(split_output[0])+".ll";
    let output = args.next().unwrap_or(default_output);
    let mut out = fs::File::create(&output)?;
    program.writetext(&mut out);
    let mut riscv = emit_asm(&program);
    Ok(())
}