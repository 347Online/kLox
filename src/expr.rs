use crate::token::{Literal, Token};

pub enum Expr {
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
