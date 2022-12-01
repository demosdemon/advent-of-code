pub(crate) mod part1;
pub(crate) mod part2;

use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

use aoc::matrix::Matrix;
use aoc::matrix::Position;
use aoc::matrix::RelativePosition;

const SURROUNDING: [RelativePosition; 8] = [
    RelativePosition::TopLeft,
    RelativePosition::TopCenter,
    RelativePosition::TopRight,
    RelativePosition::MiddleLeft,
    RelativePosition::MiddleRight,
    RelativePosition::BottomLeft,
    RelativePosition::BottomCenter,
    RelativePosition::BottomRight,
];

#[derive(Debug, Clone)]
struct Octopus {
    value: u8,
    flashed: bool,
}

impl Octopus {
    fn bump(&mut self) -> bool {
        if !self.flashed {
            self.value += 1;
            self.flashed = self.value > 9;
            self.flashed
        } else {
            false
        }
    }

    fn reset(&mut self) {
        if self.flashed {
            self.flashed = false;
            self.value = 0;
        }
    }
}

impl From<u8> for Octopus {
    fn from(value: u8) -> Self {
        Self {
            value,
            flashed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ocean(Matrix<Octopus>);

impl Ocean {
    pub fn tick(&mut self) -> usize {
        let mut queue = FlashQueue::default();

        for (pos, octopus) in self.0.iter_mut() {
            queue.bump(pos, octopus)
        }

        while let Some(pos) = queue.pop() {
            for (pos, tile) in self.0.iter_rel_mut(pos, SURROUNDING).flatten() {
                queue.bump(pos, tile);
            }
        }

        self.0.iter_mut().map(|(_, v)| v).for_each(Octopus::reset);
        queue.seen
    }
}

impl FromStr for Ocean {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|l| l.bytes().map(aoc::chardigit).map(Octopus::from))
                .collect(),
        ))
    }
}

#[derive(Default)]
struct FlashQueue {
    queue: VecDeque<Position>,
    seen: usize,
}

impl FlashQueue {
    fn bump(&mut self, pos: Position, octopus: &mut Octopus) {
        if octopus.bump() {
            self.push(pos);
        }
    }

    fn push(&mut self, pos: Position) {
        self.queue.push_back(pos);
        self.seen += 1;
    }

    fn pop(&mut self) -> Option<Position> {
        self.queue.pop_front()
    }
}
