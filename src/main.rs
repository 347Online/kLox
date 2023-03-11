pub mod lox;

use std::env;
use lox::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    match len {
        1 => Lox::run_prompt(),
        2 => Lox::run_file(args[1].clone()),
        _ => println!(),
    }
}
