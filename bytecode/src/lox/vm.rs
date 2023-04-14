use crate::lox::instruction::Instruction;

use super::{error::LoxError, chunk::Chunk, value::Value};

type InterpretResult = Result<(), LoxError>;

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
    
    //TODO: take in source instead
    pub fn interpret(&mut self, source: Chunk) -> InterpretResult {
        self.chunk = Some(source);

        self.run()
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_ptr] = value;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr]
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