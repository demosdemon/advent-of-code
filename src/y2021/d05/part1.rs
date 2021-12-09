/*
    --- Day 5: Hydrothermal Venture ---
    You come across a field of hydrothermal vents on the ocean floor! These vents
    constantly produce large, opaque clouds, so it would be best to avoid them if
    possible.

    They tend to form in lines; the submarine helpfully produces a list of nearby lines
    of vents (your puzzle input) for you to review. For example:

        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2

    Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where
    x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates
    of the other end. These line segments include the points at both ends. In other
    words:

    - An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    - An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.

    For now, only consider horizontal and vertical lines: lines where either x1 = x2 or
    y1 = y2.

    So, the horizontal and vertical lines from the above list would produce the
    following diagram:

        .......1..
        ..1....1..
        ..1....1..
        .......1..
        .112111211
        ..........
        ..........
        ..........
        ..........
        222111....

    In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each
    position is shown as the number of lines which cover that point or . if no line
    covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the
    very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.

    To avoid the most dangerous areas, you need to determine the number of points where
    at least two lines overlap. In the above example, this is anywhere in the diagram
    with a 2 or larger - a total of 5 points.

    Consider only horizontal and vertical lines. At how many points do at least two
    lines overlap?
*/

use std::fmt::Display;
use std::io::BufRead;
use std::ops::{Index, IndexMut};

use crate::errors::Error;
use crate::problem::Problem;
use crate::IntoAnswer;

#[derive(Debug)]
struct Board {
    width: usize,
    depth: usize,
    hits: Box<[usize]>,
}

impl Board {
    pub fn new(width: usize, depth: usize) -> Self {
        let size = width * depth;
        Self {
            width,
            depth,
            hits: {
                let mut v = Vec::with_capacity(size);
                v.resize_with(size, Default::default);
                v.into_boxed_slice()
            },
        }
    }

    fn pos(&self, coord: &super::Coordinate) -> usize {
        assert!((coord.x as usize) < self.width);
        assert!((coord.y as usize) < self.depth);
        let res = ((coord.y as usize) * self.width) + (coord.x as usize);
        assert!(res < self.hits.len());
        res
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.hits.chunks_exact(self.width as usize) {
            for &col in row {
                if col == 0 {
                    write!(f, ".")?;
                } else {
                    assert!(col < 10);
                    write!(f, "{}", col)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Index<&'a super::Coordinate> for Board {
    type Output = usize;

    fn index(&self, index: &'a super::Coordinate) -> &Self::Output {
        &self.hits[self.pos(index)]
    }
}

impl<'a> IndexMut<&'a super::Coordinate> for Board {
    fn index_mut(&mut self, index: &'a super::Coordinate) -> &mut Self::Output {
        &mut self.hits[self.pos(index)]
    }
}

impl<'a> Extend<&'a super::Line> for Board {
    fn extend<T: IntoIterator<Item = &'a super::Line>>(&mut self, iter: T) {
        for line in iter {
            let min = std::cmp::min(&line.0, &line.1);
            let max = std::cmp::max(&line.0, &line.1);
            if min.x == max.x {
                for y in min.y..=max.y {
                    self[&super::Coordinate::new(min.x, y)] += 1;
                }
            } else if min.y == max.y {
                for x in min.x..=max.x {
                    self[&super::Coordinate::new(x, min.y)] += 1;
                }
            } else {
                assert!(line.is_diagonal());
            }
        }
    }
}

#[derive(Debug, macros::Answer)]
#[answer(example = 5, live = 4873)]
struct Answer(super::SolutionBuilder);

impl Answer {
    pub fn into_board(self) -> Board {
        let mut board = Board::new(self.0.max_x() as usize + 1, self.0.max_y() as usize + 1);
        board.extend(self.0 .0.iter());
        board
    }
}

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl IntoAnswer for Answer {
    fn into_answer(self) -> isize {
        let board = self.into_board();
        board.hits.iter().filter(|v| **v >= 2).count() as isize
    }
}

mod test {
    #[test]
    fn test_display() {
        let example = include_str!("inputs/example");
        let problem: crate::problem::Problem<_> = example.as_bytes().into();
        let answer: super::Answer = problem.try_into().unwrap();
        let board = answer.into_board();
        assert_eq!(board.width, 10);
        assert_eq!(board.depth, 10);
        let s = board.to_string();
        println!("{}", &s);
        assert_eq!(
            s,
            ".......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
"
        );
    }
}
