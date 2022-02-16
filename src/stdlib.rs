use std::{collections::HashMap};

use crate::{
    astnode::ASTNode,
    structure::{Function, Structure, StructureKind, EvaluationResult},
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
