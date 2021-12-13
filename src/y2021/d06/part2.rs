/*
    --- Part Two ---
    Suppose the lanternfish live forever and have unlimited food and space. Would they
    take over the entire ocean?

    After 256 days in the example above, there would be a total of 26984457539
    lanternfish!
*/

use crate::{Error, IntoAnswer, ParseProblem, Problem};

use super::Ocean;

#[derive(macros::Answer)]
#[answer(example = 26984457539, live = 1710623015163)]
struct Answer(Ocean);

impl ParseProblem for Answer {
    type Error = Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error> {
        Ok(Self(Ocean(problem.expect_map_line(",", str::parse)?)))
    }
}

impl IntoAnswer for Answer {
    fn into_answer(self) -> isize {
        self.0.count(256) as isize
    }
}
