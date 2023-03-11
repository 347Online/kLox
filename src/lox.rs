use std::{fmt::Display, fs::read_to_string, io::*};

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

    pub fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: i32, message: String) {
        self.report(line, "", message);
    }

    fn report<S: Into<String> + Display>(&mut self, line: i32, at: S, message: String) {
        println!("[line {}] Error{}: {}", line, at, message);
        self.had_error = true;
    }
}
