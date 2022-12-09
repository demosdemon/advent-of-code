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

//! # Day 3: Binary Diagnostic
//!
//! The submarine has been making some odd creaking noises, so you ask it to
//! produce a diagnostic report just in case.
//!
//! The diagnostic report (your puzzle input) consists of a list of binary
//! numbers which, when decoded properly, can tell you many useful things about
//! the conditions of the submarine. The first parameter to check is the power
//! consumption.
//!
//! You need to use the binary numbers in the diagnostic report to generate two
//! new binary numbers (called the gamma rate and the epsilon rate). The power
//! consumption can then be found by multiplying the gamma rate by the epsilon
//! rate.
//!
//! Each bit in the gamma rate can be determined by finding the most common bit
//! in the corresponding position of all numbers in the diagnostic report. For
//! example, given the following diagnostic report:
//!
//! ```text
//! 00100
//! 11110
//! 10110
//! 10111
//! 10101
//! 01111
//! 00111
//! 11100
//! 10000
//! 11001
//! 00010
//! 01010
//! ```
//!
//! Considering only the first bit of each number, there are five 0 bits and
//! seven 1 bits. Since the most common bit is 1, the first bit of the gamma
//! rate is 1.
//!
//! The most common second bit of the numbers in the diagnostic report is 0, so
//! the second bit of the gamma rate is 0.
//!
//! The most common value of the third, fourth, and fifth bits are 1, 1, and 0,
//! respectively, and so the final three bits of the gamma rate are 110.
//!
//! So, the gamma rate is the binary number 10110, or 22 in decimal.
//!
//! The epsilon rate is calculated in a similar way; rather than use the most
//! common bit, the least common bit from each position is used. So, the epsilon
//! rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the
//! epsilon rate (9) produces the power consumption, 198.
//!
//! Use the binary numbers in your diagnostic report to calculate the gamma rate
//! and epsilon rate, then multiply them together. What is the power consumption
//! of the submarine? (Be sure to represent your answer in decimal, not binary.)

use super::bit::Bit;
use super::line::Line;

fn solve(input: super::Lines) -> usize {
    let len = input[0].len();
    let mut zeros = vec![0; len];
    let mut ones = vec![0; len];
    for line in input {
        for (idx, b) in line.into_iter().enumerate() {
            match b {
                Bit::Zero => zeros[idx] += 1,
                Bit::One => ones[idx] += 1,
            }
        }
    }
    let gamma = zeros
        .into_iter()
        .zip(ones)
        .map(|(zeros, ones)| zeros <= ones)
        .map(Bit::from)
        .collect::<Line>();
    let epsilon = !gamma.clone();
    let gamma = usize::from(&gamma);
    let epsilon = usize::from(&epsilon);
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 198,
        live => 4103154,
    });
}
