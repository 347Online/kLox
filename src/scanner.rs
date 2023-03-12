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

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        let token = self.create_token(c, self.line);
    }

    fn advance(&mut self) -> char {

    }

    fn create_token(&mut self, c: char, line: i32) -> Token {
        let (kind, literal) = match c {
            '(' => (TokenType::LeftParen, None),
            ')' => (TokenType::RightParen, None),
            '{' => (TokenType::LeftBrace, None),
            '}' => (TokenType::RightBrace, None),
            ',' => (TokenType::Comma, None),
            '.' => (TokenType::Dot, None),
            '-' => (TokenType::Minus, None),
            '+' => (TokenType::Plus, None),
            ';' => (TokenType::Semicolon, None),
            '*' => (TokenType::Star, None),

            _ => panic!(),
        };

        Token::new(kind, c, literal, line)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}
