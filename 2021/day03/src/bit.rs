use std::ops::Not;

use crate::errors::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl From<Bit> for bool {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::Zero => false,
            Bit::One => true,
        }
    }
}

impl TryFrom<char> for Bit {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            _ => Err(Error::InvalidBit(value)),
        }
    }
}
