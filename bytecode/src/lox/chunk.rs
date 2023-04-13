use super::{instruction::Instruction, value::Value};

#[derive(Debug)]
pub struct Chunk {
    name: String,
    constants: Vec<Value>,
    code: Vec<Instruction>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Chunk {
            name: name.to_string(),
            constants: vec![],
            code: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, instruction: Instruction, line: usize) {
        self.code.push(instruction);
        self.lines.push(line);
        // instruction
        //     .as_bytes()
        //     .iter()
        //     .for_each(|byte| self.code.push(*byte));
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        let len = self.constants.len() as u8;
        self.constants.push(value);
        len
    }

    pub fn disassemble(&self) -> String {
        let mut chunk = format!("== {} ==", self.name);
        let mut offset = 0;
        for (i, instruction) in self.code.iter().enumerate() {
            let (instruction, len) = self.disassemble_instruction(instruction);

            let line = if i > 0 && self.lines[i] == self.lines[i - 1] {
                String::from("  |")
            } else {
                self.lines[i].to_string()
            };
            chunk.push_str(&format!("\n{:04}  {} {}", offset, line, instruction));
            offset += len
        }

        chunk
    }

    pub fn disassemble_instruction(&self, instruction: &Instruction) -> (String, usize) {
        let len = instruction.as_bytes().len();

        fn simple(name: &str) -> String {
            name.to_string()
        }

        use Instruction::*;
        const WIDTH: usize = 16;
        let repr = match instruction {
            Return => simple("Return"),
            Constant(index) => {
                let constant = &self.constants[*index as usize];
                format!("{:<WIDTH$} {:>4} '{}'", "Constant", index, constant)
            }
        };

        (repr, len)
    }
}
