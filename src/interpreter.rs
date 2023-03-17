use crate::{
    environment::Environment,
    expr::Expr,
    lox::{Lox, LoxError},
    stmt::Stmt,
    token::{BinOpType, UnOpType, Value},
};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        let environment = self.env.clone();

        for stmt in statements {
            if let Err(error) = self.execute(stmt, &environment) {
                eprintln!("{}", error);
                break;
            }
        }
    }

    fn execute(&mut self, stmt: Stmt, environment: &Environment) -> Result<(), LoxError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                let output = Interpreter::output(value);
                println!("{}", output)
            }
            Stmt::Var(name, initializer) => {
                let value = self.evaluate(initializer)?;
                self.env.define(name.lexeme(), value);
            }
            Stmt::Block(statements) => {
                let environment = Environment::new_enclosed(environment);

                for stmt in statements {
                    self.execute(stmt, &environment)?;
                }
            }

            Stmt::If(condition, then_branch) => {
                let environment = self.env.clone();
                let condition = self.evaluate(condition)?;
                if Interpreter::is_truthy(condition) {
                    self.execute(*then_branch, &environment)?;
                }
            },
            Stmt::IfElse(condition, then_branch, else_branch) => {
                let environment = self.env.clone();
                let condition = self.evaluate(condition)?;
                if Interpreter::is_truthy(condition) {
                    self.execute(*then_branch, &environment)?;
                } else {
                    self.execute(*else_branch, &environment)?;
                }
            },

            Stmt::Empty => (),
        }

        Ok(())
    }

    fn output(value: Value) -> String {
        match value {
            Value::Nil => String::from("nil"),
            Value::Number(number) => number.to_string(),
            Value::Bool(boolean) => boolean.to_string(),
            Value::String(string) => string,
            #[allow(unused)] // TODO: Remove this
            Value::Identifier { name } => todo!("pull value for identifier"),
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Empty => Ok(Value::Nil),

            Expr::Grouping(sub_expr) => self.evaluate(*sub_expr),

            Expr::Literal(value) => Ok(value),

            Expr::Unary { operator, right } => {
                let op_type = operator.kind();
                let right = self.evaluate(*right)?;

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

            Expr::Variable(name) => Ok(self.env.get(name)?),

            Expr::Assign(name, expr) => {
                let value = self.evaluate(*expr)?;
                self.env.assign(name, value.clone())?;
                Ok(value)
            }

            Expr::Logical(operator, left, right) => {
                let op_type = operator.kind();
                let left = self.evaluate(*left)?;

                unimplemented!()
            },

            Expr::Binary {
                operator,
                left,
                right,
            } => {
                let op_type = operator.kind();
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

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
}
