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
