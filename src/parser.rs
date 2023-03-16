use crate::{
    expr::{Expr},
    lox::{Lox, LoxError},
    token::{BinOp, BinOpType, Token, TokenType, UnOp, UnOpType, Value}, stmt::Stmt,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.advance_if(vec![TokenType::Print]) {
            return self.print_statement()
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.advance_if(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::BangEqual => BinOp::new(BinOpType::NotEqual, token),
                TokenType::EqualEqual => BinOp::new(BinOpType::Equal, token),

                _ => unreachable!(),
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
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Greater => BinOp::new(BinOpType::Greater, token),
                TokenType::GreaterEqual => BinOp::new(BinOpType::GreaterEqual, token),
                TokenType::Less => BinOp::new(BinOpType::Less, token),
                TokenType::LessEqual => BinOp::new(BinOpType::LessEqual, token),

                _ => unreachable!(),
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
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Minus => BinOp::new(BinOpType::Subtract, token),
                TokenType::Plus => BinOp::new(BinOpType::Add, token),

                _ => unreachable!(),
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
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Slash => BinOp::new(BinOpType::Divide, token),
                TokenType::Star => BinOp::new(BinOpType::Multiply, token),

                _ => unreachable!(),
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
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Bang => UnOp::new(UnOpType::Not, token),
                TokenType::Minus => UnOp::new(UnOpType::Negative, token),

                _ => unreachable!(),
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
            self.consume(TokenType::RightParen, "Expect ')' after expression.")
                .unwrap();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(Lox::syntax_error(self.peek(), "Expect expression"))
    }

    fn consume<S: Into<String>>(&mut self, kind: TokenType, message: S) -> Result<Token, LoxError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(Lox::syntax_error(self.peek(), message))
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

    #[allow(dead_code)] // TODO: Remove this
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
