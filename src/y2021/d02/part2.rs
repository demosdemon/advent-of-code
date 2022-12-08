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
// Based on your calculations, the planned course doesn't seem to make any
// sense. You find the submarine manual and discover that the process is
// actually slightly more complicated.
//
// In addition to horizontal position and depth, you'll also need to track a
// third value, aim, which also starts at 0. The commands also mean something
// entirely different than you first thought:
//
// down X increases your aim by X units.
// up X decreases your aim by X units.
// forward X does two things:
// It increases your horizontal position by X units.
// It increases your depth by your aim multiplied by X.
//
// Again note that since you're on a submarine, down and up do the opposite of
// what you might expect: "down" means aiming in the positive direction.
//
// Now, the above example does something different:
//
// forward 5 adds 5 to your horizontal position, a total of 5. Because your aim
// is 0, your depth does not change.
// down 5 adds 5 to your aim, resulting in a value of 5.
// forward 8 adds 8 to your horizontal position, a total of 13. Because your aim
// is 5, your depth increases by 8*5=40.
// up 3 decreases your aim by 3, resulting in a value of 2.
// down 8 adds 8 to your aim, resulting in a value of 10.
// forward 2 adds 2 to your horizontal position, a total of 15. Because your aim
// is 10, your depth increases by 2*10=20 to a total of 60.
//
// After following these new instructions, you would have a horizontal position
// of 15 and a depth of 60. (Multiplying these produces 900.)
//
// Using this new interpretation of the commands, calculate the horizontal
// position and depth you would have after following the planned course. What do
// you get if you multiply your final horizontal position by your final depth?

use super::Direction;

fn solve(input: super::DirectionList) -> usize {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for dir in input {
        match dir {
            Direction::Forward(v) => {
                horizontal += v as isize;
                depth += aim * v as isize;
            }
            Direction::Up(v) => aim -= v as isize,
            Direction::Down(v) => aim += v as isize,
        }
    }
    (horizontal * depth) as usize
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 900,
        live => 1281977850,
    });
}
