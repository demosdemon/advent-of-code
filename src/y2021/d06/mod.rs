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

use std::str::FromStr;

use anyhow::Context;
use anyhow::Error;
use anyhow::Result;

#[derive(macros::FromIterator, macros::TryFromStr)]
#[from_iterator(u8)]
pub struct Ocean(Vec<u8>);

impl FromStr for Ocean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .next()
            .context("readling input line")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .context("parsing input values")
    }
}

impl Ocean {
    pub fn count(&self, days: usize) -> usize {
        let mut lanterns = [0; 9];
        for &lantern in self.0.iter() {
            assert!(lantern < 9);
            lanterns[lantern as usize] += 1;
        }
        for _ in 0..days {
            let finished = lanterns[0];
            for idx in 0..8 {
                lanterns[idx] = lanterns[idx + 1];
            }
            lanterns[8] = finished;
            lanterns[6] += finished;
        }
        lanterns.into_iter().sum()
    }
}
