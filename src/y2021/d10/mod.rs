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

#[derive(derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(ref)]
#[from_lines(Line)]
pub struct Lines(Vec<Line>);

#[derive(Debug)]
pub enum Line {
    Incomplete(Vec<u8>),
    Invalid(u8),
}

impl std::str::FromStr for Line {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.bytes().collect())
    }
}

impl Line {
    fn score(&self) -> Option<usize> {
        match self {
            Line::Incomplete(v) => Some(v.iter().fold(0, |score, c| {
                (score * 5)
                    + match c {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => unreachable!(),
                    }
            })),
            Line::Invalid(_) => None,
        }
    }
}

impl FromIterator<u8> for Line {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        use Line::*;
        let mut stack = Vec::new();
        for c in iter {
            match c {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b'<' => stack.push(b'>'),
                _ => match stack.pop() {
                    None => return Invalid(c),
                    Some(v) if c == v => {}
                    Some(_) => return Invalid(c),
                },
            }
        }
        stack.reverse();
        Incomplete(stack)
    }
}
