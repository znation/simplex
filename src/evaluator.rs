use crate::symboltable::SymbolTable;
use crate::structure::Structure;
use crate::structure::StructureKind;

pub struct Evaluator {
    symbols: SymbolTable
}

impl Evaluator {
   pub fn new() -> Evaluator {
       return Evaluator { symbols: SymbolTable::new() }
   } 

   pub fn eval(&self, str: String) -> Result<Structure, EvaluationError> {
       return Ok(Structure { kind: StructureKind::Nil })
   }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct EvaluationError {}

impl EvaluationError {
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! CHECK_MATH_1 {
        ($e: ident, $op:tt, $p: literal, $expected: literal) => {
            let result = $e.eval(stringify!(($op $a)).to_string());
            assert!(result.is_ok());
            if result.is_ok() {
                //assert_eq!(result.unwrap(), $expected);
                //assert_eq!($e.eval(stringify!((= ($op $a) $result))), true);
            }
        };
    }


    #[test]
    fn test_basic_math() {
        let e = Evaluator::new();
        CHECK_MATH_1!(e, +, 4, 4);
        //CHECK_MATH_2(+, 4, 3, 7);
        //CHECK_MATH_2(+, 34.2, 5, 39.2);
        //CHECK_MATH_1(-, 2, (- 2));
        //CHECK_MATH_1(-, 3.45, (- 3.45));
        //CHECK_MATH_2(-, 10, 2, 8);
        //CHECK_MATH_2(*, 8, 2, 16);
        //CHECK_MATH_2(*, (- 2), 24, (- 48));
        //CHECK_MATH_2(*, (- 1.5), 2, (- 3.0));
        //CHECK_MATH_2(/, 8, 2, 4);
        //CHECK_MATH_2(/, (- 2), 24, 0);
        //CHECK_MATH_2(/, (- 58), 3, (- 19));
        //CHECK_MATH_2(/, (- 1.5), 2, (- 0.75));
        //CHECK_MATH_2(/, 0.5, 2.0, 0.25);
    }
}