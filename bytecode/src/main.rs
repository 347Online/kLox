use crate::lox::{chunk::Chunk, instruction::Instruction};

pub mod lox;

fn main() {
    let mut chunk = Chunk::new("test chunk");
    chunk.write(Instruction::Constant(9));
    chunk.write(Instruction::Return);
    println!("{}", chunk.disassemble());
}
