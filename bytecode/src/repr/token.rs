#[derive(Debug, Clone, Copy)]
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

    Error,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: String, line: usize) -> Self {;
        Token { kind, lexeme, line }
    }

    pub fn kind(&self) -> TokenType {
        self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
