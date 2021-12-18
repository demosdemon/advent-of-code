/*
    --- Part Two ---
    Finish folding the transparent paper according to the instructions. The manual says
    the code is always eight capital letters.

    What code do you use to activate the infrared thermal imaging camera system?
*/

#[macros::problem]
fn problem(input: &super::Instructions) -> String {
    let matrix: super::Matrix = input.coordinates.iter().collect();
    let matrix = input.folds.iter().fold(matrix, |prev, fold| prev + fold);
    matrix.to_string()
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Problem, {
        example => include_str!("outputs/example"),
        live => include_str!("outputs/live"),
    });
}
