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
// Sometimes, it's a good idea to appreciate just how big the ocean is. Using
// the Manhattan distance, how far apart do the scanners get?
//
// In the above example, scanners 2 (1105,-1205,1229) and 3 (-92,-2380,-20) are
// the largest Manhattan distance apart. In total, they are 1197 + 1175 + 1249 =
// 3621 units apart.
//
// What is the largest Manhattan distance between any two scanners?

use itertools::Itertools;

pub fn solve(input: super::Report) -> usize {
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
    ::aoc::tests_for_problem!(super::solve, {
        example => 3621,
        live => 13348,
    });
}
