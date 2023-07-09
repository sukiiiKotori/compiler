use crate::structures::symbol::*;

/// LLVM程序
#[derive(Debug, Default)]
pub struct LLVMProgram {
    /// 全局变量列表
    pub global_var: Vec<GlobalVar>,
    /// 函数声明列表
    pub func_decl: Vec<FuncDecl>,
    /// 函数定义列表
    pub func_def: Vec<FuncDef>,
}

/// 全局变量
#[derive(Debug)]
pub struct GlobalVar {
    /// 变量名称
    pub var_name: String,
    /// 变量类型
    pub var_type: SymbolType,
    /// 初始化值列表
    pub init_num: Vec<InitNumber>,
}

/// 初始化值
#[derive(Debug)]
pub struct InitNumber {
    /// 初始化值类型
    pub init_type: SymbolType,
    /// 初始化值
    pub init_val: String,
}

/// 函数声明
#[derive(Debug)]
pub struct FuncDecl {
    /// 函数名称
    pub func_name: String,
    /// 函数类型
    pub func_type: SymbolType,
    /// 参数类型列表
    pub param_types: Vec<SymbolType>,
}

/// 函数定义
#[derive(Debug)]
pub struct FuncDef {
    /// 函数名称
    pub func_name: String,
    /// 函数类型
    pub func_type: SymbolType,
    /// 参数列表
    pub params: Vec<Param>,
    /// 基本块列表
    pub blocks: Vec<Block>,
    /// 局部变量列表
    pub local_vars: Vec<LocalVar>,
}

/// 参数
#[derive(Debug)]
pub struct Param {
    /// 参数名称
    pub param_name: String,
    /// 参数类型
    pub param_type: SymbolType,
}

/// 基本块
#[derive(Debug)]
pub struct Block {
    /// 基本块标签
    pub block_label: String,
    /// phi指令列表
    pub phi_ins: Vec<Instruction>,
    /// 一般指令列表
    pub nor_ins: Vec<Instruction>,
    /// 终结指令
    pub ter_ins: Option<Instruction>,
    /// 此基本块之前的所有基本块的指令数量
    pub ins_num: usize,
    /// 此基本块的循环嵌套深度
    pub depth: usize,
}

/// 局部变量
#[derive(Debug)]
pub struct LocalVar {
    /// 指令
    pub ins: Instruction,
    /// 标签
    pub label: String,
}


#[derive(Debug)]
pub enum Instruction {
    // 算术运算
    // int
    /// 加法指令
    Add(BinaryOp),
    /// 减法指令
    Sub(BinaryOp),
    /// 乘法指令
    Mul(BinaryOp),
    /// 整数除法指令
    Sdiv(BinaryOp),
    /// 整数求余指令
    Srem(BinaryOp),
    // float
    /// 浮点加法指令
    Fadd(BinaryOp),
    /// 浮点减法指令
    Fsub(BinaryOp),
    /// 浮点乘法指令
    Fmul(BinaryOp),
    /// 浮点除法指令
    Fdiv(BinaryOp),
    // 类型转换
    /// 零扩展指令
    ZeroExt(CastOp),
    /// 将32位整数转换为浮点数指令
    I32ToFloat(CastOp),
    /// 将浮点数转换为32位整数指令
    FloatToI32(CastOp),
    // 比较运算
    /// 整数比较指令
    Cmp(String, BinaryOp),
    /// 浮点数比较指令
    Fcmp(String, BinaryOp),
    // Phi
    /// Phi指令
    Phi(String, SymbolType, Vec<(String, String)>),
    // 变量
    /// 分配内存指令
    Alloca { res: String, ty: SymbolType, len: String },
    /// 存储指令
    Store { ty: SymbolType, value: String, ptr: String, len: String },
    /// 加载指令
    Load { res: String, ty: SymbolType, ptr: String, len: String },
    // 函数
    /// 函数调用指令
    Call(String, String, SymbolType, Vec<(String, SymbolType)>),
    // 数组
    /// 获取数组元素指针指令
    GetElemPtr(String, SymbolType, String, Vec<String>),
    /// 位转换指令
    BitCast(String, SymbolType, String, SymbolType),
    // 注释
    /// 注释指令
    Comment(String),
    // 终结指令
    /// 返回指令
    Ret(SymbolType, Option<String>),
    /// 分支指令
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
    I32ToFloat,
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


/// 二元操作
#[derive(Debug, Default)]
pub struct BinaryOp {
    /// 结果
    pub res: String,
    /// 操作类型
    pub op_type: SymbolType,
    /// 操作数1
    pub op1: String,
    /// 操作数2
    pub op2: String,
}

/// 转换操作
#[derive(Debug, Default)]
pub struct CastOp {
    /// 结果
    pub res: String,
    /// 类型1
    pub type_1: SymbolType,
    /// 类型2
    pub type_2: SymbolType,
    /// 值
    pub val: String,
}

impl LLVMProgram {
    /// 创建一个新的LLVM程序
    pub fn new() -> Self {
        Self::default()
    }
}

impl BinaryOp {
    /// 创建一个新的二元操作
    pub fn new() -> Self {
        Self::default()
    }
}

impl CastOp {
    /// 创建一个新的转换操作
    pub fn new() -> Self {
        Self::default()
    }
}