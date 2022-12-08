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
// As you prepare to give the amphipods your solution, you notice that the
// diagram they handed you was actually folded up. As you unfold it, you
// discover an extra part of the diagram.
//
// Between the first and second lines of text that contain amphipod starting
// positions, insert the following lines:
//
// #D#C#B#A#
// #D#B#A#C#
//
// So, the above example now becomes:
//
// #############
// #...........#
// ###B#C#B#D###
// #D#C#B#A#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// The amphipods still want to be organized into rooms similar to before:
//
// #############
// #...........#
// ###A#B#C#D###
// #A#B#C#D#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// In this updated example, the least energy required to organize these
// amphipods is 44169:
//
// #############
// #...........#
// ###B#C#B#D###
// #D#C#B#A#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// #############
// #..........D#
// ###B#C#B#.###
// #D#C#B#A#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// #############
// #A.........D#
// ###B#C#B#.###
// #D#C#B#.#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// #############
// #A........BD#
// ###B#C#.#.###
// #D#C#B#.#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// #############
// #A......B.BD#
// ###B#C#.#.###
// #D#C#.#.#
// #D#B#A#C#
// #A#D#C#A#
// #########
//
// #############
// #AA.....B.BD#
// ###B#C#.#.###
// #D#C#.#.#
// #D#B#.#C#
// #A#D#C#A#
// #########
//
// #############
// #AA.....B.BD#
// ###B#.#.#.###
// #D#C#.#.#
// #D#B#C#C#
// #A#D#C#A#
// #########
//
// #############
// #AA.....B.BD#
// ###B#.#.#.###
// #D#.#C#.#
// #D#B#C#C#
// #A#D#C#A#
// #########
//
// #############
// #AA...B.B.BD#
// ###B#.#.#.###
// #D#.#C#.#
// #D#.#C#C#
// #A#D#C#A#
// #########
//
// #############
// #AA.D.B.B.BD#
// ###B#.#.#.###
// #D#.#C#.#
// #D#.#C#C#
// #A#.#C#A#
// #########
//
// #############
// #AA.D...B.BD#
// ###B#.#.#.###
// #D#.#C#.#
// #D#.#C#C#
// #A#B#C#A#
// #########
//
// #############
// #AA.D.....BD#
// ###B#.#.#.###
// #D#.#C#.#
// #D#B#C#C#
// #A#B#C#A#
// #########
//
// #############
// #AA.D......D#
// ###B#.#.#.###
// #D#B#C#.#
// #D#B#C#C#
// #A#B#C#A#
// #########
//
// #############
// #AA.D......D#
// ###B#.#C#.###
// #D#B#C#.#
// #D#B#C#.#
// #A#B#C#A#
// #########
//
// #############
// #AA.D.....AD#
// ###B#.#C#.###
// #D#B#C#.#
// #D#B#C#.#
// #A#B#C#.#
// #########
//
// #############
// #AA.......AD#
// ###B#.#C#.###
// #D#B#C#.#
// #D#B#C#.#
// #A#B#C#D#
// #########
//
// #############
// #AA.......AD#
// ###.#B#C#.###
// #D#B#C#.#
// #D#B#C#.#
// #A#B#C#D#
// #########
//
// #############
// #AA.......AD#
// ###.#B#C#.###
// #.#B#C#.#
// #D#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #AA.D.....AD#
// ###.#B#C#.###
// #.#B#C#.#
// #.#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #A..D.....AD#
// ###.#B#C#.###
// #.#B#C#.#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #...D.....AD#
// ###.#B#C#.###
// #A#B#C#.#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #.........AD#
// ###.#B#C#.###
// #A#B#C#D#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #..........D#
// ###A#B#C#.###
// #A#B#C#D#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// #############
// #...........#
// ###A#B#C#D###
// #A#B#C#D#
// #A#B#C#D#
// #A#B#C#D#
// #########
//
// Using the initial configuration from the full diagram, what is the least
// energy required to organize the amphipods?

pub fn solve(input: super::Maze<7>) -> usize {
    input.shortest_path()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        part2_example => 44169,
        part2_live => 41366,
    });
}
