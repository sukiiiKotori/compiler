use crate::structures::symbol::*;

// SysY开始
#[derive(Debug)]
pub struct SysY {
    pub units: Vec<CompUnit>,
}

// 模块单元
#[derive(Debug)]
pub enum CompUnit {
    FuncDef(FuncDef),
    Decl(Decl),
}

// 函数定义
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: Type,
    pub id: String,
    pub params: Option<Vec<FuncFParam>>,
    pub block: Block,
}

// 下标
#[derive(Debug)]
pub enum Index {
    Exp(Exp),
    Ptr(i32),
}

// 函数参数
#[derive(Debug)]
pub struct FuncFParam {
    pub ty: Type,
    pub id: String,
    pub idx: Vec<Index>,
}

// 声明
#[derive(Debug)]
pub enum Decl {
    ConstDecl(ConstDecl),
    VarDecl(VarDecl),
}

// 常量声明
#[derive(Debug)]
pub struct ConstDecl {
    pub ty: Type,
    pub defs: Vec<ConstDef>,
}

// 常量定义
#[derive(Debug)]
pub struct ConstDef {
    pub id: String,
    pub dims: Vec<Exp>,
    pub init: InitVal,
}

// 变量声明
#[derive(Debug)]
pub struct VarDecl {
    pub ty: Type,
    pub defs: Vec<VarDef>,
}

// 变量定义
#[derive(Debug)]
pub struct VarDef {
    pub id: String,
    pub dims: Vec<Exp>,
    pub init: Option<InitVal>,
}

// 初始化
#[derive(Debug)]
pub enum InitVal {
    Exp(Exp),
    Arr(Box<Vec<InitVal>>),
}

// 类型
#[derive(Debug)]
pub struct Type {
    pub ty: SymbolType,
}

// 基本块
#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

// 基本块条目
#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

// 基本语句
#[derive(Debug)]
pub enum Stmt {
    Assign(Assign),
    Exp(Option<Exp>),
    Block(Block),
    Return(Return),
    Break,
    Continue,
    If{exp: Exp, stmt1: Box<Stmt>, stmt2: Option<Box<Stmt>>},
    While{exp: Exp, stmt: Box<Stmt>},
}

// 赋值语句
#[derive(Debug)]
pub struct Assign {
    pub val: LVal, 
    pub exp: Exp,
}

// 返回语句
#[derive(Debug)]
pub struct Return {
    pub val: Option<Exp>,
}

// 表达式
#[derive(Debug)]
pub struct Exp {
    pub exp: Box<LOrExp>,
}

// 字面量数值
#[derive(Debug)]
pub enum Number {
    Int(i32),
    Float(String),
}

// 左值
#[derive(Debug)]
pub struct LVal {
    pub id: String,
    pub idx: Vec<Exp>,
}

// 主表达式
#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Exp),
    Number(Number),
    LVal(LVal),
}

// 一元表达式
#[derive(Debug)]
pub enum UnaryExp {
    PrimExp(PrimaryExp),
    Pos(Box<UnaryExp>),
    Neg(Box<UnaryExp>),
    Not(Box<UnaryExp>),
    Call{id: String, params: Option<Vec<Exp>>},
}

// 乘除表达式
#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    Mul(MulExpBody),
    Div(MulExpBody),
    Mod(MulExpBody),
}

#[derive(Debug)]
pub struct MulExpBody {
    pub exp1: Box<MulExp>,
    pub exp2: UnaryExp,
}

// 加法表达式
#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    Add(AddExpBody),
    Sub(AddExpBody),
}

#[derive(Debug)]
pub struct AddExpBody {
    pub exp1: Box<AddExp>,
    pub exp2: MulExp,
}

// 真值表达式
#[derive(Debug)]
pub enum RelExp {
    AddExp(AddExp),
    Lt(RelExpBody),
    Gt(RelExpBody),
    Le(RelExpBody),
    Ge(RelExpBody),
}

#[derive(Debug)]
pub struct RelExpBody {
    pub exp1: Box<RelExp>,
    pub exp2: AddExp,
}

// 相等判断表达式
#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    EQ(EqExpBody),
    NE(EqExpBody),
}

#[derive(Debug)]
pub struct EqExpBody {
    pub exp1: Box<EqExp>,
    pub exp2: RelExp,
}

// And表达式
#[derive(Debug)]
pub enum LAndExp {
    EqExp(EqExp),
    And(Box<LAndExp>, EqExp),
}

// Or表达式
#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    Or(Box<LOrExp>, LAndExp),
}

