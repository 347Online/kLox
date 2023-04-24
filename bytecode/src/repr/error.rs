use std::{
    fmt::{Debug, Display},
    process::{ExitCode, Termination},
};

pub type LoxResult<T> = Result<T, LoxError>;

#[derive(Debug)]
pub enum LoxError {
    IncorrectArgumentsError,
    FileNotFoundError(String),

    CompileError,
    RuntimeError,
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LoxError::*;
        let repr = match self {
            CompileError | RuntimeError => format!("{:?}", self),

            IncorrectArgumentsError => String::from("Usage: klox [script]"),
            FileNotFoundError(path) => format!("File not found '{}'", path),
        };

        write!(f, "{}", repr)
    }
}

impl Termination for LoxError {
    fn report(self) -> ExitCode {
        let code = match self {
            LoxError::IncorrectArgumentsError => 64,
            LoxError::FileNotFoundError(_) => 74,
            LoxError::CompileError => 65,
            LoxError::RuntimeError => 70,
        };

        ExitCode::from(code)
    }
}

impl std::error::Error for LoxError {}
