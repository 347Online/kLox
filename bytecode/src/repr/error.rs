pub enum LoxError {
    CompileError(String),
    RuntimeError(String),
}

pub type LoxResult<T> = Result<T, LoxError>;
