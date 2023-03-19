use crate::{
    error::LoxError,
    expr::Expr,
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
        let result = 'block: {
            if let TokenType::Var = self.peek().kind() {
                self.advance();
                break 'block self.var_declaration();
            }

            self.statement()
        };

        match result {
            Ok(stmt) => stmt,
            Err(_) => {
                self.sync();
                Stmt::Empty
            }
        }
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
            Expr::Empty
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon, "")?;

        // Increment
        let increment = if self.check(TokenType::RightParen) {
            Expr::Empty
        } else {
            self.expression()?
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Expr::Empty = increment {
        } else {
            body = Stmt::Block(vec![body, Stmt::Expr(increment)]);
        }

        if let Expr::Empty = condition {
            condition = Expr::Literal(Value::Bool(true));
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
            Expr::Empty
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

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            // "We report an error if the left-hand side isn’t a valid assignment target,
            // but we don’t throw it because the parser isn’t in a confused state where
            // we need to go into panic mode and synchronize."
            // May need to handle this differently
            Lox::syntax_error(&equals, "Invalid assignment target.");
            return Ok(Expr::Empty);
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
            expr = Expr::Logical(operator, Box::new(expr), Box::new(right));
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
            expr = Expr::Logical(operator, Box::new(expr), Box::new(right));
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
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
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
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
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
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right))
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
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
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
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        while let TokenType::LeftParen = self.peek().kind() {
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if let TokenType::Comma = self.peek().kind() {
                    self.advance();
                } else {
                    break
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let token = self.peek();

        let expr = match token.kind() {
            TokenType::False => Expr::Literal(Value::Bool(false)),
            TokenType::True => Expr::Literal(Value::Bool(true)),
            TokenType::Nil => Expr::Literal(Value::Nil),
            TokenType::Number | TokenType::String => Expr::Literal(token.literal()),
            TokenType::Identifier => Expr::Variable(token.clone()),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Expr::Grouping(Box::new(expr))
            }

            _ => return Err(Lox::syntax_error(token, "Expect Expression")),
        };

        self.advance();
        Ok(expr)
    }

    fn consume<S: Into<String>>(&mut self, kind: TokenType, message: S) -> Result<Token, LoxError> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(Lox::syntax_error(self.peek(), message))
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
