// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// --- Part Two ---
// As you watch the crane operator expertly rearrange the crates, you notice the
// process isn't following your prediction.
//
// Some mud was covering the writing on the side of the crane, and you quickly
// wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
//
// The CrateMover 9001 is notable for many new and exciting features: air
// conditioning, leather seats, an extra cup holder, and the ability to pick up
// and move multiple crates at once.
//
// Again considering the example above, the crates begin in the same
// configuration:
//
//     [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
// Moving a single crate from stack 2 to stack 1 behaves the same as before:
//
// [D]
// [N] [C]
// [Z] [M] [P]
// 1   2   3
// However, the action of moving three crates from stack 1 to stack 3 means that
// those three moved crates stay in the same order, resulting in this new
// configuration:
//
//         [D]
//         [N]
//     [C] [Z]
//     [M] [P]
// 1   2   3
// Next, as both crates are moved from stack 2 to stack 1, they retain their
// order as well:
//
//         [D]
//         [N]
// [C]     [Z]
// [M]     [P]
// 1   2   3
// Finally, a single crate is still moved from stack 1 to stack 2, but now it's
// crate C that gets moved:
//
//         [D]
//         [N]
//         [Z]
// [M] [C] [P]
// 1   2   3
// In this example, the CrateMover 9001 has put the crates in a totally
// different order: MCD.
//
// Before the rearrangement process finishes, update your simulation so that the
// Elves know where they should stand to be ready to unload the final supplies.
// After the rearrangement procedure completes, what crate ends up on top of
// each stack?
//
//

fn solve(input: super::Input) -> String {
    let mut state: super::State = input.rows.clone().into();
    for i in input.instructions.iter().copied() {
        state.execute(i, true);
    }
    state.top()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => "MCD",
        live => "MHQTLJRLB",
    });
}
