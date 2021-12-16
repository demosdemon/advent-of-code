/*
    --- Part Two ---
    It seems like the individual flashes aren't bright enough to navigate. However, you
    might have a better option: the flashes seem to be synchronizing!

    In the example above, the first time all octopuses flash simultaneously is step 195:

    After step 193:
    5877777777
    8877777777
    7777777777
    7777777777
    7777777777
    7777777777
    7777777777
    7777777777
    7777777777
    7777777777

    After step 194:
    6988888888
    9988888888
    8888888888
    8888888888
    8888888888
    8888888888
    8888888888
    8888888888
    8888888888
    8888888888

    After step 195:
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000
    0000000000

    If you can calculate the exact moments when the octopuses will all flash
    simultaneously, you should be able to navigate through the cavern. What is the first
    step during which all octopuses flash?
*/

use crate::IntoAnswer;

#[derive(Debug, derive_more::FromStr)]
struct Answer(super::Ocean);

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> isize {
        let mut ocean = self.0;
        (1..)
            .find_map(|tick| (ocean.tick() == 100).then(|| tick))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_problem!(super::Answer, {
        example => 195,
        live => 237,
    });
}
