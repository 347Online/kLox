use crate::token::{Value, Token};

#[derive(Debug)]
pub enum Expr {
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
