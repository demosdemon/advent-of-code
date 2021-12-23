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

::aoc::derive_Display_for_Iter!(Instructions);

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

#[cfg(test)]
mod tests {
    macros::test_roundtrip!(
        super::Instruction,
        "on x=10..12,y=10..12,z=10..12",
        "on x=11..13,y=11..13,z=11..13",
        "off x=9..11,y=9..11,z=9..11",
        "on x=10..10,y=10..10,z=10..10",
        "on x=-20..26,y=-36..17,z=-47..7",
        "on x=-20..33,y=-21..23,z=-26..28",
        "on x=-22..28,y=-29..23,z=-38..16",
        "on x=-46..7,y=-6..46,z=-50..-1",
        "on x=-49..1,y=-3..46,z=-24..28",
        "on x=2..47,y=-22..22,z=-23..27",
        "on x=-27..23,y=-28..26,z=-21..29",
        "on x=-39..5,y=-6..47,z=-3..44",
        "on x=-30..21,y=-8..43,z=-13..34",
        "on x=-22..26,y=-27..20,z=-29..19",
        "off x=-48..-32,y=26..41,z=-47..-37",
        "on x=-12..35,y=6..50,z=-50..-2",
        "off x=-48..-32,y=-32..-16,z=-15..-5",
        "on x=-18..26,y=-33..15,z=-7..46",
        "off x=-40..-22,y=-38..-28,z=23..41",
        "on x=-16..35,y=-41..10,z=-47..6",
        "off x=-32..-23,y=11..30,z=-14..3",
        "on x=-49..-5,y=-3..45,z=-29..18",
        "off x=18..30,y=-20..-8,z=-3..13",
        "on x=-41..9,y=-7..43,z=-33..15",
        "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
        "on x=967..23432,y=45373..81175,z=27513..53682",
    );
}
