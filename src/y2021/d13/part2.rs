/*
    --- Part Two ---
    Finish folding the transparent paper according to the instructions. The manual says
    the code is always eight capital letters.

    What code do you use to activate the infrared thermal imaging camera system?
*/
use crate::IntoAnswer;

#[derive(Debug, derive_more::FromStr, macros::Answer)]
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
