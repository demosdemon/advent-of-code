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

use std::ops::Add;
use std::ops::Not;

use super::bit::Bit;

#[derive(
    Default, Debug, Clone, derive_more::IntoIterator, derive_more::Index, macros::FromBytes,
)]
#[into_iterator(owned, ref)]
#[from_bytes(Bit)]
pub struct Line(Vec<Bit>);

impl From<&[Bit]> for Line {
    fn from(v: &[Bit]) -> Self {
        v.iter().copied().collect()
    }
}

impl Line {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn partition(self) -> (Line, Line) {
        let mut bits = self.0;
        bits.sort_unstable_by_key(|b| bool::from(b));
        let mid = bits.partition_point(|b| bool::from(b));
        let (l, r) = bits.split_at(mid);
        (l.into(), r.into())
    }

    pub fn ceiling(self) -> Bit {
        let (ones, zeros) = self.partition();
        (zeros.len() <= ones.len()).into()
    }
}

impl Not for Line {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.into_iter().map(Not::not).collect()
    }
}

impl<'a> From<&'a Line> for usize {
    fn from(v: &'a Line) -> Self {
        v.into_iter().fold(0, Add::add)
    }
}

mod tests {
    #[test]
    fn test_into_usize() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(&l), 25)
    }

    #[test]
    fn test_not() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(&!l), 6);
    }
}
