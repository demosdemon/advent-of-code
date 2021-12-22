/*
    --- Part Two ---
    You still can't quite make out the details in the image. Maybe you just didn't
    enhance it enough.

    If you enhance the starting input image in the above example a total of 50 times,
    3351 pixels are lit in the final output image.

    Start again with the original input image and apply the image enhancement algorithm
    50 times. How many pixels are lit in the resulting image?
*/

pub fn solve(input: &super::Input) -> usize {
    input.fold(50)
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 3351,
        live => 18723,
    });
}
