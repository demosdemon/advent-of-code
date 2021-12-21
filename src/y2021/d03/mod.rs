mod part1;
mod part2;

mod bit;
mod line;

#[derive(derive_more::Deref, derive_more::IntoIterator)]
#[into_iterator(ref)]
struct Lines(Vec<line::Line>);

::aoc::derive_FromIterator!(Lines, line::Line);
::aoc::derive_FromStr_for_FromIterator!(Lines, line::Line);
