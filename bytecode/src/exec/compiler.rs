use crate::repr::{
    chunk::Chunk,
    error::{LoxError, LoxResult},
    instruction::Instruction,
    token::{Token, TokenType},
    value::Value,
};

use super::scanner::Scanner;

#[derive(Debug, Clone, Copy)]
enum Precedence {
    Min,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        use Precedence::*;
        match value {
            1 => Min,
            2 => Assignment,
            3 => Or,
            4 => And,
            5 => Equality,
            6 => Comparison,
            7 => Term,
            8 => Factor,
            9 => Unary,
            10 => Call,
            11 => Primary,
            _ => panic!(),
        }
    }
}

enum ParseFn {
    Unary,
    Binary,
    Grouping,
    Num,
    Null,
}

struct Rule {
    prefix: ParseFn,
    infix: ParseFn,
    prec: Precedence,
}

impl Rule {
    pub fn new(prefix: ParseFn, infix: ParseFn, prec: Precedence) -> Self {
        Rule {
            prefix,
            infix,
            prec,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            prefix: ParseFn::Null,
            infix: ParseFn::Null,
            prec: Precedence::Min,
        }
    }
}

impl From<TokenType> for Rule {
    fn from(value: TokenType) -> Self {
        use ParseFn::*;
        use TokenType::*;

        match value {
            LeftParen => Rule::new(Grouping, Null, Precedence::Min),

            Minus => Rule::new(Unary, Binary, Precedence::Term),
            Plus => Rule::new(Null, Binary, Precedence::Term),

            Star => Rule::new(Null, Binary, Precedence::Factor),
            Slash => Rule::new(Null, Binary, Precedence::Factor),

            Number => Rule::new(Num, Null, Precedence::Min),

            _ => Rule::default(),
        }
    }
}

pub struct Compiler {
    scanner: Scanner,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,

    chunk: Chunk,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        let scanner = Scanner::new(source);
        let current = Token::default();
        let previous = Token::default();

        Compiler {
            scanner,
            current,
            previous,
            had_error: false,
            panic_mode: false,
            chunk: Chunk::default(),
        }
    }

    pub fn compile(&mut self) -> LoxResult<Chunk> {
        self.advance();

        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");

        if self.had_error {
            return Err(LoxError::CompileError);
        }

        self.finish();

        Ok(self.get_chunk())
    }

    fn finish(&mut self) {
        self.instruction(Instruction::Return);

        #[cfg(debug_assertions)]
        if !self.had_error {
            self.chunk.disassemble("code");
        }

    }

    fn get_chunk(&mut self) -> Chunk {
        std::mem::take(&mut self.chunk)
    }

    fn precedence(&mut self, prec: Precedence) {
        self.advance();

        let rule = Rule::from(self.previous.kind());
        if let ParseFn::Null = rule.prefix {
            self.error("Expect expression.");
            return;
        }
        self.parse(rule.prefix);

        while prec as u8 <= Rule::from(self.current.kind()).prec as u8 {
            self.advance();
            let rule = Rule::from(self.previous.kind());
            self.parse(rule.infix);
        }
    }

    fn parse(&mut self, f: ParseFn) {
        match f {
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Num => self.number(),
            ParseFn::Null => (),
        }
    }

    fn expression(&mut self) {
        self.precedence(Precedence::Assignment);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let op = self.previous.kind();

        self.precedence(Precedence::Unary);
        match op {
            TokenType::Minus => self.instruction(Instruction::Negate),

            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let op = self.previous.kind();
        let rule = Rule::from(op);
        let prec = Precedence::from(rule.prec as u8 + 1);
        self.precedence(prec);

        match op {
            TokenType::Plus => self.instruction(Instruction::Add),
            TokenType::Minus => self.instruction(Instruction::Subtract),
            TokenType::Star => self.instruction(Instruction::Multiply),
            TokenType::Slash => self.instruction(Instruction::Divide),

            _ => unreachable!(),
        }
    }

    fn number(&mut self) {
        let value: f64 = self
            .previous
            .lexeme()
            .parse()
            .expect("Lexeme is already checked for number-only characters");

        self.constant(value);
    }

    fn instruction(&mut self, code: Instruction) {
        self.chunk.write(code, self.scanner.line());
    }

    fn constant(&mut self, value: Value) {
        let index = self.chunk.add_constant(value);
        if index > u8::MAX as usize {
            self.error("Too many constants in one chunk.");
        } else {
            self.instruction(Instruction::Constant(index as u8))
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan();
            if !matches!(self.current.kind(), TokenType::Error) {
                break;
            }

            let message = self.current.lexeme();
            self.error_current(&message);
        }
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind() == kind {
            self.advance();
            return;
        }

        self.error_current(message);
    }

    fn error(&mut self, message: &str) {
        let token = self.previous.clone();
        self.error_at(message, token);
    }

    fn error_current(&mut self, message: &str) {
        let token = self.current.clone();
        self.error_at(message, token);
    }

    fn error_at(&mut self, message: &str, token: Token) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[line {}] Error", token.line());

        let kind = token.kind();
        if let TokenType::Eof = kind {
            eprint!(" at end");
        } else if let TokenType::Error = kind {
        } else {
            eprint!(" at '{}'", token.lexeme());
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }
}