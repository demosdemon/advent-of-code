/*
    --- Part Two ---
    Suppose the lanternfish live forever and have unlimited food and space. Would they
    take over the entire ocean?

    After 256 days in the example above, there would be a total of 26984457539
    lanternfish!
*/

use crate::IntoAnswer;

use super::Ocean;

#[derive(derive_more::FromStr)]
struct Answer(Ocean);

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> isize {
        self.0.count(256) as isize
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_answer!(super::Answer, {
        example => 26984457539,
        live => 1710623015163,
    });
}
