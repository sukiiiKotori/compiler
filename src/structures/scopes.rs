use crate::structures::symbol::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Symbol {
    pub label: String,
    pub ty: SymbolType,
    pub value: SymbolVal,
}

pub enum ScopeType {
    Basic,
    If,
    While(String, String),
    Func(SymbolType),
    Global,
}

pub struct Scope {
    pub ty: ScopeType,
    pub map_tab: HashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Self {
            ty: ScopeType::Global,
            map_tab: HashMap::new(),
        }
    }
}

pub struct Scopes {
    pub scope_vec: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Self {
            scope_vec: vec![Scope::new()],
        }
    }

    pub fn is_global_scope(&self) -> bool {
        self.scope_vec.len() == 1
    }

    pub fn push(
        &mut self,
        labels: &mut Labels,
        ident: &str,
        ty: &SymbolType,
        value: &SymbolVal,
        flag: Option<bool>,
    ) -> Option<String> {
        let flag = flag.unwrap_or(self.is_global_scope());
        let curr_scope = self.scope_vec.last_mut().unwrap();

        let label: String;
        if flag {
            label = labels.pop_global(ident);
        } else {
            label = labels.pop_local(ident);
        }
        match curr_scope.map_tab.insert(
            String::from(ident),
            Symbol {
                label: String::from(&label),
                ty: ty.clone(),
                value: value.clone(),
            },
        ) {
            Some(_) => None, // 已经存在
            None => Some(label),
        }
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

    pub fn get_current_function_type(&self) -> Option<SymbolType> {
        self.scope_vec
            .iter()
            .rev()
            .find_map(|scope| match &scope.ty {
                ScopeType::Func(ty) => Some(ty.clone()),
                _ => None,
            })
    }

    pub fn get_function(&self, id: &str) -> Option<&Symbol> {
        self.scope_vec
            .iter()
            .rev()
            .find_map(|scope| scope.map_tab.get(id).filter(|item| matches!(item.value, SymbolVal::Func(_, _))))
    }

    pub fn get(&mut self, id: &str) -> Option<&mut Symbol> {
        self.scope_vec
            .iter_mut()
            .rev()
            .find_map(|scope| scope.map_tab.get_mut(id))
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
    pub number_counter: i32,
    // pure number_counter
    pub local: HashMap<String, i32>,
    // local label number_counter
    pub global: HashMap<String, i32>,
    // global label number_counter
    pub basis_block: HashMap<String, i32>,  // 基本块标号计数，只允许特定标号
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
        Self {
            number_counter: 0,
            local: HashMap::new(),
            basis_block: BLOCK_LABELS.iter().map(|label| (String::from(*label), 0)).collect(),
            global: HashMap::new(),
        }
    }

    pub fn pop_num_str(&mut self) -> String {
        let res = format!("%{}", self.number_counter);
        self.number_counter += 1;
        res
    }

    pub fn recover_num(&mut self) {
        self.number_counter -= 1;
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
        *val += 1;
        if *val == 1 {
            format!("@{}", id)
        }
        else {
            format!("@{}_{}", id, *val - 1)
        }
    }

    pub fn pop_block(&mut self, ident: &str) -> String {
        let val = self.basis_block.get_mut(ident).expect("Undefined block label");
        let res = format!("{}_{}", ident, *val);
        *val += 1;
        res
    }

    pub fn clear(&mut self) {
        self.local.clear();
        self.number_counter = 0;
        for counter in self.basis_block.values_mut() {
            *counter = 0;
        }
    }
}
