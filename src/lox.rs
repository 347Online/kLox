use std::{
    fmt::Display,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: String) {
        let code = read_to_string(path).unwrap();
        println!("{code}");
    }

    pub fn run_prompt(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout();
        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read stdin");

            if line.is_empty() {
                break;
            }

            self.run(line);
        }
    }

    pub fn run(&mut self, source: String) -> Result<(), String> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        for token in tokens {
            println!("{:?}", token);
        }

        Ok(())
    }

    pub fn error<S: Into<String> + Display>(line: i32, message: S) -> String {
        Lox::report(line, String::from(""), message.to_string())
    }

    fn report(line: i32, at: String, message: String) -> String {
        let formatted_message = format!("[line {}] Error{}: {}", line, at, message);
        println!("{formatted_message}");
        // self.had_error = true;
        formatted_message
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
