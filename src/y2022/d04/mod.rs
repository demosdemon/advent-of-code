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

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

use std::ops::RangeInclusive;

struct Assignment(RangeInclusive<u8>);

impl Assignment {
    fn contains(&self, rhs: &Self) -> bool {
        self.0.contains(rhs.0.start()) && self.0.contains(rhs.0.end())
    }

    fn overlaps(&self, rhs: &Self) -> bool {
        self.0.contains(rhs.0.start()) || self.0.contains(rhs.0.end())
    }
}

impl From<(u8, u8)> for Assignment {
    fn from((start, end): (u8, u8)) -> Self {
        Self(start..=end)
    }
}

struct AssignmentPair(Assignment, Assignment);

::aoc::derive_FromStr_for_nom!(AssignmentPair, parser::assignment_pair);

impl From<((u8, u8), (u8, u8))> for AssignmentPair {
    fn from(((start1, end1), (start2, end2)): ((u8, u8), (u8, u8))) -> Self {
        Self(
            Assignment::from((start1, end1)),
            Assignment::from((start2, end2)),
        )
    }
}

impl AssignmentPair {
    /// The pair is inclusive if either assignment range is wholly contained by
    /// the other.
    fn is_inclusive(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    /// The pair overlaps if any part of the range of one assignment is
    /// contained by the other.
    fn is_overlapping(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

#[derive(macros::FromLines)]
#[from_lines(AssignmentPair)]
struct Assignments(Vec<AssignmentPair>);
