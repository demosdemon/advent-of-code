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

use std::iter::Sum;
use std::ops::AddAssign;

use crate::{Error, IntoAnswer, ParseProblem, Problem};

use super::Direction;

#[derive(Default, Debug, macros::Answer)]
#[answer(example = 150, live = 1714950)]
struct Answer {
    horizontal: isize,
    depth: isize,
}

impl AddAssign<Direction> for Answer {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Forward(v) => self.horizontal += v,
            Direction::Up(v) => self.depth -= v,
            Direction::Down(v) => self.depth += v,
        }
    }
}

impl Sum<Direction> for Answer {
    fn sum<I: Iterator<Item = Direction>>(iter: I) -> Self {
        let mut new = Self::default();
        for dir in iter {
            new += dir
        }
        new
    }
}

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> isize {
        self.horizontal * self.depth
    }
}

impl FromIterator<Direction> for Answer {
    fn from_iter<T: IntoIterator<Item = Direction>>(iter: T) -> Self {
        iter.into_iter().sum()
    }
}

impl ParseProblem for Answer {
    type Error = Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error> {
        problem.parse_lines(str::parse).collect()
    }
}
