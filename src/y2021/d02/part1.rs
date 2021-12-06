/*
    --- Day 2: Dive! ---
    Now, you need to figure out how to pilot this thing.

    It seems like the submarine can take a series of commands like forward 1, down 2, or
    up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.

    Note that since you're on a submarine, down and up affect your depth, and so they
    have the opposite result of what you might expect.

    The submarine seems to already have a planned course (your puzzle input). You should
    probably figure out where it's going. For example:

    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2

    Your horizontal position and depth both start at 0. The steps above would then
    modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.

    After following these instructions, you would have a horizontal position of 15 and a
    depth of 10. (Multiplying these together produces 150.)

    Calculate the horizontal position and depth you would have after following the
    planned course. What do you get if you multiply your final horizontal position by
    your final depth?
*/

use std::convert::Infallible;
use std::io::BufRead;

use crate::{Error, Problem, Solution};

use super::Direction;

#[derive(Default, Debug)]
pub struct Answer {
    horizontal: isize,
    depth: isize,
}

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        value.parse_lines(str::parse::<Direction>).collect()
    }
}

impl Solution for Answer {
    type Err = Infallible;

    fn try_into_answer(self) -> Result<isize, Self::Err> {
        Ok(self.horizontal * self.depth)
    }
}

impl FromIterator<Direction> for Answer {
    fn from_iter<T: IntoIterator<Item = Direction>>(iter: T) -> Self {
        let mut pos = Answer::default();

        for dir in iter {
            match dir {
                Direction::Forward(v) => pos.horizontal += v,
                Direction::Up(v) => pos.depth -= v,
                Direction::Down(v) => pos.depth += v,
            }
        }

        pos
    }
}

mod test {
    #[test]
    fn test_example() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/example")).unwrap(),
            150
        );
    }

    #[test]
    fn test_live() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/live")).unwrap(),
            1714950
        );
    }
}