use crate::token::{BinOp, LogOp, Token, UnOp, Value};

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Unary(UnOp, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(LogOp, Box<Expr>, Box<Expr>),
}
