use std::collections::HashMap;
use crate::riscv_gen::asm_select::FLOAT_PREFIX;
use crate::structures::riscv_regs::*;

// 寄存器类型
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum RegType {
    TempInt,
    SavedInt,
    TempFloat,
    SavedFloat,
}

// 是否为float寄存器
pub fn type_is_float(phy: &str) -> bool {
    &phy[0..1] == FLOAT_PREFIX
}

// 获取被保护寄存器
pub fn get_preserved_regs() -> HashMap<RegType, Vec<&'static str>> {
    let mut res = HashMap::new();
    let preserved_int = PRESERVED.iter().rev().map(|r| *r).collect();
    let preserved_float = FLOAT_PRESERVED.iter().rev().map(|r| *r).collect();
    res.insert(RegType::TempInt, preserved_int);
    res.insert(RegType::TempFloat, preserved_float);
    res
}

// 根据前两个函数，确定寄存器类型
impl RegType {
    pub fn get_regtype(is_float: bool, is_saved: bool) -> Self {
        match (is_float, is_saved) {
            (false, false) => Self::TempInt,
            (false, true) => Self::SavedInt,
            (true, false) => Self::TempFloat,
            (true, true) => Self::SavedFloat,
        }
    }

    pub fn regtype_filter(&self,reg: &str) -> bool {
        match self {
            RegType::TempInt => &reg[0..1] != "f",
            RegType::TempFloat => &reg[0..1] == "f",
            RegType::SavedInt => SAVED_SET.contains(reg),
            RegType::SavedFloat => FLOAT_SAVED_SET.contains(reg),
        }
    }
}