mod part1;
mod part2;

#[derive(derive_more::Deref)]
struct Ocean(Vec<isize>);

::aoc::derive_FromIterator!(Ocean, isize);
::aoc::derive_FromStr_for_FromIterator!(Ocean, isize);
