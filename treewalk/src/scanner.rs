use crate::{
    error::{LoxError, LoxErrorType},
    token::*,
    value::Value,
};

pub struct Scanner {
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();

            if self.create_token(c, self.line).is_err() {
                break;
            }
        }

        let token = Token::new(TokenType::Eof, "", Value::Nil, self.line);
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn string(&mut self) -> Result<(), LoxError> {
        let mut string_value = String::new();

        while self.peek() != Some('"') && !self.is_at_end() {
            if let Some('\n') = self.peek() {
                self.line += 1;
            }
            string_value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string",
                LoxErrorType::SyntaxError,
            ));
        }

        // The closing "
        self.advance();

        let token = Token::new(
            TokenType::String,
            string_value.clone(),
            Value::String(string_value),
            self.line,
        );
        self.tokens.push(token);

        Ok(())
    }

    fn number(&mut self, first_digit: char) -> Result<(), LoxError> {
        let mut number_string = String::from(first_digit);

        macro_rules! add_digits {
            () => {
                while let Some(c) = self.peek() {
                    match c {
                        c if c.is_ascii_digit() => {
                            self.advance();
                            number_string.push(c);
                            continue;
                        }

                        _ => break,
                    }
                }
            };
        }

        add_digits!();

        if self.peek().is_some() {
            if let Some(c) = self.peek_next() {
                if c.is_ascii_digit() {
                    number_string.push(self.advance());
                    add_digits!()
                }
            }
        }

        let value: f64 = number_string.parse().expect("This should always succeed, as we have rigorously checked for number characters before adding them to number_string");
        let token = Token::new(
            TokenType::Number,
            number_string,
            Value::Number(value),
            self.line,
        );
        self.tokens.push(token);

        Ok(())
    }

    fn identifier(&mut self, first: char) -> Result<(), LoxError> {
        fn keyword_filter(name: &str) -> TokenType {
            match name {
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                "if" => TokenType::If,
                "nil" => TokenType::Nil,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "true" => TokenType::True,
                "var" => TokenType::Var,
                "while" => TokenType::While,

                _ => TokenType::Identifier,
            }
        }

        let mut ident_string = String::from(first);

        while let Some(c) = self.peek() {
            if c == '_' || c.is_alphanumeric() {
                ident_string.push(self.advance());
            } else {
                break;
            }
        }

        let kind = keyword_filter(&ident_string);

        let token = Token::new(
            kind,
            ident_string.clone(),
            Value::Identifier { name: ident_string },
            self.line,
        );
        self.tokens.push(token);

        Ok(())
    }

    fn create_token(&mut self, c: char, line: i32) -> Result<(), LoxError> {
        let literal = Value::Nil;

        let kind = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,

            '!' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }

            '=' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }

            '<' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }

            '>' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }

            '/' => {
                if let Some('/') = self.peek() {
                    self.advance();
                    // A comment goes until the end of line
                    let mut comment = String::new();
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        comment.push(self.advance());
                    }

                    return Ok(());
                } else {
                    TokenType::Slash
                }
            }

            '\n' => {
                self.line += 1;
                return Ok(());
            }

            _ if c.is_ascii_whitespace() => return Ok(()),

            '"' => {
                self.string()?;
                return Ok(());
            }

            digit if c.is_ascii_digit() => {
                self.number(digit)?;
                return Ok(());
            }

            ident_char if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier(ident_char)?;
                return Ok(());
            }

            _ => {
                return Err(LoxError::error(
                    line,
                    "Unexpected character",
                    LoxErrorType::SyntaxError,
                ))
            }
        };

        let token = Token::new(kind, c, literal, line);
        self.tokens.push(token);

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
