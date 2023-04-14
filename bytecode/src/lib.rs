pub mod repr;
pub mod exec;
pub mod lox;

use std::{
  fs::read_to_string,
  io::{stdin, stdout, ErrorKind, Write},
};

use crate::exec::vm::{VirtualMachine, InterpretResult};

pub fn run_prompt() {
  let mut vm = VirtualMachine::new();

  println!("klox, yet another Lox implementation, Katie Janzen 2023");

  let stdin = stdin();
  let mut stdout = stdout();

  loop {
      print!("> ");
      stdout.flush().expect("Failed to flush stdout");

      let mut line = String::new();
      stdin.read_line(&mut line).expect("Failed to read stdin");

      if line.is_empty() {
          break;
      }

      let result = run(line, &mut vm);
  }
}

pub fn run_file(path: &str) {
  let code = match read_to_string(path) {
      Ok(code) => code,
      Err(error) => match error.kind() {
          ErrorKind::NotFound => {
              eprintln!("File '{}' not found", path);
              return;
          }

          _ => panic!("An error occurred: {}", error),
      },
  };

  let mut vm = VirtualMachine::new();
  let result = run(code, &mut vm);
}

pub fn run(source: String, vm: &mut VirtualMachine) -> InterpretResult {
  vm.interpret(source)
}
