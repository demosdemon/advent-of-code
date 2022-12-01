use std::ops::Add;
use std::ops::Not;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

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

impl Add<&Bit> for usize {
    type Output = Self;

    fn add(mut self, rhs: &Bit) -> Self::Output {
        self <<= 1;
        self += *rhs as Self;
        self
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

impl From<&Bit> for bool {
    fn from(value: &Bit) -> Self {
        value.to_owned().into()
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl TryFrom<u8> for Bit {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'0' => Ok(Self::Zero),
            b'1' => Ok(Self::One),
            _ => Err(anyhow!("invalid bit char; got {}", value)),
        }
    }
}
