use crate::{
    expr::Expr,
    lox::{Lox, LoxError, LoxErrorKind},
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
                            Ok(Value::Number(-value))
                        } else {
                            Err(Interpreter::error("Operand must be a number."))
                        }
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

                    (BinOp::Add, _, _) => Err(Interpreter::error(
                        "Operands must be two numbers or two strings.",
                    )),
                    (
                        BinOp::Greater
                        | BinOp::GreaterEqual
                        | BinOp::Less
                        | BinOp::LessEqual
                        | BinOp::Subtract
                        | BinOp::Divide
                        | BinOp::Multiply,
                        _,
                        _,
                    ) => Err(Interpreter::error("Operands must be numbers")),
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
            _ => left == right,
        }
    }

    fn error<S: Into<String>>(message: S) -> LoxError {
        Lox::error(-13, message, LoxErrorKind::RuntimeError)
    }
}
