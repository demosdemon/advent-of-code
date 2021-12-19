use std::ops::RangeFrom;

use nom::{
    character::complete::{i32, i64, u32, u64},
    error::ParseError,
    AsChar, Compare, IResult, InputIter, InputLength, InputTake, Slice,
};

pub fn isize<I, E>(input: I) -> IResult<I, isize, E>
where
    I: InputIter
        + Slice<RangeFrom<usize>>
        + InputLength
        + InputTake
        + Clone
        + for<'a> Compare<&'a [u8]>,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
{
    if isize::BITS == 32 {
        i32(input).map(|(i, v)| (i, v as isize))
    } else {
        i64(input).map(|(i, v)| (i, v as isize))
    }
}

pub fn usize<I, E>(input: I) -> IResult<I, usize, E>
where
    I: InputIter + Slice<RangeFrom<usize>> + InputLength,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
{
    if usize::BITS == 32 {
        u32(input).map(|(i, v)| (i, v as usize))
    } else {
        u64(input).map(|(i, v)| (i, v as usize))
    }
}
