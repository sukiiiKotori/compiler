use std::collections::{HashSet, HashMap};
use crate::riscv_gen::stack::StackSlot;
use crate::structures::symbol::*;

/// 表示RISC-V汇编代码的结构体。
#[derive(Debug)]
pub struct RiscV {
    pub text: TextSection,
    // 文本段
    pub data: DataSection,
    // 数据段
    pub rodata: RoDataSection,    // 只读数据段
}

/// 表示RISC-V汇编代码中的只读数据段。
#[derive(Debug)]
pub struct RoDataSection {
    pub datas: Vec<DataSectionItem>,
    // 数据项
    pub labels: HashSet<String>,
    // 标签集合
    pub float_imm: HashMap<String, usize>,
    // 浮点数立即数
    pub float_imm_cnt: usize,    // 浮点数立即数计数器
}

/// 表示RISC-V汇编代码中的数据段。
#[derive(Debug)]
pub struct DataSection {
    pub datas: Vec<DataSectionItem>,
    // 数据项
    pub labels: HashSet<String>,    // 标签集合
}

/// 数据段中的项。
#[derive(Debug)]
pub struct DataSectionItem {
    pub label: String,
    // 标签
    pub ty: SymbolType,
    // 类型
    pub init_vals: Vec<String>,    // 初始化值
}

/// 表示RISC-V汇编代码中的文本段。
#[derive(Debug)]
pub struct TextSection {
    pub funcs: Vec<AsmFunc>,    // 函数列表
}

/// 表示RISC-V汇编代码中的函数。
#[derive(Debug)]
pub struct AsmFunc {
    pub label: String,
    // 标签
    pub ty: SymbolWidth,
    // 宽度
    pub stack: StackSlot,
    // 栈插槽
    pub blocks: Vec<AsmBlock>,
    // 基本块列表
    pub params: HashMap<String, usize>,
    // 参数列表
    pub label_type: HashMap<String, SymbolWidth>,
    // 标签类型映射
    pub call_info: Vec<(usize, Option<usize>, HashSet<String>)>,
    // 函数调用信息
    pub used_saved: HashSet<&'static str>,    // 使用的保存寄存器
}

/// 表示函数中的基本块。
#[derive(Debug)]
pub struct AsmBlock {
    pub label: String,
    // 标签
    pub instrs: Vec<AsmInstr>,
    // 指令列表
    pub sux: Vec<String>,
    // 后继基本块标签列表
    pub instr_cnt: usize,
    // 此基本块之前所有基本块的指令数之和
    pub weight: usize,
    // 循环嵌套权重
    pub depth: usize,    // 循环嵌套深度
}

/// 表示汇编指令的类型。
#[derive(Debug, PartialEq, Clone)]
pub enum AsmInstrType {
    Li,
    // 载入立即数到寄存器
    La,
    // 载入符号地址
    Mv,
    // 移动寄存器的值
    Fmv,
    // 移动浮点数寄存器
    Sextw,
    // 将数据从32位符号扩展到64位
    Add,
    // 加法指令
    Addi,
    // 加法指令（立即数）
    Sub,
    // 减法指令
    Mul,
    // 乘法指令
    Div,
    // 整数除法指令
    Rem,
    // 整数求余指令
    Xori,
    // 异或指令
    Fadd,
    // 浮点加法指令
    Fsub,
    // 浮点减法指令
    Fmul,
    // 浮点乘法指令
    Fdiv,
    // 浮点除法指令
    Fcvt,
    // 浮点数类型转换指令
    Slt,
    // 比较指令（小于）
    Slti,
    // 比较指令（小于，立即数）
    Sgt,
    // 比较指令（大于）
    Seqz,
    // 比较指令（等于零）
    Snez,
    // 比较指令（不等于零）
    Flt,
    Fle,
    Feq,
    // Memory
    Store,
    Load,
    // Branch-Jump
    Branch,
    Jump,
    Ret,
    Call,
}

/// 表示具体的汇编指令。
#[derive(Debug)]
pub enum AsmInstr {
    Li(BinInstr),
    // 伪指令，载入立即数到寄存器
    La(BinInstr),
    // 伪指令，载入符号地址
    Mv(BinInstr),
    // 伪指令，移动寄存器的值
    Fmv(BinInstr, SymbolWidth, SymbolWidth),
    // 伪指令，移动浮点数寄存器
    Sextw(BinInstr),
    // 伪指令，将数据从32位符号扩展到64位(主要用于Load)
// Arith
    Add(TriInstr),
    Addi(TriInstr),
    Sub(TriInstr),
    Mul(TriInstr),
    Div(TriInstr),
    Rem(TriInstr),
    Xori(TriInstr),
    Fadd(TriInstr),
    Fsub(TriInstr),
    Fmul(TriInstr),
    Fdiv(TriInstr),
    Fcvt(BinInstr, SymbolWidth, SymbolWidth),
    // Compare
    Slt(TriInstr),
    Slti(TriInstr),
    Sgt(TriInstr),
    Seqz(BinInstr),
    Snez(BinInstr),
    Flt(TriInstr),
    Fle(TriInstr),
    Feq(TriInstr),
    // Memory
    Store(MemInstr, String),
    Load(MemInstr, String),
    // Branch-Jump
    Branch(CondTriInstr),
    Jump(String),
    Ret(String),
    Call(String, String, Vec<String>, Vec<SymbolWidth>),
}

/// 二元指令结构体，用于存储两个操作数的指令。
#[derive(Debug)]
pub struct BinInstr {
    pub dst: String,
    // 目标寄存器
    pub src: String,    // 源寄存器
}

/// 三元指令条件结构体，用于带有条件的三元指令。
#[derive(Debug)]
pub struct CondTriInstr {
    pub cond: String,
    // 条件寄存器
    pub tri: TriInstr,    // 三元指令
}

/// 三元指令结构体，用于存储三个操作数的指令。
#[derive(Debug)]
pub struct TriInstr {
    pub width: Option<isize>,
    // 宽度
    pub dst: String,
    // 目标寄存器
    pub op1: String,
    // 操作数1
    pub op2: String,    // 操作数2
}

/// 存储/加载指令结构体。
#[derive(Debug)]
pub struct MemInstr {
    pub width: isize,
    // 宽度
    pub val: String,
    // 存储/加载的值
    pub base: String,
    // 基址寄存器
    pub offset: String,    // 偏移量
}

impl AsmFunc {
    /// 将包含指令的块转换为指令引用的向量。
    #[allow(unused)]
    pub fn make_instr_vec(blocks: &Vec<AsmBlock>) -> Vec<&AsmInstr> {
        blocks
            .iter()
            .map(|b| b.instrs.iter().collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    /// 将包含指令的块转换为可变指令引用的向量。
    #[allow(unused)]
    pub fn make_instr_vec_mut(blocks: &mut Vec<AsmBlock>) -> Vec<&mut AsmInstr> {
        blocks
            .iter_mut()
            .map(|b| b.instrs.iter_mut().collect::<Vec<_>>())
            .flatten()
            .collect()
    }
}
