use crate::token::{BinOp, Token, UnOp, Value, LogOp};

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    // Unary {
    //     operator: UnOp,
    //     right: Box<Expr>,
    // },
    Unary(UnOp, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(LogOp, Box<Expr>, Box<Expr>),
}
