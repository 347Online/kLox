use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::{
    operator::{BinOp, LogOp, UnOp},
    token::Token,
    value::Value,
};

#[derive(Debug, Clone)]
pub enum ExprType {
    Empty,
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Unary(UnOp, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(LogOp, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Expr {
    id: Uuid,
    kind: ExprType
}

impl Expr {
    pub fn new() -> Self {
        Expr::create(ExprType::Empty)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn kind(&self) -> &ExprType {
        &self.kind
    }

    pub fn grouping(expr: Expr) -> Expr {
        let kind = ExprType::Grouping(Box::new(expr));
        Expr::create(kind)
    }

    pub fn binary(operator: BinOp, left: Expr, right: Expr) -> Expr {
        let kind = ExprType::Binary(operator, Box::new(left), Box::new(right));
        Expr::create(kind)
    }

    pub fn unary(operator: UnOp, right: Expr) -> Expr {
        let kind = ExprType::Unary(operator, Box::new(right));
        Expr::create(kind)
    }

    pub fn logical(operator: LogOp, left: Expr, right: Expr) -> Expr {
        let kind = ExprType::Logical(operator, Box::new(left), Box::new(right));
        Expr::create(kind)
    }

    pub fn literal(value: Value) -> Expr {
        let kind = ExprType::Literal(value);
        Expr::create(kind)
    }

    pub fn variable(name: Token) -> Expr {
        let kind = ExprType::Variable(name);
        Expr::create(kind)
    }

    pub fn assign(name: Token, expr: Expr) -> Expr {
        let kind = ExprType::Assign(name, Box::new(expr));
        Expr::create(kind)
    }

    pub fn call(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Expr {
        let kind = ExprType::Call(Box::new(callee), paren, arguments);
        Expr::create(kind)
    }

    fn create(kind: ExprType) -> Self {
        Expr {
            id: Uuid::new_v4(),
            kind
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Expr {}