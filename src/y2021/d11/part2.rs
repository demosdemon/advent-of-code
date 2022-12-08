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
// It seems like the individual flashes aren't bright enough to navigate.
// However, you might have a better option: the flashes seem to be
// synchronizing!
//
// In the example above, the first time all octopuses flash simultaneously is
// step 195:
//
// After step 193:
// 5877777777
// 8877777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
//
// After step 194:
// 6988888888
// 9988888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
//
// After step 195:
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
//
// If you can calculate the exact moments when the octopuses will all flash
// simultaneously, you should be able to navigate through the cavern. What is
// the first step during which all octopuses flash?

pub fn solve(mut input: super::Ocean) -> usize {
    (1..)
        .find_map(|tick| (input.tick() == 100).then_some(tick))
        .unwrap()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 195,
        live => 237,
    });
}
