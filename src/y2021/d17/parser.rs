use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, combinator::eof, IResult,
};

use aoc::nom::isize;

pub(super) fn target_area(s: &str) -> IResult<&str, super::TargetArea> {
    let (s, _) = tag("target area: x=")(s)?;
    let (s, min_x) = isize(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, max_x) = isize(s)?;
    let (s, _) = tag(", y=")(s)?;
    let (s, min_y) = isize(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, max_y) = isize(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((
        s,
        super::TargetArea {
            min_x,
            max_x,
            min_y,
            max_y,
        },
    ))
}
