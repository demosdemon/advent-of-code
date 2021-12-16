mod part1;
mod part2;

#[derive(derive_more::Deref)]
struct Ocean(Vec<isize>);

crate::derive_FromIterator!(Ocean, isize);
crate::derive_FromStr_for_FromIterator!(Ocean, isize);
