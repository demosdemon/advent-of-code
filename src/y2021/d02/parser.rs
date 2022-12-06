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
