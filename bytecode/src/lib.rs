pub mod repr;
pub mod exec;

use std::io::Write;

use exec::vm::VirtualMachine;
use repr::error::{LoxResult, LoxError};

pub fn repl() -> LoxResult<()> {
    prompt();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read stdin");

        if line.is_empty() {
            break;
        }
    }

    Ok(())
}

pub fn run_file(path: &str) -> LoxResult<()> {
    let code = match std::fs::read_to_string(path) {
        Ok(code) => code,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                // eprintln!("File '{}' not found", path);
                return Err(LoxError::not_found(path));
            }

            _ => panic!("An error occurred: {}", error),
        },
    };

    let mut vm = VirtualMachine::new();
    // TODO: Handle result
    run(&code, &mut vm)
}

fn run(code: &str, vm: &mut VirtualMachine) -> LoxResult<()> {
    todo!()
}

fn prompt() {
    let v = env!("CARGO_PKG_VERSION");
    println!("klox v{v}")
}