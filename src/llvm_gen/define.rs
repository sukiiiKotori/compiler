use crate::ast::ConstDef;
use crate::ast::VarDef;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::llvm_gen::sysy_gen::*;

// 常量定义
impl Generate for ConstDef {
    type Out = (String, Vec<SymbolType>, Vec<String>, Vec<i32>);

    fn generate(
            &self,
            program: &mut LLVMProgram,
            scopes: &mut Scopes,
            labels: &mut Labels
        ) -> Self::Out {
            if !self.dims.is_empty() {
                if !scopes.is_global_scope() {
                    program.push_comment(&format!("; init {}\n", self.id));
                }
            }
            
            let dims: Vec<i32> = self.dims
                .iter()
                .map(|dim| dim.generate(program, scopes, labels).1.parse().unwrap())
                .collect::<Vec<i32>>();
            
            let (ty, val) = self.init.generate(program, scopes, labels, &dims);
            
            (String::from(self.id.as_str()), ty, val, dims)
    }
}

/// 变量定义
impl Generate for VarDef {
    type Out = (String, Vec<SymbolType>, Vec<String>, Vec<i32>);

    fn generate(&self, program: &mut LLVMProgram, scopes: &mut Scopes, labels: &mut Labels) -> Self::Out {
        let mut dims: Vec<i32> = vec!();
        for dim in self.dims.iter() {
            let (_, val) = dim.generate(program, scopes, labels);
            dims.push(val.parse().expect(&format!("{} is not integer", val)));
        }

        // 是否初始化
        if let Some(init) = &self.init {
            if !scopes.is_global_scope() {
                program.push_comment(format!("; init {}\n", self.id).as_str());
            }
            let (ty, val) = init.generate(program, scopes, labels, &dims);
            (String::from(self.id.as_str()), ty, val, dims)
        } else {
            if !scopes.is_global_scope() && !dims.is_empty() {
                program.push_comment(format!("; init {}\n", self.id).as_str());
            }
            (String::from(self.id.as_str()), vec!(), vec!(), dims)
        }
    }
}