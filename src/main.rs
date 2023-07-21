mod utils;
mod structures;
mod ast;
mod llvm_gen;
mod llvm_opt;
mod riscv_gen;
mod riscv_opt;

use std::fs;
use std::env::args;
use std::fs::read_to_string;
use llvm_gen::generate_llvm;
use riscv_gen::generate_asm;
use crate::structures::writetext_trait::*;

/*
编译器设置选项
*/
pub struct Settings {
    pub use_phi: bool,
    // 使用phi指令
    pub optimise: bool,
    // 开启优化
    pub debug: bool,
    // 调试模式
    pub log: bool,
    // 打印日志
    pub all_allocs_in_entry: bool,  // 在入口处全部分配
}

static SETTINGS: Settings = Settings {
    use_phi: false,
    optimise: true,
    debug: true,
    log: false,
    all_allocs_in_entry: true,
};

pub fn get_settings() -> &'static Settings {
    &SETTINGS
}

use lalrpop_util::lalrpop_mod;
use crate::llvm_opt::optimise_llvm;
lalrpop_mod!(parser);

fn main() {
    let mut args = args();
    //跳过第一个参数
    args.next();
    //获取待编译的文件名
    let file_name = args.next().unwrap();
    //用lalrpop解析得到ast
    let mut ast = parser::SysYParser::new().parse(&read_to_string(&file_name).unwrap()).unwrap();
    //生成llvm
    let mut llvm = generate_llvm(&mut ast);
    if SETTINGS.optimise {
        optimise_llvm(&mut llvm);
    }
    let filename_without_suffix= file_name.split(".").collect::<Vec<_>>()[0].to_string();
    //编译选项，可选-llvm和-S
    match args.next().unwrap().as_str() {
        "-llvm" => {
            let mut llvm_file = fs::File::create(filename_without_suffix + ".ll").unwrap();
            llvm.writetext(&mut llvm_file);
        }
        "-S" => {
            let mut asm = generate_asm(&llvm);
            asm.optimise_riscv();
            
            let mut asm_file = fs::File::create(filename_without_suffix + ".s").unwrap();
            asm.writetext(&mut asm_file);
        }
        _ => panic!()
    }
}