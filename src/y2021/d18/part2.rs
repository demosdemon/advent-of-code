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
// You notice a second question on the back of the homework assignment:
//
// What is the largest magnitude you can get from adding only two of the
// snailfish numbers?
//
// Note that snailfish addition is not commutative - that is, x + y and y + x
// can produce different results.
//
// Again considering the last example homework assignment above:
//
// [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
// [[[5,[2,8]],4],[5,[[9,9],0]]]
// [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
// [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
// [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
// [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
// [[[[5,4],[7,7]],8],[[8,3],8]]
// [[9,3],[[9,9],[6,[4,9]]]]
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
//
// The largest magnitude of the sum of any two snailfish numbers in this list is
// 3993. This is the magnitude of
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// + [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]], which reduces to
// [[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]].
//
// What is the largest magnitude of any sum of two different snailfish numbers
// from the homework assignment?

use itertools::Itertools;

pub fn solve(input: super::Homework) -> usize {
    input
        .into_iter()
        .permutations(2)
        .map(|v| (v[0].clone() + v[1].clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example_a => 90,
        example_b => 115,
        example_c => 140,
        example_d => 3805,
        example_e => 3993,
        live => 4909,
    });
}
