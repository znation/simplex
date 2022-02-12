use crate::astinput::ASTInput;
use crate::errors::ParseError;

#[derive(Debug, PartialEq)]
enum ASTValue {
    Int(i64),
    Double(f64),
    String(String),
    Children(Vec<ASTNode>),
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct ASTNode {
    kind: NodeKind,
    value: ASTValue,

      line: i64,
      col: i64,
}

impl ASTNode {
      pub fn parse_program(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
          let ret = ASTNode {
              kind: NodeKind::Program,
              value: ASTValue::Children(Vec::new()),
              line: input.line(),
              col: input.col(),
            };

            match ASTNode::parse_expression(&mut input) {
                Ok(v) => ret.children().push(v),
                Err(e) => return Err(e)
            }
            if (input.size() > 0) {
                match ASTNode::parse_program(&mut input) {
                    Ok(v) => ret.children().push(v),
                    Err(e) => return Err(e)
                }
            }
            return Ok(ret);
      }

      pub fn parse_expression(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

        let kind = NodeKind::Expression;

        match ASTNode::parse_optional_whitespace(&mut input) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

            if (input.size() == 0) {
                return Err(ParseError::from(kind, "(", "EOF", input.line(), input.col()));
            }

            let next = input.peek();
            let ret = if (next == '(') {
                let mut children = Vec::new();
                ASTNode::expect(kind, input, "(");
                match ASTNode::parse_expression(&mut input) {
                    Ok(v) => children.push(v),
                    Err(e) => return Err(e)
                }
                match ASTNode::parse_optional_parameter_list(&mut input) {
                    Ok(v) => children.push(v),
                    Err(e) => return Err(e)
                }
                match ASTNode::parse_optional_whitespace(&mut input) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                ASTNode::expect(kind, &mut input, ")");
                Ok(ASTNode { kind: kind, value: ASTValue::Children(children), line: input.line(), col: input.col() })
            } else if (next == '\'' || next.is_digit(10)) {
                ASTNode::parse_literal(&mut input)
            } else {
                ASTNode::parse_identifier(&mut input)
            };
            match ASTNode::parse_optional_whitespace(&mut input) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
            return ret;

      }
      pub fn parse_optional_parameter_list(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

      }
      pub fn parse_parameter_list(&self, input: &mut ASTInput) -> Result<(), ParseError> {

      }
      pub fn parse_literal(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

      }
      pub fn parse_number(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

      }
      pub fn parse_optional_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {

      }
      pub fn parse_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {

      }
      pub fn parse_identifier(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

      }
      pub fn parse_string(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

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


    fn expect<S: AsRef<str>>(kind: NodeKind, input: &mut ASTInput, token: S) -> Result<(), ParseError> {
        let tokenStr = token.as_ref();
        let tokenSize = tokenStr.chars().count();
        if (tokenSize > input.size()) {
            let remaining = input.remaining();
            return Err(ParseError::from(kind, token, remaining, input.line(), input.col()));
        }
        let should_be_token = &input.get().as_str()[0..tokenSize];
        if (should_be_token != tokenStr) {
            return Err(ParseError::from(kind, token, should_be_token, input.line(), input.col()));
        }
        input.advance(tokenSize);
        return Ok(())
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
