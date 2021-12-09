/*
    --- Part Two ---
    Based on your calculations, the planned course doesn't seem to make any sense. You
    find the submarine manual and discover that the process is actually slightly more
    complicated.

    In addition to horizontal position and depth, you'll also need to track a third
    value, aim, which also starts at 0. The commands also mean something entirely
    different than you first thought:

    down X increases your aim by X units.
    up X decreases your aim by X units.
    forward X does two things:
    It increases your horizontal position by X units.
    It increases your depth by your aim multiplied by X.

    Again note that since you're on a submarine, down and up do the opposite of what
    you might expect: "down" means aiming in the positive direction.

    Now, the above example does something different:

    forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0,
    your depth does not change.
    down 5 adds 5 to your aim, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5,
    your depth increases by 8*5=40.
    up 3 decreases your aim by 3, resulting in a value of 2.
    down 8 adds 8 to your aim, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10,
    your depth increases by 2*10=20 to a total of 60.

    After following these new instructions, you would have a horizontal position of 15
    and a depth of 60. (Multiplying these produces 900.)

    Using this new interpretation of the commands, calculate the horizontal position and
    depth you would have after following the planned course. What do you get if you
    multiply your final horizontal position by your final depth?
*/

use std::convert::Infallible;
use std::io::BufRead;

use crate::{Error, Problem, Solution};

use super::Direction;

#[derive(Default, Debug, macros::Problem)]
#[problem(example = 900, live = 1281977850)]
pub struct Answer {
    aim: isize,
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
                Direction::Forward(v) => {
                    pos.horizontal += v;
                    pos.depth += pos.aim * v;
                }
                Direction::Up(v) => pos.aim -= v,
                Direction::Down(v) => pos.aim += v,
            }
        }

        pos
    }
}
