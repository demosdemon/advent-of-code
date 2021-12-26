pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod matrix;
mod parser;

#[derive(Debug, Clone)]
pub struct Game {
    pub pulls: Vec<u8>,

    pub boards: Vec<matrix::Board>,
}

::aoc::derive_FromStr_for_nom!(Game, parser::game);
