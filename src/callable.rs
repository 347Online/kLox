use std::fmt::Display;

use crate::{
    error::LoxError,
    function::{Clock, Function},
    interpreter::Interpreter,
    value::Value,
};

#[derive(Debug, Clone)]
pub enum Callable {
    Clock(Clock),
    Function(Function),
}

impl Callable {
    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        match self {
            Callable::Function(function) => function.call(interpreter, arguments),
            Callable::Clock(clock) => clock.call(interpreter, arguments),
        }
    }

    pub fn arity(&mut self) -> usize {
        match self {
            Callable::Clock(_) => 0,
            Callable::Function(function) => function.arity(),
        }
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Callable::Clock(clock) => clock.to_string(),
            Callable::Function(function) => function.to_string(),
        };

        write!(f, "{}", display)
    }
}
