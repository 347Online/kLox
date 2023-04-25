use crate::repr::{
    chunk::Chunk,
    error::{LoxError, LoxResult},
    opcode::Instruction,
    value::Value,
};

use super::compiler::Compiler;

const STACK_MAX: usize = 256;
const STACK_INIT: Value = Value::Nil;

pub struct VirtualMachine {
    ip: usize,
    chunk: Chunk,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            chunk: Chunk::new(),
            stack: [STACK_INIT; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> LoxResult<()> {
        let mut parser = Compiler::new(source);
        self.chunk = parser.compile()?;
        self.ip = 0;

        self.run()
    }

    fn run(&mut self) -> LoxResult<()> {
        loop {
            let byte = self.read_byte();

            #[cfg(debug_assertions)]
            self.debug(byte);

            let maybe_instruction: LoxResult<Instruction> = byte.try_into();

            if let Ok(instruction) = maybe_instruction {
                macro_rules! binary {
                    ($kind:ident, $op:tt) => {{
                        if let (Value::Number(a), Value::Number(b)) = self.peek_pair() {
                            self.pop_pair();
                            self.push(Value::$kind(a $op b));
                        } else {
                            self.error("Operands must be numbers.");
                            return Err(LoxError::RuntimeError)
                        }
                    }};
                }

                use Instruction::*;
                match instruction {
                    Constant => {
                        let constant = self.read_constant();
                        self.push(constant);
                    }

                    Nil => self.push(Value::Nil),
                    True => self.push(Value::Boolean(true)),
                    False => self.push(Value::Boolean(false)),

                    Greater => binary!(Boolean, >),
                    Less => binary!(Boolean, <),

                    Equal => {
                        let (a, b) = self.pop_pair();
                        self.push(Value::Boolean(a == b))
                    }

                    Add => match self.peek_pair() {
                        (Value::String(a), Value::String(b)) => {
                            self.pop_pair();
                            self.push(Value::String(Box::new(*a + &*b)));
                        }

                        (Value::Number(a), Value::Number(b)) => {
                            self.pop_pair();
                            self.push(Value::Number(a + b));
                        }

                        _ => self.error("Operands must be two numbers or two strings."),
                    },

                    Subtract => binary!(Number, -),
                    Multiply => binary!(Number, *),
                    Divide => binary!(Number, /),

                    Not => {
                        let a = self.pop().truthy();
                        self.push(Value::Boolean(!a));
                    }

                    Negate => {
                        if let Value::Number(a) = self.peek(0) {
                            self.pop();
                            self.push(Value::Number(-a));
                        } else {
                            self.error("Operand must be a number");
                            return Err(LoxError::RuntimeError);
                        }
                    }

                    Print => println!("{}", self.pop()),
                    
                    Pop => {
                        self.pop();
                    }

                    Return => return Ok(()),
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self
            .chunk
            .read(self.ip)
            .expect("VM Instruction Pointer out of bounds");
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        self.chunk
            .read_constant(index)
            .expect("VM Read Constant Out of Bounds")
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top].clone()
    }

    fn pop_pair(&mut self) -> (Value, Value) {
        let b = self.pop();
        let a = self.pop();
        (a, b)
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top - 1 - distance].clone()
    }

    fn peek_pair(&self) -> (Value, Value) {
        (self.peek(1), self.peek(0))
    }

    fn error(&mut self, message: &str) {
        // Something something variadic? 🤷‍♀️
        eprintln!("{}", message);

        let line = self.chunk.line(-1);
        eprintln!("[line {}] in script", line);
        self.stack_top = 0;
    }

    #[cfg(debug_assertions)]
    fn debug(&self, byte: u8) {
        for slot in 0..self.stack_top {
            println!("[ {} ]", self.stack[slot]);
        }
        self.chunk.disassemble_instruction(byte, self.ip - 1);
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
