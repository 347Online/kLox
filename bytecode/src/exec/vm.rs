use crate::repr::{chunk::Chunk, error::{LoxResult, LoxError}, opcode::Instruction, value::Value};

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
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> LoxResult<()> {
        self.chunk = chunk;
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

                    match instruction {
                        Constant => {
                            let constant = self.read_constant();
                            self.push(constant);
                        }

                        Add => {
                            let (a, b) = self.pop_pair();
                            self.push(a + b)
                        }
                        Subtract => {
                            let (a, b) = self.pop_pair();
                            self.push(a - b)
                        }
                        Multiply => {
                            let (a, b) = self.pop_pair();
                            self.push(a * b)
                        }
                        Divide => {
                            let (a, b) = self.pop_pair();
                            self.push(a / b)
                        }

                        Negate => {
                            let a = self.pop();
                            self.push(-a);
                        }

                        Return => {
                            println!("{}", self.pop());
                            return Ok(())
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

    fn pop_pair(&mut self) -> (Value, Value) {
        let b = self.pop();
        let a = self.pop();
        (a, b)
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
