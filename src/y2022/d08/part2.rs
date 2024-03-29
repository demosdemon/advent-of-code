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
// Content with the amount of tree cover available, the Elves just need to know
// the best spot to build their tree house: they would like to be able to see a
// lot of trees.
//
// To measure the viewing distance from a given tree, look up, down, left, and
// right from that tree; stop if you reach an edge or at the first tree that is
// the same height or taller than the tree under consideration. (If a tree is
// right on the edge, at least one of its viewing distances will be zero.)
//
// The Elves don't care about distant trees taller than those found by the rules
// above; the proposed tree house has large eaves to keep it dry, so they
// wouldn't be able to see higher than the tree house anyway.
//
// In the example above, consider the middle 5 in the second row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is not blocked; it can see 1 tree (of height 3).
// Looking left, its view is blocked immediately; it can see only 1 tree (of
// height 5, right next to it). Looking right, its view is not blocked; it can
// see 2 trees. Looking down, its view is blocked eventually; it can see 2 trees
// (one of height 3, then the tree of height 5 that blocks its view).
// A tree's scenic score is found by multiplying together its viewing distance
// in each of the four directions. For this tree, this is 4 (found by
// multiplying 1 * 1 * 2 * 2).
//
// However, you can do even better: consider the tree of height 5 in the middle
// of the fourth row:
//
// 30373
// 25512
// 65332
// 33549
// 35390
// Looking up, its view is blocked at 2 trees (by another tree with a height of
// 5). Looking left, its view is not blocked; it can see 2 trees.
// Looking down, its view is also not blocked; it can see 1 tree.
// Looking right, its view is blocked at 2 trees (by a massive tree of height
// 9). This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for
// the tree house.
//
// Consider each tree on your map. What is the highest scenic score possible for
// any tree?
//
//

fn solve(forest: super::Forest<u8>) -> usize {
    forest.score()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 8,
        live => 535680,
    });
}
