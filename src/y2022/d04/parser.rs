use nom::character::complete::char;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use super::AssignmentPair;

pub(super) fn assignment_pair(s: &str) -> IResult<&str, AssignmentPair> {
    map(
        separated_pair(
            separated_pair(u8, char('-'), u8),
            char(','),
            separated_pair(u8, char('-'), u8),
        ),
        AssignmentPair::from,
    )(s)
}
