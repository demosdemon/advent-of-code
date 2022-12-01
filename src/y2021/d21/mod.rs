use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Not;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;

pub(crate) mod part1;
pub(crate) mod part2;

#[derive(Debug, Clone, Copy)]
pub struct Input {
    p1: u8,
    p2: u8,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let p1 = lines
            .next()
            .context("reading player 1")?
            .rsplit(' ')
            .next()
            .context("splitting player 1 input into value")?
            .parse::<u8>()
            .context("parsing player 1 position")?;
        let p2 = lines
            .next()
            .context("reading player 2")?
            .rsplit(' ')
            .next()
            .context("splitting player 2 input into value")?
            .parse::<u8>()
            .context("parsing player 2 position")?;
        if let Some(l) = lines.next() {
            Err(anyhow!("expected end of file; got {}", l))
        } else {
            Ok(Input { p1, p2 })
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Player {
    score: usize,
    pos: u8,
}

impl Add<usize> for Player {
    type Output = Player;

    fn add(self, rhs: usize) -> Self::Output {
        let pos = ((self.pos as usize + rhs) % 10) as u8;
        let score = self.score + pos as usize + 1;
        Player { score, pos }
    }
}

impl From<u8> for Player {
    fn from(v: u8) -> Self {
        Self {
            score: 0,
            pos: v - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    p1: Player,
    p2: Player,
    p1_roll: bool,
}

impl Add<usize> for State {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        let Self { p1, p2, p1_roll } = self;
        let (p1, p2) = if p1_roll {
            (p1 + rhs, p2)
        } else {
            (p1, p2 + rhs)
        };
        Self {
            p1,
            p2,
            p1_roll: !p1_roll,
        }
    }
}

impl AddAssign<usize> for State {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl Not for State {
    type Output = State;

    fn not(self) -> State {
        State {
            p1: self.p2,
            p2: self.p1,
            p1_roll: !self.p1_roll,
        }
    }
}

impl<'input> From<&'input Input> for State {
    fn from(input: &'input Input) -> State {
        State {
            p1: input.p1.into(),
            p2: input.p2.into(),
            p1_roll: true,
        }
    }
}
