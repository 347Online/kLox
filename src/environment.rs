use std::collections::HashMap;

use crate::{
    lox::{Lox, LoxError},
    token::{Token, Value},
};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        let Some(value) = self.values.get(&name.lexeme()) else {
            return Err(Lox::runtime_error(&name, format!("Undefined variable '{}'.", name.lexeme())))
        };

        Ok(value.clone())
    }
}
