use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::eof;
use nom::multi::count;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

fn dim(s: &str) -> IResult<&str, super::Pixel> {
    let (s, _) = tag(".")(s)?;
    Ok((s, super::Pixel::Dim))
}

fn lit(s: &str) -> IResult<&str, super::Pixel> {
    let (s, _) = tag("#")(s)?;
    Ok((s, super::Pixel::Lit))
}

fn pixel(s: &str) -> IResult<&str, super::Pixel> {
    alt((dim, lit))(s)
}

fn algorithm(s: &str) -> IResult<&str, super::Algorithm> {
    let (s, v) = terminated(count(pixel, super::ALGORITHM_PIXELS), line_ending)(s)?;
    Ok((s, super::Algorithm::new(v.try_into().unwrap())))
}

fn image_line(s: &str) -> IResult<&str, Vec<super::Pixel>> {
    terminated(many1(pixel), line_ending)(s)
}

fn image(s: &str) -> IResult<&str, super::Image> {
    let (s, v) = terminated(many1(image_line), eof)(s)?;
    assert!(v.iter().map(|v| v.len()).all_equal());
    Ok((s, super::Image::new(v)))
}

pub(super) fn input(s: &str) -> IResult<&str, super::Input> {
    let (s, (algo, image)) = separated_pair(algorithm, line_ending, image)(s)?;
    Ok((s, super::Input { algo, image }))
}
