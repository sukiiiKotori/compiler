use std::io::*;
use crate::structures::symbol::*;

fn get_se(dims: &[i32], pos: &[i32]) -> (i32, i32) {
    let mut start = 0;
    let mut left_size = dims.iter().product::<i32>();
    
    for (idx, &p) in pos.iter().enumerate() {
        left_size /= dims[idx];
        start += p * left_size;
    }
    
    let end = start + dims.iter().skip(pos.len()).product::<i32>();
    (start, end)
}

pub fn write_arr(output: &mut impl Write, dims: &Vec<i32>, ty: &SymbolType, vals: &Vec<String>, pos: &mut Vec<i32>) {
    let (start, end) = get_se(dims, pos);
    //pos代表递归次数
    if pos.len() == dims.len() {
        match &ty.width {
            SymbolWidth::Arr{tar, dims: _} => write!(output, "{} {}", tar.get_typename(), vals[start as usize]).unwrap(),
            _ => panic!(),
        }
    } else {
        match &ty.width {
            SymbolWidth::Arr{tar, dims} => write!(output, "{} ", tar.get_name(&dims[pos.len()..])).unwrap(),
            _ => panic!(),
        }
        if vals.is_empty() {
            write!(output, " zeroinitializer").unwrap();
        } else if vals[start as usize..end as usize].iter().all(|x| x == "0") {
            write!(output, " zeroinitializer").unwrap();
        } else {
            write!(output, " [").unwrap();
            for cnt in 0..dims[pos.len()] {
                if cnt != 0 {
                    write!(output, ", ").unwrap();
                }
                pos.push(cnt);
                write_arr(output, dims, ty, vals, pos);
                pos.pop();
            }
            write!(output, "]").unwrap();
        }
    }
}