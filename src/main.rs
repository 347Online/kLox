

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    match len {
        1 => Lox::run_prompt(),
        2 => Lox::run_file(args[1]),
        _ => println!(),
    }

    if len > 2 {
        println!("Usage: klox-vm [script]")
    } else if len == 2 {
        // Lox::run_file(args[1]);
    } else {
        Lox::run_prompt();
    }
}
