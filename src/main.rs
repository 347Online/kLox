pub mod environment;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;

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
