use std::ops::{Index, Not};
use std::str::FromStr;

use crate::bit::Bit;
use crate::errors::Error;

#[derive(Default, Debug, Clone)]
pub struct Line(Vec<Bit>);

impl Line {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn ceiling(self) -> Bit {
        let (ones, zeros): (Vec<_>, Vec<_>) = self.0.into_iter().partition(|b| b.to_owned().into());
        if ones.len() < zeros.len() {
            Bit::Zero
        } else {
            Bit::One
        }
    }
}

impl Index<usize> for Line {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Not for Line {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0.into_iter().map(Not::not).collect())
    }
}

impl From<Line> for usize {
    fn from(l: Line) -> Self {
        let mut v = Self::default();
        for b in l.0 {
            v <<= 1;
            v += b as usize;
        }
        v
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().map(TryFrom::try_from).collect()
    }
}

impl IntoIterator for Line {
    type Item = Bit;

    type IntoIter = ::std::vec::IntoIter<Bit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for &Line {
    type Item = Bit;

    type IntoIter = ::std::vec::IntoIter<Bit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

mod tests {
    #[test]
    fn test_into_usize() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(l), 25 as usize)
    }

    #[test]
    fn test_not() {
        use super::Line;
        let l: Line = "11001".parse().unwrap();
        assert_eq!(usize::from(!l), 6 as usize);
    }
}
