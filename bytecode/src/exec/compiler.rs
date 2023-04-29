use crate::repr::{
    chunk::Chunk,
    error::{LoxError, LoxResult},
    opcode::Instruction,
    precedence::{ParseFn, Precedence, Rule},
    token::{Token, TokenType},
    value::Value,
};

use super::scanner::Scanner;

struct LocalSlot {
    name: String,
    depth: isize,
}

const LOCALS_MAX: usize = crate::U8_COUNT;
const LOCAL_INIT: LocalSlot = LocalSlot {
    name: String::new(),
    depth: -2,
};

pub struct Compiler {
    scanner: Scanner,
    previous: Token,
    current: Token,

    had_error: bool,
    panic_mode: bool,

    chunk: Chunk,

    locals: [LocalSlot; LOCALS_MAX],
    local_count: usize,
    scope_depth: isize,
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

            locals: [LOCAL_INIT; LOCALS_MAX],
            local_count: 0,
            scope_depth: 0,
        }
    }

    pub fn compile(&mut self) -> LoxResult<Chunk> {
        self.advance();
        // self.expression();
        // self.consume(TokenType::Eof, "Expect end of expression.");

        while !self.catch(TokenType::Eof) {
            self.declaration();
        }

        if self.had_error {
            return Err(LoxError::CompileError);
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

    fn catch(&mut self, kind: TokenType) -> bool {
        if !self.check(kind) {
            false
        } else {
            self.advance();
            true
        }
    }

    fn check(&self, kind: TokenType) -> bool {
        self.current.kind() == kind
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind() == kind {
            self.advance();
            return;
        }

        self.error_current(message);
    }

    fn declaration(&mut self) {
        if self.catch(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name");

        if self.catch(TokenType::Equal) {
            self.expression();
        } else {
            self.emit(Instruction::Nil);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(global);
    }

    fn parse_variable(&mut self, message: &str) -> u8 {
        self.consume(TokenType::Identifier, message);

        self.declare_variable();

        self.indentifier_constant(self.previous.lexeme())
    }

    fn indentifier_constant(&mut self, name: String) -> u8 {
        self.make_constant(Value::String(Box::new(name)))
    }

    fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.initialize();
            return;
        }

        self.emit(Instruction::DefineGlobal);
        self.emit_byte(global);
    }

    fn initialize(&mut self) {
        self.locals[self.local_count - 1].depth = self.scope_depth;
    }

    fn statement(&mut self) {
        if self.catch(TokenType::Print) {
            self.print();
        } else if self.catch(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while self.local_count > 0 && self.locals[self.local_count - 1].depth > self.scope_depth {
            self.emit(Instruction::Pop);
            self.local_count -= 1;
        }
    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }

        let name = self.previous.lexeme();

        for i in (0..self.local_count).rev() {
            let local = &self.locals[i];

            if local.depth != -1 && local.depth < self.scope_depth {
                break;
            }

            if name == local.name {
                self.error("Already a variable with this name in this scope.");
            }
        }

        self.add_local(name);
    }

    fn add_local(&mut self, name: String) {
        if self.local_count == LOCALS_MAX {
            self.error("Too many local variables in function.");
            return;
        }

        let local = &mut self.locals[self.local_count];
        self.local_count += 1;

        local.name = name;
        local.depth = -1;
    }

    fn resolve_local(&mut self, name: &String) -> Option<u8> {
        for i in (0..self.local_count).rev() {
            let local = &self.locals[i];
            if name == &local.name {
                if local.depth == -1 {
                    self.error("Can't read local variable in its own initializer.");
                }
                return Some(i as u8);
            }
        }

        None
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.kind() != TokenType::Eof {
            if self.previous.kind() == TokenType::Semicolon {
                return;
            }

            match self.current.kind() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,

                _ => (),
            }

            self.advance();
        }
    }

    fn print(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit(Instruction::Print);
    }

    fn expression(&mut self) {
        self.precedence(Precedence::Assignment);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        self.emit(Instruction::Pop);
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

    fn string(&mut self) {
        let value = Value::String(Box::new(self.previous.lexeme()));
        self.emit_constant(value);
    }

    fn variable(&mut self, assign: bool) {
        self.named_variable(self.previous.lexeme(), assign);
    }

    fn named_variable(&mut self, name: String, assign: bool) {
        let get_op: Instruction;
        let set_op: Instruction;

        let arg = if let Some(byte) = self.resolve_local(&name) {
            get_op = Instruction::GetLocal;
            set_op = Instruction::SetLocal;
            byte
        } else {
            get_op = Instruction::GetGlobal;
            set_op = Instruction::SetGlobal;
            self.indentifier_constant(name)
        };

        if assign && self.catch(TokenType::Equal) {
            self.expression();
            self.emit(set_op);
            self.emit_byte(arg);
        } else {
            self.emit(get_op);
            self.emit_byte(arg);
        }
    }

    fn literal(&mut self) {
        let opcode = match self.previous.kind() {
            TokenType::Nil => Instruction::Nil,
            TokenType::True => Instruction::True,
            TokenType::False => Instruction::False,

            _ => unreachable!(),
        };

        self.emit(opcode);
    }

    fn unary(&mut self) {
        let op = self.previous.kind();

        // Compile the operand
        self.precedence(Precedence::Unary);

        match op {
            TokenType::Minus => self.emit(Instruction::Negate),
            TokenType::Bang => self.emit(Instruction::Not),

            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous.kind();

        let rule = Rule::from(operator);
        let prec =
            Precedence::try_from(rule.prec() as u8 + 1).expect("Failed to get next precedence");

        self.precedence(prec);

        match operator {
            TokenType::EqualEqual => self.emit(Instruction::Equal),
            TokenType::BangEqual => {
                self.emit(Instruction::Equal);
                self.emit(Instruction::Not);
            }
            TokenType::Greater => self.emit(Instruction::Greater),
            TokenType::GreaterEqual => {
                self.emit(Instruction::Less);
                self.emit(Instruction::Not);
            }
            TokenType::Less => self.emit(Instruction::Less),
            TokenType::LessEqual => {
                self.emit(Instruction::Greater);
                self.emit(Instruction::Not);
            }
            TokenType::Plus => self.emit(Instruction::Add),
            TokenType::Minus => self.emit(Instruction::Subtract),
            TokenType::Star => self.emit(Instruction::Multiply),
            TokenType::Slash => self.emit(Instruction::Divide),

            _ => unreachable!(),
        }
    }

    fn emit(&mut self, opcode: Instruction) {
        self.chunk.write(opcode, self.previous.line());
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.previous.line());
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit(Instruction::Constant);
        self.emit_byte(constant);
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

        let assign = prec as u8 <= Precedence::Assignment as u8;
        self.parse(prefix, assign);

        while prec as u8 <= Rule::from(self.current.kind()).prec() as u8 {
            self.advance();
            let infix = Rule::from(self.previous.kind()).infix();
            self.parse(infix, false);
        }

        if assign && self.catch(TokenType::Equal) {
            self.error("Invalid assignment target.");
        }
    }

    fn parse(&mut self, f: ParseFn, assign: bool) {
        match f {
            ParseFn::Literal => self.literal(),
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Number => self.number(),
            ParseFn::String => self.string(),
            ParseFn::Variable => self.variable(assign),
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
