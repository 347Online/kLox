use crate::lox::instruction::Instruction;

use super::{error::LoxError, chunk::Chunk, value::Value, compiler::compile};

pub type InterpretResult = Result<(), LoxError>;

const STACK_MAX: usize = 256;

pub struct VirtualMachine {
    chunk: Option<Chunk>,
    stack: [Value; STACK_MAX],
    stack_ptr: usize,
}

impl VirtualMachine {

    pub fn new() -> Self {
        VirtualMachine {
            chunk: None,
            stack: [0.0; STACK_MAX],
            stack_ptr: 0,
        }
    }
    
    pub fn interpret(&mut self, source: String) -> InterpretResult {
        compile(source);
        Ok(())
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_ptr] = value;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr]
    }

    pub fn pair(&mut self) -> (Value, Value) {
        let b = self.pop();
        let a = self.pop();

        (a, b)
    }

    pub fn binary(&mut self, f: impl FnOnce(Value, Value) -> Value) {
        let (a, b) = self.pair();
        self.push(f(a, b))
    }

    fn run(&mut self) -> InterpretResult {
        let chunk = self.chunk.take().unwrap();
        
        #[cfg(debug_assertions)]
        println!("{}", chunk.disassemble());

        use Instruction::*;
        for instruction in chunk.instructions() {

            match instruction {
                Constant(index) => {
                    let constant = chunk.read_constant(*index as usize);
                    self.push(constant);
                },

                Add => self.binary(|x, y| x + y),
                Subtract => self.binary(|x, y| x - y),
                Multiply => self.binary(|x, y| x * y),
                Divide => self.binary(|x, y| x / y),

                Negate => {
                    let a = self.pop();
                    self.push(-a);
                }

                Return => {
                    let value = self.pop();
                    println!("{}", value);
                    return Ok(())
                },
            }
        }

        todo!()
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}