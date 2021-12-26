pub(crate) mod part1;
pub(crate) mod part2;

mod parser;

pub enum Direction {
    Forward(u8),
    Up(u8),
    Down(u8),
}

::aoc::derive_FromStr_for_nom!(Direction, parser::direction);

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
pub struct DirectionList(Vec<Direction>);

::aoc::derive_FromIterator!(DirectionList, Direction);
::aoc::derive_FromStr_for_FromIterator!(DirectionList, Direction);
