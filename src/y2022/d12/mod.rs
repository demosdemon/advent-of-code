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

use std::collections::BinaryHeap;
use std::convert::Infallible;

use aoc::matrix::Matrix;
use aoc::matrix::Position;
use aoc::matrix::RelativePosition;
use hashbrown::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    matrix: Matrix<u8>,
    start: Position,
    dest: Position,
}

const REL: [RelativePosition; 4] = [
    RelativePosition::TopCenter,
    RelativePosition::MiddleLeft,
    RelativePosition::MiddleRight,
    RelativePosition::BottomCenter,
];

impl Input {
    fn shortest_path(&self, start: impl IntoIterator<Item = Position>) -> usize {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        for pos in start {
            heap.push((0isize, Step { input: self, pos }));
        }

        while let Some((cost, next)) = heap.pop() {
            let ucost = (-cost) as usize;

            if next.pos == self.dest {
                return ucost;
            }
            if let Some(&c) = dist.get(&next) {
                if c < ucost {
                    continue;
                }
            }
            let h1 = self.matrix[next.pos];
            for (p2, &h2) in self.matrix.iter_rel(next.pos, REL) {
                if h2 <= h1 + 1 {
                    let step = Step {
                        input: self,
                        pos: p2,
                    };
                    let cost = ucost + 1;
                    match dist.entry(step) {
                        hashbrown::hash_map::Entry::Occupied(mut e) => {
                            if *e.get() > cost {
                                *e.get_mut() = cost;
                                heap.push((-(cost as isize), step));
                            }
                        }
                        hashbrown::hash_map::Entry::Vacant(e) => {
                            e.insert(cost);
                            heap.push((-(cost as isize), step));
                        }
                    }
                }
            }
        }

        usize::MAX
    }
}

impl TryFrom<&str> for Input {
    type Error = Infallible;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start = None;
        let mut dest = None;
        let mut cells = Vec::new();
        for (row, line) in value.lines().enumerate() {
            let mut row_cells = Vec::with_capacity(line.len());
            for (col, cell) in line.bytes().enumerate() {
                match cell {
                    b'S' => {
                        start = Some(Position::new(row, col));
                        row_cells.push(0_u8);
                    }
                    b'E' => {
                        dest = Some(Position::new(row, col));
                        row_cells.push(b'z' - b'a');
                    }
                    cell @ b'a'..=b'z' => {
                        row_cells.push(cell - b'a');
                    }
                    _ => unreachable!(),
                }
            }
            cells.push(row_cells);
        }
        Ok(Self {
            matrix: Matrix::from_iter(cells),
            start: start.unwrap(),
            dest: dest.unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Step<'a> {
    input: &'a Input,
    pos: Position,
}

impl<'a> PartialOrd for Step<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if std::ptr::eq(self.input, other.input) {
            Some(self.cmp(other))
        } else {
            None
        }
    }
}

impl<'a> Ord for Step<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let i1 = self.input.matrix.pos_to_idx(self.pos);
        let i2 = other.input.matrix.pos_to_idx(other.pos);
        Ord::cmp(&i1, &i2)
    }
}
