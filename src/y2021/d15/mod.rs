mod part1;
mod part2;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::convert::Infallible;
use std::fmt::Display;
use std::ops::Mul;
use std::str::FromStr;

const AROUND_THE_BLOCK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

#[derive(Debug, Clone)]
struct Cave {
    width: usize,
    nodes: Vec<u8>,
}

impl FromStr for Cave {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut nodes = Vec::new();
        for line in s.lines() {
            if width == 0 {
                width = line.len();
            }
            assert_eq!(width, line.len());
            nodes.extend(line.chars().map(crate::chardigit));
        }
        let s = Self { width, nodes };
        Ok(s)
    }
}

fn clamp(v: usize) -> u8 {
    (((v as isize - 1) % 9) + 1) as u8
}

impl Mul<usize> for Cave {
    type Output = Cave;

    fn mul(self, rhs: usize) -> Self::Output {
        let width = self.width;
        let depth = self.depth();
        let mut new = Cave::new(width * rhs, depth * rhs);
        for (idx, &v) in self.nodes.iter().enumerate() {
            let (x, y) = self.idx_to_pos(idx);
            for dx in 0..rhs {
                for dy in 0..rhs {
                    let new_idx = new.pos_to_idx((x + (dx * width), y + (dy * depth)));
                    let new_value = clamp(v as usize + dx + dy);
                    new.nodes[new_idx] = new_value;
                }
            }
        }
        new
    }
}

impl Cave {
    fn new(width: usize, depth: usize) -> Self {
        Self {
            width,
            nodes: vec![0; width * depth],
        }
    }

    fn depth(&self) -> usize {
        self.nodes.len() / self.width
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, (x, y): (usize, usize)) -> usize {
        (y * self.width) + x
    }

    fn iter_surrounding(&self, idx: usize) -> impl Iterator<Item = (usize, &u8)> {
        let (x, y) = self.idx_to_pos(idx);
        AROUND_THE_BLOCK
            .iter()
            .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|(x, y)| {
                (0..self.width as isize).contains(x) && (0..self.depth() as isize).contains(y)
            })
            .map(|(x, y)| self.pos_to_idx((x as usize, y as usize)))
            .map(|idx| (idx, &self.nodes[idx]))
    }

    fn cost(&self) -> Option<usize> {
        let mut dist = (0..self.nodes.len())
            .map(|_| usize::MAX)
            .collect::<Vec<_>>();
        dist[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(State {
            cost: 0,
            position: 0,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == self.nodes.len() - 1 {
                return Some(cost);
            }

            if cost > dist[position] {
                continue;
            }

            for (next, &v) in self.iter_surrounding(position) {
                let next = State {
                    cost: cost + v as usize,
                    position: next,
                };
                if next.cost < dist[next.position] {
                    dist[next.position] = next.cost;
                    heap.push(next);
                }
            }
        }

        None
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.nodes.chunks(self.width) {
            for b in chunk {
                write!(f, "{}", b)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_example_multiply() {
        let example = include_str!("inputs/example")
            .parse::<super::Cave>()
            .unwrap()
            * 5;
        let example_5x = include_str!("inputs/example_5x")
            .parse::<super::Cave>()
            .unwrap();
        println!("{}", example);
        println!("{}", example_5x);

        assert_eq!(example.nodes, example_5x.nodes,);
    }
}
