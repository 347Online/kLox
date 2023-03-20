pub mod callable;
pub mod environment;
pub mod error;
pub mod expr;
pub mod function;
pub mod interpreter;
pub mod lox;
pub mod operator;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;
pub mod value;

use lox::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    match len {
        1 => Lox::run_prompt(),
        2 => Lox::run_file(args[1].clone()),
        _ => println!("Usage: klox [script]"),
    }
}
