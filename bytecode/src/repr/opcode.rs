use super::error::LoxError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    Constant,

    Return,
}

const MAX_OPCODE: u8 = Instruction::Return as u8;

impl TryFrom<u8> for Instruction {
    type Error = LoxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=MAX_OPCODE).contains(&value) {
            let instruction = unsafe { std::mem::transmute(value) };
            Ok(instruction)
        } else {
            let error = LoxError::compile("Unknown opcode");
            Err(error)
        }
    }
}
