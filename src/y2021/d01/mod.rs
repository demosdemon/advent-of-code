pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::Deref)]
pub struct Ocean(Vec<isize>);

::aoc::derive_FromIterator!(Ocean, isize);
::aoc::derive_FromStr_for_FromIterator!(Ocean, isize);
