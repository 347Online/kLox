use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Identifier { name: String },
    Number(f64),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
