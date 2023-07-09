use std::error::Error;
use crate::symbol::*;

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

