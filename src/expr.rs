use crate::token::{BinOp, Token, UnOp, Value};

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
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
    Logical(Token, Box<Expr>, Box<Expr>),
}
