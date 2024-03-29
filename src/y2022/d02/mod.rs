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
use std::ops::Div;
use std::ops::Mul;

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Throw {
    const fn beats(&self) -> Self {
        match self {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        }
    }
}

impl Div for Throw {
    type Output = Outcome;

    fn div(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Outcome::Draw
        } else if self.beats() == rhs {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
}

impl Mul<Outcome> for Throw {
    type Output = Throw;

    fn mul(self, rhs: Outcome) -> Self::Output {
        match rhs {
            Outcome::Draw => self,
            Outcome::Loss => self.beats().beats(),
            Outcome::Win => self.beats(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Add<Outcome> for Throw {
    type Output = usize;

    fn add(self, rhs: Outcome) -> Self::Output {
        (self as usize) + (rhs as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rhs {
    X,
    Y,
    Z,
}

impl From<Rhs> for Throw {
    fn from(value: Rhs) -> Self {
        match value {
            Rhs::X => Throw::Rock,
            Rhs::Y => Throw::Paper,
            Rhs::Z => Throw::Scissors,
        }
    }
}

impl From<Rhs> for Outcome {
    fn from(value: Rhs) -> Self {
        match value {
            Rhs::X => Outcome::Loss,
            Rhs::Y => Outcome::Draw,
            Rhs::Z => Outcome::Win,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Instruction {
    opponent: Throw,
    outcome: Rhs,
}

impl From<(Throw, Rhs)> for Instruction {
    fn from(value: (Throw, Rhs)) -> Self {
        Self {
            opponent: value.0,
            outcome: value.1,
        }
    }
}

impl Instruction {
    fn p1(&self) -> InstructionP1 {
        InstructionP1 {
            opponent: self.opponent,
            outcome: self.outcome.into(),
        }
    }

    fn p2(&self) -> InstructionP2 {
        InstructionP2 {
            opponent: self.opponent,
            outcome: self.outcome.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InstructionP1 {
    opponent: Throw,
    outcome: Throw,
}

impl InstructionP1 {
    fn score(&self) -> usize {
        self.outcome + (self.opponent / self.outcome)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InstructionP2 {
    opponent: Throw,
    outcome: Outcome,
}

impl InstructionP2 {
    fn score(&self) -> usize {
        (self.opponent * self.outcome) + self.outcome
    }
}

::aoc::derive_FromStr_for_nom!(Instruction, parser::instruction);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromLines)]
#[from_lines(Instruction)]
struct Instructions(Vec<Instruction>);
