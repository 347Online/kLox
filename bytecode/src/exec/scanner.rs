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
        self.skip_whitespace();
        if self.at_end() {
            return self.finish()
        }

        let c = self.advance();

        use TokenType::*;
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

        self.create(kind, c)
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

    fn match_next(&mut self, expected: char, a: TokenType, b: TokenType) -> TokenType {
        if self.peek() == Some(expected) && !self.at_end() {
            self.current += 1;
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
                            // A comment goes until the end of the line
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

    fn string(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }
            
            let c = self.advance();
            lexeme.push(c);

        }

        if self.at_end() {
            return self.error("Unterminated string.");
        }

        self.advance();
        self.create(TokenType::String, lexeme)
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

        self.create(TokenType::Number, lexeme)
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

        let ident = self.ident_type(&lexeme);
        self.create(ident, lexeme)
    }

    fn ident_type(&self, lexeme: &str) -> TokenType {
        use TokenType::*;

        match lexeme {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "fun" => Fun,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,

            _ => Identifier,
        }
    }

    fn create<S: Into<String>>(&self, kind: TokenType, lexeme: S) -> Token {
        Token::new(kind, lexeme.into(), self.line)
    }

    fn finish(&self) -> Token {
        self.create(TokenType::Eof, "")
    }

    fn error(&self, message: &str) -> Token {
        self.create(TokenType::Error, message)
    }

    fn at_end(&self) -> bool {
        self.peek().is_none()
    }
}