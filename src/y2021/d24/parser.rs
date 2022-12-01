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
