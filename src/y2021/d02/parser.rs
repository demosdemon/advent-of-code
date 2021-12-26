use std::ops::RangeFrom;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u8, combinator::map, error::ParseError,
    sequence::separated_pair, AsChar, Compare, IResult, InputIter, InputLength, InputTake, Parser,
    Slice,
};

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
