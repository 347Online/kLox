use std::{
    fmt::{Debug, Display},
    process::{ExitCode, Termination},
};

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Debug)]
pub enum LoxErrorType {
    IncorrectArgumentsError = 64,
    FileNotFoundError = 74,

    CompileError = 65,
    RuntimeError = 70,
}

pub struct LoxError {
    kind: LoxErrorType,
    message: String,
}

use LoxErrorType::*;

impl LoxError {
    pub fn new(kind: LoxErrorType, message: &str) -> Self {
        let message = message.to_string();
        LoxError { kind, message }
    }

    pub fn compile(message: &str) -> Self {
        LoxError::new(CompileError, message)
    }

    pub fn runtime(message: &str) -> Self {
        LoxError::new(RuntimeError, message)
    }

    pub fn not_found(path: &str) -> Self {
        let message = format!("File not found '{}'", path);
        LoxError::new(FileNotFoundError, &message)
    }

    pub fn args() -> Self {
        LoxError::new(IncorrectArgumentsError, "Usage: klox [script]")
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self.kind {
            CompileError | RuntimeError => format!("{:?}: {}", self.kind, self.message),

            IncorrectArgumentsError | FileNotFoundError => self.message.to_string(),
        };

        write!(f, "{}", repr)
    }
}

impl Debug for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Has to be possible to do this better
        write!(f, "{}", self)
    }
}

impl Termination for LoxError {
    fn report(self) -> ExitCode {
        ExitCode::from(self.kind as u8)
    }
}

impl std::error::Error for LoxError {}
