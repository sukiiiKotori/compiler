/*
检查数据类型
*/

// 检查是否为数字
fn is_digit(c: char) -> bool {
    (c >= '0') && (c <= '9')
}

// 检查是否为十进制数
pub fn is_decimal(s: &str) -> bool {
    if s.len() > 1 && (&s[0..1] == "+" || &s[0..1] == "-") {
        s.chars()
            .skip(1)
            .all(|c| is_digit(c))
    } else {
        s.chars()
            .all(|c| is_digit(c))
    }
}

// 检查是否为十六进制字符 a-f A-F
fn is_hex_letter(c: char) -> bool {
    (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

// 检查是否为十六进制数
pub fn is_hex(s: &str) -> bool {
    s.len() > 2
        && (&s[0..2] == "0x" || &s[0..2] == "0X")
        && s[2..].chars().all(|c| is_digit(c) || is_hex_letter(c))
}

// 检查是否为立即数
pub fn is_immediate(s: &str) -> bool {
    is_decimal(s) || is_hex(s) || s == "0.0"
}

// 检查标号是否为LLVM IR的临时标号
pub fn is_num_label(s: &str) -> bool {
    s.len() >= 2 && s.contains("%") && is_decimal(&s[1..])
}

// 检查标号是否为汇编的临时标号
pub fn is_temp_opr(s: &str) -> bool {
    s.contains("%temp.") && is_decimal(&s[6..])
}

// 检查立即数是否在十二位整型数的表示范围内
pub fn inside_imm_range(s: &str) -> bool {
    // 先检查是否为isize
    let num = s.parse::<isize>().expect(&format!("{} parsing isize failed", s));
    // 再检查是否在12位整数表示范围内
    num >= -2048 && num < 2048
}

