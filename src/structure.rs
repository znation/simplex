use std::{collections::HashMap, fmt};

use crate::{astnode::ASTNode, errors::EvaluationError};

pub type SymbolTable = HashMap<String, Structure>;
pub type Backtrace = Vec<(String, i64, i64)>;
pub type EvaluationResult = Result<Structure, EvaluationError>;

pub trait Empty {
    fn empty() -> Self;
}
impl Empty for SymbolTable {
    fn empty() -> Self {
        HashMap::new()
    }
}
impl Empty for Backtrace {
    fn empty() -> Self {
        Vec::new()
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
            outer_symbols: SymbolTable,
            outer_backtrace: Backtrace,
            parameterList: Vec<ASTNode>,
            params: Vec<Structure>,
        ) -> EvaluationResult,
    ),
    Native(
        fn(node: ASTNode, outer_backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub parameter_list: Vec<ASTNode>,
    pub function: FunctionBody,
}

impl Function {
    pub fn synthetic(
        function: fn(
            node: ASTNode,
            backtrace: Backtrace,
            params: Vec<Structure>,
        ) -> EvaluationResult,
    ) -> Structure {
        Structure::Function(Function {
            parameter_list: Vec::new(),
            function: FunctionBody::Native(function),
        })
    }

    pub fn call(
        &self,
        node: &ASTNode,
        outer_symbols: &SymbolTable,
        outer_backtrace: &Backtrace,
        params: Vec<Structure>,
    ) -> EvaluationResult {
        match self.function {
            FunctionBody::Lambda(lambda) => lambda(
                node.clone(),
                outer_symbols.clone(),
                outer_backtrace.clone(),
                self.parameter_list.clone(),
                params,
            ),
            FunctionBody::Native(native) => native(node.clone(), outer_backtrace.clone(), params),
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

    pub fn from_string(s: &str) -> Structure {
        // create cons from string
        let len = s.chars().count();
        if len == 0 {
            Structure::Cons(Box::new((Structure::Nil, Structure::Nil)))
        } else {
            let mut chars = s.chars();
            let car = Structure::Char(chars.next().unwrap());
            let remaining_input = chars.as_str();
            let cdr = if remaining_input.is_empty() {
                Structure::Nil
            } else {
                Structure::from_string(remaining_input)
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

    // allow implicit conversion to bool from any type
    pub fn boolean(&self) -> bool {
        match self {
            Structure::Boolean(b) => *b,
            Structure::Byte(b) => *b != 0,
            Structure::Char(c) => *c != '\0',
            Structure::Cons(c) => c.0.kind() != StructureKind::Nil,
            Structure::Dict(d) => !d.is_empty(),
            Structure::FloatingPoint(f) => *f != 0.0,
            Structure::Function(_) => true,
            Structure::Integer(i) => *i != 0,
            Structure::Invalid => panic!(),
            Structure::Nil => false,
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
    pub fn string(
        &self,
        backtrace: Backtrace,
        node: Option<&ASTNode>,
    ) -> Result<String, EvaluationError> {
        let invalid_node = ASTNode::invalid();
        let node = node.unwrap_or(&invalid_node);
        let cons = match self {
            Structure::Cons(b) => &*b,
            _ => {
                return Err(EvaluationError::type_mismatch(
                    node,
                    backtrace,
                    StructureKind::Cons,
                    self.kind(),
                ))
            }
        };
        let car = &cons.0;
        let cdr = &cons.1;
        let ret = if car.kind() == StructureKind::Nil {
            assert_eq!(cdr.kind(), StructureKind::Nil);
            "".to_string()
        } else if car.kind() == StructureKind::Char {
            assert_eq!(car.kind(), StructureKind::Char);
            if cdr.kind() == StructureKind::Nil {
                car.char().to_string()
            } else {
                let s = cdr.string(backtrace, Some(node))?;
                car.char().to_string() + &s
            }
        } else {
            return Err(EvaluationError::type_mismatch(
                node,
                backtrace,
                StructureKind::Char,
                car.kind(),
            ));
        };
        Ok(ret)
    }

    pub fn dict(&self) -> &HashMap<String, Structure> {
        match self {
            Structure::Dict(d) => d,
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
                match self.string(Backtrace::empty(), None) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => write!(f, "(cons {} {})", c.0, c.1),
                }
            }
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
            let s = Structure::from_string(&string);
            assert_eq!(s.kind(), StructureKind::Cons);
            match s.string(Backtrace::empty(), None) {
                Ok(found) => assert_eq!(found, string),
                Err(_) => assert!(false),
            }
        }
    }
}
