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

use aoc::nom::usize;
use euclid::point2;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

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

    use super::coordinate;
    use super::line;
    use super::Coordinate;
    use super::Line;

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
