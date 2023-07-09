use std::error::Error;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::llvm_gen::scopes::*;

// 获取某元素在多维数组中，实际偏移位置量
fn get_pos(dims: &Vec<i32>, pos: &Vec<i32>) -> i32 {
    let mut sub_size = dims.iter().skip(1).fold(1, | res, &a | res * a);

    let mut res = pos.first().unwrap().clone();
    res *= sub_size;
    for i in 1..pos.len() {
        sub_size /= dims[i];
        res += pos[i] * sub_size;
    }
    res
}

// 获取数值
fn get_val(vals: &Vec<String>, pos: i32) -> String {
    vals[pos as usize].to_string()
}

// 获取数据类型
fn get_type(types: &Vec<SymbolType>, pos: i32) -> SymbolType {
    types[pos as usize]
}

// 遍历多维数组，使用GetElemPtr或者Store指令初始化，成功返回true
fn traverse_array(
    program: &mut LLVMProgram,  // 程序
    labels: &mut Labels,        // 管理和生成指针标号的结构体的引用。
    ty: &SymbolType,            // 类型
    label: &String,             // 当前维度的指针标号
    types: &Vec<SymbolType>,    // 数组内类型
    vals: &Vec<String>,         // 值
    ins: &mut Vec<Instruction>, // 指令
    pos: &mut Vec<i32>,         // 索引
) -> bool {
    // 数组内类型为空或者值为空
    if types.is_empty() || vals.is_empty() {
        return false;
    }

    match &ty.width {
        SymbolWidth::Arr { tar, dims } => {
            let mut flag = true;
            // 长度相等，表示遍历到底层
            if pos.len() == dims.len() {
                let elem_pos = get_pos(dims, pos);          // 元素偏移位置
                let elem_ty = get_type(types, elem_pos);    // 元素类型

                if elem_ty.width != SymbolWidth::Void {     // 不是空
                    let tar_ty = tar.as_ref().clone();
                    let ty_vec = vec![&tar_ty];
                    let arr_val = get_val(vals, elem_pos);
                    let str_vec = vec![arr_val.as_str(), label.as_str(), "4"];
                    // 生成store指令
                    ins.push(Instruction::make_instr(InstructionType::Store, str_vec, ty_vec));
                } else {
                    flag = false;
                }
                return flag;
            } else {
                // 长度不等，表明在中间层次
                let mut flag = false;
                let dim_range = dims[pos.len()];    // 当前维度大小

                for cnt in 0..dim_range {           // 遍历当前维度
                    let mut sub_dims: Vec<i32> = vec![];    // 剩余维度
                    let sub_tar: SymbolType;

                    match &ty.width {
                        SymbolWidth::Arr { tar, dims } => {
                            sub_tar = tar.as_ref().clone();
                            sub_dims.extend_from_slice(&dims[pos.len() + 1..]); // 保存剩余维度
                        }
                        _ => panic!("Error!"),   // 不是数组（前面已经判断过是数组）
                    }
                    
                    // 生成低维度子数组
                    let left_arr = SymbolType::new(SymbolWidth::Arr {
                        tar: Box::new(sub_tar),
                        dims: sub_dims,
                    }, ty.is_const);

                    let ty_vec = vec![&left_arr];
                    let ptr = labels.pop_num_str();
                    let mut str_vec = vec![ptr.as_str(), label.as_str()];
                    let idx = cnt.to_string();
                    str_vec.push("0");
                    str_vec.push(idx.as_str());

                    let ins_len = ins.len();
                    pos.push(cnt);

                    if traverse_array(program, labels, ty, &ptr, types, vals, ins, pos) {
                        flag = true;
                        ins.insert(
                            ins_len,
                            Instruction::make_instr(InstructionType::GetElemPtr, str_vec, ty_vec),
                        );
                    } else {
                        labels.recover_num();
                    }

                    pos.pop();
                }

                return flag;
            }
        }
        _ => panic!("{} is not an array, ty = {:?}", label, ty),
    }
}

