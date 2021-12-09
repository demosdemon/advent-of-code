/*
    --- Part Two ---
    The crabs don't seem interested in your proposed solution. Perhaps you misunderstand
    crab engineering?

    As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead,
    each change of 1 step in horizontal position costs 1 more unit of fuel than the
    last: the first step costs 1, the second step costs 2, the third step costs 3, and
    so on.

    As each crab moves, moving further becomes more expensive. This changes the best
    horizontal position to align them all on; in the example above, this becomes 5:

        Move from 16 to 5: 66 fuel
        Move from 1 to 5: 10 fuel
        Move from 2 to 5: 6 fuel
        Move from 0 to 5: 15 fuel
        Move from 4 to 5: 1 fuel
        Move from 2 to 5: 6 fuel
        Move from 7 to 5: 3 fuel
        Move from 1 to 5: 10 fuel
        Move from 2 to 5: 6 fuel
        Move from 14 to 5: 45 fuel

    This costs a total of 168 fuel. This is the new cheapest possible outcome; the old
    alignment position (2) now costs 206 fuel instead.

    Determine the horizontal position that the crabs can align to using the least fuel
    possible so they can make you an escape route! How much fuel must they spend to
    align to that position?
*/

use std::{convert::Infallible, io::BufRead};

use crate::{Error, Problem, Solution};

use super::Ocean;

#[derive(derive_more::IntoIterator, macros::Answer)]
#[into_iterator(ref)]
#[answer(example = 168, live = 96361606)]
struct Answer(Ocean);

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(mut value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(Self(Ocean(value.expect_map_line(",", str::parse)?)))
    }
}

impl Solution for Answer {
    type Err = Infallible;

    fn try_into_answer(self) -> Result<isize, Self::Err> {
        Ok(self.0.solve(|a, b| {
            let d = (b - a).abs();
            (d * (d + 1)) / 2
        }))
    }
}