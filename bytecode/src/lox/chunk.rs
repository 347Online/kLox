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

    pub fn disassemble(&self) -> String {
        let mut chunk = format!("== {} ==", self.name);
        let mut offset = 0;
        for (i, instruction) in self.code.iter().enumerate() {
            let (instruction, len) = instruction.disassemble();

            let line = if i > 0 && self.lines[i] == self.lines[i-1] {
                String::from("  |")
            } else {
                self.lines[i].to_string()
            };
            chunk.push_str(&format!("\n{:04}  {} {}", offset, line, instruction));
            offset += len
        }

        chunk
    }
}
