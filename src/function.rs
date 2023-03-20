use std::fmt::Display;

use crate::{
    callable::Call,
    environment::Environment,
    error::LoxError,
    interpreter::Interpreter,
    stmt::Stmt,
    value::Value, token::Token,
};

#[derive(Debug, Clone)]
pub struct Function {
    // declaration: Stmt,
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>
}

impl Function {
    // pub fn new(declaration: Stmt) -> Self {
    //     Function { declaration }
    // }

    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Function {
            name,
            params,
            body
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

        interpreter.execute_block(self.body.clone(), &environment)?;

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
