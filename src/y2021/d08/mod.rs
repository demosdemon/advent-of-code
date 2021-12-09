pub mod part1;
pub mod part2;

mod parser;

use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

use maplit::hashmap;
use nom::Finish;

fn split_iter<I, F>(iter: I, f: F) -> (I, I)
where
    I: IntoIterator + Default + Extend<I::Item>,
    F: FnMut(&I::Item) -> bool,
{
    iter.into_iter().partition(f)
}

fn split_iter_only_remain<I, F>(iter: I, f: F) -> (I::Item, I)
where
    I: IntoIterator + Default + Extend<I::Item>,
    F: FnMut(&I::Item) -> bool,
{
    let (left, right) = split_iter(iter, f);
    (only(left), right)
}

fn split_iter_only<I, F>(iter: I, mut f: F) -> (I::Item, I::Item)
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    let mut iter = iter.into_iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    assert!(iter.next().is_none());
    let first_match = (f)(&first);
    let second_match = (f)(&second);
    assert_ne!(first_match, second_match);
    if first_match {
        (first, second)
    } else {
        (second, first)
    }
}

fn only<I>(iter: I) -> I::Item
where
    I: IntoIterator,
{
    let mut iter = iter.into_iter();
    let res = iter.next().unwrap();
    assert!(iter.next().is_none());
    res
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, derive_more::Index, derive_more::Deref)]
pub struct Digit(BTreeSet<Segment>);

#[derive(
    Debug, derive_more::Index, derive_more::IndexMut, derive_more::From, derive_more::IntoIterator,
)]
#[into_iterator(ref)]
pub struct Signal([Digit; parser::SIGNAL_DIGITS]);

impl Signal {
    pub fn consume(self) -> HashMap<Digit, usize> {
        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg

        // d0 = abcefg
        // d1 = cf
        // d2 = acdeg
        // d3 = acdfg
        // d4 = bcdf
        // d5 = abdfg
        // d6 = abdefg
        // d7 = acf
        // d8 = abcdefg
        // d9 = abcdfg

        let remain = self.into_iter().collect::<Vec<_>>();
        let (d1, remain) = split_iter_only_remain(remain, |d| d.len() == 2);
        let (d4, remain) = split_iter_only_remain(remain, |d| d.len() == 4);
        let (d7, remain) = split_iter_only_remain(remain, |d| d.len() == 3);
        let (d8, remain) = split_iter_only_remain(remain, |d| d.len() == 7);

        let (len5, len6) = split_iter(remain, |d| d.len() == 5);

        // d2, d3, d5
        assert_eq!(3, len5.len());

        // d3 is a superset of d1
        let (d3, len5) = split_iter_only_remain(len5, |d| d.is_superset(&d1));
        assert_eq!(2, len5.len());

        // d0, d6, d9
        assert_eq!(3, len6.len());

        // d9 is a superset of d4
        let (d9, len6) = split_iter_only_remain(len6, |d| d.is_superset(&d4));
        assert_eq!(2, len6.len());

        // d9 is a superset of d5
        let (d5, d2) = split_iter_only(len5, |d| d.is_subset(&d9));

        // d0 is a superset of d1
        let (d0, d6) = split_iter_only(len6, |d| d.is_superset(&d1));

        let seq_map: HashMap<Digit, usize> = hashmap! {
            d0 => 0,
            d1 => 1,
            d2 => 2,
            d3 => 3,
            d4 => 4,
            d5 => 5,
            d6 => 6,
            d7 => 7,
            d8 => 8,
            d9 => 9,
        };
        assert_eq!(seq_map.len(), 10);
        seq_map
    }
}

#[derive(
    Debug, derive_more::Index, derive_more::IndexMut, derive_more::From, derive_more::IntoIterator,
)]
#[into_iterator(ref)]
pub struct Output([Digit; parser::OUTPUT_DIGITS]);

#[derive(Debug, derive_more::Constructor)]
pub struct Line {
    pub signal: Signal,
    pub output: Output,
}

impl Line {
    pub fn consume(self) -> usize {
        let digit_map = self.signal.consume();

        let mut res: usize = 0;
        for d in self.output {
            res *= 10;
            res += digit_map.get(&d).unwrap();
        }

        res
    }
}

impl FromStr for Line {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::line(s).finish() {
            Ok((_, v)) => Ok(v),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}
