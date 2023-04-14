use crate::lox::{chunk::Chunk, instruction::Instruction, vm::VirtualMachine};

pub mod lox;

fn main() {
    use Instruction::*;

    let mut chunk = Chunk::new("test chunk");

    let constant = chunk.add_constant(1.2);
    chunk.write(Constant(constant), 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(Constant(constant), 123);
    chunk.write(Add, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(Constant(constant), 123);
    chunk.write(Divide, 123);
    chunk.write(Negate, 123);
    chunk.write(Return, 123);

    let mut vm = VirtualMachine::new();
    let _ = vm.interpret(chunk);
}
