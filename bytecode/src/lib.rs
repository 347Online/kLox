pub mod exec;
pub mod repr;

use std::io::Write;

use exec::vm::VirtualMachine;
use repr::error::{LoxError, LoxResult};

pub const U8_COUNT: usize = u8::MAX as usize + 1;

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
        let _ = vm.interpret(&line);
    }

    Ok(())
}

pub fn run_file(path: &str) -> LoxResult<()> {
    let code = match std::fs::read_to_string(path) {
        Ok(code) => code,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                return Err(LoxError::FileNotFoundError(path.to_string()));
            }

            _ => panic!("An error occurred: {}", error),
        },
    };

    let mut vm = VirtualMachine::new();
    vm.interpret(&code)
}

fn prompt() {
    let v = env!("CARGO_PKG_VERSION");
    println!("klox v{v}")
}
