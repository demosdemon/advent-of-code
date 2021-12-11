pub mod part1;
pub mod part2;

use std::collections::LinkedList;
use std::io::BufRead;

use crate::errors::Error;
use crate::problem::Problem;

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

#[derive(Debug)]
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

#[derive(Debug)]
struct Ocean([Octopus; 100]);

impl Ocean {
    pub fn tick(&mut self) -> usize {
        let mut flashed = LinkedList::new();
        let mut flashes = 0;
        for (idx, octopus) in self.0.iter_mut().enumerate() {
            if octopus.bump() {
                flashes += 1;
                flashed.push_back(idx);
            }
        }

        let width = 10;
        let depth = 10;
        while let Some(idx) = flashed.pop_front() {
            let (x, y) = (idx % 10, idx / 10);
            for idx in AROUND_THE_BLOCK
                .iter()
                .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x, y)| x >= 0 && x < width && y >= 0 && y < depth)
                .map(|(x, y)| (y * width + x) as usize)
            {
                if self.0[idx].bump() {
                    flashes += 1;
                    flashed.push_back(idx);
                }
            }
        }

        self.0.iter_mut().for_each(Octopus::reset);
        flashes
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

impl<R: BufRead> TryFrom<Problem<R>> for Ocean {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(value
            .into_iter()
            .map(|res| res.and_then(|s| Ok(s.chars().map(crate::chardigit).collect::<Vec<_>>())))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}
