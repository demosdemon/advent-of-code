// --- Part Two ---
// Finish folding the transparent paper according to the instructions. The
// manual says the code is always eight capital letters.
//
// What code do you use to activate the infrared thermal imaging camera system?

pub fn solve(input: super::Instructions) -> String {
    let matrix: super::Matrix = input.coordinates.into_iter().collect();
    let matrix = input.folds.iter().fold(matrix, |prev, fold| prev + fold);
    matrix.to_string()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => include_str!("outputs/example"),
        live => include_str!("outputs/live"),
    });
}
