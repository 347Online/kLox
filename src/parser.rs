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

    fn comparison(&mut self) -> Expr {
        let expr = self.term();

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

    fn term(&mut self) -> Expr {
        let expr = self.factor();

        while self.advance_if(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = Box::new(self.factor());
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {}

    fn check(&self, kind: TokenType) -> bool {
        !self.is_at_end() && self.peek().is(kind)
    }

    fn advance(&mut self) -> Token {}

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

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
