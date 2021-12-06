/*
    --- Part Two ---
    Unfortunately, considering only horizontal and vertical lines doesn't give you the
    full picture; you need to also consider diagonal lines.

    Because of the limits of the hydrothermal vent mapping system, the lines in your
    list will only ever be horizontal, vertical, or a diagonal line at exactly 45
    degrees. In other words:

    - An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    - An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

    Considering all lines from the above example would now produce the following diagram:

    1.1....11.
    .111...2..
    ..2.1.111.
    ...1.2.2..
    .112313211
    ...1.2....
    ..1...1...
    .1.....1..
    1.......1.
    222111....

    You still need to determine the number of points where at least two lines overlap.
    In the above example, this is still anywhere in the diagram with a 2 or larger - now
    a total of 12 points.

    Consider all of the lines. At how many points do at least two lines overlap?
*/

use std::fmt::Display;
use std::io::BufRead;
use std::ops::{Index, IndexMut};

use crate::{Error, Problem, Solution};

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
            let mut line = line.clone();
            self[&line.0] += 1;
            while line.is_valid() {
                line = line.incr();
                self[&line.0] += 1;
            }
        }
    }
}

#[derive(Debug)]
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

impl Solution for Answer {
    type Err = Error;

    fn try_into_answer(self) -> std::result::Result<isize, Self::Err> {
        let board = self.into_board();
        Ok(board.hits.iter().filter(|v| **v >= 2).count() as isize)
    }
}

mod test {
    #[test]
    fn test_display() {
        let example = include_str!("inputs/example");
        let problem = crate::Problem(example.as_bytes());
        let answer: super::Answer = problem.try_into().unwrap();
        let board = answer.into_board();
        assert_eq!(board.width, 10);
        assert_eq!(board.depth, 10);
        let s = board.to_string();
        println!("{}", &s);
        assert_eq!(
            s,
            "1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....
"
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/example")).unwrap(),
            12
        )
    }

    #[test]
    fn test_live() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/live")).unwrap(),
            19472
        )
    }
}
