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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromIterator)]
#[from_iterator(Instruction)]
struct Instructions(Vec<Instruction>);

::aoc::derive_FromStr_for_FromIterator!(Instructions, Instruction);
