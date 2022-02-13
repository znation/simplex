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
          let mut ret = ASTNode {
              kind: NodeKind::Program,
              value: ASTValue::Children(Vec::new()),
              line: input.line(),
              col: input.col(),
            };

            match ASTNode::parse_expression(input) {
                Ok(v) => ret.children().push(v),
                Err(e) => return Err(e)
            }
            if (input.size() > 0) {
                match ASTNode::parse_program(input) {
                    Ok(v) => ret.children().push(v),
                    Err(e) => return Err(e)
                }
            }
            return Ok(ret);
      }

      pub fn parse_expression(input: &mut ASTInput) -> Result<ASTNode, ParseError> {

        let kind = NodeKind::Expression;

        match ASTNode::parse_optional_whitespace(input) {
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
                match ASTNode::parse_expression(input) {
                    Ok(v) => children.push(v),
                    Err(e) => return Err(e)
                }
                match ASTNode::parse_optional_parameter_list(input) {
                    Ok(v) => children.push(v),
                    Err(e) => return Err(e)
                }
                match ASTNode::parse_optional_whitespace(input) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                ASTNode::expect(kind, input, ")");
                Ok(ASTNode { kind: kind, value: ASTValue::Children(children), line: input.line(), col: input.col() })
            } else if (next == '\'' || next.is_digit(10)) {
                ASTNode::parse_literal(input)
            } else {
                ASTNode::parse_identifier(input)
            };
            match ASTNode::parse_optional_whitespace(input) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
            return ret;

      }
      pub fn parse_optional_parameter_list(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::OptionalParameterList;
        let value = if (input.peek() != ')') {
            let mut parameterList = ASTNode { kind: NodeKind::ParameterList, value: ASTValue::Children(Vec::new()), line: input.line(), col: input.col() };
            parameterList.parse_parameter_list(input);
            vec![parameterList]
        } else {
            Vec::new()
        };
        return Ok(ASTNode { kind, value: ASTValue::Children(value), line: input.line(), col: input.col() });
      }
      pub fn parse_parameter_list(&mut self, input: &mut ASTInput) -> Result<(), ParseError> {
          match ASTNode::parse_expression(input) {
              Ok(v) => self.children().push(v),
              Err(e) => return Err(e)
          }

          match ASTNode::parse_optional_whitespace(input) {
              Ok(_) => (),
              Err(e) => return Err(e)
          }

        if (input.size() == 0) {
            return Err(ParseError::from(NodeKind::ParameterList, "end of expression ')'", "EOF", input.line(), input.col()));
        }
        if (input.peek() == ')') {
            // hit end of parameter list
            return Ok(());
        }
        // more parameters left to parse
        self.parse_parameter_list(input)
      }
      pub fn parse_literal(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::Literal;
        let line = input.line();
        let col = input.col();
        if (input.size() == 0) {
            return Err(ParseError::from(kind, "any valid literal", "EOF", input.line(), input.col()));
        }
        let result = if (input.peek() == '\'') {
            // string
            ASTNode::parse_string(input)
        } else {
            ASTNode::parse_number(input)
        };
        if result.is_err() {
            return Err(result.unwrap_err())
        }
        let value = vec![result.unwrap()];
        Ok(ASTNode { kind, value: ASTValue::Children(value), line, col })
      }
      pub fn parse_number(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
          let mut kind = NodeKind::Integer;

        // read digits until whitespace or decimal
        let mut ss = String::new();
        let mut isFloat = false;
        let inputLen = input.size();
        for i in 0..inputLen {
            let next = input.peek();
            if (i > 0 && next == '.') {
                ss.push(next);
                isFloat = true;
            } else {
            if (is_whitespace(next) || next == ')') {
                // number is done
                break;
            }
            if (!next.is_digit(10)) {
                return Err(ParseError::from(kind, "digits 0 through 9", next.to_string(), input.line(), input.col()));
            }
            ss.push(next);
            }
            input.next();
        }

        // broke out early or, hit EOF?
        // maybe we have a valid number at this point
        let result = ss;
        let line = input.line();
        let col = input.col();
        if (isFloat) {
            kind = NodeKind::FloatingPoint;
            match result.parse::<f64>() {
                Ok(value) => return Ok(ASTNode { kind, value: ASTValue::Double(value), line, col }),
                Err(e) => panic!()
            }
        } else {
            kind = NodeKind::Integer;
            match result.parse::<i64>() {
                Ok(value) => return Ok(ASTNode { kind, value: ASTValue::Int(value), line, col }),
                Err(e) => panic!()
            }
        }
      }
      pub fn parse_optional_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {
        if (input.size() == 0) {
            return Ok(());
        }
        if (!is_whitespace(input.peek())) {
            return Ok(());
        }
        ASTNode::parse_whitespace(input)
      }
      pub fn parse_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {
        let mut found_whitespace = false;
        while (input.size() != 0) {
            let next = input.peek();
            if (is_whitespace(next)) {
            found_whitespace = true;
            } else if (!found_whitespace) {
            return Err(ParseError::from(NodeKind::Whitespace, "Any of: ' ', \\r, \\n, \\t", next.to_string(), input.line(), input.col()));
            } else {
            break;
            }
            input.next();
        }
        Ok(())
      }
      pub fn parse_identifier(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::Identifier;
        let line = input.line();
        let col = input.col();
        if (input.size() == 0) {
            return Err(ParseError::from(kind, "any valid identifier", "EOF", input.line(), input.col()));
        }
        let mut next = input.peek();
        if (next == '\'') {
            return Err(ParseError::from(kind, "non-whitespace character other than '\\'', '(' and ')'", next.to_string(), input.line(), input.col()));
        }
        let mut ss = String::new();
        while (input.size() != 0) {
            next = input.peek();
            if (is_whitespace(next) ||
                next == ')') {
            break;
            }
            if (next == '(') {
            return Err(ParseError::from(kind, "non-whitespace character other than '('", next.to_string(), line, col));
            }
            ss.push(next);
            input.next();
        }
        assert!(ss.chars().count() != 0);
        Ok(ASTNode {kind, value: ASTValue::String(ss), line, col })
      }

      pub fn parse_string(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
  let kind = NodeKind::String;
  let line = input.line();
  let col = input.col();
  let mut ss = String::new();
  let mut foundEndOfString = false;
  ASTNode::expect(kind, input, "'");
  while (input.size() != 0) {
    let mut next = input.peek();
    if (next == '\'') {
      foundEndOfString = true;
      break;
    }
    if (next == '\\') {
      // escape char
      if (input.size() < 2) {
        return Err(ParseError::from(kind, "any character followed by escape sequence (\\)", "EOF", input.line(), input.col()));
      }
      ss.push(input.next());
      next = input.peek();
    }

    ss.push(next);
    input.next();
  }
  if (!foundEndOfString) {
    return Err(ParseError::from(kind, "end of string marker (')", "EOF", input.line(), input.col()));
  }

  ASTNode::expect(kind, input, "'");
  Ok(ASTNode { kind, value: ASTValue::String(ss), line, col })

      }

      // produces invalid node! should only use for testing
      pub fn new() -> ASTNode {
          ASTNode { kind: NodeKind::Invalid , value: ASTValue::Invalid, line: 0, col: 0 }
      }

      pub fn to_string(&mut self) -> String {
          let mut ss = String::new();
          // unsafe because of static mutable INDENT_LEVEL
          unsafe {
            assert!(INDENT_LEVEL >= 0);
            for i in 0..INDENT_LEVEL {
                ss.push(' ');
            }
            let kind = format!("{:#?}", self.kind());
            ss.push_str(&kind);
            ss.push('\n');
            INDENT_LEVEL += 1;
            for child in self.children() {
                let child_str = child.to_string();
                ss.push_str(&child_str);
            }
            INDENT_LEVEL -= 1;
          }
          ss
      }

      pub fn kind(&self) -> NodeKind {
          self.kind
      }

      pub fn children(&mut self) -> &mut Vec<ASTNode> {
        match &mut self.value {
            ASTValue::Children(children) => children,
            _ => panic!()
        }
      }

      pub fn integer(&self) -> i64 {
        match self.value {
            ASTValue::Int(i) => i,
            _ => panic!()
        }
      }

      pub fn floatingPoint(&self) -> f64 {
        match self.value {
            ASTValue::Double(d) => d,
            _ => panic!()
        }
      }

      pub fn string(&self) -> String {
        match &self.value {
            ASTValue::String(s) => s.clone(),
            _ => panic!()
        }
      }

      pub fn line(&self) -> i64 {
          self.line
      }

      pub fn col(&self) -> i64 {
          self.col
      }


    fn expect<S: AsRef<str>>(kind: NodeKind, input: &mut ASTInput, token: S) -> Result<(), ParseError> {
        let tokenStr = token.as_ref();
        let tokenSize = tokenStr.chars().count();
        if (tokenSize > input.size()) {
            let remaining = input.get();
            return Err(ParseError::from(kind, token, remaining, input.line(), input.col()));
        }
        let should_be_token = &input.get()[0..tokenSize];
        if (should_be_token != tokenStr) {
            return Err(ParseError::from(kind, token, should_be_token, input.line(), input.col()));
        }
        input.advance(tokenSize);
        return Ok(())
    }

}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeKind {
    Expression,
    FloatingPoint,
    Identifier,
    Integer,
    Invalid,
    Literal,
    OptionalParameterList,
    ParameterList,
    Program,
    String,
    Whitespace,
}

fn is_whitespace(c: char) -> bool {
    return c == ' ' ||
            c == '\t' ||
            c == '\n' ||
            c == '\r';
}

static mut INDENT_LEVEL: i64 = 0;