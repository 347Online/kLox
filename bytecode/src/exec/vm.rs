use crate::repr::{chunk::Chunk, error::LoxResult, opcode::Instruction, value::Value};

const STACK_MAX: usize = 256;

pub struct VirtualMachine {
    ip: usize,
    chunk: Chunk,
    stack: [Value; STACK_MAX],
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            chunk: Chunk::new(),
            stack: [0.0; STACK_MAX]
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
            let instruction: LoxResult<Instruction> = byte.try_into();

            match instruction {
                Ok(opcode) => {
                    #[cfg(debug_assertions)]
                    self.chunk.disassemble_instruction(byte, self.ip - 1);

                    use Instruction::*;

                    match opcode {
                        Constant => {
                            let constant = self.read_constant();
                            println!("{constant}");
                        }

                        Return => break,
                    }
                }

                Err(e) => println!("{e}"),
            }
        }

        Ok(())
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
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
