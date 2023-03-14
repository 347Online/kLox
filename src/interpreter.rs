use crate::token::Value;

pub struct Interpreter;

impl Interpreter {
    pub fn visit_literal_expr(literal: Value) -> Value {
        literal
    }
}