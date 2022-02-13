use std::str::Chars;

#[derive(Debug)]
pub struct ASTInput<'a> {
    input: &'a String,
    current: Chars<'a>,
    line: i64,
    col: i64,
}

impl ASTInput<'_> {

      pub fn from_str<'a>(input: &'a String) -> ASTInput<'a> {
        ASTInput {
            input: input,
            current: input.chars(),
            line: 1,
            col: 1
        }
      }

      pub fn advance(&mut self, by: usize) {
          assert!(self.size() >= by);
          for i in 0..by {
              let next = self.current.next().unwrap();
              if next == '\n' {
                  self.col = 0;
                  self.line += 1;
              } else {
                  self.col += 1;
              }
          }
      }

      pub fn get(&self) -> &Chars {
          &self.current
      }

      pub fn next(&mut self) -> char {
          let ret = self.peek();
          self.advance(1);
          ret
      }

      pub fn peek(&self) -> char {
          let current = &self.current;
          let mut peekable = current.peekable();
          let result = peekable.peek();
          match result {
              Some(char) => *char,
              None => '\0',
          }
      }

      pub fn remaining(&self) -> &str {
          self.current.as_str()
      }
     
      pub fn size(&self) -> usize {
          self.current.count()
      }
      
      pub fn line(&self) -> i64 {
            self.line 
      }

      pub fn col(&self) -> i64 {
          self.col
      }
}