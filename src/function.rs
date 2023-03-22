use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    callable::Callable,
    environment::Environment,
    error::{LoxError, LoxErrorType},
    interpreter::Interpreter,
    stmt::Stmt,
    token::Token,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Environment,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>, closure: Environment) -> Self {
        Function {
            name,
            params,
            body,
            closure,
        }
    }

    pub fn arity(&self) -> usize {
        self.params.len()
    }

    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let environment = Environment::new_enclosed(interpreter.env());

        for (param, arg) in self.params.iter().zip(arguments) {
            environment.define(param.lexeme(), arg);
        }

        let result = interpreter.execute_block(self.body.clone(), &environment);

        match result {
            Ok(()) => (),
            Err(error) => {
                if let LoxErrorType::Return(value) = error.kind() {
                    return Ok(value.clone());
                }

                return Err(error);
            }
        }

        Ok(Value::Nil)
    }

    pub fn value(self) -> Value {
        Value::Callable(Box::new(Callable::Function(self)))
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Clock
    }

    pub fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get the system time")
            .as_millis() as f64
            / 1000.0;

        Ok(Value::Number(time))
    }

    pub fn value(self) -> Value {
        Value::Callable(Box::new(Callable::Clock(self)))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
