use std::{
    fs::read_to_string,
    io::{stdin, stdout, ErrorKind, Write},
};

use crate::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

pub struct Lox;

impl Lox {
    pub const MAX_ARGS: usize = 255;

    pub fn run_file(path: String) {
        let code = match read_to_string(&path) {
            Ok(code) => code,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("File '{}' not found", path);
                    String::new()
                }

                _ => panic!("An error occurred: {}", error),
            },
        };

        let mut interpreter = Interpreter::new();
        Lox::run(code, &mut interpreter);
    }

    pub fn run_prompt() {
        println!("klox, yet another Lox implementation, Katie Janzen 2023");

        let stdin = stdin();
        let mut stdout = stdout();
        let mut interpreter = Interpreter::new();

        loop {
            print!("> ");
            stdout.flush().expect("Failed to flush stdout");

            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read stdin");

            if line.is_empty() {
                break;
            }

            Lox::run(line, &mut interpreter);
        }
    }

    fn run(source: String, interpreter: &mut Interpreter) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        interpreter.interpret(statements);
    }
}
