pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod bit;
pub(crate) mod line;

#[derive(derive_more::Deref, derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(ref)]
#[from_lines(line::Line)]
pub struct Lines(Vec<line::Line>);
