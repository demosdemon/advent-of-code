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
// The resulting polymer isn't nearly strong enough to reinforce the submarine.
// You'll need to run more steps of the pair insertion process; a total of 40
// steps should do it.
//
// In the above example, the most common element is B (occurring 2192039569602
// times) and the least common element is H (occurring 3849876073 times);
// subtracting these produces 2188189693529.
//
// Apply 40 steps of pair insertion to the polymer template and find the most
// and least common elements in the result. What do you get if you take the
// quantity of the most common element and subtract the quantity of the least
// common element?

pub fn solve(input: super::Instructions) -> usize {
    (0..40).fold(input, |i, _| i.step()).score()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 2188189693529,
        live => 1976896901756,
    });
}
