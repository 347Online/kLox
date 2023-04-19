#[derive(Debug)]
pub enum LoxErrorType {
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub struct LoxError {
    kind: LoxErrorType,
    message: String,
}

pub type LoxResult<T> = Result<T, LoxError>;

use std::fmt::Display;

use LoxErrorType::*;

impl LoxError {
    pub fn new(kind: LoxErrorType, message: &str) -> Self {
        let message = message.to_string();
        LoxError {
            kind,
            message,
        }
    }

    pub fn compile(message: &str) -> Self {
        LoxError::new(CompileError, message)
    }

    pub fn runtime(message: &str) -> Self {
        LoxError::new(RuntimeError, message)
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for LoxError {}
