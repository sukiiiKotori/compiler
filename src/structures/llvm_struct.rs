use crate::structures::symbol::*;

// llvm程序开始
#[derive(Debug, Default)]
pub struct LLVMProgram {
    pub global_var: Vec<GlobalVar>,
    pub func_decl: Vec<FuncDecl>,
    pub func_def: Vec<FuncDef>,
}

// 全局变量
#[derive(Debug)]
pub struct GlobalVar {
    pub var_name: String,
    pub var_type: SymbolType,
    pub init_num: Vec<InitNumber>,
}

// 初始化值
#[derive(Debug)]
pub struct InitNumber {
    pub init_type: SymbolType,
    pub init_val: String,
}

// 函数声明
#[derive(Debug)]
pub struct FuncDecl {
    pub func_name: String,
    pub func_type: SymbolType,
    pub param_types: Vec<SymbolType>,
}

// 函数定义
#[derive(Debug)]
pub struct FuncDef {
    pub func_name: String,
    pub func_type: SymbolType,
    pub params: Vec<Param>,
    pub blocks: Vec<Block>,
    pub local_vars: Vec<LocalVar>,
}

// 参数
#[derive(Debug)]
pub struct Param {
    pub param_name: String,
    pub param_type: SymbolType,
}

// 基本块
#[derive(Debug)]
pub struct Block {
    pub block_label: String,        // 标签
    pub phi_ins: Vec<Instruction>,  // phi指令
    pub nor_ins: Vec<Instruction>,  // 一般指令
    pub fin_ins: Vec<Instruction>,  // 终结指令
    pub ins_num: usize,             // 此基本块之前的所有基本块的指令数量
    pub depth: usize,               // 此基本块的循环嵌套深度
}

// 局部变量
#[derive(Debug)]
pub struct LocalVar {
    pub ins: Instruction,
    pub label: String,
}

// 指令
#[derive(Debug)]
pub enum Instruction {
    // 算术运算
    // int
    Add(BinaryOp),
    Sub(BinaryOp),
    Mul(BinaryOp),
    Sdiv(BinaryOp),
    Srem(BinaryOp),
    // float
    Fadd(BinaryOp),
    Fsub(BinaryOp),
    Fmul(BinaryOp),
    Fdiv(BinaryOp),
    // 类型转换
    ZeroExt(CastOp),
    I32ToFloat(CastOp),
    FloatToI32(CastOp),
    // 比较运算
    Cmp(String, BinaryOp),
    Fcmp(String, BinaryOp),
    // Phi
    Phi(String, SymbolType, Vec<(String, String)>),
    // 变量
    Alloca{res: String, ty: SymbolType, len: String},
    Store{ty: SymbolType, value: String, ptr: String, len: String},
    Load{res: String, ty: SymbolType, ptr: String, len: String},
    // 函数
    Call(String, String, SymbolType, Vec<(String, SymbolType)>),
    // 数组
    GetElemPtr(String, SymbolType, String, Vec<String>),
    BitCast(String, SymbolType, String, SymbolType),
    // 注释
    Comment(String),
    // 终结指令
    Ret(SymbolType, Option<String>),
    Br(Option<String>, String, Option<String>),
}

#[derive(Debug, PartialEq)]
pub enum InstructionType {
    Add,
    Sub,
    Mul,
    Sdiv,
    Srem,
    Fadd,
    Fsub,
    Fmul,
    Fdiv,
    ZeroExt,
    I32Tofloat,
    FloatToI32,
    Cmp,
    Fcmp,
    Phi,
    Alloca,
    Store,
    Load,
    Call,
    GetElemPtr,
    BitCast,
    Comment,
    Ret,
    Br,
}

// 二元操作
#[derive(Debug, Default)]
pub struct BinaryOp {
    pub res: String,
    pub op_type: SymbolType,
    pub op1: String,
    pub op2: String,
}

// 转换操作
#[derive(Debug, Default)]
pub struct CastOp {
    pub res: String,
    pub type_1: SymbolType,
    pub type_2: SymbolType,
    pub val: String,
}

impl LLVMProgram {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BinaryOp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CastOp {
    pub fn new() -> Self {
        Self::default()
    }
}