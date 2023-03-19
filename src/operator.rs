use crate::token::Token;

#[derive(Debug, Clone, Copy)]
pub enum UnOpType {
    Not,
    Negative,
}

#[derive(Debug, Clone)]
pub struct UnOp {
    kind: UnOpType,
    token: Token,
}

impl UnOp {
    pub fn new(kind: UnOpType, token: Token) -> Self {
        UnOp { kind, token }
    }

    pub fn kind(&self) -> UnOpType {
        self.kind
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone)]
pub struct BinOp {
    kind: BinOpType,
    token: Token,
}

impl BinOp {
    pub fn new(kind: BinOpType, token: Token) -> Self {
        BinOp { kind, token }
    }

    pub fn kind(&self) -> BinOpType {
        self.kind
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogOpType {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct LogOp {
    kind: LogOpType,
    token: Token,
}

impl LogOp {
    pub fn new(kind: LogOpType, token: Token) -> LogOp {
        LogOp { kind, token }
    }

    pub fn kind(&self) -> LogOpType {
        self.kind
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }
}
