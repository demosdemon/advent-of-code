pub(crate) mod part1;
pub(crate) mod part2;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(u8);

impl Priority {
    pub fn score(&self) -> usize {
        self.0 as usize
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            1..=26 => (b'a' + self.0 - 1) as char,
            27..=52 => (b'A' + self.0 - 27) as char,
            _ => unreachable!(),
        };
        write!(f, "{c}")
    }
}

impl TryFrom<u8> for Priority {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' => Ok(Self(value - b'a' + 1)),
            b'A'..=b'Z' => Ok(Self(value - b'A' + 27)),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromIterator)]
#[from_iterator(Priority)]
pub struct Sack(Vec<Priority>);

impl Sack {
    pub fn partition_mut(&mut self) -> (&mut [Priority], &mut [Priority]) {
        let mid = self.0.len() / 2;
        self.0.split_at_mut(mid)
    }
}

::aoc::derive_FromStr_for_bytes_TryFrom_collect!(Sack, Priority);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, macros::FromLines)]
#[from_lines(Sack)]
pub struct Sacks(Vec<Sack>);
