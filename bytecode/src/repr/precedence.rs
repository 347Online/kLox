use super::{error::LoxError, token::TokenType};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Precedence {
    Min,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

const MAX_PREC: u8 = Precedence::Primary as u8;

impl TryFrom<u8> for Precedence {
    type Error = LoxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (0..=MAX_PREC).contains(&value) {
            // SAFETY:
            // MAX_PREC is derived from Precedence::Primary, the final variant
            // Since Instruction is defined as repr(u8), the variants form a contiguous range
            // any u8 value less than or equal to Precedence::Primary as u8 is valid
            let prec = unsafe { std::mem::transmute(value) };
            Ok(prec)
        } else {
            Err(LoxError::CompileError)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseFn {
    Literal,
    Unary,
    Binary,
    Grouping,
    Num,
    Null,
}

pub struct Rule {
    prefix: ParseFn,
    infix: ParseFn,
    prec: Precedence,
}

impl Rule {
    pub fn prefix(&self) -> ParseFn {
        self.prefix
    }
    pub fn infix(&self) -> ParseFn {
        self.infix
    }

    pub fn prec(&self) -> Precedence {
        self.prec
    }
}

impl From<TokenType> for Rule {
    fn from(value: TokenType) -> Self {
        use TokenType::*;
        let (prefix, infix, prec) = match value {
            LeftParen => (ParseFn::Grouping, ParseFn::Null, Precedence::Min),

            Bang => (ParseFn::Unary, ParseFn::Null, Precedence::Min),

            BangEqual | EqualEqual => (ParseFn::Null, ParseFn::Binary, Precedence::Equality),
            Greater | GreaterEqual | Less | LessEqual => {
                (ParseFn::Null, ParseFn::Binary, Precedence::Comparison)
            }

            Minus => (ParseFn::Unary, ParseFn::Binary, Precedence::Term),
            Plus => (ParseFn::Null, ParseFn::Binary, Precedence::Term),

            Star | Slash => (ParseFn::Null, ParseFn::Binary, Precedence::Factor),

            Number => (ParseFn::Num, ParseFn::Null, Precedence::Min),

            Nil | True | False => (ParseFn::Literal, ParseFn::Null, Precedence::Min),

            _ => (ParseFn::Null, ParseFn::Null, Precedence::Min),
        };

        Rule {
            prefix,
            infix,
            prec,
        }
    }
}
