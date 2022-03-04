#[derive(Debug)]
pub struct ASTInput {
    current: String,
    line: i64,
    col: i64,
}

impl ASTInput {
    pub fn advance(&mut self, by: usize) {
        assert!(self.size() >= by);
        for _ in 0..by {
            // TODO: refactor to minimize use of O(n) remove call
            let next = self.current.remove(0);
            if next == '\n' {
                self.col = 0;
                self.line += 1;
            } else {
                self.col += 1;
            }
        }
    }

    pub fn get(&self) -> &str {
        &self.current
    }

    pub fn next_char(&mut self) -> char {
        let ret = self.peek();
        self.advance(1);
        ret
    }

    pub fn peek(&mut self) -> char {
        self.current.chars().next().unwrap_or('\0')
    }

    pub fn size(&self) -> usize {
        self.current.chars().count()
    }

    pub fn line(&self) -> i64 {
        self.line
    }

    pub fn col(&self) -> i64 {
        self.col
    }
}

impl From<String> for ASTInput {
    fn from(s: String) -> Self {
        ASTInput {
            current: s,
            line: 1,
            col: 1,
        }
    }
}

impl From<&String> for ASTInput {
    fn from(s: &String) -> Self {
        ASTInput {
            current: s.clone(),
            line: 1,
            col: 1,
        }
    }
}

impl From<&str> for ASTInput {
    fn from(s: &str) -> Self {
        ASTInput {
            current: s.to_string(),
            line: 1,
            col: 1,
        }
    }
}
