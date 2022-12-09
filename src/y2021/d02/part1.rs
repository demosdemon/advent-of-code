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

//! # Day 2: Dive!
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1,
//! down 2, or up 3:
//!
//! - `forward X` increases the horizontal position by X units.
//! - `down X` increases the depth by X units.
//! - `up X` decreases the depth by X units.
//!
//! Note that since you're on a submarine, down and up affect your depth, and so
//! they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! ```text
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at 0. The steps above would
//! then modify them as follows:
//!
//! - `forward 5` adds 5 to your horizontal position, a total of 5.
//! - `down 5` adds 5 to your depth, resulting in a value of 5.
//! - `forward 8` adds 8 to your horizontal position, a total of 13.
//! - `up 3` decreases your depth by 3, resulting in a value of 2.
//! - `down 8` adds 8 to your depth, resulting in a value of 10.
//! - `forward 2` adds 2 to your horizontal position, a total of 15.
//!
//! After following these instructions, you would have a horizontal position of
//! 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following
//! the planned course. What do you get if you multiply your final horizontal
//! position by your final depth?

use super::Direction;

fn solve(input: super::DirectionList) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    for dir in input {
        match dir {
            Direction::Forward(v) => horizontal += v as isize,
            Direction::Up(v) => depth -= v as isize,
            Direction::Down(v) => depth += v as isize,
        }
    }
    (horizontal * depth) as usize
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 150,
        live => 1714950,
    });
}
