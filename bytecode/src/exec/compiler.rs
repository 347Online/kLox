use crate::repr::{
    chunk::Chunk,
    error::{LoxError, LoxResult},
    opcode::Instruction,
    precedence::{Precedence, Rule, ParseFn},
    token::{Token, TokenType},
    value::Value,
};

use super::scanner::Scanner;

pub struct Compiler {
    scanner: Scanner,
    previous: Token,
    current: Token,

    had_error: bool,
    panic_mode: bool,

    chunk: Chunk,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        let scanner = Scanner::new(source);
        let previous = Token::default();
        let current = Token::default();

        Compiler {
            scanner,
            current,
            previous,

            had_error: false,
            panic_mode: false,

            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self) -> LoxResult<Chunk> {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");

        if self.had_error {
            return Err(LoxError::compile("Compiler Error"));
        }

        self.finish();

        let chunk = self.get_chunk();

        #[cfg(debug_assertions)]
        if !self.had_error {
            chunk.disassemble("code");
        }

        Ok(chunk)
    }

    fn get_chunk(&mut self) -> Chunk {
        std::mem::take(&mut self.chunk)
    }

    fn advance(&mut self) {
        self.previous = std::mem::take(&mut self.current);

        loop {
            self.current = self.scanner.scan();
            if !matches!(self.current.kind(), TokenType::Error) {
                break;
            }

            self.error_current(&self.current.lexeme());
        }
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind() == kind {
            self.advance();
            return;
        }

        self.error_current(message);
    }

    fn expression(&mut self) {
        self.precedence(Precedence::Assignment);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let lexeme = self.previous.lexeme();
        let value: f64 = lexeme.parse().expect("Failed to parse lexeme as number");
        self.emit_constant(Value::Number(value));
    }

    fn literal(&mut self) {
        let opcode = match self.previous.kind() {
            TokenType::Nil => Instruction::Nil,
            TokenType::True => Instruction::True,
            TokenType::False => Instruction::False,

            _ => unreachable!()
        };

        self.emit(opcode);
    }

    fn unary(&mut self) {
        let op = self.previous.kind();

        // Compile the operand
        self.precedence(Precedence::Unary);

        match op {
            TokenType::Minus => self.emit(Instruction::Negate),

            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous.kind();

        let rule = Rule::from(operator);
        let prec = Precedence::try_from(rule.prec() as u8 + 1).expect("Failed to get next precedence");

        self.precedence(prec);

        let opcode = match operator {
            TokenType::Plus => Instruction::Add,
            TokenType::Minus => Instruction::Subtract,
            TokenType::Star => Instruction::Multiply,
            TokenType::Slash => Instruction::Divide,

            _ => unreachable!()
        };

        self.emit(opcode);
    }

    fn emit(&mut self, opcode: Instruction) {
        self.chunk.write(opcode, self.previous.line());
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.previous.line());
    }

    fn emit_pair(&mut self, opcode: Instruction, operand: u8) {
        self.emit(opcode);
        self.emit_byte(operand);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_pair(Instruction::Constant, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        if constant > u8::MAX as usize {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant as u8
    }

    fn precedence(&mut self, prec: Precedence) {
        self.advance();

        let prefix = Rule::from(self.previous.kind()).prefix();
        if let ParseFn::Null = prefix {
            self.error("Expect expression.");
            return;
        }

        self.parse(prefix);

        while prec as u8 <= Rule::from(self.current.kind()).prec() as u8 {
            self.advance();
            let infix = Rule::from(self.previous.kind()).infix();
            self.parse(infix);
        }
    }

    fn parse(&mut self, f: ParseFn) {
        match f {
            ParseFn::Literal => self.literal(),
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Num => self.number(),
            ParseFn::Null => (),
        }
    }

    fn finish(&mut self) {
        self.emit(Instruction::Return);
    }

    fn error_at(&mut self, line: usize, kind: TokenType, lexeme: String, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", line);

        match kind {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", lexeme),
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn error(&mut self, message: &str) {
        let line = self.previous.line();
        let kind = self.previous.kind();
        let lexeme = self.previous.lexeme();
        self.error_at(line, kind, lexeme, message);
    }

    fn error_current(&mut self, message: &str) {
        let line = self.current.line();
        let kind = self.current.kind();
        let lexeme = self.current.lexeme();
        self.error_at(line, kind, lexeme, message);
    }
}
