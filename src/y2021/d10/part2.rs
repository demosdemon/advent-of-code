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
// Now, discard the corrupted lines. The remaining lines are incomplete.
//
// Incomplete lines don't have any incorrect characters - instead, they're
// missing some closing characters at the end of the line. To repair the
// navigation subsystem, you just need to figure out the sequence of closing
// characters that complete all open chunks in the line.
//
// You can only use closing characters (), ], }, or >), and you must add them in
// the correct order so that only legal pairs are formed and all chunks end up
// closed.
//
// In the example above, there are five incomplete lines:
//
// - [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
// - [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
// - (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
// - {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
// - <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
//
// Did you know that autocomplete tools also have contests? It's true! The score
// is determined by considering the completion string character-by-character.
// Start with a total score of 0. Then, for each character, multiply the total
// score by 5 and then increase the total score by the point value given for the
// character in the following table:
//
// - ): 1 point.
// - ]: 2 points.
// - }: 3 points.
// - >: 4 points.
//
// So, the last completion string above - ])}> - would be scored as follows:
//
// - Start with a total score of 0.
// - Multiply the total score by 5 to get 0, then add the value of ] (2) to get
//   a new
// total score of 2.
// - Multiply the total score by 5 to get 10, then add the value of ) (1) to get
//   a new
// total score of 11.
// - Multiply the total score by 5 to get 55, then add the value of } (3) to get
//   a new
// total score of 58.
// - Multiply the total score by 5 to get 290, then add the value of > (4) to
//   get a new
// total score of 294.
//
// The five lines' completion strings have total scores as follows:
//
// - }}]])})] - 288957 total points.
// - )}>]}) - 5566 total points.
// - }}>}>)))) - 1480781 total points.
// - ]]}}]}]}> - 995444 total points.
// - ])}> - 294 total points.
//
// Autocomplete tools are an odd bunch: the winner is found by sorting all of
// the scores and then taking the middle score. (There will always be an odd
// number of scores to consider.) In this example, the middle score is 288957
// because there are the same number of scores smaller and larger than it.
//
// Find the completion string for each incomplete line, score the completion
// strings, and sort the scores. What is the middle score?

pub fn solve(input: super::Lines) -> usize {
    let mut scores = input
        .into_iter()
        .filter_map(|l| l.score())
        .collect::<Vec<_>>();
    let mid = scores.len() / 2;
    let (_, v, _) = scores.select_nth_unstable(mid);
    *v
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 288957,
        live => 4001832844,
    });
}
