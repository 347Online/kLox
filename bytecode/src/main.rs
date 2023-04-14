use lox::{run_prompt, run_file};

pub mod lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    match len {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: klox [script]"),
    }
}
