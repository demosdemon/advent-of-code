pub(crate) mod part1;
pub(crate) mod part2;

mod parser;

use std::{collections::BTreeMap, ops::Not};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::Display, derive_more::IsVariant,
)]
pub enum State {
    #[display(fmt = "off")]
    Off,

    #[display(fmt = "on")]
    On,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::Off => State::On,
            State::On => State::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn compose<F>(&self, rhs: &Self, mut op: F) -> Self
    where
        F: FnMut(isize, isize) -> isize,
    {
        Self {
            x: op(self.x, rhs.x),
            y: op(self.y, rhs.y),
            z: op(self.z, rhs.z),
        }
    }

    pub fn min_coord(&self, rhs: &Self) -> Self {
        self.compose(rhs, std::cmp::min)
    }

    pub fn max_coord(&self, rhs: &Self) -> Self {
        self.compose(rhs, std::cmp::max)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
#[display(
    fmt = "x={}..{},y={}..{},z={}..{}",
    "head.x",
    "tail.x",
    "head.y",
    "tail.y",
    "head.z",
    "tail.z"
)]
pub struct Cube {
    head: Coordinate,
    tail: Coordinate,
}

impl Cube {
    pub const fn new(head: Coordinate, tail: Coordinate) -> Self {
        macro_rules! ordered {
            ($lhs:expr, $rhs:expr) => {{
                let __lhs = ($lhs);
                let __rhs = ($rhs);
                if __lhs <= __rhs {
                    (__lhs, __rhs)
                } else {
                    (__rhs, __rhs)
                }
            }};
        }
        let (x1, x2) = ordered!(head.x, tail.x);
        let (y1, y2) = ordered!(head.y, tail.y);
        let (z1, z2) = ordered!(head.z, tail.z);
        Self {
            head: Coordinate::new(x1, y1, z1),
            tail: Coordinate::new(x2, y2, z2),
        }
    }

    pub fn intersection(&self, rhs: &Self) -> Option<Self> {
        let head = self.head.max_coord(&rhs.head);
        let tail = self.tail.min_coord(&rhs.tail);
        (head.x <= tail.x && head.y <= tail.y && head.z <= tail.z).then(|| Self { head, tail })
    }

    pub fn volume(&self) -> usize {
        macro_rules! side {
            ($elem:tt) => {{
                debug_assert!(
                    self.head.$elem <= self.tail.$elem,
                    "invalid cube: {:?}",
                    self
                );
                (self.tail.$elem - self.head.$elem) as usize + 1
            }};
        }
        side!(x) * side!(y) * side!(z)
    }

    pub fn contains(&self, rhs: &Cube) -> bool {
        macro_rules! cmp {
            ($elem:tt) => {
                self.head.$elem <= rhs.head.$elem && self.tail.$elem >= rhs.tail.$elem
            };
        }
        cmp!(x) && cmp!(y) && cmp!(x)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Constructor,
    derive_more::Display,
)]
#[display(fmt = "{} {}", state, cube)]
pub struct Instruction {
    state: State,
    cube: Cube,
}

::aoc::derive_FromStr_for_nom!(Instruction, parser::instruction);

#[derive(Default, derive_more::IntoIterator)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn reduce(&self, zone: Option<&Cube>) -> isize {
        let mut grid = BTreeMap::<Cube, isize>::new();
        for i in self.0.iter().filter(|i| match zone {
            Some(cube) => cube.contains(&i.cube),
            None => true,
        }) {
            for (c, s) in grid.clone() {
                if let Some(x) = i.cube.intersection(&c) {
                    *grid.entry(x).or_default() += -s;
                    if grid[&x] == 0 {
                        grid.remove(&x);
                    }
                }
            }
            if i.state.is_on() {
                *grid.entry(i.cube).or_default() += 1;
            }
        }
        grid.iter()
            .map(|(c, s)| c.volume() as isize * s)
            .sum::<isize>()
    }
}

::aoc::derive_FromIterator!(Instructions, Instruction);
::aoc::derive_FromStr_for_FromIterator!(Instructions, Instruction);
