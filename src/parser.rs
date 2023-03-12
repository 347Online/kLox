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

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
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

    fn match_token(&self, kinds: Vec<TokenType>) -> bool {}

    fn previous(&mut self) -> Token {}
}
