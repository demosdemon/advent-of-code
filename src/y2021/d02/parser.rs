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

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::error::Error;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

use super::Direction;

fn dir<'input: 'tag, 'tag>(
    tag_val: &'tag str,
    func: fn(u8) -> Direction,
) -> impl Parser<&'input str, Direction, Error<&'input str>> + 'tag {
    map(preceded(tuple((tag(tag_val), tag(" "))), u8), func)
}

pub(super) fn direction(s: &str) -> IResult<&str, Direction> {
    let forward = dir("forward", Direction::Forward);
    let up = dir("up", Direction::Up);
    let down = dir("down", Direction::Down);
    alt((forward, up, down))(s)
}
