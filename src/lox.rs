use std::{
    fmt::Display,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use crate::{
    expr::Expr,
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum LoxErrorKind {
    SyntaxError,
    RuntimeError,
}

impl Display for LoxErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct LoxError {
    line: i32,
    message: String,
    at: String,
    kind: LoxErrorKind,
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}] {}{}: {}",
            self.line, self.kind, self.at, self.message
        )
    }
}

impl LoxError {
    pub fn new<S: Into<String>>(line: i32, message: S, kind: LoxErrorKind) -> Self {
        LoxError::at(line, "", &message.into(), kind)
    }

    pub fn at<S: Into<String>>(line: i32, at: S, message: S, kind: LoxErrorKind) -> Self {
        LoxError {
            line,
            message: message.into(),
            at: at.into(),
            kind,
        }
    }
}

pub struct Lox;

impl Lox {
    pub fn run_file(path: String) {
        let code = read_to_string(path).unwrap();
        Lox::run(code);
    }

    pub fn run_prompt() {
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

            Lox::run(line);
        }
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap_or_else(|e| {
            println!("{e}");
            vec![]
        });

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap_or_else(|e| {
            println!("{e}");
            Expr::Empty
        });

        let mut interpreter = Interpreter::new();
        interpreter.interpret(ast);
    }

    pub fn error<S: Into<String>>(line: i32, message: S, kind: LoxErrorKind) -> LoxError {
        Lox::report(line, "", &message.into(), kind)
    }

    pub fn syntax_error<S: Into<String>>(token: &Token, message: S) -> LoxError {
        let at = if token.is(TokenType::Eof) {
            " at end".to_string()
        } else {
            format!(" at '{}'", token.lexeme())
        };

        Lox::report(token.line(), at, message.into(), LoxErrorKind::SyntaxError)
    }

    pub fn runtime_error<S: Into<String>>(token: &Token, message: S) -> LoxError {
        LoxError::at(token.line(), "", &message.into(), LoxErrorKind::RuntimeError)
    }

    fn report<S: Into<String>>(line: i32, at: S, message: S, kind: LoxErrorKind) -> LoxError {
        LoxError::at(line, at, message, kind)
    }
}