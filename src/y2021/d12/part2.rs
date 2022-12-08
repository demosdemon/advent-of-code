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
// After reviewing the available paths, you realize you might have time to visit
// a single small cave twice. Specifically, big caves can be visited any number
// of times, a single small cave can be visited at most twice, and the remaining
// small caves can be visited at most once. However, the caves named start and
// end can only be visited exactly once each: once you leave the start cave, you
// may not return to it, and once you reach the end cave, the path must end
// immediately.
//
// Now, the 36 possible paths through the first example above are:
//
// start,A,b,A,b,A,c,A,end
// start,A,b,A,b,A,end
// start,A,b,A,b,end
// start,A,b,A,c,A,b,A,end
// start,A,b,A,c,A,b,end
// start,A,b,A,c,A,c,A,end
// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,d,b,A,c,A,end
// start,A,b,d,b,A,end
// start,A,b,d,b,end
// start,A,b,end
// start,A,c,A,b,A,b,A,end
// start,A,c,A,b,A,b,end
// start,A,c,A,b,A,c,A,end
// start,A,c,A,b,A,end
// start,A,c,A,b,d,b,A,end
// start,A,c,A,b,d,b,end
// start,A,c,A,b,end
// start,A,c,A,c,A,b,A,end
// start,A,c,A,c,A,b,end
// start,A,c,A,c,A,end
// start,A,c,A,end
// start,A,end
// start,b,A,b,A,c,A,end
// start,b,A,b,A,end
// start,b,A,b,end
// start,b,A,c,A,b,A,end
// start,b,A,c,A,b,end
// start,b,A,c,A,c,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,d,b,A,c,A,end
// start,b,d,b,A,end
// start,b,d,b,end
// start,b,end
//
// The slightly larger example above now has 103 paths through it, and the even
// larger example now has 3509 paths through it.
//
// Given these new rules, how many paths through this cave system are there?

pub fn solve(input: super::ocean::Ocean) -> usize {
    input.paths(true).count()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example_a => 36,
        example_b => 103,
        example_c => 3509,
        live => 128506,
    });
}
