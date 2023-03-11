pub mod lox;
pub mod scanner;
pub mod token;

use std::env;
use lox::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    let lox = Lox::new();

    match len {
        1 => lox.run_prompt(),
        2 => lox.run_file(args[1].clone()),
        _ => println!(),
    }
}
