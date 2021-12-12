use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::eof,
    multi::{count, many_m_n},
    sequence::terminated,
    IResult,
};

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
    let digit = super::Digit(segments.into_iter().collect());
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

pub fn line(s: &str) -> IResult<&str, super::Line> {
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
