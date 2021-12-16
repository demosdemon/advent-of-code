use std::ops::{Add, Not};

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Bit {
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

impl<'a> Add<&'a Bit> for usize {
    type Output = Self;

    fn add(mut self, rhs: &'a Bit) -> Self::Output {
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

impl TryFrom<char> for Bit {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            _ => Err(anyhow!("invalid bit char; got {}", value)),
        }
    }
}
