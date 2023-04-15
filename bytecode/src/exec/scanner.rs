use crate::repr::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let source = source.chars().collect();

        Scanner {
            source,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();
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

            '"' => return self.string(),
            
            c if c.is_ascii_digit() => return self.number(c),

            c if c.is_ascii_alphabetic() || c == '_' => return self.ident(c),

            _ => return self.error("Unexpected character."),
        };

        self.create_token(kind, c)
    }

    fn ident(&mut self, first: char) -> Token {
        let mut lexeme = String::from(first);

        while let Some(c) = self.peek() {
            if c == '_' || c.is_ascii_alphanumeric() {
                lexeme.push(c);
                self.advance();
            } else {
                break;
            }
        }

        self.create_token(self.ident_type(), lexeme)
    }

    fn ident_type(&self) -> TokenType {
        

        TokenType::Identifier
    }

    fn number(&mut self, first: char) -> Token {
        let mut lexeme = String::from(first);
        macro_rules! digits {
            () => {
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        lexeme.push(self.advance());
                    } else {
                        break;
                    }
                }
            };
        }

        digits!();

        if let Some('.') = self.peek() {
            if let Some(d) = self.peek_next() {
                if d.is_ascii_digit() {
                    lexeme.push(self.advance());
                    digits!();
                }
            }
        }

        self.create_token(TokenType::Number, lexeme)
    }

    fn string(&mut self) -> Token {
        let mut lexeme = String::new();
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            let c = self.advance();
            lexeme.push(c);
        }

        if self.is_at_end() {
            return self.error("Unterminated string.");
        }

        self.advance();
        self.create_token(TokenType::String, lexeme)
    }

    fn match_next(&mut self, c: char, a: TokenType, b: TokenType) -> TokenType {
        if self.peek() == Some(c) {
            self.advance();
            a
        } else {
            b
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '/' => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') {
                            // A commment goes until the end of the line
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }

                _ if c.is_ascii_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                _ => return,
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).cloned()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).cloned()
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
