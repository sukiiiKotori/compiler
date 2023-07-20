use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

impl LLVMProgram {
    pub fn push_globalvars(&self, asm: &mut RiscV) {
        self.global_var.iter().for_each(|var| var.push_globalvars(asm));
    }
}

impl RiscV {
    /// 向data段的全局变量列表中添加一个新的全局变量，使用给定的标签、类型和初始值向量
    pub fn push_globalvar(&mut self, label: &str, ty: &SymbolType, init_vals: Vec<&String>) {
        self.data.labels.insert(label.to_string());
        self.data.datas.push(DataSectionItem {
            label: label.to_string(),
            ty: ty.clone(),
            init_vals: init_vals.iter().map(|value| value.to_string()).collect(),
        });
    }
}

impl GlobalVar {
    pub fn push_globalvars(&self, asm: &mut RiscV) {
        asm.push_globalvar(
            &self.var_name,
            &self.var_type,
            self.init_values.iter().map(|value| value).collect(),
        );
    }
}