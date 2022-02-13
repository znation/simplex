use crate::{astnode::ASTNode, astinput::ASTInput, errors::ParseError};

#[derive(Debug, PartialEq)]
pub struct Parser {

}

impl Parser {
    pub fn parse(input: String) -> Result<ASTNode, ParseError> {
        let mut ast_input = ASTInput::from_str(&input);
        ASTNode::parse_program(&mut ast_input)
    }
}
