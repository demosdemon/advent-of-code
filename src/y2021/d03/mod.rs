pub(crate) mod part1;
pub(crate) mod part2;

mod bit;
mod line;

use line::Line;

#[derive(derive_more::Deref, derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(ref)]
#[from_lines(Line)]
struct Lines(Vec<Line>);
