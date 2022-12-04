use std::ops::Add;
use std::ops::Not;

use super::bit::Bit;

#[derive(
    Default, Debug, Clone, derive_more::IntoIterator, derive_more::Index, macros::FromIterator,
)]
#[into_iterator(owned, ref)]
#[from_iterator(Bit)]
pub struct Line(Vec<Bit>);

::aoc::derive_FromStr_for_bytes_TryFrom_collect!(Line, Bit);

impl<'slice> From<&'slice [Bit]> for Line {
    fn from(v: &'slice [Bit]) -> Self {
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
        v.into_iter().fold(Self::default(), Add::add)
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
