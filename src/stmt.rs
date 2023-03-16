use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr)
}