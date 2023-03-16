use std::collections::HashMap;

use crate::{
    lox::{Lox, LoxError},
    token::{Token, Value},
};

#[derive(Default)]
pub struct Environment<'a> {
    values: HashMap<String, Value>,
    enclosing: Option<&'a mut Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        let Some(value) = self.values.get(&name.lexeme()) else {
            return Err(Lox::runtime_error(&name, format!("Undefined variable '{}'.", name.lexeme())))
        };

        if let Some(environment) = &self.enclosing {
            return environment.get(name);
        }

        Ok(value.clone())
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), LoxError> {
        // ???
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.values.entry(name.lexeme())
        {
            e.insert(value);
            return Ok(());
        }

        if let Some(ref mut environment) = &mut self.enclosing {
            environment.assign(name, value)?;
            return Ok(());
        }

        Err(Lox::runtime_error(
            &name,
            format!("Undefined variable '{}'.", name.lexeme()),
        ))
    }
}
