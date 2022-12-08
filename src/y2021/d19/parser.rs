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

use aoc::nom::eol;
use aoc::nom::isize;
use aoc::nom::usize;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::eof;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::IResult;

pub(super) fn beacon(s: &str) -> IResult<&str, super::Coordinate> {
    let comma = tag(",");
    let (s, x) = isize(s)?;
    let (s, _) = comma(s)?;
    let (s, y) = isize(s)?;
    let (s, _) = comma(s)?;
    let (s, z) = isize(s)?;
    Ok((s, super::Coordinate::new(x, y, z)))
}

pub(super) fn scanner(s: &str) -> IResult<&str, super::Scanner> {
    let (s, idx) = terminated(
        delimited(tag("--- scanner "), usize, tag(" ---")),
        line_ending,
    )(s)?;
    let (s, v) = terminated(separated_list1(line_ending, beacon), eol)(s)?;
    Ok((s, super::Scanner { idx, beacons: v }))
}

pub(super) fn report(s: &str) -> IResult<&str, super::Report> {
    let (s, v) = terminated(separated_list1(line_ending, scanner), eof)(s)?;
    Ok((s, super::Report(v)))
}
