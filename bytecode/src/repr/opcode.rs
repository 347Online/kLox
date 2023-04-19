use super::error::LoxError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Constant,

    Add,
    Subtract,
    Multiply,
    Divide,

    Negate,

    Return,
}

const MAX_OPCODE: u8 = Instruction::Return as u8;

impl TryFrom<u8> for Instruction {
    type Error = LoxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=MAX_OPCODE).contains(&value) {
            // SAFETY:
            // MAX_OPCODE is derived from Instruction::Return, the final variant
            // Since Instruction is defined as repr(u8), the variants form a contiguous range
            // any u8 value less than or equal to Instruction::Return as u8 is a valid instruction
            let instruction = unsafe { std::mem::transmute(value) };
            Ok(instruction)
        } else {
            let error = LoxError::compile("Unknown opcode");
            Err(error)
        }
    }
}
