use std::fmt;

#[derive(Debug, PartialEq)]
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

impl StructureKind {
    
}

#[derive(Debug, PartialEq)]
pub struct Structure {
    pub kind: StructureKind 
}

impl Structure {

}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "TODO: implement fmt for Structure")
    }
}