use crate::structures::llvm_struct::{LLVMProgram, InstructionType};
use crate::structures::symbol::*;
use crate::llvm_gen::scopes::Labels;
use crate::utils::float::{format_double, parse_float};


/// 检查数组/指针是否为同一类型
/// 当一层出现指针时，将这一层恒定为相同类型
fn array_is_equal(arr1: &SymbolType, arr2: &SymbolType) -> bool {
    if let (SymbolWidth::Arr{tar: tar1, dims: dims1}, SymbolWidth::Arr{tar: tar2, dims: dims2}) = (&arr1.width, &arr2.width) {

        let mut dims1 = dims1.clone();
        let mut dims2 = dims2.clone();
        // 对于数组/指针为const的情况，需额外加入一层指针
        if arr1.is_const {
            dims1.insert(0, -1);
        }
        if arr2.is_const {
            dims2.insert(0, -1);
        }

        if (tar1.width != tar2.width) || (dims1.len() != dims2.len()) {
            return false;
        }
        for cnt in 0..dims1.len() {
            if (dims1[cnt] != dims2[cnt]) && (dims1[cnt] != -1) && (dims2[cnt] != -1) {
                return false;
            }
        }
        true
    } else {
        panic!("Should not appear @ llvm_gen/symbol.rs:arr_is_eq");
    }
}

/// 检查ty是否为数组/指针
fn is_array(ty: &SymbolType) -> bool {
    if let SymbolWidth::Arr{tar: _, dims: _} = ty.width {
        true
    } else {
        false
    }
}

/// 常量和变量的类型转换
pub fn type_conver(program: &mut LLVMProgram, 
    labels: &mut Labels, value: String, ty1: &SymbolType, ty2: &SymbolType) -> String {

    // 类型一样直接返回
    if ty1.width == ty2.width {
        return value;
    }
    // 处理数组/指针情况
    if let (SymbolWidth::Arr{tar: tar1, dims: dims1}, SymbolWidth::Arr{tar: tar2, dims: dims2}) = (&ty1.width, &ty2.width) {
        if array_is_equal(ty1, ty2) {
            return value;
        }

        let mut dims1 = dims1.clone();
        if ty1.is_const {
            dims1.insert(0, -1);
        }
        let mut dims2 = dims2.clone();
        if ty2.is_const {
            dims2.insert(0, -1);
        }
        dims1.remove(0);
        dims2.remove(0);
        let new_ty1 = SymbolType::new(SymbolWidth::Arr{tar: tar1.clone(), dims: dims1}, false);
        let new_ty2 = SymbolType::new(SymbolWidth::Arr{tar: tar2.clone(), dims: dims2}, false);

        let cast_res = labels.pop_num_str();
        let ty_vec = vec!(&new_ty1, &new_ty2);
        let str_vec = vec!(cast_res.as_str(), value.as_str());
        program.push_instr(InstructionType::BitCast, str_vec, ty_vec);

        return cast_res;
    }

    let is_var = ty1.is_const == false;

    if is_var {
        if ty1.width > ty2.width {
            let mut result = value;
            if is_array(ty1) {
                panic!("Should not appear @ llvm_gen/symbol.rs:type_conver 1");
            } else if ty1.width == SymbolWidth::Float {
                let label = labels.pop_num_str();
                program.push_instr(
                    InstructionType::FloatToI32, 
                    vec!(&label, &result),
                    vec!(&ty1, &ty2),
                );
                result = label;
            } else if ty2.width == SymbolWidth::I1 {
                let ty_vec = vec!(ty1);
                let label = labels.pop_num_str();
                let str_vec = vec!("ne", label.as_str(), "0", result.as_str());
                program.push_instr(InstructionType::Cmp, str_vec, ty_vec);
                result = label;
            }
            result
        } else {
            let mut result = value;
            if is_array(ty2) {
                panic!("Should not appear @ llvm_gen/symbol.rs:type_conver 2");
            } else if ty2.width == SymbolWidth::Float {
                let label = labels.pop_num_str();
                program.push_instr(
                    InstructionType::I32Tofloat, 
                    vec!(&label, &result),
                    vec!(&ty1, &ty2),
                );
                result = label;
            } else {
                let label = labels.pop_num_str();
                program.push_instr(
                    InstructionType::ZeroExt, 
                    vec!(&label, &result),
                    vec!(&ty1, &ty2),
                );
                result = label;
            } // width match
            result
        } // width compare
    } else { // is_var else
        if ty1.width > ty2.width {
            if is_array(ty1) {
                panic!("Should not appear @ llvm_gen/symbol.rs:type_conver 3");
            } else if ty1.width == SymbolWidth::Float {
                let num: f32 = parse_float(value.as_str());
                let int_num: i32 = num as i32;
                int_num.to_string()
            } else if ty2.width == SymbolWidth::I1 {
                let num: i32 = value.parse().unwrap();
                let bool_num = (num != 0) as i32;
                bool_num.to_string()
            } else {
                value
            }
        } else {
            if is_array(ty2) {
                panic!("Should not appear @ llvm_gen/symbol.rs:type_conver 4");
            } else if ty2.width == SymbolWidth::Float {
                let result: i32 = value.parse().unwrap();
                format_double(parse_float(result.to_string().as_str()))
            } else {
                value
            }
        }// if ty1.width > ty2.width
    }// is_var
}

/// 比较两个数值类型，若类型不同，调用type_conver将类型低的转换为高的，最后返回比较结果
pub fn type_cmpare(program: &mut LLVMProgram, labels: &mut Labels, 
    ty1: SymbolType, op1: String, ty2: SymbolType, op2: String) -> (SymbolType, String, String) {

    // 类型相同的情况
    if ty1.width == ty2.width {
        return (ty1, op1, op2);
    }
    // 低类型转化为高类型
    if ty1.width > ty2.width {
        let op2 = type_conver(program, labels, op2, &ty2, &ty1);
        (ty1, op1, op2)
    } else {
        let op1 = type_conver(program, labels, op1, &ty1, &ty2);
        (ty2, op1, op2)
    }
}

