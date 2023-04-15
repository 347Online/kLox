use crate::repr::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let source = source.chars().collect();

        Scanner {
            source,
            start: 0,
            current: 1,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.create_token(Eof, "");
        }

        let c = self.advance();

        let kind = match c {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            '*' => Star,
            '/' => Slash,

            '!' => self.match_next('=', BangEqual, Bang),
            '=' => self.match_next('=', EqualEqual, Equal),
            '<' => self.match_next('=', LessEqual, Less),
            '>' => self.match_next('=', GreaterEqual, Greater),

            _ => return self.error("Unexpected character."),
        };

        self.create_token(kind, c)
    }

    fn match_next(&mut self, c: char, a: TokenType, b: TokenType) -> TokenType {
        if self.peek() == Some(&c) {
            self.advance();
            a
        } else {
            b
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                if c == &'\n' {
                    self.line += 1;
                }

                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&char> {
        self.source.get(self.current + 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn error(&self, message: &str) -> Token {
        self.create_token(TokenType::Error, message)
    }

    fn create_token<S: Into<String>>(&self, kind: TokenType, lexeme: S) -> Token {
        let lexeme = lexeme.into();
        Token::new(kind, lexeme, self.line)
    }
}
