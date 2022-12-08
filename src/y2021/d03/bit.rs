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

use std::ops::Add;
use std::ops::Not;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        (*value).into()
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
