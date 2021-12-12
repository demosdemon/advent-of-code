use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::str::FromStr;

use super::Error;

#[derive(Debug, Default)]
pub(super) struct Tile {
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

impl FromStr for Tile {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tile {
            value: s.parse()?,
            marked: false,
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if f.alternate() {
            if self.marked {
                "*"
            } else {
                "_"
            }
        } else {
            ""
        };
        write!(f, "{0}{1:0>2}{0}", mark, self.value)
    }
}

#[derive(Debug)]
pub(super) struct Board {
    /// Width & Depth of the board (e.g., 5 for a 5x5 board)
    size: u8,

    /// Board tiles in a one dimensional array. [(2, 4)] is located at [2*size + 4]
    tiles: Vec<Tile>,

    /// Map of all of the board values. Used to locate a tile within the board faster
    /// scanning the entire board.
    value_map: HashMap<u8, usize>,
}

impl Board {
    pub fn new(tiles: Vec<Tile>) -> crate::errors::Result<Self> {
        let len = tiles.len();
        let sqrt = (len as f32).sqrt();

        #[allow(clippy::float_cmp)]
        if sqrt.trunc() != sqrt {
            return Err(crate::errors::Error::from_parse(Error::InvalidTileLength(
                len, sqrt,
            )));
        }

        let mut value_map = HashMap::with_capacity(len);
        let mut dupes = HashSet::new();
        for (idx, tile) in tiles.iter().enumerate() {
            if value_map.insert(tile.value, idx).is_some() {
                dupes.insert(tile.value);
            }
        }

        if dupes.is_empty() {
            Ok(Self {
                size: sqrt as u8,
                tiles,
                value_map,
            })
        } else {
            Err(crate::errors::Error::from_parse(Error::DuplicateTiles(
                dupes.len(),
                dupes
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )))
        }
    }

    #[cfg(test)]
    fn from_tile_iter(tiles: impl Iterator<Item = Tile>) -> crate::errors::Result<Self> {
        Self::new(tiles.into_iter().collect())
    }

    #[cfg(test)]
    fn from_into_tile_iter<T: Into<Tile>>(
        tiles: impl Iterator<Item = T>,
    ) -> crate::errors::Result<Self> {
        Self::from_tile_iter(tiles.map(Into::into))
    }

    pub fn mark(&mut self, value: u8) -> Option<(u8, u8)> {
        if let Some(&pos) = self.value_map.get(&value) {
            self.tiles[pos].marked = true;
            Some(self.pos_to_idx(pos as u8))
        } else {
            None
        }
    }

    fn pos_to_idx(&self, pos: u8) -> (u8, u8) {
        (pos / self.size, pos % self.size)
    }

    pub fn bingo_row(&self, row: u8) -> bool {
        self.iter_row(row).all(|v| v.marked)
    }

    pub fn bingo_column(&self, col: u8) -> bool {
        self.iter_column(col).all(|v| v.marked)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn iter_row(&self, row: u8) -> impl Iterator<Item = &Tile> {
        assert!(row < self.size);
        self.tiles
            .iter()
            .skip((row * self.size) as usize)
            .take(self.size as usize)
    }

    pub fn iter_column(&self, col: u8) -> impl Iterator<Item = &Tile> {
        assert!(col < self.size);
        self.tiles
            .iter()
            .skip(col as usize)
            .step_by(self.size as usize)
    }

    pub fn sum(&self) -> isize {
        self.iter()
            .filter(|t| !t.marked)
            .map(|t| t.value as isize)
            .sum()
    }
}

struct DisplayRow<'a>(Box<[&'a Tile]>);

impl<'a> FromIterator<&'a Tile> for DisplayRow<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Tile>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> Display for DisplayRow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return f.write_str("[]");
        }
        let len = self.0.len();
        let alt = f.alternate();
        let sep = if alt { ", " } else { " " };
        if alt {
            f.write_str("[")?;
        }
        for (idx, t) in self.0.iter().enumerate() {
            Display::fmt(t, f)?;
            if idx < len - 1 {
                f.write_str(sep)?;
            }
        }
        if alt {
            f.write_str("]")?;
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tiles.is_empty() {
            return f.write_str("[]");
        }
        let alt = f.alternate();
        let sep = if alt { ",\n " } else { "\n" };
        if alt {
            f.write_str("[")?;
        }
        for idx in 0..self.size {
            let row: DisplayRow = self.iter_row(idx).collect();
            Display::fmt(&row, f)?;
            if idx < self.size - 1 {
                f.write_str(sep)?;
            }
        }
        if alt {
            f.write_str("]")?;
        }
        f.write_str("\n")
    }
}

mod test {
    #[test]
    fn test_display_row_to_string() {
        let tiles = (0u8..5u8)
            .map(Into::<super::Tile>::into)
            .collect::<Box<[super::Tile]>>();

        let display = tiles.iter().collect::<super::DisplayRow>();

        assert_eq!(format!("{}", display), "00 01 02 03 04");
        assert_eq!(format!("{:#}", display), "[_00_, _01_, _02_, _03_, _04_]");
    }

    #[test]
    fn test_board_to_string() {
        const SIZE: u8 = 3;
        let mut board = super::Board::from_into_tile_iter(0u8..(SIZE * SIZE)).unwrap();

        assert_eq!(
            format!("{}", board),
            "00 01 02
03 04 05
06 07 08
"
        );
        assert_eq!(
            format!("{:#}", board),
            "[[_00_, _01_, _02_],
 [_03_, _04_, _05_],
 [_06_, _07_, _08_]]
"
        );

        assert_eq!(board.mark(7).unwrap(), (2, 1));

        assert_eq!(
            format!("{}", board),
            "00 01 02
03 04 05
06 07 08
"
        );
        assert_eq!(
            format!("{:#}", board),
            "[[_00_, _01_, _02_],
 [_03_, _04_, _05_],
 [_06_, *07*, _08_]]
"
        );
    }
}
