use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

impl LLVMProgram {
    pub fn push_arguments(&self, asm: &mut RiscV) {
        self.func_def.iter().for_each(|func| func.push_arguments(asm));
    }
}

impl FuncDef {
    pub fn push_arguments(&self, asm: &mut RiscV) {
        if let Some(func) = asm.text.funcs.iter_mut().find(|func| func.label == self.func_name.replace("@", "")) {
            let stack = &mut func.stack;
            let mut int_cnt = 0;
            let mut float_cnt = 0;
            self.params.iter().for_each(|param| {
                //将参数放入函数参数寄存器中。
                //若函数参数的个数多于8个，则需要把多余的参数压栈
                if param.param_type.width == SymbolWidth::Float {
                    if float_cnt >= 8 {
                        stack.push_param(&param.param_name, 4);
                    }
                    func.params.insert(param.param_name.clone(), float_cnt);
                    float_cnt += 1;
                } else {
                    //i32或者指针
                    if int_cnt >= 8 {
                        stack.push_param(&param.param_name, param.param_type.get_width() as isize);
                    }
                    func.params.insert(param.param_name.clone(), int_cnt);
                    int_cnt += 1;
                }
            });
        }
    }
}