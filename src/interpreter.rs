use crate::{
    expr::Expr,
    lox::{Lox, LoxError, LoxErrorKind},
    token::{BinOpType, Token, UnOpType, Value},
};

#[derive(Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, expr: Expr) {
        let result = Interpreter::evaluate(expr);
        let output = match result {
            Ok(value) => println!("{}", Interpreter::output(value)),
            Err(error) => eprintln!("{}", error.to_string()),
        };

    }

    fn output(value: Value) -> String {
        match value {
            Value::Nil => String::from("nil"),
            Value::Number(number) => number.to_string(),
            Value::Bool(boolean) => boolean.to_string(),
            Value::String(string) => string,
            Value::Identifier { name } => todo!("pull value for identifier"),
        }
    }

    fn evaluate(expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Empty => Ok(Value::Nil),

            Expr::Grouping(sub_expr) => Interpreter::evaluate(*sub_expr),

            Expr::Literal(value) => Ok(value),

            Expr::Unary { operator, right } => {
                let op_type = operator.kind();
                let right = Interpreter::evaluate(*right)?;

                match op_type {
                    UnOpType::Not => Ok(Value::Bool(!Interpreter::is_truthy(right))),
                    UnOpType::Negative => {
                        if let Value::Number(value) = right {
                            Ok(Value::Number(-value))
                        } else {
                            Err(Lox::runtime_error(
                                &operator.token(),
                                "Operand must be a number.",
                            ))
                        }
                    }
                }
            }

            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let op_type = operator.kind();
                let left = Interpreter::evaluate(*left)?;
                let right = Interpreter::evaluate(*right)?;

                match (op_type, left, right) {
                    // Arithmetic
                    (BinOpType::Subtract, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left - right))
                    }
                    (BinOpType::Divide, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left / right))
                    }
                    (BinOpType::Multiply, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left * right))
                    }
                    (BinOpType::Add, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Number(left + right))
                    }

                    // String Concatenation
                    (BinOpType::Add, Value::String(left), Value::String(right)) => {
                        Ok(Value::String(left + &right))
                    }

                    // Comparison
                    (BinOpType::Greater, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left > right))
                    }
                    (BinOpType::GreaterEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left >= right))
                    }
                    (BinOpType::Less, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left < right))
                    }
                    (BinOpType::LessEqual, Value::Number(left), Value::Number(right)) => {
                        Ok(Value::Bool(left <= right))
                    }

                    // Equality
                    (BinOpType::Equal, left, right) => {
                        Ok(Value::Bool(Interpreter::is_equal(left, right)))
                    }
                    (BinOpType::NotEqual, left, right) => {
                        Ok(Value::Bool(!Interpreter::is_equal(left, right)))
                    }

                    (BinOpType::Add, _, _) => Err(Lox::runtime_error(
                        &operator.token(),
                        "Operands must be two numbers or two strings.",
                    )),
                    (
                        BinOpType::Greater
                        | BinOpType::GreaterEqual
                        | BinOpType::Less
                        | BinOpType::LessEqual
                        | BinOpType::Subtract
                        | BinOpType::Divide
                        | BinOpType::Multiply,
                        _,
                        _,
                    ) => Err(Lox::runtime_error(
                        &operator.token(),
                        "Operands must be numbers",
                    )),
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

    // fn error<S: Into<String>>(token: Token, message: S) -> LoxError {
    //     Lox::error(-13, message, LoxErrorKind::RuntimeError)
    // }
}
