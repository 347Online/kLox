use crate::lox::{chunk::Chunk, instruction::Instruction};

pub mod lox;

fn main() {
    let mut chunk = Chunk::new("test chunk");
    chunk.write(Instruction::Constant(9), 123);
    chunk.write(Instruction::Return, 123);
    println!("{}", chunk.disassemble());
}
