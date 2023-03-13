use crate::{
    expr::Expr,
    lox::Lox,
    token::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.advance_if(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = Box::new(self.comparison());
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.advance_if(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = Box::new(self.term());
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.advance_if(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous()?;
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.advance_if(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous()?;
            let right = Box::new(self.unary()?);
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.advance_if(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous()?;
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary { operator, right });
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.advance_if(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.advance_if(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.advance_if(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.advance_if(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal()));
        }

        if self.advance_if(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                String::from("Expect ')' after expression."),
            ).unwrap();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        // panic!()
        Err(Parser::error(self.peek(), String::from("Expect expression")))
    }

    fn consume(&mut self, kind: TokenType, message: String) -> Result<Token, String> {
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

    fn previous(&self) -> Result<Token, String> {
        self.tokens[self.current - 1].clone()
    }

    fn error(token: &Token, message: String) -> String {
        Lox::error_token(token, message)
        // Consider refactoring to return some custom error structure
        // For now continuing to return a string
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
