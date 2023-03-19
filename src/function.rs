use std::fmt::Display;

use crate::{
    callable::Call,
    environment::{self, Environment},
    error::LoxError,
    interpreter::Interpreter,
    stmt::Stmt,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Stmt,
}

impl Function {
    pub fn new(declaration: Stmt) -> Self {
        Function { declaration }
    }
}

impl Call for Function {
    fn arity(&self) -> usize {
        let Stmt::Function(_, ref params, _) = self.declaration else {
            unreachable!();
        };

        params.len()
    }

    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, LoxError> {
        let environment = Environment::new_enclosed(&interpreter.env());

        let Stmt::Function(ref name, ref params, ref body) = self.declaration else {
            unreachable!()
        };

        for i in 0..params.len() {
            environment.define(params[i].lexeme(), arguments[i].clone())
        }

        interpreter.execute_block(body.to_vec(), &environment);

        Ok(Value::Nil)
    }

    fn box_clone(&self) -> Box<dyn Call> {
        Box::new(self.clone())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Stmt::Function(ref name, _, _) = self.declaration else {
            unreachable!();  
        };
        write!(f, "<fn {}>", name.lexeme())
    }
}
