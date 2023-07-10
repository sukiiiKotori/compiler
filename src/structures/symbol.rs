use crate::utils::float::parse_float;


// 符号宽
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SymbolWidth {
    Void,
    I1,
    I8,
    I32,
    I64,
    Float,
    Arr{tar: Box<SymbolType>, dims: Vec<i32>}, 
}

// 符号类型
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SymbolType {
    pub width: SymbolWidth,
    pub is_const : bool,
}

impl Default for SymbolType {
    fn default() -> Self {
        Self {
            width:SymbolWidth::Void,
            is_const:true,
        }
    }
}

// 符号值
#[derive(Debug, Clone)]
pub enum SymbolVal {
    Void,
    I32(String),
    Float(String),
    Func(SymbolType, Vec<SymbolType>),
}

// 符号类型方法
impl SymbolType {
    pub fn new(width: SymbolWidth, is_const: bool) -> Self{
        SymbolType {
            width,
            is_const,
        }
    }
    // 获取类型名
    pub fn get_type(&self) -> String {
        self.width.get_type()
    }
    // 获取类型宽度
    pub fn get_width(&self) -> usize {
        self.width.get_width()
    }
}

impl SymbolWidth {
    // 获取类型名实现
    pub fn get_type(&self) -> String {
        match self {
            SymbolWidth::Void => "void".to_string(),
            SymbolWidth::I1 => "i1".to_string(),
            SymbolWidth::I8 => "i8".to_string(),
            SymbolWidth::I32 => "i32".to_string(),
            SymbolWidth::I64 => "i64".to_string(),
            SymbolWidth::Float => "float".to_string(),
            SymbolWidth::Arr{tar, dims} => {
                let name = tar.get_type();
                let mut ret = String::new();
                for dim in dims.iter() {
                    if dim >= &0 {
                        ret += format!("[{} x ", dim).as_str();
                    }
                }
                ret += name.as_str();
                for dim in dims.iter().rev() {
                    if dim >= &0 {
                        ret += "]";
                    } else {
                        ret += "*";
                    }
                }
                ret
            },
        }
    }
    // 获取类型宽度实现
    pub fn get_width(&self) -> usize {
        match self {
            SymbolWidth::I1 | SymbolWidth::I8 | SymbolWidth::I32 | SymbolWidth::I64 | SymbolWidth::Float => 4,
            SymbolWidth::Arr {tar:_, dims:_} => 8,
            _ => panic!("Don't support!"),
        }
    }
}

// 生成类型值
pub fn make_symbol_val(s_type: &SymbolType, val: &String) -> SymbolVal {
    match s_type.width {
        SymbolWidth::I32 => SymbolVal::I32(val.to_string()),
        SymbolWidth::Float => SymbolVal::Float(val.to_string()),
        _ => panic!("Don't support!"),
    }
}

// 提取类型值
pub fn get_symbol_val(s_val: &SymbolVal) -> String {
    match s_val {
        SymbolVal::I32(val) => val.to_string(),
        SymbolVal::Float(val) => val.to_string(),
        _ => panic!("Don't support"),
    }
}

/// 根据符号的类型，检查值val是否为零
pub fn num_is_zero(ty: &SymbolType, val: &String) -> bool {
    match ty.width {
        SymbolWidth::I1 => {
            let num: i32 = val.parse().unwrap();
            num == 0
        },
        SymbolWidth::I32 => {
            let num: i32 = val.parse().unwrap();
            num == 0
        },
        SymbolWidth::Float => {
            let num: f32 = parse_float(val);
            num == 0.0
        },
        _ => panic!("{:?} TODO", ty),
    }
}