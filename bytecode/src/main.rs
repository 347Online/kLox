use bytecode::{exec::vm::VirtualMachine, repr::{chunk::Chunk, opcode::Instruction}};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(Instruction::Constant, 123);
    chunk.write_byte(constant, 123);

    
    let constant = chunk.add_constant(3.4);
    chunk.write(Instruction::Constant, 123);
    chunk.write_byte(constant, 123);
    
    chunk.write(Instruction::Add, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(Instruction::Constant, 123);
    chunk.write_byte(constant, 123);

    chunk.write(Instruction::Divide, 123);
    chunk.write(Instruction::Negate, 123);

    chunk.write(Instruction::Return, 123);



    let mut vm = VirtualMachine::new();
    let result = vm.interpret(chunk);

    println!("Result: {:?}", result);

    // let args: Vec<String> = std::env::args().collect();
    // let len = args.len();

    // match len {
    //     1 => run_prompt(),
    //     2 => run_file(&args[1]),
    //     _ => println!("Usage: klox [script]"),
    // }
}
