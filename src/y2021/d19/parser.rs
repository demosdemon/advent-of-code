use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::eof,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

use aoc::nom::{eol, isize, usize};

pub(super) fn beacon(s: &str) -> IResult<&str, super::Coordinate> {
    let comma = tag(",");
    let (s, x) = isize(s)?;
    let (s, _) = comma(s)?;
    let (s, y) = isize(s)?;
    let (s, _) = comma(s)?;
    let (s, z) = isize(s)?;
    Ok((s, super::Coordinate::new(x, y, z)))
}

pub(super) fn scanner(s: &str) -> IResult<&str, super::Scanner> {
    let (s, idx) = terminated(
        delimited(tag("--- scanner "), usize, tag(" ---")),
        line_ending,
    )(s)?;
    let (s, v) = terminated(separated_list1(line_ending, beacon), eol)(s)?;
    Ok((s, super::Scanner { idx, beacons: v }))
}

pub(super) fn report(s: &str) -> IResult<&str, super::Report> {
    let (s, v) = terminated(separated_list1(line_ending, scanner), eof)(s)?;
    Ok((s, super::Report(v)))
}
