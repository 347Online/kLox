use std::fmt::{Debug, Display};

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
