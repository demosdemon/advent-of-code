/*
    --- Part Two ---
    Sometimes, it's a good idea to appreciate just how big the ocean is. Using the
    Manhattan distance, how far apart do the scanners get?

    In the above example, scanners 2 (1105,-1205,1229) and 3 (-92,-2380,-20) are the
    largest Manhattan distance apart. In total, they are 1197 + 1175 + 1249 = 3621 units
    apart.

    What is the largest Manhattan distance between any two scanners?
*/

use itertools::Itertools;

#[macros::problem]
fn answer(input: &super::Report) -> usize {
    let set: super::BeaconSet = input.0.as_slice().into();
    set.scanners
        .iter()
        .permutations(2)
        .map(|w| (w[0] - w[1]).abs())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::Answer, {
        example => 3621,
        live => 13348,
    });
}
