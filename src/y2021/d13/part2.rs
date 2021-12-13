/*
    --- Part Two ---
    Finish folding the transparent paper according to the instructions. The manual says
    the code is always eight capital letters.

    What code do you use to activate the infrared thermal imaging camera system?
*/

use crate::{Error, IntoAnswer, ParseProblem, Problem};

#[derive(Debug, macros::Answer)]
#[answer(
    example = "#####
#...#
#...#
#...#
#####
",
    live = "####...##..##..#..#...##..##...##..#..#
#.......#.#..#.#..#....#.#..#.#..#.#..#
###.....#.#..#.####....#.#....#..#.####
#.......#.####.#..#....#.#.##.####.#..#
#....#..#.#..#.#..#.#..#.#..#.#..#.#..#
#.....##..#..#.#..#..##...###.#..#.#..#
"
)]
struct Answer(super::Instructions);

impl ParseProblem for Answer {
    type Error = Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error> {
        Ok(Self(
            problem.slice().parse().map_err(crate::Error::from_parse)?,
        ))
    }
}

impl IntoAnswer for Answer {
    type Output = String;

    fn into_answer(self) -> String {
        let matrix: super::Matrix = self.0.coordinates.iter().collect();
        let matrix = self.0.folds.iter().fold(matrix, |prev, fold| prev + fold);
        let s = matrix.to_string();
        println!("{}", &s);
        s
    }
}
