use std::ops::{Index, IndexMut, Range};

use euclid::{point2, Point2D};

pub struct MatrixCoordinate;

pub type Position = Point2D<usize, MatrixCoordinate>;

#[macro_export]
macro_rules! Position {
    ($idx:expr, $width:expr) => {{
        let __idx = ($idx);
        let __width = ($width);
        ::euclid::point2(__idx / __width, __idx % __width)
    }};
}

macro_rules! PositionIter {
    ($v:ty) => { impl Iterator<Item = (Position, $v)> + '_ };
    ($av:ty, $bv:ty, $l:lifetime) => { impl Iterator<Item = (Position, $av, $bv)> + $l };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::EnumIter)]
pub enum RelativePosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl RelativePosition {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            RelativePosition::TopLeft => (-1, -1),
            RelativePosition::TopCenter => (-1, 0),
            RelativePosition::TopRight => (-1, 1),
            RelativePosition::MiddleLeft => (-0, -1),
            RelativePosition::MiddleCenter => (0, 0),
            RelativePosition::MiddleRight => (0, 1),
            RelativePosition::BottomLeft => (1, -1),
            RelativePosition::BottomCenter => (1, 0),
            RelativePosition::BottomRight => (1, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<Tile> {
    width: usize,
    tiles: Box<[Tile]>,
}

impl<Tile> Matrix<Tile> {
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn depth(&self) -> usize {
        self.tiles.len() / self.width
    }

    pub fn range_x(&self) -> Range<usize> {
        0..self.depth()
    }

    pub fn range_y(&self) -> Range<usize> {
        0..self.width
    }

    pub fn irange_x(&self) -> Range<isize> {
        0..self.depth() as isize
    }

    pub fn irange_y(&self) -> Range<isize> {
        0..self.width as isize
    }

    pub fn idx_to_pos(&self, idx: usize) -> Position {
        debug_assert!(idx < self.tiles.len());
        Position!(idx, self.width)
    }

    pub fn pos_to_idx(&self, pos: Position) -> usize {
        let v = (pos.x * self.width) + pos.y;
        debug_assert!(v < self.tiles.len());
        v
    }

    pub fn relative_pos(&self, pos: Position, rel: RelativePosition) -> Option<Position> {
        if self.is_empty() {
            return None;
        }
        let (dx, dy) = rel.delta();
        let x = pos.x as isize + dx;
        let y = pos.y as isize + dy;
        (self.irange_x().contains(&x) && self.irange_y().contains(&y))
            .then(|| point2(x as usize, y as usize))
    }

    pub fn get_relative(&self, pos: Position, rel: RelativePosition) -> Option<&Tile> {
        self.relative_pos(pos, rel).map(|pos| &self[pos])
    }

    pub fn get_relative_mut(&mut self, pos: Position, rel: RelativePosition) -> Option<&mut Tile> {
        self.relative_pos(pos, rel).map(|pos| &mut self[pos])
    }

    pub fn iter(&self) -> PositionIter!(&Tile) {
        self.tiles
            .iter()
            .enumerate()
            .map(|(idx, v)| (self.idx_to_pos(idx), v))
    }

    pub fn iter_row(&self, row: usize) -> PositionIter!(&Tile) {
        let width = self.width;
        assert!(row < self.depth());
        self.tiles
            .iter()
            .enumerate()
            .skip(row * width)
            .take(width)
            .map(move |(idx, v)| (Position!(idx, width), v))
    }

    pub fn iter_row_mut(&mut self, row: usize) -> PositionIter!(&mut Tile) {
        let width = self.width;
        assert!(row < self.depth());
        self.tiles
            .iter_mut()
            .enumerate()
            .skip(row * width)
            .take(width)
            .map(move |(idx, v)| (Position!(idx, width), v))
    }

    pub fn iter_column(&self, column: usize) -> PositionIter!(&Tile) {
        let width = self.width;
        assert!(column < width);
        self.tiles
            .iter()
            .enumerate()
            .skip(column)
            .step_by(width)
            .map(move |(idx, v)| (Position!(idx, width), v))
    }

    pub fn iter_column_mut(&mut self, column: usize) -> PositionIter!(&mut Tile) {
        let width = self.width;
        assert!(column < width);
        self.tiles
            .iter_mut()
            .enumerate()
            .skip(column)
            .step_by(width)
            .map(move |(idx, v)| (Position!(idx, width), v))
    }

    pub fn swap(&mut self, lhs: Position, rhs: Position) {
        let lhs = self.pos_to_idx(lhs);
        let rhs = self.pos_to_idx(rhs);
        self.tiles.swap(lhs, rhs);
    }

    pub fn iter_relative<'a, I: 'a + IntoIterator<Item = RelativePosition>>(
        &'a self,
        pos: Position,
        iter: I,
    ) -> impl Iterator<Item = Option<Position>> + 'a {
        iter.into_iter().map(move |rel| self.relative_pos(pos, rel))
    }

    pub fn select_relative<'a, I: 'a + IntoIterator<Item = RelativePosition>>(
        &'a self,
        pos: Position,
        iter: I,
    ) -> impl Iterator<Item = Option<(Position, &'a Tile)>> + 'a {
        self.iter_relative(pos, iter)
            .map(move |pos| pos.map(|pos| (pos, &self[pos])))
    }
}

impl<Tile> Index<Position> for Matrix<Tile> {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.tiles[self.pos_to_idx(index)]
    }
}

impl<Tile> IndexMut<Position> for Matrix<Tile> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.tiles[self.pos_to_idx(index)]
    }
}

impl<Tile> Index<usize> for Matrix<Tile> {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<Tile> IndexMut<usize> for Matrix<Tile> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl<I: IntoIterator> FromIterator<I> for Matrix<I::Item> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut width = 0;
        let mut tiles = Vec::new();
        for line in iter {
            tiles.extend(line);
            if width == 0 {
                width = tiles.len();
            }
            assert_eq!(tiles.len() % width, 0);
        }
        Self {
            width,
            tiles: tiles.into_boxed_slice(),
        }
    }
}

impl<I: IntoIterator<Item = V>, V: IntoIterator> From<I> for Matrix<V::Item> {
    fn from(iter: I) -> Self {
        iter.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Bound, RangeBounds};

    use euclid::point2;
    use strum::IntoEnumIterator;

    use crate::matrix::RelativePosition;

    use super::{Matrix, Position};

    fn collect_tiles<'a, V: Copy + 'a>(
        iter: impl Iterator<Item = (Position, &'a V)> + 'a,
    ) -> Vec<V> {
        iter.into_iter().map(|(_, v)| *v).collect()
    }

    const ROW_0: [u8; 3] = [0, 1, 2];
    const ROW_1: [u8; 3] = [3, 4, 5];
    const ROW_2: [u8; 3] = [6, 7, 8];
    const ROW_3: [u8; 3] = [9, 10, 11];
    const ROW_4: [u8; 3] = [12, 13, 14];

    const COL_0: [u8; 5] = [0, 3, 6, 9, 12];
    const COL_1: [u8; 5] = [1, 4, 7, 10, 13];
    const COL_2: [u8; 5] = [2, 5, 8, 11, 14];

    fn matrix() -> Matrix<u8> {
        Matrix::from([ROW_0, ROW_1, ROW_2, ROW_3, ROW_4])
    }

    #[test]
    fn test_iter_row() {
        let matrix = matrix();

        let v = collect_tiles(matrix.iter_row(0));
        assert_eq!(&v, &ROW_0);

        let v = collect_tiles(matrix.iter_row(1));
        assert_eq!(&v, &ROW_1);

        let v = collect_tiles(matrix.iter_row(2));
        assert_eq!(&v, &ROW_2);

        let v = collect_tiles(matrix.iter_row(3));
        assert_eq!(&v, &ROW_3);

        let v = collect_tiles(matrix.iter_row(4));
        assert_eq!(&v, &ROW_4);
    }

    #[test]
    fn test_iter_column() {
        let matrix = matrix();

        let v = collect_tiles(matrix.iter_column(0));
        assert_eq!(&v, &COL_0);

        let v = collect_tiles(matrix.iter_column(1));
        assert_eq!(&v, &COL_1);

        let v = collect_tiles(matrix.iter_column(2));
        assert_eq!(&v, &COL_2);
    }

    #[test]
    fn test_idx_to_pos() {
        let matrix = matrix();

        assert_eq!(matrix.idx_to_pos(0), point2(0, 0));
        assert_eq!(matrix.idx_to_pos(1), point2(0, 1));
        assert_eq!(matrix.idx_to_pos(2), point2(0, 2));
        assert_eq!(matrix.idx_to_pos(3), point2(1, 0));
        assert_eq!(matrix.idx_to_pos(4), point2(1, 1));
        assert_eq!(matrix.idx_to_pos(5), point2(1, 2));
        assert_eq!(matrix.idx_to_pos(6), point2(2, 0));
        assert_eq!(matrix.idx_to_pos(7), point2(2, 1));
        assert_eq!(matrix.idx_to_pos(8), point2(2, 2));
        assert_eq!(matrix.idx_to_pos(9), point2(3, 0));
        assert_eq!(matrix.idx_to_pos(10), point2(3, 1));
        assert_eq!(matrix.idx_to_pos(11), point2(3, 2));
        assert_eq!(matrix.idx_to_pos(12), point2(4, 0));
        assert_eq!(matrix.idx_to_pos(13), point2(4, 1));
        assert_eq!(matrix.idx_to_pos(14), point2(4, 2));
    }

    #[test]
    fn test_pos_to_idx() {
        let matrix = matrix();

        assert_eq!(matrix.pos_to_idx(point2(0, 0)), 0);
        assert_eq!(matrix.pos_to_idx(point2(0, 1)), 1);
        assert_eq!(matrix.pos_to_idx(point2(0, 2)), 2);
        assert_eq!(matrix.pos_to_idx(point2(1, 0)), 3);
        assert_eq!(matrix.pos_to_idx(point2(1, 1)), 4);
        assert_eq!(matrix.pos_to_idx(point2(1, 2)), 5);
        assert_eq!(matrix.pos_to_idx(point2(2, 0)), 6);
        assert_eq!(matrix.pos_to_idx(point2(2, 1)), 7);
        assert_eq!(matrix.pos_to_idx(point2(2, 2)), 8);
        assert_eq!(matrix.pos_to_idx(point2(3, 0)), 9);
        assert_eq!(matrix.pos_to_idx(point2(3, 1)), 10);
        assert_eq!(matrix.pos_to_idx(point2(3, 2)), 11);
        assert_eq!(matrix.pos_to_idx(point2(4, 0)), 12);
        assert_eq!(matrix.pos_to_idx(point2(4, 1)), 13);
        assert_eq!(matrix.pos_to_idx(point2(4, 2)), 14);
    }

    #[test]
    fn test_relative_pos() {
        let matrix = matrix();

        let v = matrix
            .select_relative(point2(1, 1), RelativePosition::iter())
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(
            &v,
            &[
                (point2(0, 0), &0),
                (point2(0, 1), &1),
                (point2(0, 2), &2),
                (point2(1, 0), &3),
                (point2(1, 1), &4),
                (point2(1, 2), &5),
                (point2(2, 0), &6),
                (point2(2, 1), &7),
                (point2(2, 2), &8),
            ]
        );

        let v = matrix
            .select_relative(point2(0, 0), RelativePosition::iter())
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(
            &v,
            &[
                (point2(0, 0), &0),
                (point2(0, 1), &1),
                (point2(1, 0), &3),
                (point2(1, 1), &4),
            ]
        );
    }

    #[test]
    fn test_range() {
        let matrix = matrix();

        let bounds = matrix.range_x();
        assert_eq!(bounds.start_bound(), Bound::Included(&0));
        assert_eq!(bounds.end_bound(), Bound::Excluded(&5));

        let bounds = matrix.range_y();
        assert_eq!(bounds.start_bound(), Bound::Included(&0));
        assert_eq!(bounds.end_bound(), Bound::Excluded(&3));
    }
}
