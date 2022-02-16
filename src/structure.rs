use std::{fmt, collections::HashMap};

use crate::{astnode::ASTNode, errors::EvaluationError};

pub type SymbolTable = HashMap<String, Structure>;

#[derive(Clone, Debug, PartialEq)]
pub enum StructureKind {
    Boolean,
    Byte,
    Char,
    Cons,
    Dict,
    FloatingPoint,
    Function,
    Integer,
    Invalid,
    Nil,
}


#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBody {
    Lambda(fn(node: ASTNode, outerSymbols: SymbolTable, parameterList: Vec<ASTNode>, params: Vec<Structure>) -> Result<Structure, EvaluationError>),
    Native(fn(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub outerSymbols: SymbolTable,
    pub parameterList: Vec<ASTNode>,
    pub function: FunctionBody,
}

impl Function {
    pub fn synthetic(function: fn(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError>) -> Structure {
        Structure::Function(Function { outerSymbols: HashMap::new(), parameterList: Vec::new(), function: FunctionBody::Native(function) })
    }

    pub fn call(&self, node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
        match self.function {
            FunctionBody::Lambda(lambda) => lambda(node,
                        self.outerSymbols.clone(),
                        self.parameterList.clone(),
                        params),
            FunctionBody::Native(native) => native(node, params),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Structure {
    Boolean(bool),
    Byte(u8),
    Char(char),
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
    pub fn new() -> Structure {
        Structure::Invalid
    }

    pub fn from_string(s: String) -> Structure {
        // create cons from string      
        let len = s.len();
        if (len == 0) {
            Structure::Cons(Box::new((Structure::Nil, Structure::Nil)))
        } else {
            let mut chars = s.chars();
            let car = Structure::Char(chars.nth(0).unwrap());
            let cdr = if (len == 1) {
                Structure::Nil
            } else {
                Structure::from_string(s[1..s.len()].to_string())
            };
            Structure::Cons(Box::new((car, cdr)))
        }
    }

    pub fn kind(&self) -> StructureKind {
        match self {
            Structure::Boolean(_) => StructureKind::Boolean,
            Structure::Byte(_) => StructureKind::Byte,
            Structure::Char(_) => StructureKind::Char,
            Structure::Cons(_) => StructureKind::Cons,
            Structure::Dict(_) => StructureKind::Dict,
            Structure::FloatingPoint(_) => StructureKind::FloatingPoint,
            Structure::Function(_) => StructureKind::Function,
            Structure::Integer(_) => StructureKind::Integer,
            Structure::Invalid => StructureKind::Invalid,
            Structure::Nil => StructureKind::Nil,
        }
    }

    pub fn integer(&self) -> i64 {
        match self {
            Structure::Integer(i) => *i,
            _ => panic!()
        }
    }

    pub fn floating_point(&self) -> f64 {
        match self {
            Structure::FloatingPoint(f) => *f,
            Structure::Integer(i) => *i as f64,
            _ => panic!()
        }
    }

    pub fn boolean(&self) -> bool {
        match self {
            Structure::Boolean(b) => *b,
            _ => panic!()
        }
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