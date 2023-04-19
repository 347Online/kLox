use std::process::{ExitCode, Termination};

use bytecode::{repl, repr::error::LoxError, run_file};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    let result = match len {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => Err(LoxError::args()),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            e.report()
        }
    }
}
