mod part1;
mod part2;

use std::collections::VecDeque;
use std::convert::Infallible;
use std::str::FromStr;

const AROUND_THE_BLOCK: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
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

const SIZE: usize = 10;

#[derive(Debug, Clone)]
struct Ocean([Octopus; SIZE * SIZE]);

#[derive(Default)]
struct FlashQueue {
    queue: VecDeque<usize>,
    seen: usize,
}

impl FlashQueue {
    fn bump(&mut self, idx: usize, octopus: &mut Octopus) {
        if octopus.bump() {
            self.push(idx);
        }
    }

    fn push(&mut self, idx: usize) {
        self.queue.push_back(idx);
        self.seen += 1;
    }

    fn pop(&mut self) -> Option<usize> {
        self.queue.pop_front()
    }
}

impl Ocean {
    pub fn tick(&mut self) -> usize {
        let mut queue = FlashQueue::default();

        for (idx, octopus) in self.0.iter_mut().enumerate() {
            queue.bump(idx, octopus)
        }

        const ISIZE: isize = SIZE as isize;
        while let Some(idx) = queue.pop() {
            let (x, y) = (idx % SIZE, idx / SIZE);
            for idx in AROUND_THE_BLOCK
                .iter()
                .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|(x, y)| (0..ISIZE).contains(x) && (0..ISIZE).contains(y))
                .map(|(x, y)| (y * ISIZE + x) as usize)
            {
                queue.bump(idx, &mut self.0[idx]);
            }
        }

        self.0.iter_mut().for_each(Octopus::reset);
        queue.seen
    }
}

impl FromIterator<u8> for Ocean {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(Octopus::from)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl FromStr for Ocean {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .flat_map(|s| s.chars().map(aoc::chardigit))
            .collect())
    }
}
