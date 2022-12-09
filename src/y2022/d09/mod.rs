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
use std::str::FromStr;

use euclid::Point2D;
use hashbrown::HashSet;

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
    step: Step,
    distance: usize,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.step, self.distance)
    }
}

::aoc::derive_FromStr_for_nom!(Instruction, parser::instruction);

#[derive(macros::FromLines)]
#[from_lines(Instruction)]
struct Instructions(Vec<Instruction>);

struct Rope<const N: usize> {
    knots: [Point2D<isize, Self>; N],
    tails: HashSet<Point2D<isize, Self>>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        let mut tails = HashSet::new();
        tails.insert(Point2D::new(0, 0));
        Self {
            knots: [Point2D::new(0, 0); N],
            tails,
        }
    }

    const fn tail(&self) -> Point2D<isize, Self> {
        self.knots[N - 1]
    }

    fn num_visited(&self) -> usize {
        self.tails.len()
    }

    fn step(&mut self, step: Step) {
        match step {
            Step::Up => self.knots[0].y += 1,
            Step::Down => self.knots[0].y -= 1,
            Step::Left => self.knots[0].x -= 1,
            Step::Right => self.knots[0].x += 1,
        }

        fn ripple<C>(a: &Point2D<isize, C>, b: &mut Point2D<isize, C>) {
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            if dx.abs() > 1 || dy.abs() > 1 {
                b.x += dx.signum();
                b.y += dy.signum();
            }
        }

        for n in 1..N {
            let (head, tail) = self.knots.split_at_mut(n);
            ripple(&head[n - 1], &mut tail[0]);
        }

        self.tails.insert(self.tail());
    }

    fn instruction(&mut self, instruction: Instruction) {
        let Instruction { step, mut distance } = instruction;
        while distance > 0 {
            self.step(step);
            distance -= 1;
        }
    }
}

impl<const N: usize> TryFrom<&str> for Rope<N> {
    type Error = <Instruction as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut this = Self::new();
        for line in value.lines() {
            let instruction = line.parse()?;
            this.instruction(instruction);
        }
        Ok(this)
    }
}

impl<const N: usize> Display for Rope<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self
            .tails
            .iter()
            .chain(self.knots.iter())
            .fold((0, 0), |(max_x, max_y), p| (max_x.max(p.x), max_y.max(p.y)));

        let (min_x, min_y) = self
            .tails
            .iter()
            .chain(self.knots.iter())
            .fold((0, 0), |(min_x, min_y), p| (min_x.min(p.x), min_y.min(p.y)));

        let mut pixels = hashbrown::HashMap::new();
        for p in self.tails.iter() {
            pixels.insert(*p, '#');
        }
        // reverse so overlapping pixes give priority to lower-indexed knots
        for (i, p) in self.knots.iter().enumerate().rev() {
            pixels.insert(*p, (b'0' + i as u8) as char);
        }

        let width = (max_x - min_x + 1) as usize;
        let depth = (max_y - min_y + 1) as usize;

        for y in (0..depth).rev() {
            for x in 0..width {
                let p = Point2D::new(x as isize + min_x, y as isize + min_y);
                write!(f, "{}", pixels.get(&p).unwrap_or(&'.'))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
