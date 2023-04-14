use std::{io::{stdin, stdout, Write, ErrorKind}, path::PathBuf, fs::read_to_string};

use self::vm::{VirtualMachine, InterpretResult};

pub mod chunk;
pub mod instruction;
pub mod value;
pub mod vm;
pub mod error;

pub struct Lox;

impl Lox {
    pub fn run_prompt() {
        let mut vm = VirtualMachine::new();

        println!("klox, yet another Lox implementation, Katie Janzen 2023");

        let stdin = stdin();
        let mut stdout = stdout();

        loop {
            print!("> ");
            stdout.flush().expect("Failed to flush stdout");

            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read stdin");

            if line.is_empty() {
                break;
            }

            Lox::run(line, &mut vm);
        }
    }

    pub fn run_file(path: PathBuf) {
        let code = match read_to_string(&path) {
            Ok(code) => code,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("File '{}' not found", path.to_string_lossy());
                    String::new()
                }

                _ => panic!("An error occurred: {}", error),
            },
        };

        let mut vm = VirtualMachine::new();
        let result = Lox::run(code, &mut vm);
    }

    pub fn run(source: String, vm: &mut VirtualMachine) -> InterpretResult {
        vm.interpret(source)
    }
}