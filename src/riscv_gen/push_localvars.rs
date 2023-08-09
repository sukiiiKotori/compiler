use crate::structures::llvm_struct::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

impl LLVMProgram {
    pub fn push_localvars(&self, asm: &mut RiscV) {
        self.func_def.iter().for_each(|func| func.push_localvars(asm));
    }
}

impl FuncDef {
    pub fn push_localvars(&self, asm: &mut RiscV) {
        if let Some(func) = asm.text.funcs.iter_mut().find(|func| func.label == self.func_name.replace("@", "")) {
            let stack = &mut func.stack;
            let label_type = &mut func.label_type;
            //局部变量全都存入栈中
            self.local_vars.iter().for_each(|local_var|{
                if let Instruction::Alloca{res, ty, len: _} = &local_var.ins {
                    label_type.insert(res.to_string(), ty.width.clone());
                    if let SymbolWidth::Arr{tar:_, dims} = &ty.width {
                        //如果是指针，把长度设为8
                        if dims[0] == -1 {
                            stack.push_normal(res, 8);
                        } else {
                            let len = dims.iter().fold(4, |acc, x| acc * x);
                            stack.push_normal(res, len as isize);
                        }
                    } else {
                        stack.push_normal(res,  4);
                    }
                }
            });
        }
    }
}