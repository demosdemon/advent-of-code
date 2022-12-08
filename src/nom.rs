// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::clone::Clone;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;

use nom::branch::alt;
use nom::character::complete::i32;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::character::complete::u32;
use nom::character::complete::u64;
use nom::combinator::eof;
use nom::error::ParseError;
use nom::AsChar;
use nom::Compare;
use nom::IResult;
use nom::InputIter;
use nom::InputLength;
use nom::InputTake;
use nom::Slice;

pub fn eol<I, E>(input: I) -> IResult<I, I, E>
where
    I: Clone
        + Compare<&'static str>
        + InputIter
        + InputLength
        + InputTake
        + Slice<Range<usize>>
        + Slice<RangeFrom<usize>>
        + Slice<RangeTo<usize>>,
    E: ParseError<I>,
{
    alt((line_ending, eof))(input)
}

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
