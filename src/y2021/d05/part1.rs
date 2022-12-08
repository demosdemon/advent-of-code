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

// --- Day 5: Hydrothermal Venture ---
// You come across a field of hydrothermal vents on the ocean floor! These vents
// constantly produce large, opaque clouds, so it would be best to avoid them if
// possible.
//
// They tend to form in lines; the submarine helpfully produces a list of nearby
// lines of vents (your puzzle input) for you to review. For example:
//
// 0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2
//
// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2
// where x1,y1 are the coordinates of one end the line segment and x2,y2 are the
// coordinates of the other end. These line segments include the points at both
// ends. In other words:
//
// - An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
// - An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//
// For now, only consider horizontal and vertical lines: lines where either x1 =
// x2 or y1 = y2.
//
// So, the horizontal and vertical lines from the above list would produce the
// following diagram:
//
// .......1..
// ..1....1..
// ..1....1..
// .......1..
// .112111211
// ..........
// ..........
// ..........
// ..........
// 222111....
//
// In this diagram, the top left corner is 0,0 and the bottom right corner is
// 9,9. Each position is shown as the number of lines which cover that point or
// . if no line covers that point. The top-left pair of 1s, for example, comes
// from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9
// -> 5,9 and 0,9 -> 2,9.
//
// To avoid the most dangerous areas, you need to determine the number of points
// where at least two lines overlap. In the above example, this is anywhere in
// the diagram with a 2 or larger - a total of 5 points.
//
// Consider only horizontal and vertical lines. At how many points do at least
// two lines overlap?

use super::builder::SolutionBuilder;

fn solve(input: SolutionBuilder) -> usize {
    input.board(|l| !l.is_diagonal()).overlaps()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 5,
        live => 4873,
    });
}
