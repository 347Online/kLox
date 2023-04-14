use crate::lox::instruction::Instruction;

use super::{error::LoxError, chunk::Chunk};

type InterpretResult = Result<(), LoxError>;

pub struct VirtualMachine {
    chunk: Option<Chunk>,
    ptr: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            chunk: None,
            ptr: 0
        }
    }
    
    //TODO: take in source instead
    pub fn interpret(&mut self, source: Chunk) -> InterpretResult {
        self.chunk = Some(source);

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let chunk = self.chunk.take().unwrap();
        use Instruction::*;
        for instruction in chunk.instructions() {
            match instruction {
                Constant(index) => {
                    let constant = chunk.read_constant(*index as usize);
                    println!("{}", constant);
                },
                Return => return Ok(()),
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