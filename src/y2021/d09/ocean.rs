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

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

use aoc::matrix::Matrix;
use aoc::matrix::Position;
use aoc::matrix::RelativePosition;

const SURROUNDING: [RelativePosition; 4] = [
    RelativePosition::TopCenter,
    RelativePosition::MiddleLeft,
    RelativePosition::MiddleRight,
    RelativePosition::BottomCenter,
];

#[derive(Debug, macros::TryFromStr)]
pub struct Ocean(Matrix<u8>);

impl Ocean {
    pub fn basins(&self) -> Vec<usize> {
        self.iter_low_points()
            .map(|(root, &root_value)| {
                let mut basin = BTreeSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((root, root_value));
                while let Some((point, point_value)) = queue.pop_front() {
                    if basin.insert(point.to_tuple()) {
                        for (idx, &adj_value) in self.iter_surrounding(point) {
                            if adj_value < 9 && point_value < adj_value {
                                queue.push_back((idx, adj_value));
                            }
                        }
                    }
                }
                basin.len()
            })
            .collect()
    }

    pub fn iter_low_points(&self) -> impl Iterator<Item = (Position, &u8)> {
        self.0
            .iter()
            .filter(|&(idx, value)| self.is_low_point(idx, *value))
    }

    fn iter_surrounding(&self, pos: Position) -> impl Iterator<Item = (Position, &u8)> + '_ {
        self.0.iter_rel(pos, SURROUNDING)
    }

    fn is_low_point(&self, idx: Position, value: u8) -> bool {
        match value {
            0 => true,
            9 => false,
            _ => self.iter_surrounding(idx).all(|(_, &v)| value < v),
        }
    }
}

impl FromStr for Ocean {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|l| l.bytes().map(::aoc::chardigit)).collect(),
        ))
    }
}
