use std::collections::{BTreeSet, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

use aoc::matrix::{Matrix, Position, RelativePosition};

const SURROUNDING: [RelativePosition; 4] = [
    RelativePosition::TopCenter,
    RelativePosition::MiddleLeft,
    RelativePosition::MiddleRight,
    RelativePosition::BottomCenter,
];

#[derive(Debug)]
pub struct Ocean(Matrix<u8>);

impl Ocean {
    pub fn basins(&self) -> Vec<usize> {
        self.iter_low_points()
            .map(|(root, &root_value)| {
                let mut basin = BTreeSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((root, root_value));
                while let Some((point, point_value)) = queue.pop_front() {
                    if basin.insert(point.to_tuple()) {
                        for (idx, &adj_value) in self.iter_surrounding(point) {
                            if adj_value < 9 && point_value < adj_value {
                                queue.push_back((idx, adj_value));
                            }
                        }
                    }
                }
                basin.len()
            })
            .collect()
    }

    pub fn iter_low_points(&self) -> impl Iterator<Item = (Position, &u8)> {
        self.0
            .iter()
            .filter(|&(idx, value)| self.is_low_point(idx, *value))
    }

    fn iter_surrounding(&self, pos: Position) -> impl Iterator<Item = (Position, &u8)> + '_ {
        self.0.select_relative(pos, SURROUNDING).flatten()
    }

    fn is_low_point(&self, idx: Position, value: u8) -> bool {
        match value {
            0 => true,
            9 => false,
            _ => self.iter_surrounding(idx).all(|(_, &v)| value < v),
        }
    }
}

impl FromStr for Ocean {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|l| l.bytes().map(::aoc::chardigit)).collect(),
        ))
    }
}
