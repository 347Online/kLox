use crate::{
    error::LoxError,
    expr::{Expr, ExprType},
    lox::Lox,
    operator::{BinOp, BinOpType, LogOp, LogOpType, UnOp, UnOpType},
    stmt::Stmt,
    token::{Token, TokenType},
    value::Value,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    eof: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            eof: Token::new(TokenType::Eof, "", Value::Nil, -1),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }

    fn declaration(&mut self) -> Stmt {
        let result = match self.peek().kind() {
            TokenType::Fun => {
                self.advance();
                self.function("function")
            }

            TokenType::Var => {
                self.advance();
                self.var_declaration()
            }

            _ => self.statement(),
        };

        match result {
            Ok(stmt) => stmt,
            Err(_) => {
                self.sync();
                Stmt::Empty
            }
        }
    }

    fn function<S: Into<String>>(&mut self, kind: S) -> Result<Stmt, LoxError> {
        let kind = kind.into();
        let name = self.consume(TokenType::Identifier, format!("Expect {} name.", kind))?;

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= Lox::MAX_ARGS {
                    LoxError::runtime(
                        self.peek(),
                        format!("Can't have more than {} parameters.", Lox::MAX_ARGS),
                    );
                }

                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if let TokenType::Comma = self.peek().kind() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;
        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body", kind),
        )?;

        let body = self.block()?;
        Ok(Stmt::Function(name, parameters, body))
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        let token = self.peek();

        match token.kind() {
            TokenType::For => {
                self.advance();
                self.for_statement()
            }

            TokenType::If => {
                self.advance();
                self.if_statement()
            }

            TokenType::Print => {
                self.advance();
                self.print_statement()
            }

            TokenType::Return => {
                self.advance();
                self.return_statement()
            }

            TokenType::While => {
                self.advance();
                self.while_statment()
            }

            TokenType::LeftBrace => {
                self.advance();
                self.block_statement()
            }

            _ => self.expression_statement(),
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous();
        let mut expr = Expr::new();
        if !self.check(TokenType::Semicolon) {
            expr = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(keyword, expr))
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        // Initializer
        let initializer = match self.peek().kind() {
            TokenType::Semicolon => {
                self.advance();
                Stmt::Empty
            }

            TokenType::Var => {
                self.advance();
                self.var_declaration()?
            }

            _ => self.expression_statement()?,
        };

        // Condition
        let mut condition = if self.check(TokenType::Semicolon) {
            Expr::new()
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon, "")?;

        // Increment
        let increment = if self.check(TokenType::RightParen) {
            Expr::new()
        } else {
            self.expression()?
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let ExprType::Empty = increment.kind() {
        } else {
            body = Stmt::Block(vec![body, Stmt::Expr(increment)]);
        }

        if let ExprType::Empty = condition.kind() {
            condition = Expr::literal(Value::Bool(true));
        }

        body = Stmt::While(condition, Box::new(body));

        if let Stmt::Empty = initializer {
        } else {
            body = Stmt::Block(vec![initializer, body]);
        }

        Ok(body)
    }

    fn while_statment(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        if let TokenType::Else = self.peek().kind() {
            self.advance();
            let else_branch = self.statement()?;
            return Ok(Stmt::IfElse(
                condition,
                Box::new(then_branch),
                Box::new(else_branch),
            ));
        }

        Ok(Stmt::If(condition, Box::new(then_branch)))
    }

    fn block_statement(&mut self) -> Result<Stmt, LoxError> {
        Ok(Stmt::Block(self.block()?))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration())
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if let TokenType::Equal = self.peek().kind() {
            self.advance();
            self.expression()?
        } else {
            Expr::new()
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;

        Ok(Stmt::Var(name, initializer))
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if let TokenType::Equal = self.peek().kind() {
            self.advance();
            let equals = self.previous();
            let value = self.assignment()?;

            if let ExprType::Variable(name) = expr.kind() {
                return Ok(Expr::assign(name.clone(), value));
            }

            LoxError::syntax(&equals, "Invalid assignment target.");
            return Ok(Expr::new());
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while let TokenType::Or = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = LogOp::new(LogOpType::Or, token);
            let right = self.and()?;
            expr = Expr::logical(operator, expr, right);
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while let TokenType::And = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = LogOp::new(LogOpType::And, token);
            let right = self.equality()?;
            expr = Expr::logical(operator, expr, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while let TokenType::BangEqual | TokenType::EqualEqual = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::BangEqual => BinOp::new(BinOpType::NotEqual, token),
                TokenType::EqualEqual => BinOp::new(BinOpType::Equal, token),

                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = self.peek().kind()
        {
            self.advance();
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Greater => BinOp::new(BinOpType::Greater, token),
                TokenType::GreaterEqual => BinOp::new(BinOpType::GreaterEqual, token),
                TokenType::Less => BinOp::new(BinOpType::Less, token),
                TokenType::LessEqual => BinOp::new(BinOpType::LessEqual, token),

                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while let TokenType::Minus | TokenType::Plus = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Minus => BinOp::new(BinOpType::Subtract, token),
                TokenType::Plus => BinOp::new(BinOpType::Add, token),

                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while let TokenType::Slash | TokenType::Star = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Slash => BinOp::new(BinOpType::Divide, token),
                TokenType::Star => BinOp::new(BinOpType::Multiply, token),

                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if let TokenType::Bang | TokenType::Minus = self.peek().kind() {
            self.advance();
            let token = self.previous();
            let operator = match token.kind() {
                TokenType::Bang => UnOp::new(UnOpType::Not, token),
                TokenType::Minus => UnOp::new(UnOpType::Negative, token),

                _ => unreachable!(),
            };
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        while let TokenType::LeftParen = self.peek().kind() {
            self.advance();
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= Lox::MAX_ARGS {
                    LoxError::runtime(
                        self.peek(),
                        format!("Can't have more than {} arguments.", Lox::MAX_ARGS),
                    );
                }

                arguments.push(self.expression()?);

                if let TokenType::Comma = self.peek().kind() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::call(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let token = self.peek();

        let expr = match token.kind() {
            TokenType::False => Expr::literal(Value::Bool(false)),
            TokenType::True => Expr::literal(Value::Bool(true)),
            TokenType::Nil => Expr::literal(Value::Nil),
            TokenType::Number | TokenType::String => Expr::literal(token.literal()),
            TokenType::Identifier => Expr::variable(token.clone()),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Expr::grouping(expr)
            }

            _ => return Err(LoxError::syntax(token, "Expect Expression")),
        };

        self.advance();
        Ok(expr)
    }

    fn consume<S: Into<String>>(&mut self, kind: TokenType, message: S) -> Result<Token, LoxError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(LoxError::syntax(self.peek(), message))
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.is_at_end() && self.peek().kind() == kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind(), TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &self.eof
        } else {
            &self.tokens[self.current]
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let TokenType::Semicolon = self.previous().kind() {
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
