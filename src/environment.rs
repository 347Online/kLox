use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::LoxError, lox::Lox, token::Token, value::Value};

#[derive(Debug, Default)]
pub struct EnvironmentData {
    values: HashMap<String, Value>,
    enclosing: Option<Environment>,
}

#[derive(Debug, Default, Clone)]
pub struct Environment {
    data: Rc<RefCell<EnvironmentData>>,
}

impl Environment {
    pub fn new() -> Self {
        let data = EnvironmentData {
            values: HashMap::new(),
            enclosing: None,
        };

        Environment {
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn new_enclosed(parent: &Environment) -> Self {
        let data = EnvironmentData {
            values: HashMap::new(),
            enclosing: Some(parent.clone()),
        };

        Environment {
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn define(&self, name: String, value: Value) {
        self.data.borrow_mut().values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        let data = self.data.borrow();

        if let Some(value) = data.values.get(&name.lexeme()) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &data.enclosing {
            return enclosing.get(name);
        }

        Err(Lox::runtime_error(
            name,
            format!("Undefined variable '{}'.", name.lexeme()),
        ))
    }

    pub fn assign(&self, name: &Token, value: Value) -> Result<(), LoxError> {
        let mut data = self.data.borrow_mut();
        let key = name.lexeme();
        // ???
        if let std::collections::hash_map::Entry::Occupied(mut e) = data.values.entry(key) {
            e.insert(value);
            return Ok(());
        }

        if let Some(enclosing) = &data.enclosing {
            enclosing.assign(name, value)?;
            return Ok(());
        }

        Err(Lox::runtime_error(
            name,
            format!("Undefined variable '{}'.", name.lexeme()),
        ))
    }
}
