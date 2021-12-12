mod part1;
mod part2;

use std::io::BufRead;

use crate::errors::Error;
use crate::problem::Problem;

#[derive(derive_more::Deref)]
struct Ocean(Vec<isize>);

impl FromIterator<isize> for Ocean {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<R: BufRead> TryFrom<Problem<R>> for Ocean {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        value.parse_lines(str::parse).collect()
    }
}
