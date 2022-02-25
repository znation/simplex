use std::{error::Error, fmt::Display};

use crate::{
    astnode::{ASTNode, NodeKind},
    structure::{Backtrace, StructureKind},
};

fn format_backtrace(backtrace: &Backtrace) -> String {
    let mut backtrace_str = String::new();
    let mut i= 0;
    for (function_name, line, _col) in backtrace.iter().rev() {
        backtrace_str += format!("frame #{}: {} at <stdin>:{}\n", i, function_name, line).as_ref();
        i += 1;
    }
    backtrace_str
}

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    ParseError(String),
    RuntimeError(String, Backtrace),
}

impl EvaluationError {
    pub fn type_mismatch(
        node: &ASTNode,
        backtrace: Backtrace,
        expected: StructureKind,
        found: StructureKind,
    ) -> EvaluationError {
        EvaluationError::RuntimeError(
            format!(
                "{}|{}: type mismatch error: expected {:?}, found {:?}",
                node.line(),
                node.col(),
                expected,
                found
            ),
            backtrace)
    }
    pub fn parse_error<S1: AsRef<str>, S2: AsRef<str>>(
        kind: NodeKind,
        expected: S1,
        actual: S2,
        line: i64,
        col: i64,
    ) -> EvaluationError {
        EvaluationError::ParseError(
            format!(
                "{}|{}: parse error while attempting to parse {:?}: expected {}, found {}",
                line,
                col,
                kind,
                expected.as_ref(),
                actual.as_ref()
            )
        )
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::ParseError(msg) => write!(f, "{}", msg),
            EvaluationError::RuntimeError(msg, backtrace) => {
                write!(f, "\nbacktrace: \n\n{}\nexception message:\n\n{}", format_backtrace(backtrace), msg)
            },
        }
    }
}

impl Error for EvaluationError {}