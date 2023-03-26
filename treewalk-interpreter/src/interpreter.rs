use crate::{
    environment::Environment,
    error::LoxError,
    expr::{Expr, ExprType},
    function::{Clock, Function},
    operator::{BinOpType, LogOpType, UnOpType},
    stmt::Stmt,
    value::Value,
};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new();
        let environment = Environment::new_enclosed(&globals);
        globals.define("clock", Clock::new().value());

        Interpreter {
            env: environment,
            globals,
        }
    }

    pub fn env(&self) -> &Environment {
        &self.env
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
                self.evaluate(&expr, environment)?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(&expr, environment)?;
                let output = Interpreter::output(value);
                println!("{}", output)
            }
            Stmt::Var(name, initializer) => {
                let value = self.evaluate(&initializer, environment)?;
                self.env.define(name.lexeme(), value);
            }

            Stmt::Block(statements) => {
                self.execute_block(statements, &Environment::new_enclosed(environment))?;
            }

            Stmt::If(condition, then_branch) => {
                if Interpreter::is_truthy(&self.evaluate(&condition, environment)?) {
                    self.execute(*then_branch, environment)?;
                }
            }
            Stmt::IfElse(condition, then_branch, else_branch) => {
                if Interpreter::is_truthy(&self.evaluate(&condition, environment)?) {
                    self.execute(*then_branch, environment)?;
                } else {
                    self.execute(*else_branch, environment)?;
                }
            }

            Stmt::While(condition, body) => {
                while Interpreter::is_truthy(&self.evaluate(&condition, environment)?) {
                    self.execute(*body.clone(), environment)?;
                }
            }

            Stmt::Function(name, params, body) => {
                let display_name = name.lexeme();

                //TODO: Investigate this further
                let function = Function::new(name, params, body, Environment::new());
                environment.define(display_name, function.value());
            }

            Stmt::Return(keyword, expr) => {
                let value = self.evaluate(&expr, environment)?;
                return Err(LoxError::return_value(keyword, value));
            }

            Stmt::Empty => (),
        }

        Ok(())
    }

    pub fn execute_block(
        &mut self,
        body: Vec<Stmt>,
        environment: &Environment,
    ) -> Result<(), LoxError> {
        for stmt in body {
            self.execute(stmt, environment)?;
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
            Value::Callable(callable) => callable.to_string(),
        }
    }

    fn evaluate(&mut self, expr: &Expr, environment: &Environment) -> Result<Value, LoxError> {
        let value = match expr.kind() {
            ExprType::Empty => Value::Nil,

            ExprType::Grouping(sub_expr) => self.evaluate(sub_expr, environment)?,

            ExprType::Literal(value) => value.clone(),

            ExprType::Unary(operator, right) => {
                let op_type = operator.kind();
                let right = self.evaluate(right, environment)?;

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

            ExprType::Variable(name) => environment.get(name)?,

            ExprType::Assign(name, expr) => {
                let value = self.evaluate(expr, environment)?;
                environment.assign(name, value.clone())?;
                value
            }

            ExprType::Logical(operator, left, right) => {
                let op_type = operator.kind();
                let left = self.evaluate(left, environment)?;

                match op_type {
                    LogOpType::And => {
                        if Interpreter::is_truthy(&left) {
                            self.evaluate(right, environment)?
                        } else {
                            left
                        }
                    }
                    LogOpType::Or => {
                        if Interpreter::is_truthy(&left) {
                            left
                        } else {
                            self.evaluate(right, environment)?
                        }
                    }
                }
            }

            ExprType::Binary(operator, left, right) => {
                let op_type = operator.kind();
                let left = self.evaluate(left, environment)?;
                let right = self.evaluate(right, environment)?;

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

            ExprType::Call(callee, paren, args) => {
                let callee = self.evaluate(callee, environment)?;

                let mut arguments = vec![];
                for arg in args {
                    arguments.push(self.evaluate(arg, environment)?);
                }

                if let Value::Callable(mut function) = callee {
                    if arguments.len() != function.arity() {
                        return Err(LoxError::runtime(
                            paren,
                            format!(
                                "Expected {} arguments but got {}.",
                                function.arity(),
                                arguments.len()
                            ),
                        ));
                    }
                    function.call(self, arguments)?
                } else {
                    return Err(LoxError::runtime(
                        paren,
                        "Can only call functions and classes.",
                    ));
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
