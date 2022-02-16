use std::collections::HashMap;

use crate::{
    astnode::ASTNode,
    structure::{EvaluationResult, Function, Structure, StructureKind},
};

fn extract_float(n: &Structure) -> f64 {
    if n.kind() == StructureKind::Integer {
        return n.integer() as f64;
    }
    n.floating_point()
}

fn unary_plus(n: &Structure) -> Structure {
    assert!(n.kind() == StructureKind::Integer || n.kind() == StructureKind::FloatingPoint);
    n.clone()
}

fn unary_minus(n: &Structure) -> Structure {
    if n.kind() == StructureKind::Integer {
        Structure::Integer(-(n.integer()))
    } else {
        assert_eq!(n.kind(), StructureKind::FloatingPoint);
        Structure::FloatingPoint(-(n.floating_point()))
    }
}

fn plus(_node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
    if params.len() == 1 {
        return Ok(unary_plus(&params[0]));
    }
    assert_eq!(params.len(), 2);
    if params[0].kind() == StructureKind::Integer && params[1].kind() == StructureKind::Integer {
        Ok(Structure::Integer(
            params[0].integer() + params[1].integer(),
        ))
    } else {
        Ok(Structure::FloatingPoint(
            extract_float(&params[0]) + extract_float(&params[1]),
        ))
    }
}

fn minus(_node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
    if params.len() == 1 {
        return Ok(unary_minus(&params[0]));
    }
    assert_eq!(params.len(), 2);
    if params[0].kind() == StructureKind::Integer && params[1].kind() == StructureKind::Integer {
        Ok(Structure::Integer(
            params[0].integer() - params[1].integer(),
        ))
    } else {
        Ok(Structure::FloatingPoint(
            extract_float(&params[0]) - extract_float(&params[1]),
        ))
    }
}

fn times(_node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
    assert!(!params.is_empty());
    let mut all_integer = true;
    for param in &params {
        if param.kind() != StructureKind::Integer {
            all_integer = false;
        }
    }
    if all_integer {
        let mut ret: i64 = 1;
        for param in params {
            ret *= param.integer();
        }
        Ok(Structure::Integer(ret))
    } else {
        let mut ret: f64 = 1.0;
        for param in params {
            ret *= extract_float(&param);
        }
        Ok(Structure::FloatingPoint(ret))
    }
}

fn divide(_node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
    assert_eq!(params.len(), 2);
    if params[0].kind() == StructureKind::Integer && params[1].kind() == StructureKind::Integer {
        Ok(Structure::Integer(
            params[0].integer() / params[1].integer(),
        ))
    } else {
        Ok(Structure::FloatingPoint(
            extract_float(&params[0]) / extract_float(&params[1]),
        ))
    }
}

fn equals(_node: ASTNode, params: Vec<Structure>) -> EvaluationResult {
    let params_size = params.len();
    assert!(params_size >= 2);
    let reference = &params[0];
    let mut ret = true;
    for param in params.iter().skip(1) {
        ret = ret && (reference == param);
    }
    Ok(Structure::Boolean(ret))
}

pub struct Stdlib {}
impl Stdlib {
    pub fn symbols() -> HashMap<String, Structure> {
        // TODO: populate built-in symbols with Rust implementations
        let mut symbols = HashMap::new();

        // math & comparison operators
        symbols.insert("+".to_string(), Function::synthetic(plus));
        symbols.insert("-".to_string(), Function::synthetic(minus));
        symbols.insert("*".to_string(), Function::synthetic(times));
        symbols.insert("/".to_string(), Function::synthetic(divide));
        symbols.insert("=".to_string(), Function::synthetic(equals));
        /*
        symbols["<"] = Structure(static_cast<Structure::Function>(lessthan));
        symbols[">"] = Structure(static_cast<Structure::Function>(greaterthan));

        // control flow
        symbols["sequence"] = Structure(static_cast<Structure::Function>(sequence));

        // structural operators
        symbols["cons"] = Structure(static_cast<Structure::Function>(cons));
        symbols["car"] = Structure(static_cast<Structure::Function>(car));
        symbols["cdr"] = Structure(static_cast<Structure::Function>(cdr));
        symbols["list"] = Structure(static_cast<Structure::Function>(list));

        symbols["dict"] = Structure(static_cast<Structure::Function>(dict));
        symbols["dict.get"] = Structure(static_cast<Structure::Function>(dict_get));
        symbols["dict.set"] = Structure(static_cast<Structure::Function>(dict_set));

        // values
        const static std::string endl("\n");
        symbols["endl"] = Structure(endl);
        symbols["nil"] = Structure::Nil();

        // conversion
        symbols["string"] = Structure(static_cast<Structure::Function>(string));

        // i/o
        symbols["print"] = Structure(print(symbols));
        symbols["read"] = Structure(read(symbols));
        */
        symbols
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::Evaluator;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! CHECK_COMPARISON_2 {
        ($e: ident, $op:tt, $p1: literal, $p2: literal, $expected: literal) => {
            // run the given operator and compare the result in Rust
            let result = $e.eval(stringify!(($op $p1 $p2)).to_string());
            dbg!(&result);
            assert!(result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: bool = result.unwrap().boolean();
            dbg!(&unwrapped);
            assert_eq!(unwrapped, $expected);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p1 $p2) $expected)).to_string());
            dbg!(&result2);
            assert!(result2.is_ok());
            let unwrapped2: bool = result2.unwrap().boolean();
            dbg!(&unwrapped2);
            assert!(unwrapped2);
        };
    }

    #[test]
    fn append() {
    let mut e = Evaluator::new();
    assert_eq!(e.eval("(append (list) (list))"), e.eval("(list)"));
    assert_eq!(e.eval("(append (list) (list 1 2 3))"), e.eval("(list 1 2 3)"));
    assert_eq!(e.eval("(append (list 1) (list 2 3))"), e.eval("(list 1 2 3)"));
    assert_eq!(e.eval("(append (list 1 2) (list 3 4))"), e.eval("(list 1 2 3 4)"));
    assert_eq!(e.eval("(append (list 1 2) (list 3))"), e.eval("(list 1 2 3)"));
    assert_eq!(e.eval("(append (list 1 2 3) (list))"), e.eval("(list 1 2 3)"));
    }

    #[test]
    fn operators_stdlib() {
    let mut e = Evaluator::new();
    CHECK_COMPARISON_2!(e, <=, 2, 3, true);
    CHECK_COMPARISON_2!(e, <=, 3, 3, true);
    CHECK_COMPARISON_2!(e, <=, 4, 3, false);
    CHECK_COMPARISON_2!(e, >=, 2, 3, false);
    CHECK_COMPARISON_2!(e, >=, 3, 3, true);
    CHECK_COMPARISON_2!(e, >=, 4, 3, true);
    }

    #[test]
    fn len() {
    let mut e = Evaluator::new();
    assert_eq!(e.eval("(len (list))"), Ok(Structure::Integer(0)));
    assert_eq!(e.eval("(len (list 1))"), Ok(Structure::Integer(1)));
    assert_eq!(e.eval("(len (list 1 2))"), Ok(Structure::Integer(2)));
    assert_eq!(e.eval("(len (list 1 2 3))"), Ok(Structure::Integer(3)));
    }

    #[test]
    fn reverse() {
    let mut e = Evaluator::new();
    assert_eq!(e.eval("(reverse (list))"), e.eval("(list)"));
    assert_eq!(e.eval("(reverse (list 1))"), e.eval("(list 1)"));
    assert_eq!(e.eval("(reverse (list 1 2))"), e.eval("(list 2 1)"));
    assert_eq!(e.eval("(reverse (list 1 2 3))"), e.eval("(list 3 2 1)"));
    assert_eq!(e.eval("(reverse '')"), e.eval("''"));
    assert_eq!(e.eval("(reverse 'a')"), e.eval("'a'"));
    assert_eq!(e.eval("(reverse 'ab')"), e.eval("'ba'"));
    assert_eq!(e.eval("(reverse 'hello')"), e.eval("'olleh'"));
    }

    #[test]
    fn readLine() {
        // TODO: port readLine test from C++ and implement overriding stdin/stdout
    }

}