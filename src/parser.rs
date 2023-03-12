use crate::{
    expr::Expr,
    token::{Token, TokenType},
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
        let expr = self.comparison();

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

    fn comparison(&mut self) -> Expr {}

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> Token {

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
        if let Token {kind: TokenType::Eof, ..} = self.peek() {
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
