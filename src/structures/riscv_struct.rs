use std::collections::{HashSet, HashMap};
use crate::riscv_gen::stack::StackSlot;
use crate::structures::symbol::*;

#[derive(Debug)]
pub struct RiscV {
    pub text: TextSection,
    pub data: DataSection,
    pub rodata: RoDataSection,
}

#[derive(Debug)]
pub struct RoDataSection {
    pub datas: Vec<DataSectionItem>,
    pub labels: HashSet<String>,
    pub float_imm: HashMap<String, usize>,
    pub float_imm_cnt: usize,
}

#[derive(Debug)]
pub struct DataSection {
    pub datas: Vec<DataSectionItem>,
    pub labels: HashSet<String>,
}

#[derive(Debug)]
pub struct DataSectionItem {
   pub label: String,
   pub ty: SymbolType,
   pub init_vals: Vec<String>,
}

#[derive(Debug)]
pub struct TextSection {
    pub funcs: Vec<AsmFunc>,
}

#[derive(Debug)]
pub struct AsmFunc {
    pub label: String,
    pub ty: SymbolWidth,
    pub stack: StackSlot,
    pub blocks: Vec<AsmBlock>,
    pub params: HashMap<String, usize>,
    pub label_type: HashMap<String, SymbolWidth>,
    pub call_info: Vec<(usize, Option<usize>, HashSet<String>)>,
    pub used_saved: HashSet<&'static str>,
}

#[derive(Debug)]
pub struct AsmBlock {
    pub label: String,
    pub instrs: Vec<AsmInstr>,
    pub sux: Vec<String>,
    pub instr_cnt: usize, // 此block之前所有block的指令数之和
    pub weight: usize, // 循环嵌套权重
    pub depth: usize, // 循环嵌套深度
}

#[derive(Debug, PartialEq, Clone)]
pub enum AsmInstrType {
    Li,
    La,
    Mv,
    Fmv,
    Sextw,
    // Arith
    Add,
    Addi,
    Sub,
    Mul,
    Div,
    Rem,
    Xori,
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    Fcvt,
    // Compare
    Slt,
    Slti,
    Sgt,
    Seqz,
    Snez,
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

#[derive(Debug)]
pub enum AsmInstr {
    Li(BinInstr), // 伪指令，载入立即数到寄存器
    La(BinInstr), // 伪指令，载入符号地址
    Mv(BinInstr), // 伪指令，移动寄存器的值 
    Fmv(BinInstr, SymbolWidth, SymbolWidth), // 伪指令，移动浮点数寄存器
    Sextw(BinInstr), // 伪指令，将数据从32为符号扩展到64位(主要用于Load)
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

#[derive(Debug)]
pub struct BinInstr {
    pub dst: String,
    pub src: String,
}

#[derive(Debug)]
pub struct CondTriInstr {
    pub cond: String,
    pub tri: TriInstr
}

#[derive(Debug)]
pub struct TriInstr {
    pub width: Option<isize>,
    pub dst: String,
    pub op1: String,
    pub op2: String,
}

#[derive(Debug)]
pub struct MemInstr {
    pub width: isize,
    pub val: String,
    pub base: String,
    pub offset: String,
}

impl AsmFunc {
    #[allow(unused)]
    pub fn make_instr_vec(blocks: &Vec<AsmBlock>) -> Vec<&AsmInstr> {
        blocks.iter()
            .map(|b| b.instrs.iter().collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    #[allow(unused)]
    pub fn make_instr_vec_mut(blocks: &mut Vec<AsmBlock>) -> Vec<&mut AsmInstr> {
        blocks.iter_mut()
            .map(|b| b.instrs.iter_mut().collect::<Vec<_>>())
            .flatten()
            .collect()
    }
}

