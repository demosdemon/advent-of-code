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
// Unfortunately, considering only horizontal and vertical lines doesn't give
// you the full picture; you need to also consider diagonal lines.
//
// Because of the limits of the hydrothermal vent mapping system, the lines in
// your list will only ever be horizontal, vertical, or a diagonal line at
// exactly 45 degrees. In other words:
//
// - An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
// - An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//
// Considering all lines from the above example would now produce the following
// diagram:
//
// 1.1....11.
// .111...2..
// ..2.1.111.
// ...1.2.2..
// .112313211
// ...1.2....
// ..1...1...
// .1.....1..
// 1.......1.
// 222111....
//
// You still need to determine the number of points where at least two lines
// overlap. In the above example, this is still anywhere in the diagram with a 2
// or larger - now a total of 12 points.
//
// Consider all of the lines. At how many points do at least two lines overlap?

use super::builder::SolutionBuilder;

fn solve(input: SolutionBuilder) -> usize {
    input.board(|_| true).overlaps()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 12,
        live => 19472,
    });
}
