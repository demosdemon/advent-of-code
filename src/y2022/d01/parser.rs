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
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

use super::Elf;
use super::Elves;

pub fn elves(s: &str) -> IResult<&str, Elves> {
    map(all_consuming(separated_list1(line_ending, elf)), |mut v| {
        v.sort();
        Elves(v)
    })(s)
}

fn elf(s: &str) -> IResult<&str, Elf> {
    map(
        terminated(separated_list1(line_ending, usize), line_ending),
        Elf,
    )(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_example() {
        const TEXT: &str = include_str!("./inputs/example");
        let _ = elves(TEXT).unwrap();
    }

    #[test]
    fn test_parse_live() {
        const TEXT: &str = include_str!("./inputs/live");
        let _ = elves(TEXT).unwrap();
    }
}
