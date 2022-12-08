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
// The crabs don't seem interested in your proposed solution. Perhaps you
// misunderstand crab engineering?
//
// As it turns out, crab submarine engines don't burn fuel at a constant rate.
// Instead, each change of 1 step in horizontal position costs 1 more unit of
// fuel than the last: the first step costs 1, the second step costs 2, the
// third step costs 3, and so on.
//
// As each crab moves, moving further becomes more expensive. This changes the
// best horizontal position to align them all on; in the example above, this
// becomes 5:
//
// Move from 16 to 5: 66 fuel
// Move from 1 to 5: 10 fuel
// Move from 2 to 5: 6 fuel
// Move from 0 to 5: 15 fuel
// Move from 4 to 5: 1 fuel
// Move from 2 to 5: 6 fuel
// Move from 7 to 5: 3 fuel
// Move from 1 to 5: 10 fuel
// Move from 2 to 5: 6 fuel
// Move from 14 to 5: 45 fuel
//
// This costs a total of 168 fuel. This is the new cheapest possible outcome;
// the old alignment position (2) now costs 206 fuel instead.
//
// Determine the horizontal position that the crabs can align to using the least
// fuel possible so they can make you an escape route! How much fuel must they
// spend to align to that position?

pub fn solve(input: super::Ocean) -> usize {
    input.solve(|d| (d * (d + 1)) / 2)
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 168,
        live => 96361606,
    });
}
