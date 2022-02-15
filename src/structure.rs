use std::{fmt, collections::HashMap};

use crate::{astnode::ASTNode, errors::EvaluationError};

pub type SymbolTable = HashMap<String, Structure>;

#[derive(Clone, Debug, PartialEq)]
pub enum StructureKind {
    Boolean,
    Byte,
    Cons,
    Dict,
    FloatingPoint,
    Function,
    Integer,
    Invalid,
    Nil,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub outerSymbols: SymbolTable,
    pub parameterList: Vec<ASTNode>,
    pub function: fn(outerSymbols: SymbolTable, parameterList: Vec<ASTNode>, params: Vec<Structure>) -> Result<Structure, EvaluationError>
}

impl Function {
    pub fn call(&self, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
        (self.function)(self.outerSymbols.clone(),
                        self.parameterList.clone(),
                        params)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Structure {
    Boolean(bool),
    Byte(u8),
    Cons(Box<(Structure,Structure)>),
    Dict(HashMap<String, Structure>),
    FloatingPoint(f64),
    Function(Function),
    Integer(i64),
    Invalid,
    Nil,
}

#[derive(Debug, PartialEq)]
pub struct TypeMismatchError {
    pub expected: StructureKind,
    pub found: StructureKind,
}
impl TypeMismatchError {}

impl Structure {
    pub fn unbox<T>(&self) -> Result<T, TypeMismatchError> {
        Err(TypeMismatchError {
            expected: StructureKind::Invalid,
            found: StructureKind::Invalid,
        })
    }

    pub fn new() -> Structure {
        Structure::Invalid
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: implement fmt for Structure")
    }
}

impl Default for Structure {
    fn default() -> Self {
        Structure::new()
    }
}