pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::Deref)]
pub struct Ocean(Vec<usize>);

::aoc::derive_FromIterator!(Ocean, usize);
::aoc::derive_FromStr_for_FromIterator!(Ocean, usize);
