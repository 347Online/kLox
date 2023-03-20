use std::fmt::Display;

use crate::{token::{Token, TokenType}, value::Value};

#[derive(Debug)]
pub enum LoxErrorKind {
    SyntaxError,
    RuntimeError,
    Return(Value)
}

impl Display for LoxErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct LoxError {
    line: i32,
    message: String,
    at: String,
    kind: LoxErrorKind,
}

impl LoxError {
    fn report<S: Into<String>>(line: i32, at: S, message: S, kind: LoxErrorKind) -> LoxError {
        let error = LoxError::at(line, at, message, kind);
        eprintln!("{}", error);
        error
    }

    pub fn error<S: Into<String>>(line: i32, message: S, kind: LoxErrorKind) -> LoxError {
        LoxError::report(line, "", &message.into(), kind)
    }

    pub fn syntax<S: Into<String>>(token: &Token, message: S) -> LoxError {
        let at = if let TokenType::Eof = token.kind() {
            " at end".to_string()
        } else {
            format!(" at '{}'", token.lexeme())
        };

        LoxError::report(token.line(), at, message.into(), LoxErrorKind::SyntaxError)
    }

    pub fn runtime<S: Into<String>>(token: &Token, message: S) -> LoxError {
        LoxError::at(
            token.line(),
            "",
            &message.into(),
            LoxErrorKind::RuntimeError,
        )
    }

    pub fn return_value(token: Token, value: Value) -> LoxError {
        LoxError {
            line: token.line(),
            kind: LoxErrorKind::Return(value.clone()),
            message: format!("return {}", value),
            at: String::new(),
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] {}{}: {}",
            self.line, self.kind, self.at, self.message
        )
    }
}

impl LoxError {
    pub fn new<S: Into<String>>(line: i32, message: S, kind: LoxErrorKind) -> Self {
        LoxError::at(line, "", &message.into(), kind)
    }

    pub fn at<S: Into<String>>(line: i32, at: S, message: S, kind: LoxErrorKind) -> Self {
        LoxError {
            line,
            message: message.into(),
            at: at.into(),
            kind,
        }
    }
}
