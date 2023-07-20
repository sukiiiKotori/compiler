use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;
use crate::utils::check::*;
use crate::utils::float::*;

impl LLVMProgram {
    pub fn push_datasection(&self, asm: &mut RiscV) {
        self.global_var.iter().for_each(|var| var.push_datasection(asm));
    }
}

impl GlobalVar {
    pub fn push_datasection(&self, asm: &mut RiscV) {
        asm.push_datasection(
            &self.var_name,
            &self.var_type,
            self.init_values.iter().map(|value| value).collect(),
        );
    }
}

impl RiscV {
    /// 向data段的全局变量列表中添加一个新的全局变量，使用给定的标签、类型和初始值向量
    pub fn push_datasection(&mut self, label: &str, ty: &SymbolType, init_vals: Vec<&String>) {
        self.data.labels.insert(label.to_string());
        self.data.datas.push(DataSectionItem {
            label: label.to_string(),
            ty: ty.clone(),
            init_vals: init_vals.iter().map(|value| {
                //如果是IEEE754 double, 那么把他规格化成IEEE754 float
                if is_float_immediate(value) {
                    double_to_float(value)
                } else {
                    value.to_string()
                }
            }).collect(),
        });
    }
}

