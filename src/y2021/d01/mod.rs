pub(crate) mod part1;
pub(crate) mod part2;

#[derive(derive_more::Deref, macros::FromLines)]
#[from_lines(usize)]
pub struct Ocean(Vec<usize>);
