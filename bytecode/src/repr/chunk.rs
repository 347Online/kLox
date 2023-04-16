use super::{instruction::Instruction, value::Value};

#[derive(Default, Debug)]
pub struct Chunk {
    code: Vec<Instruction>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            constants: vec![],
            code: vec![],
            lines: vec![],
        }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.code
    }

    pub fn read_constant(&self, index: usize) -> Value {
        self.constants[index]
    }

    pub fn write(&mut self, instruction: Instruction, line: usize) {
        self.code.push(instruction);
        self.lines.push(line); // TODO: This is "hilariously wasteful" of memory
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let len = self.constants.len();
        self.constants.push(value);
        len
    }

    #[cfg(debug_assertions)]
    pub fn disassemble(&self, name: &str) -> String {
        let mut chunk = format!("== {} ==", name);
        let mut offset = 0;
        for (i, instruction) in self.code.iter().enumerate() {
            let (instruction, len) = self.disassemble_instruction(instruction);

            let line = if i > 0 && self.lines[i] == self.lines[i - 1] {
                String::from("   |")
            } else {
                format!("{:>4}", self.lines[i])
            };
            chunk.push_str(&format!("\n{:04}  {} {}", offset, line, instruction));
            offset += len
        }

        chunk
    }

    #[cfg(debug_assertions)]
    pub fn disassemble_instruction(&self, instruction: &Instruction) -> (String, usize) {
        use Instruction::*;

        macro_rules! simple {
            () => {
                format!("{:?}", instruction)
            };
        }

        const WIDTH: usize = 16;

        let len = instruction.as_bytes().len();

        let repr = match instruction {
            Return => simple!(),

            Add => simple!(),
            Subtract => simple!(),
            Multiply => simple!(),
            Divide => simple!(),

            Negate => simple!(),

            Constant(index) => {
                let constant = &self.constants[*index as usize];
                format!("{:<WIDTH$} {:>4} '{}'", "Constant", index, constant)
            }
        };

        (repr, len)
    }
}
