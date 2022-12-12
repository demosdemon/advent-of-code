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

pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod parser;

use std::collections::BTreeSet;

use hashbrown::HashMap;

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

pub type Digit = BTreeSet<Segment>;

#[derive(
    Debug,
    Clone,
    derive_more::Index,
    derive_more::IndexMut,
    derive_more::From,
    derive_more::IntoIterator,
)]
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

        [
            (d0, 0),
            (d1, 1),
            (d2, 2),
            (d3, 3),
            (d4, 4),
            (d5, 5),
            (d6, 6),
            (d7, 7),
            (d8, 8),
            (d9, 9),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(
    Debug,
    Clone,
    derive_more::Index,
    derive_more::IndexMut,
    derive_more::From,
    derive_more::IntoIterator,
)]
pub struct Output([Digit; parser::OUTPUT_DIGITS]);

#[derive(Debug, Clone, derive_more::Constructor)]
pub struct Line {
    pub signal: Signal,
    pub output: Output,
}

::aoc::derive_FromStr_for_nom!(Line, parser::line);

impl From<Line> for usize {
    fn from(value: Line) -> usize {
        let digit_map = value.signal.consume();
        value
            .output
            .into_iter()
            .fold(0, |prev, next| (prev * 10) + digit_map.get(&next).unwrap())
    }
}

#[derive(Debug, Clone, derive_more::IntoIterator, macros::FromLines)]
#[from_lines(Line)]
pub struct Lines(Vec<Line>);

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
