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

pub(crate) mod part1;
pub(crate) mod part2;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(u8);

impl Priority {
    pub fn score(&self) -> usize {
        self.0 as usize
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            1..=26 => (b'a' + self.0 - 1) as char,
            27..=52 => (b'A' + self.0 - 27) as char,
            _ => unreachable!(),
        };
        write!(f, "{c}")
    }
}

impl TryFrom<u8> for Priority {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' => Ok(Self(value - b'a' + 1)),
            b'A'..=b'Z' => Ok(Self(value - b'A' + 27)),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromBytes)]
#[from_bytes(Priority)]
pub struct Sack(Vec<Priority>);

impl Sack {
    pub fn partition_mut(&mut self) -> (&mut [Priority], &mut [Priority]) {
        let mid = self.0.len() / 2;
        self.0.split_at_mut(mid)
    }
}

// ::aoc::derive_FromStr_for_bytes_TryFrom_collect!(Sack, Priority);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromLines)]
#[from_lines(Sack)]
pub struct Sacks(Vec<Sack>);
