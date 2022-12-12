// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;

use euclid::point2;
use euclid::Point2D;

pub use self::into_iter::IntoIter;
pub use self::iter_mut::IterMut;
pub use self::iter_ref::IterRef;
pub use self::iter_rel::IterRel;
pub use self::iter_rel_mut::IterRelMut;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const ALL: [Self; 9] = [
        Self::TopLeft,
        Self::TopCenter,
        Self::TopRight,
        Self::MiddleLeft,
        Self::MiddleCenter,
        Self::MiddleRight,
        Self::BottomLeft,
        Self::BottomCenter,
        Self::BottomRight,
    ];

    pub const fn delta(&self) -> (isize, isize) {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Matrix<Tile> {
    width: usize,
    tiles: Box<[Tile]>,
}

impl<Tile> Matrix<Tile> {
    pub fn new(width: usize, tiles: impl IntoIterator<Item = Tile>) -> Self {
        let this = Self {
            width,
            tiles: tiles.into_iter().collect(),
        };
        assert!(this.tiles.len() % this.width == 0);
        this
    }

    pub const fn len(&self) -> usize {
        self.tiles.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.width == 0
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn depth(&self) -> usize {
        self.tiles.len() / self.width
    }

    pub const fn range_x(&self) -> Range<usize> {
        0..self.depth()
    }

    pub const fn range_y(&self) -> Range<usize> {
        0..self.width
    }

    pub const fn idx_to_pos(&self, idx: usize) -> Position {
        debug_assert!(idx < self.tiles.len());
        Position!(idx, self.width)
    }

    pub const fn pos_to_idx(&self, pos: Position) -> usize {
        let v = (pos.x * self.width) + pos.y;
        debug_assert!(v < self.tiles.len());
        v
    }

    pub const fn relative_pos(&self, pos: Position, rel: RelativePosition) -> Option<Position> {
        if self.is_empty() {
            return None;
        }
        let (dx, dy) = rel.delta();
        let x = pos.x as isize + dx;
        let y = pos.y as isize + dy;
        if 0 <= x && x < self.depth() as isize && 0 <= y && y < self.width() as isize {
            Some(point2(x as usize, y as usize))
        } else {
            None
        }
    }

    pub const fn get(&self, pos: Position) -> Option<&Tile> {
        if pos.x < self.depth() && pos.y < self.width() {
            Some(&self.tiles[self.pos_to_idx(pos)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        if pos.x < self.depth() && pos.y < self.width() {
            Some(&mut self.tiles[self.pos_to_idx(pos)])
        } else {
            None
        }
    }

    pub const fn get_relative(&self, pos: Position, rel: RelativePosition) -> Option<&Tile> {
        if let Some(pos) = self.relative_pos(pos, rel) {
            self.get(pos)
        } else {
            None
        }
    }

    pub fn get_relative_mut(&mut self, pos: Position, rel: RelativePosition) -> Option<&mut Tile> {
        if let Some(pos) = self.relative_pos(pos, rel) {
            self.get_mut(pos)
        } else {
            None
        }
    }

    pub const fn iter(&self) -> IterRef<'_, Tile> {
        IterRef::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Tile> {
        self.into()
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

    pub fn iter_rel<I: IntoIterator<Item = RelativePosition>>(
        &self,
        pos: Position,
        reliter: I,
    ) -> IterRel<'_, Tile, I::IntoIter> {
        IterRel::new(self, pos, reliter.into_iter())
    }

    pub fn iter_rel_mut<I: IntoIterator<Item = RelativePosition>>(
        &mut self,
        pos: Position,
        reliter: I,
    ) -> IterRelMut<'_, Tile, I::IntoIter> {
        IterRelMut::new(self, pos, reliter)
    }

    pub fn map<O, F>(self, mut func: F) -> Matrix<O>
    where
        F: FnMut(Tile) -> O,
    {
        let width = self.width;
        Matrix {
            tiles: self.into_iter().map(|(_, tile)| func(tile)).collect(),
            width,
        }
    }
}

impl<Tile> Index<Position> for Matrix<Tile> {
    type Output = Tile;

    fn index(&self, pos: Position) -> &Self::Output {
        self.get(pos).expect("index out of bounds")
    }
}

impl<Tile> IndexMut<Position> for Matrix<Tile> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        self.get_mut(pos).expect("index out of bounds")
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

impl<Tile> Index<Range<usize>> for Matrix<Tile> {
    type Output = [Tile];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.tiles[range]
    }
}

impl<Tile> IndexMut<Range<usize>> for Matrix<Tile> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.tiles[range]
    }
}

impl<Tile> IntoIterator for Matrix<Tile> {
    type IntoIter = IntoIter<Tile>;
    type Item = (Position, Tile);

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl<'a, Tile> IntoIterator for &'a Matrix<Tile> {
    type IntoIter = IterRef<'a, Tile>;
    type Item = (Position, &'a Tile);

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl<'a, Tile> IntoIterator for &'a mut Matrix<Tile> {
    type IntoIter = IterMut<'a, Tile>;
    type Item = (Position, &'a mut Tile);

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl<I> FromIterator<I> for Matrix<I::Item>
where
    I: IntoIterator,
{
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

mod iter_rel {
    use super::Matrix;
    use super::Position;
    use super::RelativePosition;

    pub struct IterRel<'a, Tile, I> {
        matrix: &'a Matrix<Tile>,
        center: Position,
        reliter: I,
    }

    impl<'a, Tile, I> IterRel<'a, Tile, I>
    where
        I: Iterator<Item = RelativePosition>,
    {
        pub(super) fn new(
            matrix: &'a Matrix<Tile>,
            center: Position,
            reliter: impl IntoIterator<IntoIter = I>,
        ) -> Self {
            Self {
                matrix,
                center,
                reliter: reliter.into_iter(),
            }
        }
    }

    impl<'a, Tile, I> Iterator for IterRel<'a, Tile, I>
    where
        I: Iterator<Item = RelativePosition>,
    {
        type Item = (Position, &'a Tile);

        fn next(&mut self) -> Option<Self::Item> {
            for rel in self.reliter.by_ref() {
                if let Some(pos) = self.matrix.relative_pos(self.center, rel) {
                    return Some((pos, &self.matrix[pos]));
                }
            }

            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let (_, upper) = self.reliter.size_hint();
            (0, upper)
        }
    }

    impl<'a, Tile, I> DoubleEndedIterator for IterRel<'a, Tile, I>
    where
        I: DoubleEndedIterator<Item = RelativePosition>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            for rel in self.reliter.by_ref().rev() {
                if let Some(pos) = self.matrix.relative_pos(self.center, rel) {
                    return Some((pos, &self.matrix[pos]));
                }
            }

            None
        }
    }
}

mod iter_rel_mut {
    use super::Matrix;
    use super::Position;
    use super::RelativePosition;

    pub struct IterRelMut<'a, Tile, I> {
        matrix: &'a mut Matrix<Tile>,
        center: Position,
        reliter: I,
    }

    impl<'a, Tile, I> IterRelMut<'a, Tile, I>
    where
        I: Iterator<Item = RelativePosition>,
    {
        pub(super) fn new(
            matrix: &'a mut Matrix<Tile>,
            center: Position,
            reliter: impl IntoIterator<IntoIter = I>,
        ) -> Self {
            Self {
                matrix,
                center,
                reliter: reliter.into_iter(),
            }
        }

        unsafe fn read(&mut self, pos: Position) -> (Position, &'a mut Tile) {
            let idx = self.matrix.pos_to_idx(pos);
            let tile = &mut *(self.matrix.tiles.get_unchecked_mut(idx) as *mut _);
            (pos, tile)
        }
    }

    impl<'a, Tile, I> Iterator for IterRelMut<'a, Tile, I>
    where
        I: Iterator<Item = RelativePosition>,
    {
        type Item = (Position, &'a mut Tile);

        fn next(&mut self) -> Option<Self::Item> {
            for rel in self.reliter.by_ref() {
                if let Some(pos) = self.matrix.relative_pos(self.center, rel) {
                    return Some(unsafe { self.read(pos) });
                }
            }

            None
        }
    }

    impl<'a, Tile, I> DoubleEndedIterator for IterRelMut<'a, Tile, I>
    where
        I: DoubleEndedIterator<Item = RelativePosition>,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.reliter
                .by_ref()
                .rev()
                .find_map(|rel| self.matrix.relative_pos(self.center, rel))
                .map(|pos| unsafe { self.read(pos) })
        }
    }
}

mod into_iter {
    use std::mem::MaybeUninit;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use super::Matrix;
    use super::Position;

    pub struct IntoIter<Tile> {
        matrix: Matrix<MaybeUninit<Tile>>,
        head: usize,
        tail: usize,
    }

    impl<Tile> IntoIter<Tile> {
        pub const fn is_empty(&self) -> bool {
            self.head == self.tail
        }

        pub const fn len(&self) -> usize {
            self.tail - self.head
        }

        fn uninit_slice(&self) -> &[MaybeUninit<Tile>] {
            &self.matrix.tiles[self.head..self.tail]
        }

        fn uninit_slice_mut(&mut self) -> &mut [MaybeUninit<Tile>] {
            &mut self.matrix.tiles[self.head..self.tail]
        }

        unsafe fn read(&self, idx: usize) -> (Position, Tile) {
            let pos = self.matrix.idx_to_pos(idx);
            let tile = self.matrix.tiles.get_unchecked(idx).assume_init_read();
            (pos, tile)
        }
    }

    impl<Tile> Iterator for IntoIter<Tile> {
        type Item = (Position, Tile);

        fn next(&mut self) -> Option<Self::Item> {
            let idx = self.head;
            if idx < self.tail {
                self.head += 1;
                Some(unsafe { self.read(idx) })
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = IntoIter::len(self);
            (len, Some(len))
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            IntoIter::len(&self)
        }
    }

    impl<Tile> DoubleEndedIterator for IntoIter<Tile> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.head < self.tail {
                self.tail -= 1;
                Some(unsafe { self.read(self.tail) })
            } else {
                None
            }
        }
    }

    impl<Tile> ExactSizeIterator for IntoIter<Tile> {}

    impl<Tile> Drop for IntoIter<Tile> {
        fn drop(&mut self) {
            unsafe { drop_uninit_slice(self.uninit_slice_mut()) };
            let len = self.matrix.len();
            self.head = len;
            self.tail = len;
        }
    }

    impl<Tile> From<Matrix<Tile>> for IntoIter<Tile> {
        fn from(matrix: Matrix<Tile>) -> Self {
            let Matrix { width, tiles } = matrix;
            let tail = tiles.len();
            let matrix = Matrix {
                width,
                tiles: into_uninit_slice(tiles),
            };
            Self {
                matrix,
                head: 0,
                tail,
            }
        }
    }

    impl<Tile> AsRef<[Tile]> for IntoIter<Tile> {
        fn as_ref(&self) -> &[Tile] {
            unsafe { from_uninit_slice(self.uninit_slice()) }
        }
    }

    impl<Tile> AsMut<[Tile]> for IntoIter<Tile> {
        fn as_mut(&mut self) -> &mut [Tile] {
            unsafe { from_uninit_slice_mut(self.uninit_slice_mut()) }
        }
    }

    impl<Tile> Deref for IntoIter<Tile> {
        type Target = [Tile];

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<Tile> DerefMut for IntoIter<Tile> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut()
        }
    }

    fn into_uninit_slice<T>(boxed: Box<[T]>) -> Box<[MaybeUninit<T>]> {
        let len = boxed.len();
        let ptr = Box::into_raw(boxed) as *mut MaybeUninit<T>;
        unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)) }
    }

    unsafe fn from_uninit_slice<T>(slice: &[MaybeUninit<T>]) -> &[T] {
        let ptr = slice.as_ptr() as *const T;
        std::slice::from_raw_parts(ptr, slice.len())
    }

    unsafe fn from_uninit_slice_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
        let ptr = slice.as_mut_ptr() as *mut T;
        std::slice::from_raw_parts_mut(ptr, slice.len())
    }

    unsafe fn drop_uninit_slice<T>(slice: &mut [MaybeUninit<T>]) {
        for uninit in slice {
            uninit.assume_init_drop();
        }
    }
}

mod iter_ref {
    use std::ops::Deref;

    use super::Matrix;
    use super::Position;

    pub struct IterRef<'a, Tile> {
        matrix: &'a Matrix<Tile>,
        head: usize,
        tail: usize,
    }

    impl<'a, Tile> IterRef<'a, Tile> {
        pub const fn new(matrix: &'a Matrix<Tile>) -> Self {
            Self {
                matrix,
                head: 0,
                tail: matrix.len(),
            }
        }

        pub const fn is_empty(&self) -> bool {
            self.head == self.tail
        }

        pub const fn len(&self) -> usize {
            self.tail - self.head
        }

        unsafe fn read(&self, idx: usize) -> (Position, &'a Tile) {
            let pos = self.matrix.idx_to_pos(idx);
            let tile = self.matrix.tiles.get_unchecked(idx);
            (pos, tile)
        }
    }

    impl<'a, Tile> Iterator for IterRef<'a, Tile> {
        type Item = (Position, &'a Tile);

        fn next(&mut self) -> Option<Self::Item> {
            let idx = self.head;
            if idx < self.tail {
                self.head += 1;
                Some(unsafe { self.read(idx) })
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = IterRef::len(self);
            (len, Some(len))
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.len()
        }
    }

    impl<'a, Tile> DoubleEndedIterator for IterRef<'a, Tile> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.head < self.tail {
                self.tail -= 1;
                Some(unsafe { self.read(self.tail) })
            } else {
                None
            }
        }
    }

    impl<'a, Tile> ExactSizeIterator for IterRef<'a, Tile> {}

    impl<'a, Tile> From<&'a Matrix<Tile>> for IterRef<'a, Tile> {
        fn from(matrix: &'a Matrix<Tile>) -> Self {
            Self::new(matrix)
        }
    }

    impl<'a, Tile> AsRef<[Tile]> for IterRef<'a, Tile> {
        fn as_ref(&self) -> &[Tile] {
            &self.matrix.tiles[self.head..self.tail]
        }
    }

    impl<'a, Tile> Deref for IterRef<'a, Tile> {
        type Target = [Tile];

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }
}

mod iter_mut {
    use std::ops::Deref;
    use std::ops::DerefMut;

    use super::Matrix;
    use super::Position;

    pub struct IterMut<'a, Tile> {
        matrix: &'a mut Matrix<Tile>,
        head: usize,
        tail: usize,
    }

    impl<'a, Tile> IterMut<'a, Tile> {
        pub fn new(matrix: &'a mut Matrix<Tile>) -> Self {
            let tail = matrix.len();
            Self {
                matrix,
                head: 0,
                tail,
            }
        }

        pub const fn is_empty(&self) -> bool {
            self.head == self.tail
        }

        pub const fn len(&self) -> usize {
            self.tail - self.head
        }

        unsafe fn read_mut(&mut self, idx: usize) -> (Position, &'a mut Tile) {
            let pos = self.matrix.idx_to_pos(idx);
            let tile = &mut *(self.matrix.tiles.get_unchecked_mut(idx) as *mut _);
            (pos, tile)
        }
    }

    impl<'a, Tile> Iterator for IterMut<'a, Tile> {
        type Item = (Position, &'a mut Tile);

        fn next(&mut self) -> Option<Self::Item> {
            let idx = self.head;
            if idx < self.tail {
                self.head += 1;
                Some(unsafe { self.read_mut(idx) })
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = IterMut::len(self);
            (len, Some(len))
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.len()
        }
    }

    impl<'a, Tile> DoubleEndedIterator for IterMut<'a, Tile> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.head < self.tail {
                self.tail -= 1;
                Some(unsafe { self.read_mut(self.tail) })
            } else {
                None
            }
        }
    }

    impl<'a, Tile> ExactSizeIterator for IterMut<'a, Tile> {}

    impl<'a, Tile> From<&'a mut Matrix<Tile>> for IterMut<'a, Tile> {
        fn from(value: &'a mut Matrix<Tile>) -> Self {
            Self::new(value)
        }
    }

    impl<'a, Tile> AsRef<[Tile]> for IterMut<'a, Tile> {
        fn as_ref(&self) -> &[Tile] {
            &self.matrix.tiles[self.head..self.tail]
        }
    }

    impl<'a, Tile> AsMut<[Tile]> for IterMut<'a, Tile> {
        fn as_mut(&mut self) -> &mut [Tile] {
            &mut self.matrix.tiles[self.head..self.tail]
        }
    }

    impl<'a, Tile> Deref for IterMut<'a, Tile> {
        type Target = [Tile];

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<'a, Tile> DerefMut for IterMut<'a, Tile> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;
    use std::ops::RangeBounds;

    use euclid::point2;
    use itertools::Itertools;

    use super::Matrix;
    use super::Position;
    use super::RelativePosition;

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
        [ROW_0, ROW_1, ROW_2, ROW_3, ROW_4].into_iter().collect()
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
            .iter_rel(point2(1, 1), RelativePosition::ALL)
            .collect_vec();

        assert_eq!(&v, &[
            (point2(0, 0), &0),
            (point2(0, 1), &1),
            (point2(0, 2), &2),
            (point2(1, 0), &3),
            (point2(1, 1), &4),
            (point2(1, 2), &5),
            (point2(2, 0), &6),
            (point2(2, 1), &7),
            (point2(2, 2), &8),
        ]);

        let v = matrix
            .iter_rel(point2(0, 0), RelativePosition::ALL)
            .collect_vec();

        assert_eq!(&v, &[
            (point2(0, 0), &0),
            (point2(0, 1), &1),
            (point2(1, 0), &3),
            (point2(1, 1), &4),
        ]);
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
