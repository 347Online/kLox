use crate::token::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", Literal {}, self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {

        }
    }

    fn advance(&mut self) -> char {

    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}
