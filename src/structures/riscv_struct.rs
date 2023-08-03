use std::collections::{HashSet, HashMap, BTreeSet};
use crate::riscv_gen::stack_slot::StackSlot;
use crate::structures::symbol::*;

pub const NORMAL_WIDTH: isize = 4;
pub const PTR_WIDTH: isize = 8;

/// 表示RISC-V汇编代码的结构体。
#[derive(Debug)]
pub struct RiscV {
    pub text: TextSection,
    // 文本段
    pub data: DataSection,
    // 数据段
}

impl RiscV {
    /// 创建一个新的RiscV结构体实例，并初始化各个部分的数据结构
    pub fn new() -> Self {
        RiscV {
            text: TextSection::new(),
            data: DataSection::new(),
        }
    }
}

/// 表示RISC-V汇编代码中的数据段。
#[derive(Debug, Default)]
pub struct DataSection {
    pub datas: Vec<DataSectionItem>,
    // 数据项
    pub labels: HashSet<String>,    // 标签集合
}

impl DataSection {
    /// 创建一个新的RoDataSection结构体实例，并初始化各个字段
    pub fn new() -> Self {
        Self::default()
    }
}

/// 数据段中的项。
#[derive(Debug, Default)]
pub struct DataSectionItem {
    pub label: String,
    // 标签
    pub ty: SymbolType,
    // 类型
    pub init_vals: Vec<String>,    // 初始化值
}

/// 表示RISC-V汇编代码中的文本段。
#[derive(Debug, Default)]
pub struct TextSection {
    pub funcs: Vec<AsmFunc>,    // 函数列表
}

impl TextSection {
    pub fn new() -> Self {
        Self::default()
    }
}

/// 表示RISC-V汇编代码中的函数。
#[derive(Debug)]
pub struct AsmFunc {
    pub label: String,
    // 标签
    pub ret_type: SymbolWidth,
    // 返回值类型
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
    pub used_saved: BTreeSet<&'static str>,    
    // 使用的保存寄存器
}

impl AsmFunc {
    pub fn new(label: &str, ret_type: SymbolWidth) -> Self {
        AsmFunc{
            label: String::from(label), 
            ret_type,
            stack: StackSlot::new(), 
            blocks: Vec::new(),
            params: HashMap::new(),
            label_type: HashMap::new(),
            call_info: Vec::new(),
            used_saved: BTreeSet::new(),
        }
    }
}

/// 表示函数中的基本块。
#[derive(Debug)]
pub struct AsmBlock {
    pub label: String,
    // 标签
    pub instrs: Vec<AsmInstruction>,
    // 指令列表
    pub successor: Vec<String>,
    // 后继基本块标签列表
    pub pre_instr_cnt: usize,
    // 此基本块之前所有基本块的指令数之和
    pub weight: usize,
    // 循环嵌套权重
    pub depth: usize,    // 循环嵌套深度
}

impl AsmBlock {
    pub fn new(label: &str, pre_instr_cnt: usize, depth: usize) -> Self {
        AsmBlock {
            label: String::from(label),
            instrs: Vec::new(),
            successor: Vec::new(),
            pre_instr_cnt,
            weight: 10_usize.pow(depth as u32),
            depth,
        }
    }
}

/// 表示汇编指令的类型。
#[derive(Debug, PartialEq, Clone)]
pub enum AsmInstructionType {
    Li,
    // 载入立即数到寄存器
    La,
    // 载入符号地址
    Mv,
    // 移动寄存器的值
    Fmv,
    // 移动浮点数寄存器
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
    Slli,
    // 立即数左移
    Srli,
    //立即数逻辑右移
    Srai,
    //立即数算术右移
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
pub enum AsmInstruction {
    Li(BinInstr),
    // 伪指令，载入立即数到寄存器
    La(BinInstr),
    // 伪指令，载入符号地址
    Mv(BinInstr),
    // 伪指令，移动寄存器的值
    Fmv(BinInstr, SymbolWidth, SymbolWidth),
    // 伪指令，移动浮点数寄存器，前一个类型代表目的寄存器的类型，后一个代表源寄存器的类型
    // 该指令有三种形式，分别是浮点->浮点，整数->浮点，浮点->整数
// Arith
    Add(TriInstr),
    Addi(TriInstr),
    Sub(TriInstr),
    Mul(TriInstr),
    Div(TriInstr),
    Rem(TriInstr),
    Xori(TriInstr),
    Slli(TriInstr),
    Srli(TriInstr),
    Srai(TriInstr),
    Fadd(TriInstr),
    Fsub(TriInstr),
    Fmul(TriInstr),
    Fdiv(TriInstr),
    //类型转换指令
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
    //ret指令，返回值可无
    Ret(),
    Call(String, String, Vec<String>, Vec<SymbolWidth>),
}

/// 二元指令结构体，用于存储两个操作数的指令。
#[derive(Debug)]
pub struct BinInstr {
    pub dst: String,
    // 目标寄存器
    pub src: String,
    // 源寄存器
}

impl BinInstr {
    pub fn new(dst: &str, src: &str) -> Self {
        BinInstr {
            dst: String::from(dst),
            src: String::from(src),
        }
    }
}

/// 三元指令条件结构体，用于带有条件的三元指令。
#[derive(Debug)]
pub struct CondTriInstr {
    pub cond: String,
    // 条件寄存器
    pub tri: TriInstr,    // 三元指令
}

impl CondTriInstr {
    pub fn new(cond: &str, width: Option<isize>, dst: &str, op1: &str, op2: &str) -> Self {
        CondTriInstr {
            cond: String::from(cond),
            tri: TriInstr::new(width, dst, op1, op2),
        }
    }
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
    pub op2: String,
    // 操作数2
}

impl TriInstr {
    pub fn new(width: Option<isize>, dst: &str, op1: &str, op2: &str) -> Self {
        TriInstr {
            width,
            dst: String::from(dst),
            op1: String::from(op1),
            op2: String::from(op2),
        }
    }
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
    pub offset: String,
    // 偏移量
}

impl MemInstr {
    pub fn new(width: isize, val: &str, base: &str, offset: &str) -> Self {
        MemInstr {
            width,
            val: String::from(val),
            base: String::from(base),
            offset: String::from(offset),
        }
    }
}


impl RiscV {
    pub fn deterministic_stack(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.deterministic_stack();
        }
    }

    pub fn stack_alloc_free(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.stack_alloc_free();
        }
    }

    pub fn map_stack_address(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.map_stack_address();
        }
    }
}

impl AsmInstruction {
    pub fn fetch_type(&self) -> AsmInstructionType {
        match self {
            AsmInstruction::Li(_) => AsmInstructionType::Li,
            AsmInstruction::La(_) => AsmInstructionType::La,
            AsmInstruction::Mv(_) => AsmInstructionType::Mv,
            AsmInstruction::Fmv(_, _, _) => AsmInstructionType::Fmv,
            AsmInstruction::Add(_) => AsmInstructionType::Add,
            AsmInstruction::Addi(_) => AsmInstructionType::Addi,
            AsmInstruction::Sub(_) => AsmInstructionType::Sub,
            AsmInstruction::Mul(_) => AsmInstructionType::Mul,
            AsmInstruction::Div(_) => AsmInstructionType::Div,
            AsmInstruction::Rem(_) => AsmInstructionType::Rem,
            AsmInstruction::Xori(_) => AsmInstructionType::Xori,
            AsmInstruction::Slli(_) => AsmInstructionType::Slli,
            AsmInstruction::Srli(_) => AsmInstructionType::Srli,
            AsmInstruction::Srai(_) => AsmInstructionType::Srai,
            AsmInstruction::Fadd(_) => AsmInstructionType::Fadd,
            AsmInstruction::Fsub(_) => AsmInstructionType::Fsub,
            AsmInstruction::Fmul(_) => AsmInstructionType::Fmul,
            AsmInstruction::Fdiv(_) => AsmInstructionType::Fdiv,
            AsmInstruction::Fcvt(_, _, _) => AsmInstructionType::Fcvt,
            AsmInstruction::Slt(_) => AsmInstructionType::Slt,
            AsmInstruction::Slti(_) => AsmInstructionType::Slti,
            AsmInstruction::Sgt(_) => AsmInstructionType::Sgt,
            AsmInstruction::Seqz(_) => AsmInstructionType::Seqz,
            AsmInstruction::Snez(_) => AsmInstructionType::Snez,
            AsmInstruction::Flt(_) => AsmInstructionType::Flt,
            AsmInstruction::Fle(_) => AsmInstructionType::Fle,
            AsmInstruction::Feq(_) => AsmInstructionType::Feq,
            AsmInstruction::Store(_, _) => AsmInstructionType::Store,
            AsmInstruction::Load(_, _) => AsmInstructionType::Load,
            AsmInstruction::Branch(_) => AsmInstructionType::Branch,
            AsmInstruction::Jump(_) => AsmInstructionType::Jump,
            AsmInstruction::Ret() => AsmInstructionType::Ret,
            AsmInstruction::Call(_, _, _, _) => AsmInstructionType::Call,
        }
    }
}
