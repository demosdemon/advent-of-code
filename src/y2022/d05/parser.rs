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

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::Input;
use super::Instruction;
use super::Slot;

fn slot(s: &str) -> IResult<&str, Slot> {
    nom::branch::alt((
        map(tag("   "), |_| Slot(None)),
        map(
            delimited(tag("["), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), tag("]")),
            |c| Slot(Some(c)),
        ),
    ))(s)
}

fn row(s: &str) -> IResult<&str, Vec<Slot>> {
    separated_list1(tag(" "), slot)(s)
}

fn rows(s: &str) -> IResult<&str, Vec<Vec<Slot>>> {
    separated_list1(line_ending, row)(s)
}

fn instruction(s: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), u8),
            preceded(tag(" from "), u8),
            preceded(tag(" to "), u8),
        )),
        |(quantity, from, to)| Instruction { quantity, from, to },
    )(s)
}

fn col_ident(s: &str) -> IResult<&str, ()> {
    let seq = separated_pair(tag(" "), u8, tag(" "));
    let list = separated_list1(tag(" "), seq);
    map(list, |_| ())(s)
}

pub(super) fn input(s: &str) -> IResult<&str, Input> {
    map(
        tuple((
            terminated(rows, line_ending),
            terminated(col_ident, line_ending),
            line_ending,
            separated_list1(line_ending, instruction),
        )),
        |(rows, _, _, instructions)| Input { rows, instructions },
    )(s)
}
