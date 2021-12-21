use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum Bit {
    Zero = 0,
    One = 1,
}

impl Bit {
    pub fn from_byte(b: u8) -> [Bit; 8] {
        [
            (((b >> 7) & 0b1) == 1).into(),
            (((b >> 6) & 0b1) == 1).into(),
            (((b >> 5) & 0b1) == 1).into(),
            (((b >> 4) & 0b1) == 1).into(),
            (((b >> 3) & 0b1) == 1).into(),
            (((b >> 2) & 0b1) == 1).into(),
            (((b >> 1) & 0b1) == 1).into(),
            ((b & 0b1) == 1).into(),
        ]
    }

    pub fn from_char(c: char) -> Result<[Bit; 4], Error> {
        use Bit::*;
        match c {
            '0' => Ok([Zero, Zero, Zero, Zero]),
            '1' => Ok([Zero, Zero, Zero, One]),
            '2' => Ok([Zero, Zero, One, Zero]),
            '3' => Ok([Zero, Zero, One, One]),
            '4' => Ok([Zero, One, Zero, Zero]),
            '5' => Ok([Zero, One, Zero, One]),
            '6' => Ok([Zero, One, One, Zero]),
            '7' => Ok([Zero, One, One, One]),
            '8' => Ok([One, Zero, Zero, Zero]),
            '9' => Ok([One, Zero, Zero, One]),
            'A' | 'a' => Ok([One, Zero, One, Zero]),
            'B' | 'b' => Ok([One, Zero, One, One]),
            'C' | 'c' => Ok([One, One, Zero, Zero]),
            'D' | 'd' => Ok([One, One, Zero, One]),
            'E' | 'e' => Ok([One, One, One, Zero]),
            'F' | 'f' => Ok([One, One, One, One]),
            _ => Err(anyhow!("expected a hexadecimal character; got {}", c)),
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

impl<'a> Sum<&'a Bit> for usize {
    fn sum<I: Iterator<Item = &'a Bit>>(iter: I) -> Self {
        iter.into_iter().fold(Default::default(), Add::add)
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

pub(super) struct BitVector(Vec<Bit>);

::aoc::derive_FromIterator!(BitVector, Bit);

impl BitVector {
    fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop(&mut self) -> Option<Bit> {
        self.0.pop()
    }

    pub fn take_array<const LEN: usize>(&mut self) -> Option<[Bit; LEN]> {
        if self.0.len() < LEN {
            None
        } else {
            let mut rv = [Bit::Zero; LEN];
            for v in rv.iter_mut() {
                *v = self.0.pop().unwrap();
            }
            Some(rv)
        }
    }
}

impl FromIterator<u8> for BitVector {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        iter.into_iter().flat_map(Bit::from_byte).collect()
    }
}

impl FromStr for BitVector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this: Self = s
            .trim()
            .chars()
            .map(Bit::from_char)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        this.reverse();
        Ok(this)
    }
}

impl Display for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0.iter().rev() {
            write!(f, "{}", *b as u8)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[macros::problem]
    fn problem(v: &super::BitVector) -> String {
        v.to_string()
    }

    ::aoc::tests_for_problem!(Problem, {
        example_a => "110100101111111000101000",
        example_b => "00111000000000000110111101000101001010010001001000000000",
        example_c => "11101110000000001101010000001100100000100011000001100000",
        example_d => "100010100000000001001010100000000001101010000000000000101111010001111000",
        example_e => "01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100",
        example_f => "1100000000000001010100000000000000000001011000010001010110100010111000001000000000101111000110000010001101000000",
        example_g => "101000000000000101101100100010000000000101100010000000010111110000110110100001101011000110001010001111010100011110000000",
        example_h => "1100001000000000101101000000101010000010",
        example_i => "000001000000000001011010110000110011100010010000",
        example_j => "10001000000000001000011011000011111010001000000100010010",
        example_k => "11001110000000001100010000111101100010000001000100100000",
        example_l => "110110000000000001011010110000101010100011110000",
        example_m => "1111011000000000101111000010110110001111",
        example_n => "100111000000000001011010110000101111100011110000",
        example_o => "10011100000000010100000100001000000000100101000000110010000011110001100000000010000100000100101000001000",
    });
}
