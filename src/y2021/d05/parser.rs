use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::eof,
    IResult,
};

pub(super) fn coordinate(s: &str) -> IResult<&str, super::Coordinate> {
    let (s, x) = i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = i32(s)?;
    Ok((s, super::Coordinate::new(x, y)))
}

pub(super) fn line(s: &str) -> IResult<&str, super::Line> {
    let (s, a) = coordinate(s)?;
    let (s, _) = tag(" -> ")(s)?;
    let (s, b) = coordinate(s)?;
    let (s, _) = alt((eof, line_ending))(s)?;
    Ok((s, super::Line(a, b)))
}

#[cfg(test)]
mod test {
    use nom::Finish;

    use super::{
        super::{Coordinate, Line},
        coordinate, line,
    };

    #[test]
    fn test_valid_coordinate() {
        assert_eq!(
            coordinate("32,52").finish(),
            Ok(("", Coordinate::new(32, 52)))
        );
    }

    #[test]
    fn test_valid_line() {
        assert_eq!(
            line("42,69 -> 0,10").finish(),
            Ok(("", Line(Coordinate::new(42, 69), Coordinate::new(0, 10))))
        )
    }

    #[test]
    fn test_line_consumes_newline() {
        assert_eq!(
            line("42,69 -> 0,10\n").finish(),
            Ok(("", Line(Coordinate::new(42, 69), Coordinate::new(0, 10))))
        )
    }
}
