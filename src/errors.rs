use crate::astnode::NodeKind;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn from<S1: AsRef<str>, S2: AsRef<str>>(
        kind: NodeKind,
        expected: S1,
        actual: S2,
        line: i64,
        col: i64,
    ) -> ParseError {
        ParseError {
            message: format!(
                "{}|{}: parse error while attempting to parse {:?}: expected {}, found {}",
                line,
                col,
                kind,
                expected.as_ref(),
                actual.as_ref()
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EvaluationError {
    pub message: String,
}

impl EvaluationError {
    pub fn from_parse_error(e: ParseError) -> EvaluationError {
        EvaluationError { message: e.message }
    }
}
