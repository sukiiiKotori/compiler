use std::collections::HashSet;
use crate::structures::symbol::*;

/*
     +----------------------+
     |    LLVMProgram       |
     +----------------------+
     |  global_var: Vector  |
     |  func_decl: Vector   |
     |  func_def: Vector    |
     +----------------------+
            ^
            |
            |
+-----------------------+
|       GlobalVar        |
+-----------------------+
|  var_name: String     |
|  var_type: SymbolType |
|  init_num: Vector     |
+-----------------------+
            ^
            |
            |
+-----------------------+
|       InitNumber       |
+-----------------------+
|  init_type: SymbolType |
|  init_val: String      |
+-----------------------+
            ^
            |
            |
+-----------------------+
|       FuncDecl         |
+-----------------------+
|  func_name: String    |
|  func_type: SymbolType |
|  param_types: Vector  |
+-----------------------+
            ^
            |
            |
+-----------------------+
|       FuncDef          |
+-----------------------+
|  func_name: String    |
|  func_type: SymbolType |
|  params: Vector       |
|  blocks: Vector       |
|  local_vars: Vector   |
+-----------------------+
            ^
            |
            |
+-----------------------+
|        Param           |
+-----------------------+
|  param_name: String   |
|  param_type: SymbolType |
+-----------------------+
            ^
            |
            |
+-----------------------+
|        Block           |
+-----------------------+
|  block_label: String  |
|  phi_ins: Vector      |
|  nor_ins: Vector      |
|  ter_ins: Option      |
|  ins_num: Integer     |
|  depth: Integer       |
+-----------------------+
            ^
            |
            |
+-----------------------+
|      LocalVar          |
+-----------------------+
|  ins: Instruction    |
|  label: String       |
+-----------------------+
            ^
            |
            |
+-----------------------+
|     Instruction        |
+-----------------------+
|  Add: BinaryOp       |
|  Sub: BinaryOp       |
|  Mul: BinaryOp       |
|  Sdiv: BinaryOp      |
|  Srem: BinaryOp      |
|  Fadd: BinaryOp      |
|  Fsub: BinaryOp      |
|  Fmul: BinaryOp      |
|  Fdiv: BinaryOp      |
|  ZeroExt: CastOp     |
|  I32ToFloat: CastOp  |
|  FloatToI32: CastOp  |
|  Cmp: BinaryOp       |
|  Fcmp: BinaryOp      |
|  Phi: Tuple          |
|  Alloca: Tuple       |
|  Store: Tuple        |
|  Load: Tuple         |
|  Call: Tuple         |
|  GetElemPtr: Tuple   |
|  BitCast: Tuple      |
|  Comment: Tuple      |
|  Ret: Tuple          |
|  Br: Tuple           |
+-----------------------+
            ^
            |
            |
+-----------------------+
|       BinaryOp         |
+-----------------------+
|  res: String         |
|  op_type: SymbolType |
|  op1: String         |
|  op2: String         |
+-----------------------+
            ^
            |
            |
+-----------------------+
|         CastOp          |
+-----------------------+
|  res: String         |
|  type_1: SymbolType  |
|  type_2: SymbolType  |
|  val: String         |
+-----------------------+

 */

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
    pub init_values: Vec<String>,
}

/// 函数声明
#[derive(Debug)]
pub struct FuncDecl {
    /// 函数名称
    pub func_name: String,
    /// 函数返回值类型
    pub func_type: SymbolType,
    /// 参数类型
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

/// for eliminate
impl Instruction {
    #[allow(unused)]
    pub fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        match self {
            Instruction::Add(bin_op) => bin_op.is_active(active_labels),
            Instruction::Sub(bin_op) => bin_op.is_active(active_labels),
            Instruction::Mul(bin_op) => bin_op.is_active(active_labels),
            Instruction::Sdiv(bin_op) => bin_op.is_active(active_labels),
            Instruction::Srem(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fadd(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fsub(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fmul(bin_op) => bin_op.is_active(active_labels),
            Instruction::Fdiv(bin_op) => bin_op.is_active(active_labels),
            Instruction::Cmp(_, bin_op) => bin_op.is_active(active_labels),
            Instruction::Fcmp(_, bin_op) => bin_op.is_active(active_labels),
            Instruction::ZeroExt(conver_op) => conver_op.is_active(active_labels),
            Instruction::I32ToFloat(conver_op) => conver_op.is_active(active_labels),
            Instruction::FloatToI32(conver_op) => conver_op.is_active(active_labels),
            Instruction::Phi(res, _, _) => active_labels.contains(res.as_str()),
            Instruction::Alloca { res, ty: _, len: _ } => active_labels.contains(res.as_str()),
            Instruction::Store { ty: _, value: _, ptr, len: _ } => active_labels.contains(ptr.as_str()),
            Instruction::Load { res, ty: _, ptr: _, len: _ } => active_labels.contains(res.as_str()),
            Instruction::Call(_, _, _, _) => true, // Call指令总是执行
            Instruction::GetElemPtr(_, _, _, _) => true, // GetElemPtr总是执行
            Instruction::BitCast(res, _, _, _) => active_labels.contains(res.as_str()),
            // 由于删除死代码后，代码结构发生变化，注释不再打印
            Instruction::Comment(_) => false,
            // 终结指令总是执行
            Instruction::Ret(_, _) => true,
            Instruction::Br(_, _, _) => true,
        } // match
    } // fn
} // impl

impl BinaryOp {
    #[allow(unused)]
    fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        active_labels.contains(self.res.as_str())
    }
}

impl CastOp {
    #[allow(unused)]
    fn is_active(&self, active_labels: &HashSet<String>) -> bool {
        active_labels.contains(self.res.as_str())
    }
}

///fetch

impl Instruction {
    pub fn fetch_info(&self) -> (InstructionType, Vec<&str>, Vec<&SymbolType>) {
        match &self {
            Instruction::Add(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Add, str_ty.0, str_ty.1)
            }
            Instruction::Sub(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Sub, str_ty.0, str_ty.1)
            }
            Instruction::Mul(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Mul, str_ty.0, str_ty.1)
            }
            Instruction::Sdiv(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Sdiv, str_ty.0, str_ty.1)
            }
            Instruction::Srem(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Srem, str_ty.0, str_ty.1)
            }
            Instruction::Fadd(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Fadd, str_ty.0, str_ty.1)
            }
            Instruction::Fsub(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Fsub, str_ty.0, str_ty.1)
            }
            Instruction::Fmul(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Fmul, str_ty.0, str_ty.1)
            }
            Instruction::Fdiv(bin_op) => {
                let str_ty = bin_op.fetch_info();
                (InstructionType::Fdiv, str_ty.0, str_ty.1)
            }
            Instruction::ZeroExt(conver_op) => {
                let str_ty = conver_op.fetch_info();
                (InstructionType::ZeroExt, str_ty.0, str_ty.1)
            }
            Instruction::I32ToFloat(conver_op) => {
                let str_ty = conver_op.fetch_info();
                (InstructionType::I32ToFloat, str_ty.0, str_ty.1)
            }
            Instruction::FloatToI32(conver_op) => {
                let str_ty = conver_op.fetch_info();
                (InstructionType::FloatToI32, str_ty.0, str_ty.1)
            }
            Instruction::Cmp(cond, bin_op) => {
                let (mut str_vec, ty_vec) = bin_op.fetch_info();
                str_vec.insert(0, cond.as_str());
                (InstructionType::Cmp, str_vec, ty_vec)
            }
            Instruction::Fcmp(cond, bin_op) => {
                let (mut str_vec, ty_vec) = bin_op.fetch_info();
                str_vec.insert(0, cond.as_str());
                (InstructionType::Fcmp, str_vec, ty_vec)
            }
            Instruction::Phi(res, ty, candidates) => {
                let mut str_vec = vec![res.as_str()];
                for (v, b) in candidates.iter() {
                    str_vec.push(v.as_str());
                    str_vec.push(b.as_str());
                }
                (InstructionType::Phi, str_vec, vec![&ty])
            }
            Instruction::Alloca { res, ty, len } => (
                InstructionType::Alloca,
                vec![res.as_str(), len.as_str()],
                vec![&ty],
            ),
            Instruction::Store {
                ty,
                value,
                ptr,
                len,
            } => (
                InstructionType::Store,
                vec![value.as_str(), ptr.as_str(), len.as_str()],
                vec![&ty],
            ),
            Instruction::Load {
                res,
                ty,
                ptr,
                len,
            } => (
                InstructionType::Load,
                vec![res.as_str(), ptr.as_str(), len.as_str()],
                vec![&ty],
            ),
            Instruction::Call(res, label, ty, params) => {
                let mut str_vec = vec![res.as_str(), label.as_str()];
                for (v, _) in params.iter() {
                    str_vec.push(v.as_str());
                }
                let mut ty_vec = vec![ty];
                for (_, t) in params.iter() {
                    ty_vec.push(t);
                }
                (InstructionType::Call, str_vec, ty_vec)
            }
            Instruction::GetElemPtr(dst, ty, ptr, idx) => {
                let mut str_vec = vec![dst.as_str(), ptr.as_str()];
                for i in idx.iter() {
                    str_vec.push(i.as_str());
                }
                (InstructionType::GetElemPtr, str_vec, vec![&ty])
            }
            Instruction::BitCast(res, ty, val, ty2) => (
                InstructionType::BitCast,
                vec![res.as_str(), val.as_str()],
                vec![&ty, &ty2],
            ),
            Instruction::Comment(content) => (InstructionType::Comment, vec![content.as_str()], vec![]),
            Instruction::Ret(ty, val) => {
                if let Some(v) = val {
                    (InstructionType::Ret, vec![v.as_str()], vec![&ty])
                } else {
                    (InstructionType::Ret, vec![], vec![&ty])
                }
            }
            Instruction::Br(cond, label1, label2) => {
                if let (Some(c), Some(l2)) = (cond, label2) {
                    (
                        InstructionType::Br,
                        vec![c.as_str(), label1.as_str(), l2.as_str()],
                        vec![],
                    )
                } else {
                    (InstructionType::Br, vec!["", label1.as_str(), ""], vec![])
                }
            }
        }
    }
}

impl BinaryOp {
    fn fetch_info(&self) -> (Vec<&str>, Vec<&SymbolType>) {
        (
            vec![self.res.as_str(), self.op1.as_str(), self.op2.as_str()],
            vec![&self.op_type],
        )
    }
}

impl CastOp {
    fn fetch_info(&self) -> (Vec<&str>, Vec<&SymbolType>) {
        (
            vec![self.res.as_str(), self.val.as_str()],
            vec![&self.type_1, &self.type_2],
        )
    }
}
