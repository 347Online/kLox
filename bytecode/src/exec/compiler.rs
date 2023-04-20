use crate::repr::token::TokenType;

use super::scanner::Scanner;

pub struct Compiler {
    scanner: Scanner,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Compiler {
            scanner: Scanner::new(source),
        }
    }

    pub fn compile(&mut self) {
        let mut line = 0;
        loop {
            let token = self.scanner.scan();
            // println!("{:?}", token);
            if line != token.line() {
                line = token.line();
                print!("{:>4} ", line);
            } else {
                print!("   | ");
            }
            println!("{:>2} '{}'", token.kind() as u8, token.lexeme());

            if let TokenType::Eof = token.kind() {
                break;
            }
        }
    }
}
