/*
    --- Part Two ---
    Suppose the lanternfish live forever and have unlimited food and space. Would they
    take over the entire ocean?

    After 256 days in the example above, there would be a total of 26984457539
    lanternfish!
*/

#[macros::problem]
fn problem(input: &super::Ocean) -> isize {
    input.count(256)
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Problem, {
        example => 26984457539,
        live => 1710623015163,
    });
}
