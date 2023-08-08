use crate::ast::InitVal;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::llvm_gen::sysy_gen::*;

// 初始化值
impl InitVal {
    // 确定对齐的维度
    fn init_align(dims: &Vec<i32>, mut fill: i32, now_depth: usize) -> usize {
        if fill < 1 {
            return now_depth + 1;
        } else {
            let mut sub_size: i32 = dims.iter().skip(now_depth).product();
            let mut pos: Vec<i32> = vec!();     // 确定索引
            for i in (now_depth as usize)..(dims.len() - 1) {
                sub_size /= dims[i];
                pos.push(fill/sub_size);
                fill %= sub_size;
            }
            for i in (0..pos.len()).rev() {
                if pos[i] != 0 {
                    return now_depth + i + 1;
                }
            }
            panic!("Error");
        }
    }

    // 填充初始值，缺省值设置为0，类型为Void
    // 这部分实现数组嵌套初始化的功能
    fn init_padding(
        program: &mut LLVMProgram,
        scopes: &mut Scopes,
        labels: &mut Labels,
        items: &Vec<InitVal>,
        dims: &Vec<i32>,
        tys: &mut Vec<SymbolType>,
        vals: &mut Vec<String>, 
        now_depth: usize
    ) -> i32 {
        let sub_size = dims.iter().skip(now_depth).product();
        let mut fill = 0;
    
        for item in items {
            match item {
                InitVal::Exp(exp) => {
                    // 如果是表达式，直接解析初始化值
                    let (ty, val) = exp.generate(program, scopes, labels);
                    tys.push(ty);
                    vals.push(val);
                    fill += 1;
                }
                InitVal::Arr(arr) => {
                    // 数组嵌套初始化中，如果初始化值是一个数组，那么此时已经初始化的部分必须和最低维对齐
                    if fill % dims.last().unwrap() != 0 {
                        panic!("Wrong format of init array");
                    }
                    let next_depth = InitVal::init_align(dims, fill, now_depth);
                    fill += InitVal::init_padding(program, scopes, labels, arr, dims, tys, vals, next_depth);
                }
            }
        }
    
        tys.extend((0..sub_size - fill).map(|_| SymbolType::new(SymbolWidth::Void, true)));
        vals.extend((0..sub_size - fill).map(|_| String::from("0")));
    
        sub_size
    }

    // 生成初始化值
    pub fn generate(
        &self, 
        program: &mut LLVMProgram, 
        scopes: &mut Scopes, 
        labels: &mut Labels, 
        dims: &Vec<i32>
    ) -> (Vec<SymbolType>, Vec<String>) {
        match self {
            InitVal::Exp(exp) => {
                let (ty, val) = exp.generate(program, scopes, labels);
                (vec!(ty), vec!(val))
            },
            InitVal::Arr(arr) => {
                let mut ty: Vec<SymbolType> = vec!();
                let mut val: Vec<String> = vec!();
                if !arr.is_empty() {
                    InitVal::init_padding(program, scopes, labels, &arr, dims, &mut ty, &mut val, 0);
                }
                (ty, val)
            },
        }
    }
}