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
use nom::combinator::map;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

fn variable(s: &str) -> IResult<&str, super::Variable> {
    let w = map(tag("w"), |_| super::Variable::W);
    let x = map(tag("x"), |_| super::Variable::X);
    let y = map(tag("y"), |_| super::Variable::Y);
    let z = map(tag("z"), |_| super::Variable::Z);
    alt((w, x, y, z))(s)
}

fn value(s: &str) -> IResult<&str, super::Value> {
    let literal = map(isize, super::Value::Literal);
    let variable = map(variable, super::Value::Variable);
    alt((literal, variable))(s)
}

pub(super) fn instruction(s: &str) -> IResult<&str, super::Instruction> {
    let input = map(preceded(tag("inp "), variable), super::Instruction::Input);
    let add = map(
        preceded(tag("add "), separated_pair(variable, tag(" "), value)),
        |(l, r)| super::Instruction::Add(l, r),
    );
    let mul = map(
        preceded(tag("mul "), separated_pair(variable, tag(" "), value)),
        |(l, r)| super::Instruction::Multiply(l, r),
    );
    let div = map(
        preceded(tag("div "), separated_pair(variable, tag(" "), value)),
        |(l, r)| super::Instruction::Divide(l, r),
    );
    let mod_ = map(
        preceded(tag("mod "), separated_pair(variable, tag(" "), value)),
        |(l, r)| super::Instruction::Modulo(l, r),
    );
    let eql = map(
        preceded(tag("eql "), separated_pair(variable, tag(" "), value)),
        |(l, r)| super::Instruction::Equals(l, r),
    );
    alt((input, add, mul, div, mod_, eql))(s)
}
