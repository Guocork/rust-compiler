use std::collections::HashMap;

use crate::ast::Statement;

pub struct CodeGen {
    instructions: Vec<Instruction>,
    constants: Vec<Object>,
    symbol_table: SymbolTable,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConstant(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    LessThan, 
    GreaterThan,
    And,
    Or,
    Not,
    Jump(usize),
    JumpNotTruthy(usize),
    SetGlobal(usize),
    GetGlobal(usize),
    SetLocal(usize),
    GetLocal(usize),
    Call(usize),
    Return,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            instructions: Vec::new(),
            constants: Vec::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    // pub fn compile_statement(&mut self, stmt: Statement) -> Result<(), String> {
        
    // }
}



#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Function(Function),
    Null,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub num_locals: usize,
    pub num_parameters: usize
}


pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    definitions: Vec<Symbol>,
    scope_index: usize,
    outer: Option<Box<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolScope {
    Global,
    Local,
    Function
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            definitions: Vec::new(),
            scope_index: 0,
            outer: None
        }
    }

    pub fn new_enclosed(outer: SymbolTable) -> Self {
        let mut enclosed = Self::new();
        enclosed.outer = Some(Box::new(outer));
        enclosed
    }

    pub fn define(&mut self, name: String) -> Symbol {
        let scope = if self.outer.is_none() {
            SymbolScope::Global
        } else {
            SymbolScope::Local
        };

        let symbol = Symbol {
            name: name.clone(),
            scope,
            index: self.definitions.len(),
        };

        self.store.insert(name, symbol.clone());
        self.definitions.push(symbol.clone());

        symbol
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        match self.store.get(name) {
            Some(symbol) => Some(symbol.clone()),
            None => {
                match &self.outer {
                    Some(outer) => outer.resolve(name),
                    None => None,
                }
            }
        }
    }
}