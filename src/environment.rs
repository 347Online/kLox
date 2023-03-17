use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{
    lox::{Lox, LoxError},
    token::{Token, Value},
};

#[derive(Default)]
pub struct Environment {
    values: RefCell<HashMap<String, Value>>,
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: RefCell::new(HashMap::new()),
            enclosing: None,
        }
    }

    // pub fn chain(&mut self, environment: &mut Environment) {
    //     self.enclosing = Some(environment);
    // }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        let values = self.values.borrow();
        let Some(value) = values.get(&name.lexeme()) else {
            return Err(Lox::runtime_error(&name, format!("Undefined variable '{}'.", name.lexeme())))
        };

        // if let Some(envr) = &self.enclosing {
        //     return envr.get(name);
        // }

        Ok(value.clone())
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), LoxError> {
        let mut values = self.values.borrow_mut();
        let key = name.lexeme();
        // ???
        if let std::collections::hash_map::Entry::Occupied(mut e) = values.entry(key) {
            e.insert(value);
            return Ok(());
        }

        // if let Some(ref mut environment) = &mut self.enclosing {
        //     environment.assign(name, value)?;
        //     return Ok(());
        // }

        Err(Lox::runtime_error(
            &name,
            format!("Undefined variable '{}'.", name.lexeme()),
        ))
    }
}
