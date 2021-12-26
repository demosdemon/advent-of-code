use euclid::point2;
use nom::{bytes::complete::tag, combinator::map, sequence::separated_pair, IResult};

use aoc::nom::usize;

use super::line::Line;
use super::Coordinate;

fn coordinate(s: &str) -> IResult<&str, Coordinate> {
    map(separated_pair(usize, tag(","), usize), |(x, y)| {
        point2(x, y)
    })(s)
}

pub(super) fn line(s: &str) -> IResult<&str, Line> {
    map(
        separated_pair(coordinate, tag(" -> "), coordinate),
        |(a, b)| Line(a, b),
    )(s)
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
}
