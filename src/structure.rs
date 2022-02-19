use std::{collections::HashMap, fmt, rc::Rc, cell::RefCell};

use crate::{astnode::ASTNode, errors::EvaluationError};

pub type SymbolTable = Rc<RefCell<HashMap<String, Structure>>>;
pub type EvaluationResult = Result<Structure, EvaluationError>;

pub trait Empty {
    fn empty() -> Self;
}
impl Empty for SymbolTable {
    fn empty() -> Self {
        Rc::new(RefCell::new(HashMap::new()))
    }
}

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
    Lambda(
        fn(
            node: ASTNode,
            outerSymbols: SymbolTable,
            parameterList: Vec<ASTNode>,
            params: Vec<Structure>,
        ) -> EvaluationResult,
    ),
    Native(fn(node: ASTNode, params: Vec<Structure>) -> EvaluationResult),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub outer_symbols: SymbolTable,
    pub parameter_list: Vec<ASTNode>,
    pub function: FunctionBody,
}

impl Function {
    pub fn synthetic(
        function: fn(node: ASTNode, params: Vec<Structure>) -> EvaluationResult,
    ) -> Structure {
        Structure::Function(Function {
            outer_symbols: SymbolTable::empty(),
            parameter_list: Vec::new(),
            function: FunctionBody::Native(function),
        })
    }

    pub fn call(&self, node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
        match self.function {
            FunctionBody::Lambda(lambda) => lambda(
                node,
                self.outer_symbols.clone(),
                self.parameter_list.clone(),
                params,
            ),
            FunctionBody::Native(native) => native(node, params),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Structure {
    Boolean(bool),
    Byte(u8),
    Char(char),
    Cons(Box<(Structure, Structure)>),
    Dict(HashMap<String, Structure>),
    FloatingPoint(f64),
    Function(Function),
    Integer(i64),
    Invalid,
    Nil,
}

impl Structure {
    pub fn new() -> Structure {
        Structure::Invalid
    }

    pub fn from_string(s: String) -> Structure {
        // create cons from string
        let len = s.len();
        if len == 0 {
            Structure::Cons(Box::new((Structure::Nil, Structure::Nil)))
        } else {
            let mut chars = s.chars();
            let car = Structure::Char(chars.next().unwrap());
            let cdr = if len == 1 {
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
            _ => panic!(),
        }
    }

    pub fn floating_point(&self) -> f64 {
        match self {
            Structure::FloatingPoint(f) => *f,
            Structure::Integer(i) => *i as f64,
            _ => panic!(),
        }
    }

    pub fn boolean(&self) -> bool {
        match self {
            Structure::Boolean(b) => *b,
            _ => panic!(),
        }
    }

    pub fn byte(&self) -> u8 {
        match self {
            Structure::Byte(b) => *b,
            _ => panic!(),
        }
    }

    pub fn char(&self) -> char {
        match self {
            Structure::Char(c) => *c,
            _ => panic!(),
        }
    }

    /// Turns a Simplex list of Chars into a Rust String
    pub fn string(&self) -> Result<String, EvaluationError> {
        let cons = match self {
            Structure::Cons(b) => &*b,
            _ => panic!(),
        };
        let car = &cons.0;
        let cdr = &cons.1;
        let ret = if car.kind() == StructureKind::Nil {
            assert_eq!(cdr.kind(), StructureKind::Nil);
            "".to_string()
        } else {
            assert_eq!(car.kind(), StructureKind::Char);
            if cdr.kind() == StructureKind::Nil {
                car.char().to_string()
            } else {
                match cdr.string() {
                    Ok(s) => car.char().to_string() + &s,
                    Err(e) => return Err(e)
                }
            }
        };
        Ok(ret)
    }

    pub fn dict(&self) -> HashMap<String, Structure> {
        match self {
            Structure::Dict(d) => d.clone(),
            _ => panic!(),
        }
    }
}

fn fmt_dict(d: &HashMap<String, Structure>) -> String {
    let mut ret = String::new();
    ret += "(dict ";
    for (k, v) in d {
        ret += "\n";
        ret += "    '";
        ret += k;
        ret += "' ";
        ret += &v.to_string();
    }
    ret += ")";
    ret
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Structure::Boolean(b) => write!(f, "{}", b),
            Structure::Byte(b) => write!(f, "{}", b),
            Structure::Char(c) => write!(f, "{}", c),
            Structure::Cons(c) => {
                // see if we can interpret it as a string;
                // otherwise, write it as raw cons cells
                match self.string() {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => write!(f, "(cons {} {})", c.0, c.1)
                }
            },
            Structure::Dict(d) => write!(f, "{}", fmt_dict(d)),
            Structure::FloatingPoint(v) => write!(f, "{}", v),
            Structure::Function(function) => match function.function {
                FunctionBody::Lambda(l) => write!(f, "<lambda: {:?}>", l),
                FunctionBody::Native(n) => write!(f, "<native: {:?}>", n),
            },
            Structure::Integer(i) => write!(f, "{}", i),
            Structure::Invalid => panic!(),
            Structure::Nil => write!(f, "()"),
        }
    }
}

impl Default for Structure {
    fn default() -> Self {
        Structure::new()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn boolean_structure() {
        let bools = vec![false, true];
        for b in bools {
            let s = Structure::Boolean(b);
            assert_eq!(s.kind(), StructureKind::Boolean);
            assert_eq!(s.boolean(), b);
        }
        assert_eq!(Structure::Boolean(true).to_string(), "true");
        assert_eq!(Structure::Boolean(false).to_string(), "false");
    }

    #[test]
    fn byte_structure() {
        let bytes: Vec<u8> = vec![0, 1, 2, 254, 255];
        for b in bytes {
            let s = Structure::Byte(b);
            assert_eq!(s.kind(), StructureKind::Byte);
            assert_eq!(s.byte(), b);
            assert_eq!(b.to_string(), s.to_string());
        }
    }

    #[test]
    fn cons_structure() {
        let strings = vec![
            "".to_string(),
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
        ];
        for string in strings {
            let s = Structure::from_string(string.clone());
            assert_eq!(s.kind(), StructureKind::Cons);
            match s.string() {
                Ok(found) => assert_eq!(found, string),
                Err(_) => assert!(false)
            }
        }
    }
}
