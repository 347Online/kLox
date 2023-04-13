use super::instruction::Instruction;

#[derive(Debug)]
pub struct Chunk {
    name: String,
    code: Vec<Instruction>,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Chunk {
            name: name.to_string(),
            code: vec![],
        }
    }

    pub fn write(&mut self, instruction: Instruction) {
        self.code.push(instruction);
        // instruction
        //     .as_bytes()
        //     .iter()
        //     .for_each(|byte| self.code.push(*byte));
    }

    pub fn disassemble(&self) -> String {
        let mut chunk_str = format!("== {} ==", self.name);
        let mut offset = 0;
        
        for instruction in self.code.iter() {
            let (instruction, len) = instruction.disassemble();

            chunk_str.push_str(&format!("\n{:04} {}", offset, instruction));
            offset += len
        }

        chunk_str
    }
}
