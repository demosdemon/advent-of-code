/*
    --- Part Two ---
    Suppose the lanternfish live forever and have unlimited food and space. Would they
    take over the entire ocean?

    After 256 days in the example above, there would be a total of 26984457539
    lanternfish!
*/

use std::{convert::Infallible, io::BufRead};

use crate::{Error, Problem, Solution};

use super::Ocean;

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
        Ok(self.0.count(256) as isize)
    }
}

mod test {
    #[test]
    fn test_example() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/example")).unwrap(),
            26984457539
        )
    }

    #[test]
    fn test_live() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/live")).unwrap(),
            1710623015163
        )
    }
}
