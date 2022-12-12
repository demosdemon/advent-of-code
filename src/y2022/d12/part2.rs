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

//! --- Part Two ---
//! As you walk up the hill, you suspect that the Elves will want to turn this
//! into a hiking trail. The beginning isn't very scenic, though; perhaps you
//! can find a better starting point.
//!
//! To maximize exercise while hiking, the trail should start as low as
//! possible: elevation a. The goal is still the square marked E. However, the
//! trail should still be direct, taking the fewest steps to reach its goal. So,
//! you'll need to find the shortest path from any square at elevation a to the
//! square marked E.
//!
//! Again consider the example from above:
//!
//! Sabqponm
//! abcryxxl
//! accszExk
//! acctuvwj
//! abdefghi
//! Now, there are six choices for starting position (five marked a, plus the
//! square marked S that counts as being at elevation a). If you start at the
//! bottom-left square, you can reach the goal most quickly:
//!
//! ...v<<<<
//! ...vv<<^
//! ...v>E^^
//! .>v>>>^^
//! >^>>>>>^
//! This path reaches the goal in only 29 steps, the fewest possible.
//!
//! What is the fewest steps required to move starting from any square with
//! elevation a to the location that should get the best signal?

fn solve(input: super::Input) -> usize {
    let starts = input
        .matrix
        .iter()
        .filter_map(|(pos, &cell)| if cell == 0 { Some(pos) } else { None });
    input.shortest_path(starts)
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 29,
        live => 512,
    });
}
