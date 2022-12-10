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

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    Noop,
    Addx(isize),
}

::aoc::derive_FromStr_for_nom!(Instruction, parser::instruction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Clock {
    x: isize,
    tick: usize,
    signal_strengths: [isize; 6],
}

impl Clock {
    const SIGNAL_TICKS: [usize; 6] = [20, 60, 100, 140, 180, 220];

    const fn new() -> Self {
        Self {
            x: 1,
            tick: 0,
            signal_strengths: [0; 6],
        }
    }

    fn instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Noop => {
                self.tick();
            }
            Instruction::Addx(x) => {
                self.tick();
                self.tick();
                self.x += x;
            }
        }
    }

    fn tick(&mut self) {
        self.tick += 1;
        if let Ok(i) = Self::SIGNAL_TICKS.binary_search(&self.tick) {
            self.signal_strengths[i] = self.x * self.tick as isize;
        }
    }

    fn signal_strengths(&self) -> isize {
        self.signal_strengths.iter().sum()
    }
}

impl TryFrom<&str> for Clock {
    type Error = <Instruction as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut this = Self::new();
        for line in value.lines() {
            let inst: Instruction = line.parse()?;
            // eprintln!("instruction: {inst:?}\n{this:?}");
            this.instruction(inst);
        }
        Ok(this)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Crt {
    x: isize,
    tick: usize,
    pixels: [bool; 6 * 40],
}

impl Crt {
    const fn new() -> Self {
        Self {
            x: 1,
            tick: 0,
            pixels: [false; 6 * 40],
        }
    }

    fn instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Noop => {
                self.tick();
            }
            Instruction::Addx(x) => {
                self.tick();
                self.tick();
                self.x += x;
            }
        }
    }

    fn tick(&mut self) {
        let col = self.tick as isize % 40;
        self.tick += 1;
        if (self.x - 1..=self.x + 1).contains(&col) {
            self.pixels[self.tick - 1] = true;
        }
    }
}

impl TryFrom<&str> for Crt {
    type Error = <Instruction as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut this = Self::new();
        for line in value.lines() {
            let inst: Instruction = line.parse()?;
            eprintln!("instruction: {inst:?}\n{this}");
            this.instruction(inst);
        }
        Ok(this)
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                let c = if self.pixels[y * 40 + x] { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
