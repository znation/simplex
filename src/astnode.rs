use crate::astinput::ASTInput;

#[derive(Debug, PartialEq)]
pub struct ASTNode {

}

impl ASTNode {
      pub fn parse_program(input: ASTInput) -> ASTNode {

      }
      pub fn parse_expression(&mut input: ASTInput) -> ASTNode {

      }
      pub fn parse_optional_parameter_list(&mut input: ASTInput) -> ASTNode {

      }
      pub fn parse_parameter_list(&self, &mut input: ASTInput) {

      }
      pub fn parse_literal(&mut input: ASTInput) -> ASTNode {

      }
      pub fn parse_number(&mut input: ASTInput) -> ASTNode {

      }
      pub fn parse_optional_whitespace(&mut input: ASTInput) {

      }
      pub fn parse_whitespace(&mut input: ASTInput) {

      }
      pub fn parse_identifier(&mut input: ASTInput) -> ASTNode {

      }
      pub fn parse_string(&mut input: ASTInput) -> ASTNode {

      }

      // produces invalid node! should only use for testing
      pub fn new() -> ASTNode {
          ASTNode { }
      }

      pub fn to_string(&self) -> String {

      }

      pub fn kind(&self) -> NodeKind {

      }

      pub fn children(&self) -> Vec<ASTNode> {

      }

      pub fn integer(&self) -> i64 {

      }

      pub fn floatingPoint(&self) -> f64 {

      }

      pub fn string(&self) -> String {

      }

      pub fn line(&self) -> i64 {

      }

      pub fn col(&self) -> i64 {

      }

}


#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Expression,
    FloatingPoint,
    Identifier,
    Integer,
    Invalid,
    Literal,
    Number,
    OptionalParameterList,
    ParameterList,
    Program,
    String,
    Whitespace,
}