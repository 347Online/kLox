use crate::{expr::Expr, token::Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty,
    Print(Expr),
    Expr(Expr),
    Var(Token, Expr),
}