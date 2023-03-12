use crate::{token::*, lox::Lox};

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

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof, "", Literal::Empty, self.line));
        Ok(())
    }

    fn scan_token(&mut self) -> Result<Vec<Token>, String> {
        let c = self.advance();

        let token = self.create_token(c, self.line)?;
        self.tokens.push(token);

        Ok(self.tokens.clone())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().collect::<Vec<char>>()[self.current as usize];
        self.current += 1;
        c
    }

    fn create_token(&mut self, c: char, line: i32) -> Result<Token, String> {
        let (kind, literal) = match c {
            '(' => (TokenType::LeftParen, Literal::Empty),
            ')' => (TokenType::RightParen, Literal::Empty),
            '{' => (TokenType::LeftBrace, Literal::Empty),
            '}' => (TokenType::RightBrace, Literal::Empty),
            ',' => (TokenType::Comma, Literal::Empty),
            '.' => (TokenType::Dot, Literal::Empty),
            '-' => (TokenType::Minus, Literal::Empty),
            '+' => (TokenType::Plus, Literal::Empty),
            ';' => (TokenType::Semicolon, Literal::Empty),
            '*' => (TokenType::Star, Literal::Empty),

            _ => return Err(Lox::error(line, "Unexpected character")),
        };

        Ok(Token::new(kind, c, literal, line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}
