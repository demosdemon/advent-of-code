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

// --- Day 7: The Treachery of Whales ---
// A giant whale has decided your submarine is its next meal, and it's much
// faster than you are. There's nowhere to run!
//
// Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep
// for them otherwise) zooms in to rescue you! They seem to be preparing to
// blast a hole in the ocean floor; sensors indicate a massive underground cave
// system just beyond where they're aiming!
//
// The crab submarines all need to be aligned before they'll have enough power
// to blast a large enough hole for your submarine to get through. However, it
// doesn't look like they'll be aligned before the whale catches you! Maybe you
// can help?
//
// There's one major catch - crab submarines can only move horizontally.
//
// You quickly make a list of the horizontal position of each crab (your puzzle
// input). Crab submarines have limited fuel, so you need to find a way to make
// all of their horizontal positions match while requiring them to spend as
// little fuel as possible.
//
// For example, consider the following horizontal positions:
//
// 16,1,2,0,4,2,7,1,2,14
//
// This means there's a crab with horizontal position 16, a crab with horizontal
// position 1, and so on.
//
// Each change of 1 step in horizontal position of a single crab costs 1 fuel.
// You could choose any horizontal position to align them all on, but the one
// that costs the least fuel is horizontal position 2:
//
// Move from 16 to 2: 14 fuel
// Move from 1 to 2: 1 fuel
// Move from 2 to 2: 0 fuel
// Move from 0 to 2: 2 fuel
// Move from 4 to 2: 2 fuel
// Move from 2 to 2: 0 fuel
// Move from 7 to 2: 5 fuel
// Move from 1 to 2: 1 fuel
// Move from 2 to 2: 0 fuel
// Move from 14 to 2: 12 fuel
//
// This costs a total of 37 fuel. This is the cheapest possible outcome; more
// expensive outcomes include aligning at position 1 (41 fuel), position 3 (39
// fuel), or position 10 (71 fuel).
//
// Determine the horizontal position that the crabs can align to using the least
// fuel possible. How much fuel must they spend to align to that position?

pub fn solve(input: super::Ocean) -> usize {
    input.solve(|d| d)
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 37,
        live => 345197,
    });
}
