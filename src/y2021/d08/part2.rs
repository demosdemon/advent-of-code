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
// Through a little deduction, you should now be able to determine the remaining
// digits. Consider again the first example above:
//
// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb
// cdfeb cdbaf
//
// After some careful analysis, the mapping between signal wires and segments
// only make sense in the following configuration:
//
// dddd
// e    a
// e    a
// ffff
// g    b
// g    b
// cccc
//
// So, the unique signal patterns would correspond to the following digits:
//
// acedgfb: 8
// cdfbe: 5
// gcdfa: 2
// fbcad: 3
// dab: 7
// cefabd: 9
// cdfgeb: 6
// eafb: 4
// cagedb: 0
// ab: 1
//
// Then, the four digits of the output value can be decoded:
//
// cdfeb: 5
// fcadb: 3
// cdfeb: 5
// cdbaf: 3
//
// Therefore, the output value for this entry is 5353.
//
// Following this same process for each entry in the second, larger example
// above, the output value of each entry can be determined:
//
// fdgacbe cefdb cefbgd gcbe: 8394
// fcgedb cgb dgebacf gc: 9781
// cg cg fdcagb cbg: 1197
// efabcd cedba gadfec cb: 9361
// gecf egdcabf bgf bfgea: 4873
// gebdcfa ecba ca fadegcb: 8418
// cefg dcbef fcge gbcadfe: 4548
// ed bcgafe cdgba cbgef: 1625
// gbdfcae bgc cg cgb: 8717
// fgae cfgab fg bagce: 4315
//
// Adding all of the output values in this larger example produces 61229.
//
// For each entry, determine all of the wire/segment connections and decode the
// four-digit output values. What do you get if you add up all of the output
// values?

pub fn solve(input: super::Lines) -> usize {
    input.into_iter().map(usize::from).sum()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 61229,
        live => 986163,
    });
}
