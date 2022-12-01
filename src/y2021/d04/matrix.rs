use std::collections::BTreeMap;
use std::fmt::Debug;

use aoc::matrix::Matrix;
use aoc::matrix::Position;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    /// Value on the bingo board.
    pub(crate) value: u8,

    /// Flag indicating where this value has been drawn.
    pub(crate) marked: bool,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        Tile {
            value,
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    matrix: Matrix<Tile>,
    value_map: BTreeMap<u8, Position>,
}

impl Board {
    pub fn new<I: IntoIterator<Item = V>, V: IntoIterator<Item = Tile>>(tiles: I) -> Self {
        let matrix = tiles.into_iter().collect::<Matrix<_>>();
        let value_map = matrix.iter().map(|(pos, tile)| (tile.value, pos)).collect();
        Self { matrix, value_map }
    }

    pub fn mark(&mut self, value: u8) -> Option<Position> {
        if let Some(&pos) = self.value_map.get(&value) {
            let t = &mut self.matrix[pos];
            debug_assert_eq!(t.value, value);
            t.marked = true;
            Some(pos)
        } else {
            None
        }
    }

    pub fn bingo_row(&self, row: usize) -> bool {
        self.matrix.iter_row(row).all(|(_, v)| v.marked)
    }

    pub fn bingo_column(&self, col: usize) -> bool {
        self.matrix.iter_column(col).all(|(_, v)| v.marked)
    }

    pub fn sum(&self) -> usize {
        self.matrix
            .iter()
            .filter(|(_, t)| !t.marked)
            .map(|(_, t)| t.value as usize)
            .sum()
    }
}
