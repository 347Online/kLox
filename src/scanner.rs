use crate::{lox::Lox, token::*};

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

        self.tokens
            .push(Token::new(TokenType::Eof, "", Literal::Empty, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<Vec<Token>, String> {
        let c = self.advance();

        self.create_token(c, self.line)?;

        Ok(self.tokens.clone())
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
        // let c = self.source.chars().collect::<Vec<char>>()[self.current as usize];
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn advance_if(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Result<(), String> {
        let mut string_value = String::new();

        while self.peek() != Some('"') && !self.is_at_end() {
            if let Some('\n') = self.peek() {
                self.line += 1;
            }
            string_value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(Lox::error(self.line, "Unterminated string"));
        }

        // The closing "
        self.advance();

        let token = Token::new(
            TokenType::String,
            string_value.clone(),
            Literal::String(string_value),
            self.line,
        );
        self.tokens.push(token);

        Ok(())
    }

    fn number(&mut self, first_digit: char) -> Result<(), String> {
        let mut number_string = String::from(first_digit);

        macro_rules! add_digits {
            () => {
                while self.peek().is_some() && self.peek().unwrap().is_ascii_digit() {
                    match self.advance() {
                        c if c.is_ascii_digit() => {
                            number_string.push(c);
                            continue;
                        }

                        _ => break,
                    }
                }
            };
        }

        add_digits!();

        dbg!(self.peek(), self.peek_next());

        if self.peek().is_some() && self.peek().unwrap() == '.' && self.peek_next().is_some() && self.peek_next().unwrap().is_ascii_digit() {
            number_string.push(self.advance());
            add_digits!()
        }

        let value: f64 = number_string.parse().expect("This should always succeed, as we have rigorously checked for number characters before adding them to number_string");
        let token = Token::new(
            TokenType::Number,
            number_string,
            Literal::Number(value),
            self.line,
        );
        self.tokens.push(token);

        Ok(())
    }

    fn identifier(&mut self, first: char) -> Result<(), String> {
        let mut ident_string = String::from(first);

        while self.peek().is_some() && self.peek().unwrap().is_ascii_alphanumeric() || self.peek().unwrap() == '_' {
            ident_string.push(self.advance());
        }

        match ident_string.as_str() {
            "or" => {
                let token = Token::new(
                    TokenType::Or,
                    ident_string.clone(),
                    Literal::Keyword(ident_string),
                    self.line,
                );
            }

            _ => {
                let token = Token::new(
                    TokenType::Identifier,
                    ident_string.clone(),
                    Literal::Identifier { name: ident_string },
                    self.line,
                );
                self.tokens.push(token);
            }
        }

        Ok(())
    }

    fn create_token(&mut self, c: char, line: i32) -> Result<(), String> {
        let literal = Literal::Empty;

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
                if self.advance_if('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }

            '=' => {
                if self.advance_if('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }

            '<' => {
                if self.advance_if('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }

            '>' => {
                if self.advance_if('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }

            '/' => {
                if self.advance_if('/') {
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

            _ => return Err(Lox::error(line, "Unexpected character")),
        };

        let token = Token::new(kind, c, literal, line);
        self.tokens.push(token);

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
