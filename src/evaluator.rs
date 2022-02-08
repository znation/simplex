use std::io::{Stdin, Stdout};

use crate::symboltable::SymbolTable;

pub struct Evaluator {
    symbols: SymbolTable
}

impl Evaluator {
   pub fn new() -> Evaluator {
       return Evaluator { symbols: SymbolTable::new() }
   } 

   pub fn eval(&self, str: String) -> Result<(), EvaluationError> {
       return Ok(())
   }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct EvaluationError {}

impl EvaluationError {
    
}
