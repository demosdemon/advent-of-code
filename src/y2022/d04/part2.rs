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
// It seems like there is still quite a bit of duplicate work planned. Instead,
// the Elves would like to know the number of pairs that overlap at all.
//
// In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't
// overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and
// 2-6,4-8) do overlap:
//
// 5-7,7-9 overlaps in a single section, 7.
// 2-8,3-7 overlaps all of the sections 3 through 7.
// 6-6,4-6 overlaps in a single section, 6.
// 2-6,4-8 overlaps in sections 4, 5, and 6.
// So, in this example, the number of overlapping assignment pairs is 4.
//
// In how many assignment pairs do the ranges overlap?
//

fn solve(input: super::Assignments) -> usize {
    input
        .0
        .iter()
        .filter(|assignment_pair| assignment_pair.is_overlapping())
        .count()
}

#[cfg(test)]
mod test {
    ::aoc::tests_for_problem!(super::solve, {
        example => 4,
        live => 891,
    });
}
