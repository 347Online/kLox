use crate::{
    expr::Expr,
    lox::{Lox, LoxError},
    token::{Value, Token, TokenType, BinOp, UnOp},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.advance_if(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = match self.previous().kind() {
                TokenType::BangEqual => BinOp::NotEqual,
                TokenType::EqualEqual => BinOp::Equal,

                _ => unreachable!()
            };
            let right = Box::new(self.comparison()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.advance_if(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = match self.previous().kind() {
                TokenType::Greater => BinOp::Greater,
                TokenType::GreaterEqual => BinOp::GreaterEqual,
                TokenType::Less => BinOp::Less,
                TokenType::LessEqual => BinOp::LessEqual,

                _ => unreachable!()
            };
            let right = Box::new(self.term()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.advance_if(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = match self.previous().kind() {
                TokenType::Minus => BinOp::Subtract,
                TokenType::Plus => BinOp::Add,

                _ => unreachable!()
            };
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.advance_if(vec![TokenType::Slash, TokenType::Star]) {
            let operator = match self.previous().kind() {
                TokenType::Slash => BinOp::Divide,
                TokenType::Star => BinOp::Multiply,

                _ => unreachable!()
            };
            let right = Box::new(self.unary()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.advance_if(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = match self.previous().kind() {
                TokenType::Bang => UnOp::Not,
                TokenType::Minus => UnOp::Negative,

                _ => unreachable!()
            };
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary { operator, right });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.advance_if(vec![TokenType::False]) {
            return Ok(Expr::Literal(Value::Bool(false)));
        }
        if self.advance_if(vec![TokenType::True]) {
            return Ok(Expr::Literal(Value::Bool(true)));
        }
        if self.advance_if(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Value::Nil));
        }

        if self.advance_if(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal()));
        }

        if self.advance_if(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.",
            )
            .unwrap();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(Parser::error(
            self.peek(),
            "Expect expression",
        ))
    }

    fn consume<S: Into<String>>(&mut self, kind: TokenType, message: S) -> Result<Token, LoxError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(Parser::error(self.peek(), message))
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.is_at_end() && self.peek().is(kind)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn advance_if(&mut self, kinds: Vec<TokenType>) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error<S: Into<String>>(token: &Token, message: S) -> LoxError {
        Lox::error_token(token, message)
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }

            match self.peek().kind() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,

                _ => (),
            }

            self.advance();
        }
    }
}
