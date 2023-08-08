use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;
use crate::structures::scopes::*;
use crate::llvm_gen::type_utils::*;

// 获取某元素在多维数组中，实际偏移位置量
fn get_pos(dims: &Vec<i32>, pos: &Vec<i32>) -> i32 {
    let mut sub_size = dims.iter().skip(1).fold(1, | res, a | res * a);

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
fn get_type(types: &Vec<SymbolType>, pos: i32) -> &SymbolType {
    &types[pos as usize]
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
                    ins.push(Instruction::make_instruction(InstructionType::Store, str_vec, ty_vec));
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
                            // 保存剩余维度
                            sub_dims.extend_from_slice(&dims[pos.len()..]);
                        }
                        _ => panic!("Error!"),   // 不是数组（前面已经判断过是数组）
                    }
                    // 生成低维度子数组
                    let sub_arr = SymbolType::new(SymbolWidth::Arr {
                        tar: Box::new(sub_tar),
                        dims: sub_dims,
                    }, ty.is_const);

                    let ty_vec = vec![&sub_arr];
                    let ptr = labels.pop_num_str();
                    let mut str_vec = vec![ptr.as_str(), label.as_str()];
                    let idx = cnt.to_string();      // 当前维度的下标
                    str_vec.push("0");
                    str_vec.push(idx.as_str());

                    let ins_len = ins.len();
                    pos.push(cnt);                          // 放入当前维度下标

                    if traverse_array(program, labels, ty, &ptr, types, vals, ins, pos) {
                        flag = true;
                        ins.insert(ins_len, Instruction::make_instruction(InstructionType::GetElemPtr, str_vec, ty_vec));
                    } else {
                        labels.recover_num();
                    }
                    pos.pop();
                }
                return flag;
            }
        },
        _ => panic!("{} is not an array, ty = {:?}", label, ty),
    }
}

// 数组声明
pub fn decl_arr(
    program: &mut LLVMProgram,
    scopes: &mut Scopes,
    labels: &mut Labels,
    id: &String,
    ty: &SymbolType,
    dims: &Vec<i32>,
    types: &Vec<SymbolType>,
    vals: &Vec<String>,
) {
    // 生成数组类型
    let ty_arr = SymbolType::new(SymbolWidth::Arr { tar: Box::new(ty.clone()), dims: dims.clone()}, false);
    let val_init: Vec<String> = vals.iter().enumerate().map(|(cnt, val)| {
        if types[cnt].width != SymbolWidth::Void {
            type_conver(program, labels, val.clone(), &types[cnt], &ty)
        } else {
            val.clone()
        }
    }).collect();

    // 作用域更新
    if let Some(label) = scopes.push(labels, id.as_str(), &ty_arr, &SymbolVal::Void, None) {
        // 如果是全局作用域，放到Program中
        if scopes.is_global_scope() {
            let init_vals: Vec<&String> = val_init.iter().collect();
            program.push_global_var(&id, &ty_arr, init_vals);
            return;
        }

        // 然后更新作用域
        let str_vec = vec!(label.as_str(), "16");
        let ty_vec = vec!(&ty_arr);
        let settings = crate::get_settings();
        if scopes.is_in_while() || settings.all_allocs_in_entry {
            let x=program.get_block_label();
            program.insert_alloc(
                Instruction::make_instruction(InstructionType::Alloca, str_vec, ty_vec), 
                &x
            )
        } else {
            program.push_instr(InstructionType::Alloca, str_vec, ty_vec);
        }
        
        // 做类型转换，使用BitCase将label转换为一个数组指针类型
        let res = labels.pop_num_str();
        let ty_i8 = SymbolType::new(SymbolWidth::I8, false);    // label类型
        let binary_ty = vec!(&ty_arr, &ty_i8);
        let str_vec = vec!(res.as_str(), label.as_str());
        program.push_instr(InstructionType::BitCast, str_vec, binary_ty);

        // 数组初始化
        let memset_size = dims.iter().fold(1, |acc, x| acc * x) * 4;
        let memset_size_string = memset_size.to_string();
        let memset_funcname = "@llvm.memset.p0i8.i64".to_string();
        let str_vec = vec!("", memset_funcname.as_str(), res.as_str(), "0", &memset_size_string, "false");
        // type向量是对应上面数据向量的类型
        let ty_void = SymbolType::new(SymbolWidth::Void, false);
        let ty_i1 = SymbolType::new(SymbolWidth::Bool, false);
        let ty_i64 = SymbolType::new(SymbolWidth::I64, false);
        let ty_ptr_i8 = SymbolType::new(SymbolWidth::Arr{tar: Box::new(ty_i8.clone()), dims: vec!(-1)}, false);
        let ty_vec = vec!(&ty_void, &ty_ptr_i8, &ty_i8, &ty_i64, &ty_i1);
        program.push_instr(InstructionType::Call, str_vec, ty_vec);

        let mut ins = vec!();
        let mut pos = vec!();
        // 遍历数组
        let tra_flag = traverse_array(program, labels, &ty_arr, &label, types, &val_init, &mut ins, &mut pos);
        if tra_flag {
            for i in ins.into_iter() {
                program.insert_instr(i);
            }
        }
    } else {
        panic!("{} has been declared!", id);
    }
}