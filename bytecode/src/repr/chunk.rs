use std::fmt::Debug;

use super::{opcode::Instruction, value::Value};

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, instruction: Instruction, line: usize) {
        self.write_byte(instruction as u8, line);
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_byte_at(&mut self, byte: u8, offset: usize) {
        self.code[offset] = byte;
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read(&self, offset: usize) -> Option<u8> {
        self.code.get(offset).cloned()
    }

    pub fn read_constant(&self, index: u8) -> Option<Value> {
        self.constants.get(index as usize).cloned()
    }

    pub fn line(&self, pos: i32) -> usize {
        if pos.is_negative() {
            let len = self.lines.len();
            self.lines[len - pos.unsigned_abs() as usize]
        } else {
            self.lines[pos as usize]
        }
    }

    pub fn len(&self) -> usize {
        #[cfg(debug_assertions)]
        assert_eq!(self.code.len(), self.lines.len(), "Chunk Error — Size mismatch");

        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(debug_assertions)]
impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while let Some(byte) = self.read(offset) {
            offset = self.disassemble_instruction(byte, offset);
        }
    }

    pub fn disassemble_instruction(&self, byte: u8, offset: usize) -> usize {
        use crate::repr::error::LoxResult;
        use Instruction::*;

        let maybe_instruction: LoxResult<Instruction> = byte.try_into();

        match maybe_instruction {
            Ok(instruction) => {
                print!("{:04} ", offset);
                if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
                    print!("   | ")
                } else {
                    print!("{:>4} ", self.lines[offset])
                };

                match instruction {
                    Constant | DefineGlobal | SetGlobal | GetGlobal => {
                        let index = self.code[offset + 1];
                        let constant = self.constants[index as usize].clone();
                        println!("{:<16?} {:>4} '{}'", instruction, index, constant);
                        offset + 2
                    }

                    GetLocal | SetLocal => {
                        let slot = self.code[offset + 1];
                        println!("{:<16?} {:>4}", self, slot);
                        offset + 2
                    }

                    Jump | JumpIfFalse => {
                        let addr_a = self.code[offset + 1];
                        let addr_b = self.code[offset + 2];

                        let jump = u16::from_be_bytes([addr_a, addr_b]);

                        println!("{:<16?} {:>4} -> {}", instruction, offset, offset + 3 + jump as usize);
                        offset + 3
                    }

                    _ => {
                        println!("{:?}", instruction);
                        offset + 1
                    }
                }
            }

            Err(e) => {
                println!("{e}");
                offset + 1
            }
        }
    }
}
