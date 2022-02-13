use std::collections::HashMap;

use crate::astnode::ASTNode;
use crate::errors::EvaluationError;

use crate::parser::Parser;
use crate::stdlib::Stdlib;
use crate::structure::Structure;
use crate::structure::StructureKind;

type SymbolTable = HashMap<String, Structure>;

pub struct Evaluator {
    symbols: SymbolTable,
}

impl Evaluator {
    pub fn add_symbols(&mut self, new_symbols: SymbolTable) {
        for (k, v) in new_symbols {
            self.symbols.insert(k, v);
        }
    }

    pub fn new() -> Evaluator {
        let mut ret = Evaluator {
            symbols: SymbolTable::new(),
        };

        // Rust-native parts of the standard library
        ret.add_symbols(Stdlib::symbols());

        // Simplex stdlib (written in simplex)
        let simplex_lib = include_str!("stdlib.simplex");
        let result = ret.eval(simplex_lib.to_string());
        assert!(result.is_ok());

        ret
    }

    pub fn eval_node(&self, _node: ASTNode) -> Result<Structure, EvaluationError> {
        Ok(Structure {
            kind: StructureKind::Nil,
        })
    }

    pub fn eval(&self, str: String) -> Result<Structure, EvaluationError> {
        let node = match Parser::parse(str) {
            Ok(n) => n,
            Err(e) => return Err(EvaluationError::from_parse_error(e)),
        };
        self.eval_node(node)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! CHECK_MATH_1 {
        ($e: ident, $op:tt, $p: literal, $expected: literal) => {
            // run the given operator and compare the result in Rust
            let result = $e.eval(stringify!(($op $p)).to_string());
            assert!(result.is_ok());
            let type_conversion_result = result.unwrap().unbox();
            assert!(type_conversion_result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = type_conversion_result.unwrap();
            assert_eq!(unwrapped, $expected as f64);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p) $expected)).to_string());
            assert!(result2.is_ok());
            let type_conversion_result2 = result2.unwrap().unbox();
            assert!(type_conversion_result2.is_ok());
            let unwrapped2: bool = type_conversion_result2.unwrap();
            assert!(unwrapped2);
        };
    }

    macro_rules! CHECK_MATH_2 {
        ($e: ident, $op:tt, $p1: literal, $p2: literal, $expected: literal) => {
            // run the given operator and compare the result in Rust
            let result = $e.eval(stringify!(($op $p1 $p2)).to_string());
            assert!(result.is_ok());
            let type_conversion_result = result.unwrap().unbox();
            assert!(type_conversion_result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = type_conversion_result.unwrap();
            assert_eq!(unwrapped, $expected as f64);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p1 p2) $expected)).to_string());
            assert!(result2.is_ok());
            let type_conversion_result2 = result2.unwrap().unbox();
            assert!(type_conversion_result2.is_ok());
            let unwrapped2: bool = type_conversion_result2.unwrap();
            assert!(unwrapped2);
        };
    }

    #[test]
    fn test_basic_math() {
        let e = Evaluator::new();
        CHECK_MATH_1!(e, +, 4, 4);
        CHECK_MATH_2!(e, +, 4, 3, 7);
        CHECK_MATH_2!(e, +, 34.2, 5, 39.2);
        //CHECK_MATH_1!(e, -, 2, (- 2));
        //CHECK_MATH_1!(e, -, 3.45, (- 3.45));
        CHECK_MATH_2!(e, -, 10, 2, 8);
        CHECK_MATH_2!(e, *, 8, 2, 16);
        //CHECK_MATH_2!(e, *, (- 2), 24, (- 48));
        //CHECK_MATH_2!(e, *, (- 1.5), 2, (- 3.0));
        CHECK_MATH_2!(e, /, 8, 2, 4);
        //CHECK_MATH_2!(e, /, (- 2), 24, 0);
        //CHECK_MATH_2!(e, /, (- 58), 3, (- 19));
        //CHECK_MATH_2!(e, /, (- 1.5), 2, (- 0.75));
        CHECK_MATH_2!(e, /, 0.5, 2.0, 0.25);
    }
}
