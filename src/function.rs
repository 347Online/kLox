use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    callable::Call,
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
}

impl Call for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
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

    fn box_clone(&self) -> Box<dyn Call> {
        Box::new(self.clone())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Clock {
    arity: usize,
}
impl Clock {
    pub fn new() -> Self {
        Clock { arity: 0 }
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
            .as_millis() as f64
            / 1000.0;

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
