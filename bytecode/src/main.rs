use bytecode2::{repr::{chunk::Chunk, opcode::Opcode, error::LoxResult}, exec::vm::VirtualMachine};

fn main() {

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);

    chunk.write(Opcode::Constant, 123);
    chunk.write_byte(constant as u8, 123);

    chunk.write(Opcode::Return, 123);
    chunk.disassemble("test chunk");

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(chunk);

    println!("{:?}", result);

    // let args: Vec<String> = std::env::args().collect();
    // let len = args.len();

    // match len {
    //     1 => run_prompt(),
    //     2 => run_file(&args[1]),
    //     _ => println!("Usage: klox [script]"),
    // }
}
