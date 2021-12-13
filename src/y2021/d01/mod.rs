mod part1;
mod part2;

use crate::{Error, ParseProblem, Problem};

#[derive(derive_more::Deref)]
struct Ocean(Vec<isize>);

impl FromIterator<isize> for Ocean {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl ParseProblem for Ocean {
    type Error = Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error> {
        problem.parse_lines(str::parse).collect()
    }
}
