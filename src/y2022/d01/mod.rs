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

use std::cmp::Ordering;
use std::iter::Sum;

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

#[derive(macros::TryFromStr)]
pub struct Elves(Vec<Elf>);

pub struct Elf(Vec<usize>);

::aoc::derive_FromStr_for_nom!(Elves, parser::elves);

impl Elf {
    pub fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.sum() == other.sum()
    }
}

impl Eq for Elf {}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sum().partial_cmp(&other.sum())
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum().cmp(&other.sum())
    }
}

impl<'a> Sum<&'a Elf> for usize {
    fn sum<I: Iterator<Item = &'a Elf>>(iter: I) -> Self {
        iter.map(|e| e.sum()).sum()
    }
}
