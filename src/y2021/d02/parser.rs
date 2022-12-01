use std::ops::RangeFrom;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::separated_pair;
use nom::AsChar;
use nom::Compare;
use nom::IResult;
use nom::InputIter;
use nom::InputLength;
use nom::InputTake;
use nom::Parser;
use nom::Slice;

fn dir<F, I, E>(key: &'static str, mut f: F) -> impl Parser<I, super::Direction, E>
where
    F: FnMut(u8) -> super::Direction,
    I: InputIter + InputTake + Slice<RangeFrom<usize>> + InputLength + Compare<&'static str>,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
{
    map(separated_pair(tag(key), tag(" "), u8), move |(_, v)| f(v))
}

pub(super) fn direction(s: &str) -> IResult<&str, super::Direction> {
    let forward = dir("forward", super::Direction::Forward);
    let up = dir("up", super::Direction::Up);
    let down = dir("down", super::Direction::Down);
    alt((forward, up, down))(s)
}
