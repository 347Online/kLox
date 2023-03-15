use crate::{
    expr::Expr,
    lox::{LoxError},
    token::{BinOp, UnOp, Value},
};

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<Value, LoxError> {
        Self::evaluate(expr)
    }

    fn evaluate(expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Empty => Ok(Value::Nil),

            Expr::Grouping(sub_expr) => Interpreter::evaluate(*sub_expr),

            Expr::Literal(value) => Ok(value),

            Expr::Unary { operator, right } => {
                let right = Interpreter::evaluate(*right)?;

                match operator {
                    UnOp::Not => Ok(Value::Bool(!Interpreter::is_truthy(right))),
                    UnOp::Negative => {
                        if let Value::Number(value) = right {
                            return Ok(Value::Number(-value))
                        }
                        
                        todo!("Runtime error")
                    }
                }
            }

            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let left = Interpreter::evaluate(*left)?;
                let right = Interpreter::evaluate(*right)?;

                match (operator, left, right) {
                    // Arithmetic
                    (BinOp::Subtract, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }
                    (BinOp::Divide, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }
                    (BinOp::Multiply, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }
                    (BinOp::Add, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }

                    // String Concatenation
                    (BinOp::Add, Value::String(left), Value::String(right)) => {
                        Ok(Value::String(left + &right))
                    }

                    // Comparison
                    (BinOp::Greater, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left > right))
                    }
                    (BinOp::GreaterEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left >= right))
                    }
                    (BinOp::Less, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left < right))
                    }
                    (BinOp::LessEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left <= right))
                    }

                    // Equality
                    (BinOp::Equal, left, right) => {
                        Ok(Value::Bool(Interpreter::is_equal(left, right)))
                    }
                    (BinOp::NotEqual, left, right) => {
                        Ok(Value::Bool(!Interpreter::is_equal(left, right)))
                    }

                    _ => todo!("Runtime error, bad comparison"),
                }
            }
        }
    }

    fn is_truthy(value: Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(boolean) => boolean,
            _ => true,
        }
    }

    fn is_equal(left: Value, right: Value) -> bool {
        match (&left, &right) {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,

            _ => left == right
        }
    }
}
