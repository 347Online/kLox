use crate::repr::{
    chunk::Chunk,
    error::{LoxResult, LoxError},
    opcode::Instruction,
    value::Value,
};

use super::compiler::Compiler;

const STACK_MAX: usize = 256;

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
            stack: [Value::Nil; STACK_MAX],
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

            match maybe_instruction {
                Ok(instruction) => {
                    use Instruction::*;

                    macro_rules! binary {
                        ($kind:ident, $op:tt) => {{
                            // let (a, b) = self.pop_pair();
                            // self.push($kind(a $op b));
                            if let (Value::Number(a), Value::Number(b)) = (self.peek(1), self.peek(0)) {
                                self.pop_pair();
                                self.push(Value::$kind(a $op b));
                            } else {
                                self.error("Operands must be numbers.");
                                return Err(LoxError::runtime("Operands must be numbers."))
                            }
                        }};
                    }

                    match instruction {
                        Constant => {
                            let constant = self.read_constant();
                            self.push(constant);
                        }

                        Nil => self.push(Value::Nil),
                        True => self.push(Value::Boolean(true)),
                        False => self.push(Value::Boolean(false)),

                        Equal => {
                            let (a, b) = self.pop_pair();
                            self.push(Value::Boolean(a == b))
                        }

                        Greater => binary!(Boolean, >),
                        Less => binary!(Boolean, <),

                        Add => binary!(Number, +),
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
                                return Err(LoxError::runtime("Operand must be a number."))
                            }
                        },

                        Return => {
                            println!("{}", self.pop());
                            return Ok(());
                        }
                    }
                }

                Err(e) => println!("{e}"),
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
        self.stack[self.stack_top]
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top - 1 - distance]
    }

    fn pop_pair(&mut self) -> (Value, Value) {
        let b = self.pop();
        let a = self.pop();
        (a, b)
    }

    fn error(&mut self, message: &str) {
        eprintln!("{}", message)
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
