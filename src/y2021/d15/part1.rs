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

// --- Day 15: Chiton ---
// You've almost reached the exit of the cave, but the walls are getting closer
// together. Your submarine can barely still fit, though; the main problem is
// that the walls of the cave are covered in chitons, and it would be best not
// to bump any of them.
//
// The cavern is large, but has a very low ceiling, restricting your motion to
// two dimensions. The shape of the cavern resembles a square; a quick scan of
// chiton density produces a map of risk level throughout the cave (your puzzle
// input). For example:
//
// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581
//
// You start in the top left position, your destination is the bottom right
// position, and you cannot move diagonally. The number at each position is its
// risk level; to determine the total risk of an entire path, add up the risk
// levels of each position you enter (that is, don't count the risk level of
// your starting position unless you enter it; leaving it adds no risk to your
// total).
//
// Your goal is to find a path with the lowest total risk. In this example, a
// path with the lowest total risk is highlighted here:
//
// 1163751742
// _
// 1381373672
// _
// 2136511328
// _______
// 3694931569
// __
// 7463417111
// _
// 1319128137
// __
// 1359912421
// _
// 3125421639
// _
// 1293138521
// __
// 2311944581
// _
//
// The total risk of this path is 40 (the starting position is never entered, so
// its risk is not counted).
//
// What is the lowest total risk of any path from the top left to the bottom
// right?

pub fn solve(input: super::Cave) -> usize {
    input.cost().unwrap()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 40,
        live => 562,
    });
}
