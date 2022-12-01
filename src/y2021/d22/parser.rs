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
