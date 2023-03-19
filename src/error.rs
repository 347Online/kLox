use std::fmt::Display;

#[derive(Debug)]
pub enum LoxErrorKind {
    SyntaxError,
    RuntimeError,
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
