pub enum LoxError {
    CompileError,
    RuntimeError,
}

pub type LoxResult<T> = Result<T, LoxError>;
