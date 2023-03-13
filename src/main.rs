pub mod expr;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;

use lox::*;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    let mut lox = Lox::new();

    match len {
        1 => Ok(lox.run_prompt()?),
        2 => Ok(lox.run_file(args[1].clone())?),
        _ => Err(String::from("Usage: rlox-vm [script]")),
    }
}
