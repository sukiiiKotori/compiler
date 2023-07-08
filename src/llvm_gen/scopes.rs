use crate::sym_type::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Symbol {
    pub label: String,
    pub sym_type: SymType,
    pub sym_val: SymVal,
}

pub enum ScopeType {
    BasicBlock,
    If,
    While(String, String),
    Func(SymType),
    Global,
}

pub struct Scope {
    pub ty: ScopeType,
    pub tab: HashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            ty: ScopeType::Global,
            tab: HashMap::new(),
        }
    }
}

pub struct Scopes {
    pub scope_vec: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Scopes {
            scope_vec: vec![Scope::new()],
        }
    }

    pub fn is_global(&self) -> bool {
        self.scope_vec.len() == 1
    }

    pub fn push(
        &mut self,
        labels: &mut Labels,
        ident: &str,
        sym_type: &SymType,
        sym_val: &SymVal,
        flag: Option<bool>,
    ) -> Option<String> {
        let flag = flag.unwrap_or(self.is_global());
        let curr_scope = self.scope_vec.last_mut().unwrap();

        let label: String;
        if flag {
            label = labels.pop_global(ident);
        } else {
            label = labels.pop_local(ident);
        }
        match curr_scope.tab.insert(
            String::from(ident),
            Symbol {
                label: String::from(&label),
                sym_type: sym_type.clone(),
                sym_val: sym_val.clone(),
            },
        ) {
            Some(_) => None, // 已经存在
            None => Some(label),
        }
    }

    pub fn enter_func(&mut self, func_type: &SymType) {
        // 进入函数作用域
        // 在作用域堆栈中推入一个新的函数作用域
        self.scope_vec.push(Scope {
            ty: ScopeType::Func(func_type.clone()),
            tab: HashMap::new(),
        });
    }

    pub fn enter_block(&mut self) {
        // 进入基本块作用域
        // 在作用域堆栈中推入一个新的基本块作用域
        self.scope_vec.push(Scope {
            ty: ScopeType::BasicBlock,
            tab: HashMap::new(),
        });
    }

    pub fn enter_if(&mut self) {
        // 进入If语句作用域
        // 在作用域堆栈中推入一个新的If语句作用域
        self.scope_vec.push(Scope {
            ty: ScopeType::If,
            tab: HashMap::new(),
        });
    }

    pub fn enter_while(&mut self, entry: &String, end: &String) {
        // 进入While循环作用域
        // 在作用域堆栈中推入一个新的While循环作用域，并指定循环入口和循环结束标签
        self.scope_vec.push(Scope {
            ty: ScopeType::While(String::from(entry), String::from(end)),
            tab: HashMap::new(),
        });
    }

    pub fn exit(&mut self) {
        // 退出当前作用域
        // 从作用域堆栈中弹出顶部的作用域
        self.scope_vec.pop();
    }


    pub fn is_inside_while(&self) -> bool {
        self.scope_vec
            .iter()
            .rev()
            .any(|scope| matches!(scope.ty, ScopeType::While(_, _)))
    }

    pub fn get_while_entry(&mut self) -> Option<String> {
        self.scope_vec.iter().rev().find_map(|scope| {
            if let ScopeType::While(entry, _) = &scope.ty {
                Some(entry.clone())
            } else {
                None
            }
        })
    }

    pub fn get_while_end(&mut self) -> Option<String> {
        self.scope_vec.iter().rev().find_map(|scope| {
            if let ScopeType::While(_, end) = &scope.ty {
                Some(end.clone())
            } else {
                None
            }
        })
    }

    pub fn get_curr_func_type(&self) -> Option<SymType> {
        self.scope_vec
            .iter()
            .rev()
            .find_map(|scope| match &scope.ty {
                ScopeType::Func(ty) => Some(ty.clone()),
                _ => None,
            })
    }

    pub fn get_func(&self, id: &str) -> Option<&Symbol> {
        self.scope_vec
            .iter()
            .rev()
            .find_map(|scope| scope.tab.get(id).filter(|item| matches!(item.sym_val, SymVal::Func(_, _))))
    }

    pub fn get(&mut self, id: &str) -> Option<&mut Symbol> {
        self.scope_vec
            .iter_mut()
            .rev()
            .find_map(|scope| scope.tab.get_mut(id))
    }

    pub fn get_depth(&self) -> usize {
        self.scope_vec
            .iter()
            .filter(|scope| {
                if let ScopeType::While(_, _) = scope.ty {
                    true
                } else {
                    false
                }
            })
            .count()
    }
}

pub struct Labels {
    pub counter: i32,
    // pure number counter
    pub local: HashMap<String, i32>,
    // local label counter
    pub global: HashMap<String, i32>,
    // global label counter
    pub block: HashMap<String, i32>,  // 基本块标号计数，只允许特定标号
}

const BLOCK_LABELS: [&str; 14] = [
    "_L",
    "ret_then",
    "or_false",
    "or_end",
    "and_true",
    "and_end",
    "break_then",
    "continue_then",
    "if_then",
    "if_else",
    "if_end",
    "while_entry",
    "while_body",
    "while_end",
];

impl Labels {
    pub fn new() -> Self {
        let mut labels = Labels {
            counter: 0,
            local: HashMap::new(),
            block: BLOCK_LABELS.iter().map(|label| (String::from(*label), 0)).collect(),
            global: HashMap::new(),
        };
        labels
    }

    pub fn pop_num_str(&mut self) -> String {
        let res = format!("%{}", self.counter);
        self.counter += 1;
        res
    }

    pub fn recover_num(&mut self) {
        self.counter -= 1;
    }

    pub fn pop_local(&mut self, ident: &str) -> String {
        let id = &ident[..std::cmp::min(ident.len(), 15)];
        let val = self.local.entry(String::from(id)).or_insert(0);
        let res = format!("%{}_{}", id, *val);
        *val += 1;
        res
    }

    pub fn pop_global(&mut self, ident: &str) -> String {
        let id = &ident[..std::cmp::min(ident.len(), 15)];
        let val = self.global.entry(String::from(id)).or_insert(0);
        let res = format!("@{}_{}", id, *val);
        *val += 1;
        res
    }

    pub fn pop_block(&mut self, ident: &str) -> String {
        let val = self.block.get_mut(ident).expect("Undefined block label");
        let res = format!("{}_{}", ident, *val);
        *val += 1;
        res
    }

    pub fn clear(&mut self) {
        self.local.clear();
        self.counter = 0;
        for counter in self.block.values_mut() {
            *counter = 0;
        }
    }
}
