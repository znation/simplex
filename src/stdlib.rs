use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use crate::{
    astnode::ASTNode,
    errors::EvaluationError,
    structure::{Backtrace, EvaluationResult, Function, Structure, StructureKind, SymbolTable},
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

fn plus(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() == 1 {
        return Ok(unary_plus(&params[0]));
    }
    if params.len() != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("+ expects 1 or 2 parameters, got {}", params.len()),
            backtrace,
        ));
    }
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

fn minus(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() == 1 {
        return Ok(unary_minus(&params[0]));
    }
    if params.len() != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("- expects 1 or 2 parameters, got {}", params.len()),
            backtrace,
        ));
    }
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

fn times(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() < 2 {
        return Err(EvaluationError::RuntimeError(
            format!("* expects 2 or more parameters, got {}", params.len()),
            backtrace,
        ));
    }
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

fn divide(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("/ expects 2 or more parameters, got {}", params.len()),
            backtrace,
        ));
    }
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

fn equals(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() < 2 {
        return Err(EvaluationError::RuntimeError(
            format!("= expects 2 or more parameters, got {}", params.len()),
            backtrace,
        ));
    }
    let reference = &params[0];
    let mut ret = true;
    for param in params.iter().skip(1) {
        ret = ret && (reference == param);
    }
    Ok(Structure::Boolean(ret))
}

fn lessthan(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    let params_size = params.len();
    if params_size != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("< expects 2 parameters, got {}", params_size),
            backtrace,
        ));
    }
    let reference = params[0].clone();
    let compare = params[1].clone();
    if reference.kind() != compare.kind() {
        return Err(EvaluationError::type_mismatch(
            &node,
            backtrace,
            reference.kind(),
            compare.kind(),
        ));
    }
    let ret = match reference.kind() {
        StructureKind::Byte => reference.byte() < compare.byte(),
        StructureKind::Char => reference.char() < compare.char(),
        StructureKind::FloatingPoint => reference.floating_point() < compare.floating_point(),
        StructureKind::Integer => reference.integer() < compare.integer(),
        _ => {
            return Err(EvaluationError::type_mismatch(
                &node,
                backtrace,
                StructureKind::FloatingPoint,
                compare.kind(),
            ))
        }
    };
    Ok(Structure::Boolean(ret))
}

fn greaterthan(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    let params_size = params.len();
    if params_size != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("> expects 2 parameters, got {}", params_size),
            backtrace,
        ));
    }
    let reference = params[0].clone();
    let compare = params[1].clone();
    if reference.kind() != compare.kind() {
        return Err(EvaluationError::type_mismatch(
            &node,
            backtrace,
            reference.kind(),
            compare.kind(),
        ));
    }
    let ret = match reference.kind() {
        StructureKind::Byte => reference.byte() > compare.byte(),
        StructureKind::Char => reference.char() > compare.char(),
        StructureKind::FloatingPoint => reference.floating_point() > compare.floating_point(),
        StructureKind::Integer => reference.integer() > compare.integer(),
        _ => {
            return Err(EvaluationError::type_mismatch(
                &node,
                backtrace,
                StructureKind::FloatingPoint,
                compare.kind(),
            ))
        }
    };
    Ok(Structure::Boolean(ret))
}

fn sequence(_node: ASTNode, _backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    // rely on the interpreter itself being sequential (single threaded)
    // simply return the last accumulated result
    let params_size = params.len();
    assert_ne!(params_size, 0);
    Ok(params[params_size - 1].clone())
}

fn cons(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 2 {
        return Err(EvaluationError::RuntimeError(
            format!("cons expected 2 arguments, got {}", params.len()),
            backtrace,
        ));
    }
    Ok(Structure::Cons(Box::new((
        params[0].clone(),
        params[1].clone(),
    ))))
}

fn car(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 1 {
        return Err(EvaluationError::RuntimeError(
            format!("car expected 1 argument, got {}", params.len()),
            backtrace,
        ));
    }
    let cons = params[0].clone();
    match cons {
        Structure::Cons(c) => Ok(c.0),
        _ => Err(EvaluationError::type_mismatch(
            &node,
            backtrace,
            StructureKind::Cons,
            cons.kind(),
        )),
    }
}

fn cdr(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 1 {
        return Err(EvaluationError::RuntimeError(
            format!("cdr expected 1 argument, got {}", params.len()),
            backtrace,
        ));
    }
    let cons = params[0].clone();
    match cons {
        Structure::Cons(c) => Ok(c.1),
        _ => Err(EvaluationError::type_mismatch(
            &node,
            backtrace,
            StructureKind::Cons,
            cons.kind(),
        )),
    }
}

fn list_impl(params: Vec<Structure>, idx: usize) -> Structure {
    let size = params.len() - idx;
    if size == 0 {
        Structure::Cons(Box::new((Structure::Nil, Structure::Nil)))
    } else if size == 1 {
        Structure::Cons(Box::new((params[idx].clone(), Structure::Nil)))
    } else {
        Structure::Cons(Box::new((params[idx].clone(), list_impl(params, idx + 1))))
    }
}

fn list(_node: ASTNode, _backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    Ok(list_impl(params, 0))
}

fn dict(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    let size = params.len();

    if size % 2 != 0 {
        return Err(EvaluationError::RuntimeError(
            "expected an even number of parameters to `dict`".to_string(),
            backtrace,
        ));
    }

    let mut result = HashMap::new();
    let mut i = 0;
    while i < size {
        let key = params[i].string(backtrace.clone(), Some(&node))?;
        let value = params[i + 1].clone();
        result.insert(key, value);
        i += 2;
    }
    Ok(Structure::Dict(result))
}

fn dict_get(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 2 {
        return Err(EvaluationError::RuntimeError(
            "expected 2 parameters to `dict.get`".to_string(),
            backtrace,
        ));
    }
    let key = params[0].string(backtrace.clone(), Some(&node))?;
    let dict = params[1].dict();
    match dict.get(&key) {
        Some(s) => Ok(s.clone()),
        None => Err(EvaluationError::RuntimeError(
            format!("could not find key {} in dict", key),
            backtrace,
        )),
    }
}

fn dict_set(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.len() != 3 {
        return Err(EvaluationError::RuntimeError(
            "expected 3 parameters to `dict.set`".to_string(),
            backtrace,
        ));
    }
    let key = params[0].string(backtrace, Some(&node))?;
    let value = params[1].clone();
    let mut dict = params[2].dict().clone();
    dict.insert(key, value);
    Ok(Structure::Dict(dict))
}

fn string(node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if params.is_empty() {
        return Err(EvaluationError::RuntimeError(
            "not enough parameters to `string`".to_string(),
            backtrace,
        ));
    }
    if params.len() > 1 {
        return Err(EvaluationError::RuntimeError(
            "too many parameters to `string`".to_string(),
            backtrace,
        ));
    }
    let param = params[0].clone();
    match param.string(backtrace, Some(&node)) {
        Ok(s) => Ok(Structure::from_string(&s)),
        Err(e) => Err(e),
    }
}

fn print(_node: ASTNode, _backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    for param in params {
        println!("{}", param);
    }
    Ok(Structure::Nil)
}

const MAX_READ_COUNT: usize = 1073741824;

fn read_bytes(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    let max_count = if params.is_empty() {
        MAX_READ_COUNT
    } else if params.len() == 1 {
        params[1].integer() as usize
    } else {
        return Err(EvaluationError::RuntimeError(
            "too many parameters to `read_bytes`".to_string(),
            backtrace,
        ));
    };
    let mut bytes: Vec<Structure> = Vec::new();
    for byte in stdin().bytes().take(max_count) {
        // use match structure with explicit error handling so we can preserve the backtrace
        match byte {
            Ok(b) => bytes.push(Structure::Byte(b)),
            Err(e) => return Err(EvaluationError::RuntimeError(format!("{}", e), backtrace)),
        }
    }
    let ret = list_impl(bytes, 0);
    Ok(ret)
}

fn read_line(_node: ASTNode, backtrace: Backtrace, params: Vec<Structure>) -> EvaluationResult {
    if !params.is_empty() {
        return Err(EvaluationError::RuntimeError(
            "too many parameters to `read_line`".to_string(),
            backtrace,
        ));
    }
    let mut value = String::new();
    let result = stdin().read_line(&mut value);
    // use match structure with explicit error handling so we can preserve the backtrace
    match result {
        Ok(_) => (),
        Err(e) => return Err(EvaluationError::RuntimeError(format!("{}", e), backtrace)),
    }
    Ok(Structure::from_string(&value))
}

pub struct Stdlib {}
impl Stdlib {
    pub fn symbols() -> SymbolTable {
        let mut symbols = HashMap::new();

        // math & comparison operators
        symbols.insert("+".to_string(), Function::synthetic(plus));
        symbols.insert("-".to_string(), Function::synthetic(minus));
        symbols.insert("*".to_string(), Function::synthetic(times));
        symbols.insert("/".to_string(), Function::synthetic(divide));
        symbols.insert("=".to_string(), Function::synthetic(equals));
        symbols.insert("<".to_string(), Function::synthetic(lessthan));
        symbols.insert(">".to_string(), Function::synthetic(greaterthan));

        // control flow
        symbols.insert("sequence".to_string(), Function::synthetic(sequence));

        // structural operators
        symbols.insert("cons".to_string(), Function::synthetic(cons));
        symbols.insert("car".to_string(), Function::synthetic(car));
        symbols.insert("cdr".to_string(), Function::synthetic(cdr));
        symbols.insert("list".to_string(), Function::synthetic(list));

        symbols.insert("dict".to_string(), Function::synthetic(dict));
        symbols.insert("dict.get".to_string(), Function::synthetic(dict_get));
        symbols.insert("dict.set".to_string(), Function::synthetic(dict_set));

        // values
        let endl = "\n".to_string();
        symbols.insert("endl".to_string(), Structure::from_string(&endl));
        symbols.insert("()".to_string(), Structure::Nil);
        symbols.insert("nil".to_string(), Structure::Nil);

        // conversion
        symbols.insert("string".to_string(), Function::synthetic(string));

        // i/o
        symbols.insert("print".to_string(), Function::synthetic(print));
        symbols.insert("read_bytes".to_string(), Function::synthetic(read_bytes));
        symbols.insert("read_line".to_string(), Function::synthetic(read_line));
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
            assert!(result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: bool = result.unwrap().boolean();
            assert_eq!(unwrapped, $expected);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p1 $p2) $expected)).to_string());
            assert!(result2.is_ok());
            let unwrapped2: bool = result2.unwrap().boolean();
            assert!(unwrapped2);
        };
    }

    #[test]
    fn append() {
        let mut e = Evaluator::new();
        assert_eq!(e.eval("(append (list) (list))"), e.eval("(list)"));
        assert_eq!(
            e.eval("(append (list) (list 1 2 3))"),
            e.eval("(list 1 2 3)")
        );
        assert_eq!(
            e.eval("(append (list 1) (list 2 3))"),
            e.eval("(list 1 2 3)")
        );
        assert_eq!(
            e.eval("(append (list 1 2) (list 3 4))"),
            e.eval("(list 1 2 3 4)")
        );
        assert_eq!(
            e.eval("(append (list 1 2) (list 3))"),
            e.eval("(list 1 2 3)")
        );
        assert_eq!(
            e.eval("(append (list 1 2 3) (list))"),
            e.eval("(list 1 2 3)")
        );
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
    fn read_line() {
        // TODO: port readLine test from C++ and implement overriding stdin/stdout
    }
}
