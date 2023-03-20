use crate::{
    environment::Environment,
    error::LoxError,
    expr::Expr,
    operator::{BinOpType, LogOpType, UnOpType},
    stmt::Stmt,
    value::Value,
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
            if self.execute(stmt, &environment).is_err() {
                break;
            }
        }
    }

    fn execute(&mut self, stmt: Stmt, environment: &Environment) -> Result<(), LoxError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate(&expr)?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(&expr)?;
                let output = Interpreter::output(value);
                println!("{}", output)
            }
            Stmt::Var(name, initializer) => {
                let value = self.evaluate(&initializer)?;
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
                if Interpreter::is_truthy(&self.evaluate(&condition)?) {
                    self.execute(*then_branch, &environment)?;
                }
            }
            Stmt::IfElse(condition, then_branch, else_branch) => {
                let environment = self.env.clone();
                if Interpreter::is_truthy(&self.evaluate(&condition)?) {
                    self.execute(*then_branch, &environment)?;
                } else {
                    self.execute(*else_branch, &environment)?;
                }
            }

            Stmt::While(condition, body) => {
                let environment = self.env.clone();

                while Interpreter::is_truthy(&self.evaluate(&condition)?) {
                    self.execute(*body.clone(), &environment)?;
                }
            }

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
            Value::Identifier { name: _ } => todo!("pull value for identifier"),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LoxError> {
        let value = match expr {
            Expr::Empty => Value::Nil,

            Expr::Grouping(sub_expr) => self.evaluate(sub_expr)?,

            Expr::Literal(value) => value.clone(),

            Expr::Unary(operator, right) => {
                let op_type = operator.kind();
                let right = self.evaluate(right)?;

                match op_type {
                    UnOpType::Not => Value::Bool(!Interpreter::is_truthy(&right)),
                    UnOpType::Negative => {
                        if let Value::Number(value) = right {
                            Value::Number(-value)
                        } else {
                            return Err(LoxError::runtime(
                                &operator.token(),
                                "Operand must be a number.",
                            ));
                        }
                    }
                }
            }

            Expr::Variable(name) => self.env.get(name)?,

            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr)?;
                self.env.assign(name, value.clone())?;
                value
            }

            Expr::Logical(operator, left, right) => {
                let op_type = operator.kind();
                let left = self.evaluate(left)?;

                match op_type {
                    LogOpType::And => {
                        if Interpreter::is_truthy(&left) {
                            self.evaluate(right)?
                        } else {
                            left
                        }
                    }
                    LogOpType::Or => {
                        if Interpreter::is_truthy(&left) {
                            left
                        } else {
                            self.evaluate(right)?
                        }
                    }
                }
            }

            Expr::Binary(operator, left, right) => {
                let op_type = operator.kind();
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match (op_type, left, right) {
                    // Arithmetic
                    (BinOpType::Subtract, Value::Number(left), Value::Number(right)) => {
                        Value::Number(left - right)
                    }
                    (BinOpType::Divide, Value::Number(left), Value::Number(right)) => {
                        Value::Number(left / right)
                    }
                    (BinOpType::Multiply, Value::Number(left), Value::Number(right)) => {
                        Value::Number(left * right)
                    }
                    (BinOpType::Add, Value::Number(left), Value::Number(right)) => {
                        Value::Number(left + right)
                    }

                    // String Concatenation
                    (BinOpType::Add, Value::String(left), Value::String(right)) => {
                        Value::String(left + &right)
                    }

                    // Comparison
                    (BinOpType::Greater, Value::Number(left), Value::Number(right)) => {
                        Value::Bool(left > right)
                    }
                    (BinOpType::GreaterEqual, Value::Number(left), Value::Number(right)) => {
                        Value::Bool(left >= right)
                    }
                    (BinOpType::Less, Value::Number(left), Value::Number(right)) => {
                        Value::Bool(left < right)
                    }
                    (BinOpType::LessEqual, Value::Number(left), Value::Number(right)) => {
                        Value::Bool(left <= right)
                    }

                    // Equality
                    (BinOpType::Equal, left, right) => {
                        Value::Bool(Interpreter::is_equal(left, right))
                    }
                    (BinOpType::NotEqual, left, right) => {
                        Value::Bool(!Interpreter::is_equal(left, right))
                    }

                    (BinOpType::Add, _, _) => {
                        return Err(LoxError::runtime(
                            &operator.token(),
                            "Operands must be two numbers or two strings.",
                        ))
                    }
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
                    ) => {
                        return Err(LoxError::runtime(
                            &operator.token(),
                            "Operands must be numbers",
                        ))
                    }
                }
            }
        };

        Ok(value)
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(boolean) => *boolean,
            _ => true,
        }
    }

    fn is_equal(left: Value, right: Value) -> bool {
        match (&left, &right) {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,
            (Value::Number(left_num), Value::Number(right_num)) => left_num == right_num,
            (Value::String(left_str), Value::String(right_str)) => left_str == right_str,
            _ => false,
        }
    }
}
