use std::collections::HashSet;
use lazy_static::lazy_static;


pub const TEMPORARY: [&str; 7] = ["t0", "t1", "t2", "t3", "t4", "t5", "t6"];
// 临时整数寄存器数组
pub const SAVED: [&str; 12] = [  // 保存整数寄存器数组
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
];
pub const FUNC_ARG: [&str; 8] = ["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7"];
// 函数参数寄存器数组
pub const RETURN: [&str; 2] = ["a0", "a1"];
// 返回值寄存器数组
pub const PRESERVED: [&str; 2] = ["t0", "t1"];
// 保留寄存器数组
pub const FLOAT_TEMPORARY: [&str; 7] = ["ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6"];
// 临时浮点寄存器数组
pub const FLOAT_SAVED: [&str; 12] = [  // 保存浮点寄存器数组
    "fs0", "fs1", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7", "fs8", "fs9", "fs10", "fs11",
];
pub const FLOAT_FUNC_ARG: [&str; 8] = ["fa0", "fa1", "fa2", "fa3", "fa4", "fa5", "fa6", "fa7"];
// 浮点函数参数寄存器数组
pub const FLOAT_RETURN: [&str; 2] = ["fa0", "fa1"];
// 浮点返回值寄存器数组
pub const FLOAT_PRESERVED: [&str; 2] = ["ft0", "ft1"];  // 保留浮点寄存器数组


lazy_static! {
    pub static ref TEMP_SET: HashSet<&'static str> = HashSet::from_iter(TEMPORARY.iter().chain(FUNC_ARG.iter()).map(|r| *r));
    pub static ref TEMP_VEC: Vec<&'static str> = Vec::from_iter(TEMPORARY.iter().chain(FUNC_ARG.iter()).rev().map(|r| *r));
    pub static ref SAVED_SET: HashSet<&'static str> = HashSet::from_iter(SAVED.iter().map(|r| *r));
    pub static ref SAVED_VEC: Vec<&'static str> = Vec::from_iter(SAVED.iter().rev().map(|r| *r));
    pub static ref PRESERVED_SET: HashSet<&'static str> = PRESERVED.iter().map(|r| *r).collect();

    pub static ref FLOAT_TEMP_SET: HashSet<&'static str> = HashSet::from_iter(FLOAT_TEMPORARY.iter().chain(FLOAT_FUNC_ARG.iter()).map(|r| *r));
    pub static ref FLOAT_TEMP_VEC: Vec<&'static str> = Vec::from_iter(FLOAT_TEMPORARY.iter().chain(FLOAT_FUNC_ARG.iter()).rev().map(|r| *r));
    pub static ref FLOAT_SAVED_SET: HashSet<&'static str> = HashSet::from_iter(FLOAT_SAVED.iter().map(|r| *r));
    pub static ref FLOAT_SAVED_VEC: Vec<&'static str> = Vec::from_iter(FLOAT_SAVED.iter().rev().map(|r| *r));
    pub static ref FLOAT_PRESERVED_SET: HashSet<&'static str> = FLOAT_PRESERVED.iter().map(|r| *r).collect();
    
    pub static ref ALL_REGS: HashSet<&'static str> = HashSet::from_iter(
        TEMP_SET.iter()
        .chain(SAVED_SET.iter())
        .chain(FLOAT_TEMP_SET.iter())
        .chain(FLOAT_SAVED_SET.iter())
        .map(|r| *r)
    );
}