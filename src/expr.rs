use crate::token::{Value, Token, UnOp, BinOp};

#[derive(Debug)]
pub enum Expr {
    Binary {
        operator: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Unary {
        operator: UnOp,
        right: Box<Expr>,
    },
}
