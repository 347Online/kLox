use crate::{
    expr::Expr,
    lox::LoxError,
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
                            todo!("Runtime error")
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

                fn check_number(value: Value) -> Result<f64, LoxError> {
                    if let Value::Number(num) = value {
                        Ok(num)
                    } else {
                        todo!("Runtime error, invalid types for operation")
                    }
                }

                match operator {
                    BinOp::Subtract => {
                        Ok(Value::Number(check_number(left)? - check_number(right)?))
                    }
                    BinOp::Divide => Ok(Value::Number(check_number(left)? / check_number(right)?)),
                    BinOp::Multiply => {
                        Ok(Value::Number(check_number(left)? * check_number(right)?))
                    }

                    _ => todo!(),
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
}
