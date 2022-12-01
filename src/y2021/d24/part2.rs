// --- Part Two ---
// As the submarine starts booting up things like the Retro Encabulator, you
// realize that maybe you don't need all these submarine features after all.
//
// What is the smallest model number accepted by MONAD?

pub fn solve(input: &super::Instructions) -> usize {
    let (min, _) = input.opt();
    min
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        live => 19518121316118,
    });
}
