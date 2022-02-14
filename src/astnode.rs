use std::fmt::Display;

use crate::astinput::ASTInput;
use crate::errors::ParseError;

#[derive(Clone, Debug, PartialEq)]
enum ASTValue {
    Int(i64),
    Double(f64),
    String(String),
    Children(Vec<ASTNode>),
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ASTNode {
    kind: NodeKind,
    value: ASTValue,

    line: i64,
    col: i64,
}

impl ASTNode {
    pub fn parse_program(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::Program;
        let line = input.line();
        let col = input.col();
        let mut children = Vec::new();
        match ASTNode::parse_expression(input) {
            Ok(v) => children.push(v),
            Err(e) => return Err(e),
        }
        if input.size() > 0 {
            match ASTNode::parse_program(input) {
                Ok(v) => children.push(v),
                Err(e) => return Err(e),
            }
        }
        Ok(ASTNode {
            kind,
            value: ASTValue::Children(children),
            line,
            col,
        })
    }

    pub fn parse_expression(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::Expression;

        match ASTNode::parse_optional_whitespace(input) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        if input.size() == 0 {
            return Err(ParseError::from(
                kind,
                "(",
                "EOF",
                input.line(),
                input.col(),
            ));
        }

        let next = input.peek();
        let ret = if next == '(' {
            let mut children = Vec::new();
            match ASTNode::expect(kind, input, "(") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            match ASTNode::parse_expression(input) {
                Ok(v) => children.push(v),
                Err(e) => return Err(e),
            }
            match ASTNode::parse_optional_parameter_list(input) {
                Ok(v) => children.push(v),
                Err(e) => return Err(e),
            }
            match ASTNode::parse_optional_whitespace(input) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            match ASTNode::expect(kind, input, ")") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            Ok(ASTNode {
                kind,
                value: ASTValue::Children(children),
                line: input.line(),
                col: input.col(),
            })
        } else if next == '\'' || next.is_digit(10) {
            ASTNode::parse_literal(input)
        } else {
            ASTNode::parse_identifier(input)
        };
        match ASTNode::parse_optional_whitespace(input) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        ret
    }
    pub fn parse_optional_parameter_list(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::OptionalParameterList;
        let value = if input.peek() != ')' {
            let mut parameter_list = ASTNode {
                kind: NodeKind::ParameterList,
                value: ASTValue::Children(Vec::new()),
                line: input.line(),
                col: input.col(),
            };
            match parameter_list.parse_parameter_list(input) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            vec![parameter_list]
        } else {
            Vec::new()
        };
        Ok(ASTNode {
            kind,
            value: ASTValue::Children(value),
            line: input.line(),
            col: input.col(),
        })
    }
    pub fn parse_parameter_list(&mut self, input: &mut ASTInput) -> Result<(), ParseError> {
        let mut new_children = Vec::new();
        match ASTNode::parse_expression(input) {
            Ok(v) => new_children.push(v),
            Err(e) => return Err(e),
        }
        match &mut self.value {
            ASTValue::Children(children) => children.extend(new_children),
            _ => panic!(),
        }

        match ASTNode::parse_optional_whitespace(input) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        if input.size() == 0 {
            return Err(ParseError::from(
                NodeKind::ParameterList,
                "end of expression ')'",
                "EOF",
                input.line(),
                input.col(),
            ));
        }
        if input.peek() == ')' {
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
        if input.size() == 0 {
            return Err(ParseError::from(
                kind,
                "any valid literal",
                "EOF",
                input.line(),
                input.col(),
            ));
        }
        let result = if input.peek() == '\'' {
            // string
            ASTNode::parse_string(input)
        } else {
            ASTNode::parse_number(input)
        };
        let value = match result {
            Err(e) => return Err(e),
            _ => vec![result.unwrap()],
        };
        Ok(ASTNode {
            kind,
            value: ASTValue::Children(value),
            line,
            col,
        })
    }
    pub fn parse_number(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let mut kind = NodeKind::Integer;

        // read digits until whitespace or decimal
        let mut ss = String::new();
        let mut is_float = false;
        let input_len = input.size();
        for i in 0..input_len {
            let next = input.peek();
            if i > 0 && next == '.' {
                ss.push(next);
                is_float = true;
            } else {
                if is_whitespace(next) || next == ')' {
                    // number is done
                    break;
                }
                if !next.is_digit(10) {
                    return Err(ParseError::from(
                        kind,
                        "digits 0 through 9",
                        next.to_string(),
                        input.line(),
                        input.col(),
                    ));
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
        if is_float {
            kind = NodeKind::FloatingPoint;
            match result.parse::<f64>() {
                Ok(value) => Ok(ASTNode {
                    kind,
                    value: ASTValue::Double(value),
                    line,
                    col,
                }),
                Err(_e) => panic!(),
            }
        } else {
            kind = NodeKind::Integer;
            match result.parse::<i64>() {
                Ok(value) => Ok(ASTNode {
                    kind,
                    value: ASTValue::Int(value),
                    line,
                    col,
                }),
                Err(_e) => panic!(),
            }
        }
    }
    pub fn parse_optional_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {
        if input.size() == 0 {
            return Ok(());
        }
        if !is_whitespace(input.peek()) {
            return Ok(());
        }
        ASTNode::parse_whitespace(input)
    }
    pub fn parse_whitespace(input: &mut ASTInput) -> Result<(), ParseError> {
        let mut found_whitespace = false;
        while input.size() != 0 {
            let next = input.peek();
            if is_whitespace(next) {
                found_whitespace = true;
            } else if !found_whitespace {
                return Err(ParseError::from(
                    NodeKind::Whitespace,
                    "Any of: ' ', \\r, \\n, \\t",
                    next.to_string(),
                    input.line(),
                    input.col(),
                ));
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
        if input.size() == 0 {
            return Err(ParseError::from(
                kind,
                "any valid identifier",
                "EOF",
                input.line(),
                input.col(),
            ));
        }
        let mut next = input.peek();
        if next == '\'' {
            return Err(ParseError::from(
                kind,
                "non-whitespace character other than '\\'', '(' and ')'",
                next.to_string(),
                input.line(),
                input.col(),
            ));
        }
        let mut ss = String::new();
        while input.size() != 0 {
            next = input.peek();
            if is_whitespace(next) || next == ')' {
                break;
            }
            if next == '(' {
                return Err(ParseError::from(
                    kind,
                    "non-whitespace character other than '('",
                    next.to_string(),
                    line,
                    col,
                ));
            }
            ss.push(next);
            input.next();
        }
        assert!(ss.chars().count() != 0);
        Ok(ASTNode {
            kind,
            value: ASTValue::String(ss),
            line,
            col,
        })
    }

    pub fn parse_string(input: &mut ASTInput) -> Result<ASTNode, ParseError> {
        let kind = NodeKind::String;
        let line = input.line();
        let col = input.col();
        let mut ss = String::new();
        let mut found_end_of_string = false;
        match ASTNode::expect(kind, input, "'") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        while input.size() != 0 {
            let mut next = input.peek();
            if next == '\'' {
                found_end_of_string = true;
                break;
            }
            if next == '\\' {
                // escape char
                if input.size() < 2 {
                    return Err(ParseError::from(
                        kind,
                        "any character followed by escape sequence (\\)",
                        "EOF",
                        input.line(),
                        input.col(),
                    ));
                }
                ss.push(input.next());
                next = input.peek();
            }

            ss.push(next);
            input.next();
        }
        if !found_end_of_string {
            return Err(ParseError::from(
                kind,
                "end of string marker (')",
                "EOF",
                input.line(),
                input.col(),
            ));
        }

        match ASTNode::expect(kind, input, "'") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(ASTNode {
            kind,
            value: ASTValue::String(ss),
            line,
            col,
        })
    }

    // produces invalid node! should only use for testing
    pub fn new() -> ASTNode {
        ASTNode {
            kind: NodeKind::Invalid,
            value: ASTValue::Invalid,
            line: 0,
            col: 0,
        }
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn children(&self) -> &Vec<ASTNode> {
        match &self.value {
            ASTValue::Children(children) => children,
            _ => panic!(),
        }
    }

    pub fn integer(&self) -> i64 {
        match self.value {
            ASTValue::Int(i) => i,
            _ => panic!(),
        }
    }

    pub fn floating_point(&self) -> f64 {
        match self.value {
            ASTValue::Double(d) => d,
            _ => panic!(),
        }
    }

    pub fn string(&self) -> String {
        match &self.value {
            ASTValue::String(s) => s.clone(),
            _ => panic!(),
        }
    }

    pub fn line(&self) -> i64 {
        self.line
    }

    pub fn col(&self) -> i64 {
        self.col
    }

    fn expect<S: AsRef<str>>(
        kind: NodeKind,
        input: &mut ASTInput,
        token: S,
    ) -> Result<(), ParseError> {
        let token_str = token.as_ref();
        let token_size = token_str.chars().count();
        if token_size > input.size() {
            let remaining = input.get();
            return Err(ParseError::from(
                kind,
                token,
                remaining,
                input.line(),
                input.col(),
            ));
        }
        let should_be_token = &input.get()[0..token_size];
        if should_be_token != token_str {
            return Err(ParseError::from(
                kind,
                token,
                should_be_token,
                input.line(),
                input.col(),
            ));
        }
        input.advance(token_size);
        Ok(())
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ss = String::new();
        // unsafe because of static mutable INDENT_LEVEL
        unsafe {
            assert!(INDENT_LEVEL >= 0);
            for _i in 0..INDENT_LEVEL {
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
        write!(f, "{}", ss)
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
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

static mut INDENT_LEVEL: i64 = 0;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn whitespace_variations(input: Vec<String>) -> Vec<String> {
        // inserts whitespace before, after, and before/after
        let mut ret = Vec::new();
        for str in input {
            ret.push(str.clone());
            ret.push(format!("  {}", str.clone()));
            ret.push(format!("{}  ", str.clone()));
            ret.push(format!(" {} ", str));
        }
        ret
    }
    fn identifiers() -> Vec<String> {
        vec!["identifier", "foo", "@#*&%&$#", "...", "💩"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn strings() -> Vec<String> {
        vec!["'foo bar'", "'&\"\\'+~💩$'"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn integers() -> Vec<String> {
        vec!["0", "1", "928453821"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn floats() -> Vec<String> {
        vec!["0.23592", "29384."]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn literals() -> Vec<String> {
        let mut ret = Vec::new();
        ret.extend(strings());
        ret.extend(integers());
        ret.extend(floats());
        ret
    }

    fn expressions() -> Vec<String> {
        let mut ret: Vec<String> = vec!["( + 3 4)", " (  +  3  4  ) ", "(- 1.5)", "(* (- 1.5) 2)"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        ret.extend(whitespace_variations(identifiers()));
        ret.extend(whitespace_variations(literals()));
        ret
    }

    fn programs() -> Vec<String> {
        let mut ret = expressions();
        for i in 0..expressions().len() - 1 {
            let item1 = ret[i].clone();
            let item2 = ret[i + 1].clone();
            let combined = item1 + " " + &item2;
            ret.push(combined);
        }
        ret
    }

    #[test]
    fn test_parse_program() {
        for str in programs() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_program(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::Program),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_expression() {
        for str in expressions() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_expression(&mut input);
            match result {
                // resulting NodeKind could be any of:
                // * expression
                // * literal
                // * identifier
                // due to simplified parse tree structure (useless expressions are skipped).
                Ok(node) => assert!(
                    node.kind == NodeKind::Expression
                        || node.kind == NodeKind::Literal
                        || node.kind == NodeKind::Identifier
                ),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_identifier() {
        for str in identifiers() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_identifier(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::Identifier),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_literal() {
        for str in literals() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_literal(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::Literal),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_string() {
        for str in strings() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_string(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::String),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_number() {
        for str in floats() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_number(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::FloatingPoint),
                Err(_) => assert!(false),
            }
        }
        for str in integers() {
            let mut input = ASTInput::from_str(&str);
            let result = ASTNode::parse_number(&mut input);
            match result {
                Ok(node) => assert_eq!(node.kind(), NodeKind::Integer),
                Err(_) => assert!(false),
            }
        }
    }
}
