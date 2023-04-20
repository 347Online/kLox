pub mod exec;
pub mod repr;

use std::io::Write;

use exec::vm::VirtualMachine;
use repr::error::{LoxError, LoxResult};

pub fn repl() -> LoxResult<()> {
    prompt();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut vm = VirtualMachine::new();

    loop {
        print!("> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut line = String::new();
        stdin.read_line(&mut line).expect("Failed to read stdin");

        if line.is_empty() {
            break;
        }
        vm.interpret(&line)?
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
    vm.interpret(&code)
}

fn prompt() {
    let v = env!("CARGO_PKG_VERSION");
    println!("klox v{v}")
}
