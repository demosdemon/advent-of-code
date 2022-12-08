// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
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

    pub fn from_char(c: u8) -> Result<[Bit; 4], Error> {
        use Bit::*;
        match c {
            b'0' => Ok([Zero, Zero, Zero, Zero]),
            b'1' => Ok([Zero, Zero, Zero, One]),
            b'2' => Ok([Zero, Zero, One, Zero]),
            b'3' => Ok([Zero, Zero, One, One]),
            b'4' => Ok([Zero, One, Zero, Zero]),
            b'5' => Ok([Zero, One, Zero, One]),
            b'6' => Ok([Zero, One, One, Zero]),
            b'7' => Ok([Zero, One, One, One]),
            b'8' => Ok([One, Zero, Zero, Zero]),
            b'9' => Ok([One, Zero, Zero, One]),
            b'A' | b'a' => Ok([One, Zero, One, Zero]),
            b'B' | b'b' => Ok([One, Zero, One, One]),
            b'C' | b'c' => Ok([One, One, Zero, Zero]),
            b'D' | b'd' => Ok([One, One, Zero, One]),
            b'E' | b'e' => Ok([One, One, One, Zero]),
            b'F' | b'f' => Ok([One, One, One, One]),
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

#[derive(macros::FromIterator, macros::TryFromStr)]
#[from_iterator(Bit)]
pub struct BitVector(Vec<Bit>);

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
            .bytes()
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
    fn solve(v: super::BitVector) -> String {
        v.to_string()
    }

    ::aoc::tests_for_problem!(solve, {
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
