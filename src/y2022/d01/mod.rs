use std::cmp::Ordering;
use std::iter::Sum;

mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

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
