pub mod lox;
pub mod token;
pub mod scanner;
pub mod expr;
pub mod parser;
pub mod interpreter;

use lox::*;
use std::env;

fn main() -> Result<(), String> {
    println!("klox, yet another Lox implementation, Katie Janzen 2023");

    let args: Vec<String> = env::args().collect();
    let len = args.len();

    let mut lox = Lox::new();

    match len {
        1 => lox.run_prompt().map_err(|e| e.to_string()),
        2 => lox.run_file(args[1].clone()).map_err(|e| e.to_string()),
        _ => Err(String::from("Usage: klox [script]")),
    }
}
