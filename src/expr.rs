use crate::token::{BinOp, UnOp, Value, Token};

#[derive(Debug, Clone)]
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
    Variable(Token),
    Assign(Token, Box<Expr>),
    Empty,
}
