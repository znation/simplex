use crate::{astnode::ASTNode, astinput::ASTInput};

#[derive(Debug, PartialEq)]
pub struct Parser {

}

impl Parser {
    pub fn parse(input: String) -> ASTNode {
        let input = ASTInput::from_str(&input);
    }
}
