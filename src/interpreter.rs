use crate::{expr::Expr, token::{Value, TokenType, BinOp}};

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<Value, String> {
        Self::evaluate(expr)
    }

    fn evaluate(expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Grouping(sub_expr) => Interpreter::evaluate(*sub_expr),

            Expr::Literal(value) => {
                Ok(value)
            }

            Expr::Unary { operator, right } => todo!(),

            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let left = Interpreter::evaluate(*left)?;
                let right = Interpreter::evaluate(*right)?;

                if let Value::Number(left_num) = left {
                    if let Value::Number(right_num) = right {

                        match operator {
                            BinOp::Add => return Ok(Value::Number(left_num + right_num)),
                            BinOp::Subtract => return Ok(Value::Number(left_num - right_num)),
                            _ => unimplemented!()
                        }
                    }
                }

                if let Value::String(left_str) = left {
                    if let Value::String(right_str) = right {
                        let new_str = left_str + right_str.as_str();

                        return Ok(Value::String(new_str))
                    }
                }

                unimplemented!()
            }
        }
    }
}