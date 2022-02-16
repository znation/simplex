use std::{collections::HashMap, hash::Hash, iter::empty};

use crate::{structure::{Structure, StructureKind, Function}, astnode::ASTNode, errors::EvaluationError};

fn extractFloat(n: &Structure) -> f64 {
  if (n.kind() == StructureKind::Integer) {
    return n.integer() as f64;
  }
  return n.floating_point();
}

fn unaryPlus(n: &Structure) -> Structure {
  assert!(n.kind() == StructureKind::Integer ||
         n.kind() == StructureKind::FloatingPoint);
  return n.clone(); // don't modify
}

fn unaryMinus(n: &Structure) -> Structure {
  if (n.kind() == StructureKind::Integer) {
    return Structure::Integer(-(n.integer()));
  } else {
    assert_eq!(n.kind(), StructureKind::FloatingPoint);
    return Structure::FloatingPoint(-(n.floating_point()));
  }
}

fn plus(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
  if (params.len() == 1) {
    return Ok(unaryPlus(&params[0]));
  }
  assert_eq!(params.len(), 2);
  if (params[0].kind() == StructureKind::Integer &&
      params[1].kind() == StructureKind::Integer) {
    return Ok(Structure::Integer(params[0].integer() + params[1].integer()));
  }
  return Ok(Structure::FloatingPoint(extractFloat(&params[0]) + extractFloat(&params[1])));
}

fn minus(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
  if (params.len() == 1) {
    return Ok(unaryMinus(&params[0]));
  }
  assert_eq!(params.len(), 2);
  if (params[0].kind() == StructureKind::Integer &&
      params[1].kind() == StructureKind::Integer) {
    return Ok(Structure::Integer(params[0].integer() - params[1].integer()));
  }
  return Ok(Structure::FloatingPoint(extractFloat(&params[0]) - extractFloat(&params[1])));
}


fn times(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
  assert!(params.len() >= 1);
  let mut allInteger = true;
  for param in &params {
    if (param.kind() != StructureKind::Integer) {
      allInteger = false;
    }
  }
  if (allInteger) {
    let mut ret: i64 = 1;
    for param in params {
      ret *= param.integer();
    }
    return Ok(Structure::Integer(ret))
  } else {
    let mut ret: f64 = 1.0;
    for param in params {
      ret *= extractFloat(&param);
    }
    return Ok(Structure::FloatingPoint(ret));
  }
}

fn divide(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
  assert_eq!(params.len(), 2);
  if (params[0].kind() == StructureKind::Integer &&
      params[1].kind() == StructureKind::Integer) {
    return Ok(Structure::Integer(params[0].integer() / params[1].integer()));
  } else {
    return Ok(Structure::FloatingPoint(extractFloat(&params[0]) / extractFloat(&params[1])));
  }
}

fn equals(node: ASTNode, params: Vec<Structure>) -> Result<Structure, EvaluationError> {
  let paramsSize = params.len();
  assert!(paramsSize >= 2);
  let reference = &params[0];
  let mut ret = true;
  for i in 1..paramsSize {
    let param = &params[i];
    ret = ret && (reference == param);
  }
  return Ok(Structure::Boolean(ret));
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
