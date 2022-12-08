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

use aoc::nom::isize;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

fn state(s: &str) -> IResult<&str, super::State> {
    alt((
        value(super::State::Off, tag("off")),
        value(super::State::On, tag("on")),
    ))(s)
}

fn range(s: &str) -> IResult<&str, (isize, isize)> {
    separated_pair(isize, tag(".."), isize)(s)
}

fn cube(s: &str) -> IResult<&str, super::Cube> {
    let (s, ((x1, x2), ((y1, y2), (z1, z2)))) = separated_pair(
        preceded(tag("x="), range),
        tag(","),
        separated_pair(
            preceded(tag("y="), range),
            tag(","),
            preceded(tag("z="), range),
        ),
    )(s)?;
    let head = super::Coordinate::new(x1, y1, z1);
    let tail = super::Coordinate::new(x2, y2, z2);
    Ok((s, super::Cube::new(head, tail)))
}

pub(super) fn instruction(s: &str) -> IResult<&str, super::Instruction> {
    let (s, (state, cube)) = separated_pair(state, tag(" "), cube)(s)?;
    Ok((s, super::Instruction::new(state, cube)))
}
