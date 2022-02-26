use crate::{astinput::ASTInput, astnode::ASTNode, errors::EvaluationError};

#[derive(Debug, PartialEq)]
pub struct Parser {}

impl Parser {
    pub fn parse<S: AsRef<str>>(input: S) -> Result<ASTNode, EvaluationError> {
        let mut ast_input = ASTInput::from(input.as_ref());
        ASTNode::parse_program(&mut ast_input)
    }
}

#[cfg(test)]
mod tests {
    use crate::astnode::NodeKind;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(Parser::parse("(+ 3 4)"), Parser::parse(" ( +  3  4 ) "));
        match Parser::parse("(let 'asdf' 3") {
            Ok(_) => assert!(false),
            Err(_) => (),
        }
        assert_eq!(
            Parser::parse("(let asdf 3)").unwrap().kind(),
            NodeKind::Program
        );
        assert_eq!(
            Parser::parse("(let asdf' 3)").unwrap().kind(),
            NodeKind::Program
        );
        assert_eq!(
            Parser::parse("(+ 3 4)(- 3 4)"),
            Parser::parse(" ( + 3 4)\n\r\n( - 3 4  )\n")
        );
        assert_eq!(
            Parser::parse("'\nasdf\r\n'").unwrap().children()[0].children()[0].string(),
            "\nasdf\r\n"
        );
    }
}
