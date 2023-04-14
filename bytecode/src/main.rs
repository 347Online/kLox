use crate::lox::{chunk::Chunk, instruction::Instruction, vm::VirtualMachine};

pub mod lox;

fn main() {
    let mut chunk = Chunk::new("test chunk");
    let constant = chunk.add_constant(1.2);
    chunk.write(Instruction::Constant(constant), 123);
    chunk.write(Instruction::Return, 123);

    let mut vm = VirtualMachine::new();
    let _ = vm.interpret(chunk);
}
