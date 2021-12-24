use nom::{bytes::complete::tag, IResult};

use aoc::nom::{eol, isize};

use super::coordinate::Coordinate;
use super::line::Line;

pub(super) fn coordinate(s: &str) -> IResult<&str, Coordinate> {
    let (s, x) = isize(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y) = isize(s)?;
    Ok((s, Coordinate::new(x, y)))
}

pub(super) fn line(s: &str) -> IResult<&str, Line> {
    let (s, a) = coordinate(s)?;
    let (s, _) = tag(" -> ")(s)?;
    let (s, b) = coordinate(s)?;
    let (s, _) = eol(s)?;
    Ok((s, Line(a, b)))
}

#[cfg(test)]
mod test {
    use nom::Finish;

    use super::{coordinate, line, Coordinate, Line};

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
