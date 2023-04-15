use crate::repr::{error::LoxResult, token::TokenType};

use super::scanner::Scanner;

pub fn compile(source: &str) -> LoxResult<()> {
    let mut scanner = Scanner::new(source);

    let mut line = 0;

    loop {
        let token = scanner.scan();
        let kind = token.kind();
        if token.line() != line {
            print!("{:04} ", token.line());
            line = token.line();
        } else {
            print!("   | ")
        }
        println!("{:?} '{}'", kind, token.lexeme());
        if let TokenType::Eof = kind {
            break;
        }
    }

    Ok(())
}
