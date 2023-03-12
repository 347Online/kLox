use crate::{token::*, lox::Lox};

pub struct Scanner {
    // source_string: String,
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof, "", Literal::Empty, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<Vec<Token>, String> {
        let c = self.advance();

        let token = self.create_token(c, self.line)?;
        self.tokens.push(token);

        Ok(self.tokens.clone())
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn advance(&mut self) -> char {
        // let c = self.source.chars().collect::<Vec<char>>()[self.current as usize];
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn advance_if(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false
        }

        if self.source[self.current] != c {
            return false
        }

        self.current += 1;
        true
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

            '!' => if self.advance_if('=') {
                (TokenType::BangEqual, Literal::Empty)
            } else {
                (TokenType::Bang, Literal::Empty)
            }

            '=' => if self.advance_if('=') {
                (TokenType::EqualEqual, Literal::Empty)
            } else {
                (TokenType::Equal, Literal::Empty)
            }

            '<' => if self.advance_if('=') {
                (TokenType::LessEqual, Literal::Empty)
            } else {
                (TokenType::Less, Literal::Empty)
            }

            '>' => if self.advance_if('=') {
                (TokenType::GreaterEqual, Literal::Empty)
            } else {
                (TokenType::Greater, Literal::Empty)
            }

            '/' => if self.advance_if('/') {
                // A comment goes until the end of line
                let mut comment = String::new();
                while self.peek() != Some('\n') && !self.is_at_end() {
                    comment.push(self.advance());
                }

                (TokenType::Comment, Literal::Comment(comment))
            } else {
                (TokenType::Slash, Literal::Empty)
            }

            _ => return Err(Lox::error(line, "Unexpected character")),
        };

        Ok(Token::new(kind, c, literal, line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
