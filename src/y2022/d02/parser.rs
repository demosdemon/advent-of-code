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
use nom::character::complete::char;
use nom::combinator::into;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use super::Instruction;
use super::Rhs;
use super::Throw;

fn lhs(s: &str) -> IResult<&str, Throw> {
    alt((
        map(char('A'), |_| Throw::Rock),
        map(char('B'), |_| Throw::Paper),
        map(char('C'), |_| Throw::Scissors),
    ))(s)
}

fn rhs(s: &str) -> IResult<&str, Rhs> {
    alt((
        map(char('X'), |_| Rhs::X),
        map(char('Y'), |_| Rhs::Y),
        map(char('Z'), |_| Rhs::Z),
    ))(s)
}

pub(super) fn instruction(s: &str) -> IResult<&str, Instruction> {
    into(separated_pair(lhs, char(' '), rhs))(s)
}
