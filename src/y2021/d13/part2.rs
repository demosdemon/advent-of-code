/*
    --- Part Two ---
    Finish folding the transparent paper according to the instructions. The manual says
    the code is always eight capital letters.

    What code do you use to activate the infrared thermal imaging camera system?
*/
use crate::IntoAnswer;

#[derive(Debug, derive_more::FromStr)]
struct Answer(super::Instructions);

impl IntoAnswer for Answer {
    type Output = String;

    fn into_answer(self) -> String {
        let matrix: super::Matrix = self.0.coordinates.iter().collect();
        let matrix = self.0.folds.iter().fold(matrix, |prev, fold| prev + fold);
        let s = format!("{}", matrix);
        println!("{}", s);
        s
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Answer, {
        example => include_str!("outputs/example"),
        live => include_str!("outputs/live"),
    });
}
