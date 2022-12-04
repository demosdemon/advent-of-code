pub(crate) mod part1;
pub(crate) mod part2;

mod parser;

pub enum Direction {
    Forward(u8),
    Up(u8),
    Down(u8),
}

::aoc::derive_FromStr_for_nom!(Direction, parser::direction);

#[derive(derive_more::IntoIterator, macros::FromIterator)]
#[into_iterator(ref)]
#[from_iterator(Direction)]
pub struct DirectionList(Vec<Direction>);

::aoc::derive_FromStr_for_FromIterator!(DirectionList, Direction);
