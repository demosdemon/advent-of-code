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

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::eof;
use nom::multi::count;
use nom::multi::many_m_n;
use nom::sequence::terminated;
use nom::IResult;

const SEGMENTS: &str = "abcdefg";
pub const SIGNAL_DIGITS: usize = 10;
pub const OUTPUT_DIGITS: usize = 4;

fn segment(s: &str) -> IResult<&str, super::Segment> {
    let (s, c) = one_of(SEGMENTS)(s)?;
    use super::Segment::*;
    let segment = match c {
        'a' => A,
        'b' => B,
        'c' => C,
        'd' => D,
        'e' => E,
        'f' => F,
        'g' => G,
        _ => unreachable!(),
    };
    Ok((s, segment))
}

fn digit(s: &str) -> IResult<&str, super::Digit> {
    let (s, segments) = many_m_n(1, SEGMENTS.len(), segment)(s)?;
    let len_before_collect = segments.len();
    let digit = segments.into_iter().collect::<super::Digit>();
    assert_eq!(
        len_before_collect,
        digit.len(),
        "digit has duplicate segments"
    );
    Ok((s, digit))
}

fn digits<const LEN: usize>(s: &str) -> IResult<&str, [super::Digit; LEN]> {
    let terminus = alt((tag(" "), line_ending, eof));
    let segment = terminated(digit, terminus);
    let (s, res) = count(segment, LEN)(s)?;
    // safe: an error is unwrapped from `count` if res.len() != LEN
    Ok((s, res.try_into().unwrap()))
}

fn signal_digits(s: &str) -> IResult<&str, super::Signal> {
    let (s, v) = digits(s)?;
    Ok((s, v.into()))
}

fn output_digits(s: &str) -> IResult<&str, super::Output> {
    let (s, v) = digits(s)?;
    Ok((s, v.into()))
}

pub(super) fn line(s: &str) -> IResult<&str, super::Line> {
    let (s, signal) = signal_digits(s)?;
    // digits consumes the trailing space
    let (s, _) = tag("| ")(s)?;
    let (s, output) = output_digits(s)?;
    Ok((s, super::Line::new(signal, output)))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_example_no_panic() {
        nom::multi::many1(super::line)(include_str!("inputs/example")).unwrap();
    }
}
