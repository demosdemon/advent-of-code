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
// The Elf finishes helping with the tent and sneaks back over to you. "Anyway,
// the second column says how the round needs to end: X means you need to lose,
// Y means you need to end the round in a draw, and Z means you need to win.
// Good luck!"
//
// The total score is still calculated in the same way, but now you need to
// figure out what shape to choose so the round ends as indicated. The example
// above now goes like this:
//
// In the first round, your opponent will choose Rock (A), and you need the
// round to end in a draw (Y), so you also choose Rock. This gives you a score
// of 1 + 3 = 4. In the second round, your opponent will choose Paper (B), and
// you choose Rock so you lose (X) with a score of 1 + 0 = 1. In the third
// round, you will defeat your opponent's Scissors with Rock for a score of 1 +
// 6 = 7. Now that you're correctly decrypting the ultra top secret strategy
// guide, you would get a total score of 12.
//
// Following the Elf's instructions for the second column, what would your total
// score be if everything goes exactly according to your strategy guide?

fn solve(s: super::Instructions) -> usize {
    s.0.iter().map(|i| i.p2().score()).sum()
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 12,
        live => 11618,
    });
}
