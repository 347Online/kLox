use std::fmt::Display;

use crate::value::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Value,
    line: i32,
}

impl Token {
    pub fn new<S: Into<String>>(kind: TokenType, lexeme: S, literal: Value, line: i32) -> Token {
        Token {
            kind,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }

    pub fn kind(&self) -> TokenType {
        self.kind
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn literal(&self) -> Value {
        self.literal.clone()
    }

    pub fn line(&self) -> i32 {
        self.line
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.kind, self.lexeme, self.literal)
    }
}
