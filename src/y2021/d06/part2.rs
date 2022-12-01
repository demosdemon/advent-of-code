// --- Part Two ---
// Suppose the lanternfish live forever and have unlimited food and space. Would
// they take over the entire ocean?
//
// After 256 days in the example above, there would be a total of 26984457539
// lanternfish!

pub fn solve(input: &super::Ocean) -> usize {
    input.count(256)
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 26984457539,
        live => 1710623015163,
    });
}
