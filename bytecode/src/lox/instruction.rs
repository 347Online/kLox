#[derive(Debug)]
#[repr(u8)]
pub enum Instruction {
    Return,
    Negate,
    Constant(u8),
}

use Instruction::*;
impl Instruction {
    pub fn opcode(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        //
        // Copied verbatim from https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let code = self.opcode();

        macro_rules! simple {
            () => {
                vec![code]
            }
        }

        match self {
            Constant(index) => vec![code, *index],
            Return => simple!(),
            Negate => simple!(),
        }
    }
}