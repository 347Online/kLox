use std::{
    fmt::{Debug, Display},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{error::LoxError, interpreter::Interpreter, value::Value};

pub trait Call: Debug + Display {
    fn arity(&self) -> usize;
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError>;
    fn box_clone(&self) -> Box<dyn Call>;
}

impl Clone for Box<dyn Call> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    arity: usize,
}

impl Call for Function {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn Call> {
        Box::new(self.clone())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Clock {
    arity: usize,
}
impl Clock {
    pub fn new() -> Self {
        Clock {
            arity: 0
        }
    }

    // TODO: Genericize this and apply to trait
    pub fn value() -> Value {
        Value::Callable(Box::new(Clock::new()))
    }
}

impl Call for Clock {
    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get the system time")
            .as_millis() as f64 / 1000.0;

        Ok(Value::Number(time))
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn box_clone(&self) -> Box<dyn Call> {
        Box::new(self.clone())
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}