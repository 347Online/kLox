use std::fmt::Display;

#[derive(Clone, Debug)]
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
pub enum Literal {
    Empty
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO: <Literal>")
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32,
}

impl Token {
    pub fn new<S: Into<String> + Display>(kind: TokenType, lexeme: S, literal: Literal, line: i32) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            literal,
            line
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.kind, self.lexeme, self.literal)
    }
}