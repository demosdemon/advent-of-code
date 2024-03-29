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

use std::borrow::Cow;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Error;

#[derive(Debug)]
enum EdgeType {
    Big,
    Little,
}

impl EdgeType {
    fn big(&self) -> bool {
        matches!(self, &Self::Big)
    }
}

impl FromStr for EdgeType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.bytes().all(|c| c.is_ascii_uppercase()) {
            Ok(Self::Big)
        } else {
            Ok(Self::Little)
        }
    }
}

#[derive(Default)]
struct Names<'a>(Vec<&'a str>);

impl<'a> Names<'a> {
    fn get(&mut self, k: &'a str) -> usize {
        self.find(k).unwrap_or_else(|| {
            self.0.push(k);
            self.0.len() - 1
        })
    }

    fn find(&self, k: &'a str) -> Option<usize> {
        self.0.iter().position(|v| v == &k)
    }
}

#[derive(Default)]
struct Edges<'a> {
    names: Names<'a>,
    edges: Vec<Vec<(usize, EdgeType)>>,
}

impl<'a> Edges<'a> {
    fn resize(&mut self, min_len: usize) {
        if min_len > self.edges.len() {
            self.edges.resize_with(min_len, Default::default);
        }
    }

    fn add_edge(&mut self, src: &'a str, dst: &'a str) {
        let typ = dst.parse().unwrap();
        let lhs = self.names.get(src);
        let rhs = self.names.get(dst);
        self.resize(std::cmp::max(lhs, rhs) + 1);
        self.edges[lhs].push((rhs, typ))
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Edges<'a> {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let mut edges = Self::default();
        for (lhs, rhs) in iter {
            edges.add_edge(lhs, rhs);
            edges.add_edge(rhs, lhs);
        }
        edges
    }
}

#[derive(Clone)]
struct State<'a, 'b> {
    ocean: &'a Ocean,
    bonus: bool,
    bonus_visit: Option<usize>,
    visited: Cow<'b, [usize]>,
    paths: Cow<'b, [usize]>,
}

impl<'a, 'b> State<'a, 'b> {
    fn new(ocean: &'a Ocean, bonus: bool) -> (Self, usize) {
        (
            Self {
                ocean,
                bonus,
                bonus_visit: None,
                visited: Default::default(),
                paths: Default::default(),
            },
            ocean.head,
        )
    }

    fn visit(&self, (next, typ): &(usize, EdgeType)) -> Option<(Self, usize)> {
        let next_paths = || {
            let mut copy = self.paths.clone();
            copy.to_mut().push(*next);
            copy
        };
        if self.ocean.head == *next {
            None
        } else if typ.big() {
            Some((
                Self {
                    paths: next_paths(),
                    ..self.clone()
                },
                *next,
            ))
        } else if let Err(ins) = self.visited.binary_search(next) {
            Some((
                Self {
                    visited: {
                        let mut copy = self.visited.clone();
                        copy.to_mut().insert(ins, *next);
                        copy
                    },
                    paths: next_paths(),
                    ..self.clone()
                },
                *next,
            ))
        } else {
            (self.bonus && self.bonus_visit.is_none()).then(|| {
                (
                    Self {
                        bonus_visit: Some(*next),
                        paths: next_paths(),
                        ..self.clone()
                    },
                    *next,
                )
            })
        }
    }

    fn path(&self) -> Path<'a> {
        let mut paths = self.paths.clone();
        paths.to_mut().push(self.ocean.tail);
        Path(
            paths
                .iter()
                .map(|idx| self.ocean.names[*idx].as_str())
                .collect(),
        )
    }
}

pub struct Path<'a>(Box<[&'a str]>);

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.join(",");
        f.write_str(&s)
    }
}

struct PathIter<'a, 'b> {
    ocean: &'a Ocean,
    stack: VecDeque<(State<'a, 'b>, usize)>,
}

impl<'a, 'b> PathIter<'a, 'b> {
    fn new(ocean: &'a Ocean, bonus: bool) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(State::new(ocean, bonus));
        Self { ocean, stack }
    }
}

impl<'a, 'b> Iterator for PathIter<'a, 'b> {
    type Item = Path<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((state, cur)) = self.stack.pop_front() {
            if cur == self.ocean.tail {
                return Some(state.path());
            }
            self.stack.extend(
                self.ocean
                    .edges
                    .get(cur)
                    .unwrap()
                    .iter()
                    .filter_map(|edge| state.visit(edge)),
            );
            self.next()
        } else {
            None
        }
    }
}

impl Ocean {
    pub fn paths(&self, bonus: bool) -> impl Iterator<Item = Path<'_>> {
        PathIter::new(self, bonus)
    }
}

#[derive(Debug, Default, macros::TryFromStr)]
pub struct Ocean {
    names: Box<[String]>,
    edges: Vec<Vec<(usize, EdgeType)>>,
    head: usize,
    tail: usize,
}

impl FromStr for Ocean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s
            .lines()
            .map(|l| l.split_once('-').context("splitting on hyphen separator"))
            .collect::<Result<Edges, _>>()?;
        let head = edges.names.find("start").context("finding `start` node")?;
        let tail = edges.names.find("end").context("finding `end` node")?;
        Ok(Self {
            names: edges.names.0.into_iter().map(|n| n.to_owned()).collect(),
            edges: edges.edges,
            head,
            tail,
        })
    }
}
