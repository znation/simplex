use crate::structure::Backtrace;
use crate::structure::Empty;
use crate::structure::StructureKind;
use std::collections::HashMap;

use crate::astnode::ASTNode;
use crate::astnode::NodeKind;
use crate::errors::EvaluationError;

use crate::parser::Parser;
use crate::stdlib::Stdlib;
use crate::structure::EvaluationResult;
use crate::structure::Function;
use crate::structure::FunctionBody;
use crate::structure::Structure;
use crate::structure::SymbolTable;

fn dict_of_params(
    parameter_list: &[ASTNode],
    parameter_values: &[Structure],
) -> HashMap<String, Structure> {
    let mut ret = HashMap::new();
    let n_params = parameter_list.len() - 1;
    let n_values = parameter_values.len();
    assert_eq!(n_params, n_values);
    for i in 0..n_params {
        let param = &parameter_list[i];
        assert_eq!(param.kind(), NodeKind::Identifier);
        let value = parameter_values[i].clone();
        ret.insert(param.string().clone(), value);
    }
    ret
}

pub struct Evaluator {
    symbols: SymbolTable,
    backtrace: Backtrace,
}

impl Evaluator {
    pub fn add_symbols(&mut self, new_symbols: SymbolTable) {
        for (k, v) in new_symbols {
            self.symbols.insert(k, v);
        }
    }

    pub fn new() -> Evaluator {
        let mut ret = Evaluator {
            symbols: SymbolTable::empty(),
            backtrace: Backtrace::empty(),
        };

        // Rust-native parts of the standard library
        ret.add_symbols(Stdlib::symbols());

        // Simplex stdlib (written in simplex)
        let simplex_lib = include_str!("stdlib.simplex");
        let result = ret.eval(simplex_lib);
        assert!(result.is_ok());

        ret
    }

    pub fn eval_node(&mut self, node: &ASTNode) -> EvaluationResult {
        match node.kind() {
            crate::astnode::NodeKind::Expression => self.eval_expression(node),
            crate::astnode::NodeKind::Identifier => self.eval_identifier(node),
            crate::astnode::NodeKind::Literal => self.eval_literal(node),
            crate::astnode::NodeKind::Program => self.eval_program(node),
            _ => panic!(),
        }
    }

    pub fn eval<S: AsRef<str>>(&mut self, str: S) -> EvaluationResult {
        let node = Parser::parse(str)?;
        self.eval_node(&node)
    }

    pub fn eval_lambda_expression(&self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::Identifier);
        assert_eq!(children[0].string(), "lambda");
        assert_eq!(children[1].kind(), NodeKind::OptionalParameterList);
        let children_of_children = children[1].children();
        if children_of_children.is_empty() {
            return Err(EvaluationError::RuntimeError(
                "lambda expects 2 or more parameters, got 0".to_string(),
                self.backtrace.clone(),
            ));
        }
        let parameter_list = children_of_children[0].children();
        let function_body = FunctionBody::Lambda(
            |_node, outer_symbols, outer_backtrace, parameter_list, params| {
                let body = parameter_list.last().unwrap();
                let mut symbols = outer_symbols;

                // TODO: support variadic functions
                // for now, just ensure that the function is getting the number of parameters it expects.
                if parameter_list.len() - 1 != params.len() {
                    return Err(EvaluationError::RuntimeError(
                        format!(
                            "lambda expression expected {} parameters, got {}",
                            parameter_list.len() - 1,
                            params.len()
                        ),
                        outer_backtrace,
                    ));
                }

                symbols.extend(dict_of_params(&parameter_list, &params));
                let mut e = Evaluator {
                    symbols,
                    backtrace: outer_backtrace,
                };
                e.eval_node(body)
            },
        );
        let function = Function {
            parameter_list: parameter_list.clone(),
            function: function_body,
        };
        Ok(Structure::Function(function))
    }

    pub fn eval_let_expression(&mut self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        let child0 = children.get(0).unwrap();
        let child1 = children.get(1).unwrap();
        assert_eq!(child0.kind(), NodeKind::Identifier);
        assert_eq!(child0.string(), "let");
        assert_eq!(child1.kind(), NodeKind::OptionalParameterList);
        let children_of_children = child1.children();
        if children_of_children.is_empty() {
            return Err(EvaluationError::RuntimeError(
                "let expression expects 2 parameters, got 0".to_string(),
                self.backtrace.clone(),
            ));
        }
        let parameter_list = children_of_children.get(0).unwrap();
        let id_with_value = parameter_list.children();
        if id_with_value.len() != 2 {
            return Err(EvaluationError::RuntimeError(
                format!(
                    "let expression expects 2 parameters, got {}",
                    id_with_value.len()
                ),
                self.backtrace.clone(),
            ));
        }
        let id = id_with_value.get(0).unwrap();
        if id.kind() != NodeKind::Identifier {
            return Err(EvaluationError::RuntimeError(
                format!(
                    "first parameter to let expression should be an identifier, found {:#?}",
                    id.kind()
                ),
                self.backtrace.clone(),
            ));
        }
        let new_symbol = self.eval_node(id_with_value.get(1).unwrap())?;
        self.symbols.insert(id.string().clone(), new_symbol);
        Ok(Structure::Boolean(true))
    }
    pub fn eval_if_expression(&mut self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::Identifier);
        assert_eq!(children[0].string(), "if");
        assert_eq!(children[1].kind(), NodeKind::OptionalParameterList);
        let children_of_children = children[1].children();
        if children_of_children.is_empty() {
            return Err(EvaluationError::RuntimeError(
                "if expression expects 3 parameters, got 0".to_string(),
                self.backtrace.clone(),
            ));
        }
        let parameters = children_of_children[0].children();
        if parameters.len() != 3 {
            return Err(EvaluationError::RuntimeError(
                format!(
                    "if expression expects 3 parameters, got {}",
                    parameters.len()
                ),
                self.backtrace.clone(),
            ));
        }
        assert_eq!(parameters.len(), 3);
        let result = self.eval_node(parameters.get(0).unwrap());
        let condition = result?.boolean();
        if condition {
            self.eval_node(parameters.get(1).unwrap())
        } else {
            self.eval_node(parameters.get(2).unwrap())
        }
    }
    pub fn eval_cond_expression(&mut self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::Identifier);
        assert_eq!(children[0].string(), "cond");
        assert_eq!(children[1].kind(), NodeKind::OptionalParameterList);
        let children_of_children = children[1].children();
        if children_of_children.is_empty() {
            return Err(EvaluationError::RuntimeError(
                "cond expression expects pairs of conditions and expressions as parameters; got none".to_string(),
                self.backtrace.clone(),
            ));
        }
        let parameters = children_of_children[0].children();
        if parameters.len() % 2 != 0 {
            return Err(EvaluationError::RuntimeError(
                "cond must take an even number of parameters (pairs of condition and expression)"
                    .to_string(),
                self.backtrace.clone(),
            ));
        }
        let mut i = 0;
        while i < parameters.len() {
            let result = self.eval_node(parameters.get(i).unwrap());
            let condition = result?.boolean();
            if condition {
                return self.eval_node(parameters.get(i + 1).unwrap());
            }
            i += 2;
        }
        Err(EvaluationError::RuntimeError(
            "`cond` expression did not return a value (no condition evaluated to true)".to_string(),
            self.backtrace.clone(),
        ))
    }

    pub fn eval_parameters(&mut self, node: &ASTNode) -> Result<Vec<Structure>, EvaluationError> {
        if node.kind() == NodeKind::OptionalParameterList {
            let children = node.children();
            if children.is_empty() {
                return Ok(Vec::new());
            }
            assert_eq!(children.len(), 1);
            return self.eval_parameters(children.get(0).unwrap());
        }

        assert_eq!(node.kind(), NodeKind::ParameterList);
        let mut ret = Vec::new();
        for child in node.children() {
            let result = self.eval_node(child)?;
            ret.push(result);
        }
        Ok(ret)
    }

    pub fn eval_expression(&mut self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Expression);
        let children = node.children();
        assert_eq!(children.len(), 2);
        let first_child = children.get(0).unwrap();
        if first_child.kind() == NodeKind::Identifier {
            if first_child.string() == "lambda" {
                return self.eval_lambda_expression(node);
            } else if first_child.string() == "let" {
                return self.eval_let_expression(node);
            } else if first_child.string() == "if" {
                return self.eval_if_expression(node);
            } else if first_child.string() == "cond" {
                return self.eval_cond_expression(node);
            }
        }

        let function_node = self.eval_node(first_child)?;
        let params = self.eval_parameters(children.get(1).unwrap())?;
        match function_node {
            Structure::Function(ref callable) => {
                self.backtrace
                    .push((function_node.to_string(), node.line(), node.col()));
                let retval = callable.call(node, &self.symbols, &self.backtrace, params);
                self.backtrace.pop();
                retval
            }
            _ => Err(EvaluationError::type_mismatch(
                node,
                self.backtrace.clone(),
                StructureKind::Function,
                function_node.kind(),
            )),
        }
    }

    pub fn eval_identifier(&self, node: &ASTNode) -> EvaluationResult {
        let str = node.string();
        if str == "true" {
            return Ok(Structure::Boolean(true));
        } else if str == "false" {
            return Ok(Structure::Boolean(false));
        }

        let result = self.symbols.get(str);
        match result {
            Some(structure) => Ok(structure.clone()),
            None => Err(EvaluationError::RuntimeError(
                format!("undeclared identifier: {}", str),
                self.backtrace.clone(),
            )),
        }
    }

    pub fn eval_literal(&self, node: &ASTNode) -> EvaluationResult {
        assert_eq!(node.kind(), NodeKind::Literal);
        let children = node.children();
        assert_eq!(children.len(), 1);
        let child = children.get(0).unwrap();
        match child.kind() {
            NodeKind::FloatingPoint => Ok(Structure::FloatingPoint(child.floating_point())),
            NodeKind::Integer => Ok(Structure::Integer(child.integer())),
            NodeKind::Literal => todo!(), // TODO: why isn't Literal used?
            NodeKind::String => Ok(Structure::from_string(child.string())),
            _ => panic!(),
        }
    }

    pub fn eval_program(&mut self, node: &ASTNode) -> EvaluationResult {
        let mut ret = Structure::new();
        assert_eq!(node.kind(), NodeKind::Program);
        for exp in node.children() {
            ret = self.eval_node(exp)?;
        }
        Ok(ret)
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

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = result.unwrap().floating_point();
            assert_eq!(unwrapped, $expected as f64);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p) $expected)).to_string());
            assert!(result2.is_ok());
            let unwrapped2: bool = result2.unwrap().boolean();
            assert!(unwrapped2);
        };
    }

    macro_rules! CHECK_MATH_2 {
        ($e: ident, $op:tt, $p1: literal, $p2: literal, $expected: literal) => {
            // run the given operator and compare the result in Rust
            let result = $e.eval(stringify!(($op $p1 $p2)).to_string());
            assert!(result.is_ok());

            // comparing all math in f64 should be sufficient
            let unwrapped: f64 = result.unwrap().floating_point();
            assert_eq!(unwrapped, $expected as f64);

            // now, run the same operator and compare within the evaluator
            // (the = expression should return true)
            let result2 = $e.eval(stringify!((= ($op $p1 $p2) $expected)).to_string());
            assert!(result2.is_ok());
            let unwrapped2: bool = result2.unwrap().boolean();
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

    #[test]
    fn test_if_expressions() {
        let mut e = Evaluator::new();
        assert_eq!(e.eval("(if false 1 2)"), Ok(Structure::Integer(2)));
        assert_eq!(e.eval("(if true 1 2)"), Ok(Structure::Integer(1)));

        // make sure the false path doesn't
        // get executed, by using an undeclared identifier
        assert_eq!(e.eval("(if true 1 missing2)"), Ok(Structure::Integer(1)));
        assert_eq!(e.eval("(if false missing1 2)"), Ok(Structure::Integer(2)));
    }

    #[test]
    fn test_cond_expressions() {
        let mut e = Evaluator::new();
        assert_eq!(
            e.eval(
                "(cond
            false 1
            true 2
            false 3
            true 4)"
            ),
            Ok(Structure::Integer(2))
        );

        // make sure the false or redundant paths
        // don't get executed, by using an undeclared identifier
        assert_eq!(
            e.eval(
                "(cond
            false missing1
            false missing2
            true 3
            true missing4)"
            ),
            Ok(Structure::Integer(3))
        );
    }
}
