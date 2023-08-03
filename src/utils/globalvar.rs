use std::io::*;
use crate::structures::symbol::*;
use crate::structures::llvm_struct::*;

impl GlobalVar {
    fn get_range(dims: &Vec<i32>, pos: &Vec<i32>) -> (i32, i32) {
        let mut start = 0;
        let mut left_size = dims.iter().fold(1, |acc, x| acc*x);
        for cnt in 0..pos.len() {
            left_size /= dims[cnt];
            start += pos[cnt] * left_size;
        }
        let end = start+dims.iter().skip(pos.len()).fold(1, |acc, x| acc*x);
        (start, end)
    }

    fn all_is_zero(vals: &Vec<String>, start: i32, end: i32) -> bool {
        if vals.is_empty() {
            return true;
        }
        let start = start as usize;
        let end = end as usize;
        vals[start..end].iter().all(|x| x == "0")
    }

    pub fn dump_arr_init(output: &mut impl Write, dims: &Vec<i32>, ty: &SymbolType, vals: &Vec<String>, pos: &mut Vec<i32>) {
        let (start, end) = GlobalVar::get_range(dims, pos);
        if pos.len() == dims.len() {
            match &ty.width {
                SymbolWidth::Arr{tar, dims: _} => write!(output, "{} {}", tar.get_typename(), vals[start as usize]).unwrap(),
                _ => panic!("Should not appear"),
            }
        } else {
            match &ty.width {
                SymbolWidth::Arr{tar, dims} => write!(output, "{} ", tar.get_name(&dims[pos.len()..])).unwrap(),
                _ => panic!("Should not appear"),
            }
            if vals.is_empty() || GlobalVar::all_is_zero(vals, start, end) {
                write!(output, " zeroinitializer").unwrap();
            } else {
                write!(output, " [").unwrap();
                for cnt in 0..dims[pos.len()] {
                    if cnt != 0 {
                        write!(output, ", ").unwrap();
                    }
                    pos.push(cnt);
                    GlobalVar::dump_arr_init(output, dims, ty, vals, pos);
                    pos.pop();
                }
                write!(output, "]").unwrap();
            }
        }
    }
}