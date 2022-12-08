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
// By the time you calculate the answer to the Elves' question, they've already
// realized that the Elf carrying the most Calories of food might eventually run
// out of snacks.
//
// To avoid this unacceptable situation, the Elves would instead like to know
// the total Calories carried by the top three Elves carrying the most Calories.
// That way, even if one of those Elves runs out of snacks, they still have two
// backups.
//
// In the example above, the top three Elves are the fourth Elf (with 24000
// Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with
// 10000 Calories). The sum of the Calories carried by these three elves is
// 45000.
//
// Find the top three Elves carrying the most Calories. How many Calories are
// those Elves carrying in total?

pub fn solve(input: super::Elves) -> usize {
    input.0.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 45000,
        live => 196804,
    });
}
