use std::{
    fmt::Display,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use crate::{
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType}, interpreter::Interpreter,
};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: String) -> Result<(), String> {
        let code = read_to_string(path).unwrap();
        self.run(code)?;

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), String> {
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

            self.run(line)?;
        }

        Ok(())
    }

    fn run(&mut self, source: String) -> Result<(), String> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        // println!("AST: {:?}", ast);

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(ast)?;

        println!("Result: {}", result);

        // if hadError???

        Ok(())
    }

    pub fn error<S: Into<String> + Display>(line: i32, message: S) -> String {
        Lox::report(line, String::from(""), message.to_string())
    }

    pub fn error_token<S: Into<String> + Display>(token: &Token, message: S) -> String {
        let at = if token.is(TokenType::Eof) {
            " at end ".to_string()
        } else {
            format!(" at '{}'", token.lexeme())
        };

        Lox::report(token.line(), at, message.to_string())
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
