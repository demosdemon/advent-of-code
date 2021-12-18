use std::collections::BTreeMap;

use super::coordinate::Coordinate;
use super::line::Line;

#[derive(Debug)]
pub(super) struct Board(BTreeMap<(i64, i64), usize>);

impl Board {
    pub fn overlaps(&self) -> usize {
        self.0.iter().filter(|&(_, &v)| v >= 2).count()
    }
}

impl FromIterator<Coordinate> for Board {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Self(iter.into_iter().fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c.into()).or_default() += 1;
            map
        }))
    }
}

impl FromIterator<Line> for Board {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        iter.into_iter()
            .flat_map(|l| l.into_iter().map(|l| l.0))
            .collect()
    }
}
