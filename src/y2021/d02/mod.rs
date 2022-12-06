pub(crate) mod part1;
pub(crate) mod part2;

mod parser;

enum Direction {
    Forward(u8),
    Up(u8),
    Down(u8),
}

::aoc::derive_FromStr_for_nom!(Direction, parser::direction);

#[derive(derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(ref)]
#[from_lines(Direction)]
struct DirectionList(Vec<Direction>);
