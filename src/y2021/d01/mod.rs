pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::Deref, macros::FromIterator)]
#[from_iterator(usize)]
pub struct Ocean(Vec<usize>);

::aoc::derive_FromStr_for_FromIterator!(Ocean, usize);
