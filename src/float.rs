use std::str::FromStr;

/// 将f32转为IEEE754 double
pub fn format_double(float: f32) -> String {
    let double = float as f64;
    let num64 = unsafe {
        std::mem::transmute::<f64,u64>(double)
    };
    format!("0x{:016X}",num64)
}

/// 将IEEE754 double 转为 IEEE754 float
pub fn double_to_float(double: &str) -> String {
    let float: f32;
    if double == "0.0" {
        float = 0.0;
    } else {
        float = parse_float(double);
    }
    let num32 = unsafe {
        std::mem::transmute::<f32,u32>(float)
    };
    format!("0X{:08X}",num32)
}

/// 把IEEE754标准表示的单精度或双精度浮点转换为f32
fn parse_IEEE(hex_string_mem: &str) -> f32 {
    if hex_string_mem.len() == 10 { // 2(0x)+8(8*4=32)
        let num32 = u32::from_str_radix(&hex_string_mem[2..], 16).unwrap();
        unsafe {
            std::mem::transmute::<u32, f32>(num32)
        }
    } else if hex_string_mem.len() == 18 {
        let num64 = u64::from_str_radix(&hex_string_mem[2..], 16).unwrap();
        unsafe {
            std::mem::transmute::<u64, f64>(num64) as f32
        }
    } else {
        panic!("Length of hex {} wrong", hex_string_mem);
    }
}

/// 解析符合ISO标准的C语言浮点数
pub fn parse_float(s: &str) -> f32 {
    if s.contains("0x") || s.contains("0X") { // hex
        if s.contains("p") || s.contains("P") { // 0x(...)P/p
            let v : Vec<&str> = if s.contains("p") {
                s.split("p").collect()
            } else {
                s.split("P").collect()
            };

            if v[0].contains(".") { // 0x1af.p2, 0xaf.fep-4
                let w: Vec<&str> = v[0].split(".").collect();
                let integer: f64;
                let decimal: f64;
                if &w[0][2..] != "" {// 0xaf.fep-4
                    integer = u64::from_str_radix(&w[0][2..], 16).unwrap() as f64;
                } else {// 0x.fep-4
                    integer = 0.0;
                }
                if w[1] != ""{// 0xaf.fep-4
                    decimal = u64::from_str_radix(w[1], 16).unwrap() as f64 
                    /16_f64.powi(s.len() as i32);
                } else {// 0x1af.p2
                    decimal = 0.0;
                }

                let exp: f64 = f64::from_str(v[1]).unwrap();
                let res = (integer + decimal) * 2_f64.powf(exp);
                res as f32
            } else { // 0xap3
                let integer: f64;
                if &v[0][2..] != "" {
                    integer = u64::from_str_radix(&v[0][2..], 16).unwrap() as f64;
                } else {
                    integer = 0.0;
                }
                let exp: f64 = f64::from_str(v[1]).unwrap();
                let res = integer * 2_f64.powf(exp);
                res as f32
            }
        } 
        else { // IEEE
            parse_IEEE(s)
        }
    } else { // dec
        f32::from_str(s).unwrap()
    }
}

#[cfg(test)]
mod test_float {
    use super::*;
    use std::io::Write;
    #[test]
	fn conver_floats() {
	    const EXAMPLES: [&str; 14] = [
	        "5.5", 
	        "03.141592653589793", 
	        "1e-6", 
	        "0x1.921fb6p+1", 
	        "0x.AP-3", 
	        ".33E+5", 
	        "1e9",
	        "233",
	        "3.2",
	        "2.",
	        "1e1",
	        "0x1.7487b2p+7",
            "0x5b.acp1",
            "0x1af.p2"
	    ];
        let mut output = std::fs::File::create("test_float.log").unwrap();
	    for example in EXAMPLES.iter() {
	        write!(output, "example = {}\n", example).unwrap();
	        let x = parse_float(example);
	        let hex_x = format_double(x);
	        write!(output, "x = {}, hex x = {}\n", x, hex_x).unwrap();
	
	        let new_x = parse_float(hex_x.as_str());
	        let new_hex_x = format_double(new_x);
	        write!(output, "new x = {}, new hex x = {}\n\n", new_x, new_hex_x).unwrap();
	    }
	}
}