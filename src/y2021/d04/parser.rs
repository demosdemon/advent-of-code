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

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::character::complete::u8;
use nom::combinator::eof;
use nom::combinator::map;
use nom::multi::count;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

fn pull_list(s: &str) -> IResult<&str, Vec<u8>> {
    terminated(separated_list1(tag(","), u8), line_ending)(s)
}

fn tile(s: &str) -> IResult<&str, super::matrix::Tile> {
    map(u8, |v| v.into())(s)
}

fn bingo_board_line(s: &str) -> IResult<&str, Vec<super::matrix::Tile>> {
    preceded(
        space0,
        terminated(separated_list1(space1, tile), line_ending),
    )(s)
}

fn bingo_board(s: &str) -> IResult<&str, super::matrix::Board> {
    map(count(bingo_board_line, 5), super::matrix::Board::new)(s)
}

pub(super) fn game(s: &str) -> IResult<&str, super::Game> {
    map(
        terminated(
            separated_pair(
                pull_list,
                line_ending,
                separated_list1(line_ending, bingo_board),
            ),
            eof,
        ),
        |(pulls, boards)| super::Game { pulls, boards },
    )(s)
}
