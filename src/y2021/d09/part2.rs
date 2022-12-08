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
// Next, you need to find the largest basins so you know what areas are most
// important to avoid.
//
// A basin is all locations that eventually flow downward to a single low point.
// Therefore, every low point has a basin, although some basins are very small.
// Locations of height 9 do not count as being in any basin, and all other
// locations will always be part of exactly one basin.
//
// The size of a basin is the number of locations within the basin, including
// the low point. The example above has four basins.
//
// The top-left basin, size 3:
//
// 21**99943210
// 3**987894921
// 9856789892
// 8767896789
// 9899965678
//
// The top-right basin, size 9:
//
// 21999**43210**
// 398789**4**9**21**
// 985678989**2**
// 8767896789
// 9899965678
//
// The middle basin, size 14:
//
// 2199943210
// 39**878**94921
// 9**85678**9892
// 87678**96789
// 9**8**99965678
//
// The bottom-right basin, size 9:
//
// 2199943210
// 3987894921
// 9856789**8**92
// 876789**678**9
// 98999**65678**
//
// Find the three largest basins and multiply their sizes together. In the above
// example, this is 9 * 14 * 9 = 1134.
//
// What do you get if you multiply together the sizes of the three largest
// basins?

pub fn solve(input: super::Ocean) -> usize {
    let mut v = input.basins();
    v.sort_unstable();
    v.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 1134,
        live => 920448,
    });
}
