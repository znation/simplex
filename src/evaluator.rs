use std::collections::HashMap;

use crate::astnode::ASTNode;
use crate::astnode::NodeKind;
use crate::errors::EvaluationError;

use crate::parser::Parser;
use crate::stdlib::Stdlib;
use crate::structure::Function;
use crate::structure::FunctionBody;
use crate::structure::Structure;
use crate::structure::StructureKind;
use crate::structure::SymbolTable;

fn dictOfParams(
    parameterList: &Vec<ASTNode>,
    parameterValues: &Vec<Structure>,
) -> HashMap<String, Structure> {
    let mut ret = HashMap::new();
    let nParams = parameterList.len() - 1;
    let nValues = parameterValues.len();
    assert_eq!(nParams, nValues);
    for i in 0..nParams {
        let param = &parameterList[i];
        assert_eq!(param.kind(), NodeKind::Identifier);
        let value = parameterValues[i].clone();
        ret.insert(param.string(), value);
    }
    ret
}

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

    pub fn eval_node(&mut self, node: ASTNode) -> Result<Structure, EvaluationError> {
        match node.kind() {
            crate::astnode::NodeKind::Expression => self.eval_expression(node),
            crate::astnode::NodeKind::Identifier => self.eval_identifier(node),
            crate::astnode::NodeKind::Literal => self.eval_literal(node),
            crate::astnode::NodeKind::Program => self.eval_program(node),
            _ => panic!(),
        }
    }

    pub fn eval(&mut self, str: String) -> Result<Structure, EvaluationError> {
        let node = match Parser::parse(str) {
            Ok(n) => n,
            Err(e) => return Err(EvaluationError::from_parse_error(e)),
        };
        self.eval_node(node)
    }

    pub fn eval_lambda_expression(&self, node: ASTNode) -> Result<Structure, EvaluationError> {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::Identifier);
        assert_eq!(children[0].string(), "lambda");
        assert_eq!(children[1].kind(), NodeKind::OptionalParameterList);
        let parameterList = children[1].children()[0].children().clone();
        let function_body = FunctionBody::Lambda(|node, outerSymbols, parameterList, params| {
            let body = parameterList[parameterList.len() - 1].clone();
            let mut symbols = outerSymbols.clone();
            symbols.extend(dictOfParams(&parameterList, &params));
            let mut e = Evaluator { symbols };
            return e.eval_node(body);
        });
        let function = Function {
            outerSymbols: self.symbols.clone(),
            parameterList: parameterList,
            function: function_body,
        };
        Ok(Structure::Function(function))
    }

    pub fn eval_let_expression(&mut self, node: ASTNode) -> Result<Structure, EvaluationError> {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        let child0 = children[0].clone();
        let child1 = children[1].clone();
        assert_eq!(child0.kind(), NodeKind::Identifier);
        assert_eq!(child0.string(), "let");
        assert_eq!(child1.kind(), NodeKind::OptionalParameterList);
        let parameterList = child1.children()[0].clone();
        let id_with_value = parameterList.children();
        assert_eq!(id_with_value.len(), 2);
        let id = id_with_value[0].clone();
        assert_eq!(id.kind(), NodeKind::Identifier);
        let new_symbol = match self.eval_node(id_with_value[1].clone()) {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        self.symbols.insert(id.string(), new_symbol);
        return Ok(Structure::Boolean(true));
    }
    pub fn eval_if_expression(&mut self, node: ASTNode) -> Result<Structure, EvaluationError> {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::Identifier);
        assert_eq!(children[0].string(), "if");
        assert_eq!(children[1].kind(), NodeKind::OptionalParameterList);
        let parameters = children[1].children()[0].children();
        assert_eq!(parameters.len(), 3);
        let result = self.eval_node(parameters[0].clone());
        let condition = match result {
            Ok(c) => c.boolean(),
            Err(e) => return Err(e),
        };
        if (condition) {
            return self.eval_node(parameters[1].clone());
        } else {
            return self.eval_node(parameters[2].clone());
        }
    }
    pub fn eval_cond_expression(&self, node: ASTNode) -> Result<Structure, EvaluationError> {
        todo!()
    }

    pub fn eval_parameters(&mut self, node: ASTNode) -> Result<Vec<Structure>, EvaluationError> {
        if (node.kind() == NodeKind::OptionalParameterList) {
            let children = node.children();
            if (children.len() == 0) {
                return Ok(Vec::new());
            }
            assert_eq!(children.len(), 1);
            return self.eval_parameters(children[0].clone());
        }

        assert_eq!(node.kind(), NodeKind::ParameterList);
        let mut ret = Vec::new();
        for child in node.children().clone() {
            let result = self.eval_node(child);
            match result {
                Ok(s) => ret.push(s),
                Err(e) => return Err(e),
            }
        }
        return Ok(ret);
    }

    pub fn eval_expression(&mut self, node: ASTNode) -> Result<Structure, EvaluationError> {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        let first_child = children[0].clone();
        if (first_child.kind() == NodeKind::Identifier) {
            if (first_child.string() == "lambda") {
                return self.eval_lambda_expression(node);
            } else if (first_child.string() == "let") {
                return self.eval_let_expression(node);
            } else if (first_child.string() == "if") {
                return self.eval_if_expression(node);
            } else if (first_child.string() == "cond") {
                return self.eval_cond_expression(node);
            }
        }

        let function_node = match self.eval_node(first_child) {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        let params = match self.eval_parameters(children[1].clone()) {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        let function = match function_node {
            Structure::Function(callable) => callable,
            _ => panic!(),
        };
        return function.call(node, params);
    }

    pub fn eval_identifier(&self, node: ASTNode) -> Result<Structure, EvaluationError> {
        let str = node.string();
        if (str == "true") {
            return Ok(Structure::Boolean(true));
        } else if (str == "false") {
            return Ok(Structure::Boolean(false));
        }

        let result = self.symbols.get(&str);
        match result {
            Some(structure) => Ok(structure.clone()),
            None => Err(EvaluationError {
                message: format!("undeclared identifier: {}", str),
            }),
        }
    }

    pub fn eval_literal(&self, node: ASTNode) -> Result<Structure, EvaluationError> {
        assert_eq!(node.kind(), NodeKind::Literal);
        let children = node.children();
        assert_eq!(children.len(), 1);
        let child = children[0].clone();
        match child.kind() {
            NodeKind::FloatingPoint => Ok(Structure::FloatingPoint(child.floating_point())),
            NodeKind::Integer => Ok(Structure::Integer(child.integer())),
            NodeKind::Literal => todo!(), // TODO: why isn't Literal used?
            NodeKind::String => Ok(Structure::from_string(child.string())),
            _ => panic!(),
        }
    }

    pub fn eval_program(&mut self, node: ASTNode) -> Result<Structure, EvaluationError> {
        let mut ret = Ok(Structure::new());
        assert_eq!(node.kind(), NodeKind::Program);
        for exp in node.children() {
            ret = self.eval_node(exp.clone());
        }
        return ret;
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
            dbg!(&result);
            assert!(result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = result.unwrap().floating_point();
            dbg!(&unwrapped);
            assert_eq!(unwrapped, $expected as f64);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p) $expected)).to_string());
            dbg!(&result2);
            assert!(result2.is_ok());
            let unwrapped2: bool = result2.unwrap().boolean();
            dbg!(&unwrapped2);
            assert!(unwrapped2);
        };
    }

    macro_rules! CHECK_MATH_2 {
        ($e: ident, $op:tt, $p1: literal, $p2: literal, $expected: literal) => {
            // run the given operator and compare the result in Rust
            let result = $e.eval(stringify!(($op $p1 $p2)).to_string());
            dbg!(&result);
            assert!(result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = result.unwrap().floating_point();
            dbg!(&unwrapped);
            assert_eq!(unwrapped, $expected as f64);

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
    fn test_basic_math() {
        let mut e = Evaluator::new();
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
