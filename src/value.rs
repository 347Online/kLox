use std::fmt::Display;

use crate::callable::Callable;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Identifier { name: String },
    Number(f64),
    String(String),
    Nil,
    Callable(Box<Callable>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
