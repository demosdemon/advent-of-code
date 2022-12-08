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

//! # Part Two
//!
//! Considering every single measurement isn't as useful as you expected:
//! there's just too much noise in the data.
//!
//! Instead, consider sums of a three-measurement sliding window. Again
//! considering the above example:
//!
//! ```text
//! 199  A
//! 200  A B
//! 208  A B C
//! 210    B C D
//! 200  E   C D
//! 207  E F   D
//! 240  E F G
//! 269    F G H
//! 260      G H
//! 263        H
//! ```
//!
//! Start by comparing the first and second three-measurement windows. The
//! measurements in the first window are marked A (199, 200, 208); their sum is
//! 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its
//! sum is 618. The sum of measurements in the second window is larger than the
//! sum of the first, so this first comparison increased.
//!
//! Your goal now is to count the number of times the sum of measurements in
//! this sliding window increases from the previous sum. So, compare A with B,
//! then compare B with C, then C with D, and so on. Stop when there aren't
//! enough measurements left to create a new three-measurement sum.
//!
//! In the above example, the sum of each three-measurement window is as
//! follows:
//!
//! ```text
//! A: 607 (N/A - no previous sum)
//! B: 618 (increased)
//! C: 618 (no change)
//! D: 617 (decreased)
//! E: 647 (increased)
//! F: 716 (increased)
//! G: 769 (increased)
//! H: 792 (increased)
//! ```
//!
//! In this example, there are 5 sums that are larger than the previous sum.
//!
//! Consider sums of a three-measurement sliding window. How many sums are
//! larger than the previous sum?

fn solve(input: super::Ocean) -> usize {
    input.windows(4).filter(|s| s[0] < s[3]).count()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 5,
        live => 1748,
    });
}
