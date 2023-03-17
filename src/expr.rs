use crate::token::{BinOp, Token, UnOp, Value, LogOp};

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    // TODO: Refactor this to tuple type rather than struct
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
    Logical(LogOp, Box<Expr>, Box<Expr>),
}
