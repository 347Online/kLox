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

    pub fn chain(&mut self, environment: Environment) {
        self.enclosing = Some(Rc::new(environment));
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        let values = self.values.borrow();

        if let Some(value) = values.get(&name.lexeme()) {
            return Ok(value.clone());
        }
        
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
            
        Err(Lox::runtime_error(&name, format!("Undefined variable '{}'.", name.lexeme())))
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
