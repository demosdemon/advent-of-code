use std::ops::{Add, Not};
use std::str::FromStr;

use super::bit::Bit;

#[derive(Default, Debug, Clone, derive_more::IntoIterator, derive_more::Index)]
#[into_iterator(owned, ref)]
pub(super) struct Line(Vec<Bit>);

impl Line {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn partition(self) -> (Line, Line) {
        self.into_iter().partition(|b| b.into())
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

impl Extend<Bit> for Line {
    fn extend<T: IntoIterator<Item = Bit>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl FromIterator<Bit> for Line {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for Line {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().map(TryFrom::try_from).collect()
    }
}

mod tests {
    #[test]
    fn test_into_usize() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(&l), 25 as usize)
    }

    #[test]
    fn test_not() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(&!l), 6 as usize);
    }
}
