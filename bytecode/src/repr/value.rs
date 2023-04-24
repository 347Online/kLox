use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(Box<String>),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Boolean(false))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Value::Number(number) => number.to_string(),
            Value::Boolean(boolean) => boolean.to_string(),
            Value::Nil => String::from("nil"),
            Value::String(string) => format!("\"{}\"", *string),
        };

        write!(f, "{}", repr)
    }
}
