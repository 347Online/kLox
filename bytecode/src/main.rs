pub mod lox;
use lox::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    match len {
        1 => Lox::run_prompt(),
        2 => Lox::run_file(args[1].into()),
        _ => println!("Usage: klox [script]"),
    }
}
