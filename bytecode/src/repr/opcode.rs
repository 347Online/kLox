use super::error::LoxError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Constant,

    Return,
}

const MAX_OPCODE: u8 = Opcode::Return as u8;

impl TryFrom<u8> for Opcode {
    type Error = LoxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=MAX_OPCODE).contains(&value) {
            let opcode = unsafe { std::mem::transmute(value) };
            Ok(opcode)
        } else {
            let error = LoxError::compile("Unknown opcode");
            Err(error)
        }
    }
}
