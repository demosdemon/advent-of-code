/*
    --- Part Two ---
    On the other hand, it might be wise to try a different strategy: let the giant squid
    win.

    You aren't sure how many bingo boards a giant squid could play at once, so rather
    than waste time counting its arms, the safe thing to do is to figure out which board
    will win last and choose that one. That way, no matter which boards it picks, it
    will win for sure.

    In the above example, the second board is the last to win, which happens after 13 is
    eventually called and its middle column is completely marked. If you were to keep
    playing until this point, the second board would have a sum of unmarked numbers
    equal to 148 for a final score of 148 * 13 = 1924.

    Figure out which board will win last. Once it wins, what would its final score be?
*/

use itertools::Itertools;

use crate::IntoAnswer;

use super::builder::SolutionBuilder;
use super::matrix::Board;

#[derive(Debug, derive_more::FromStr)]
struct Answer(SolutionBuilder);

impl IntoAnswer for Answer {
    type Output = isize;

    fn into_answer(self) -> isize {
        let mut bingo = Bingo(self.0.boards);
        self.0
            .pulls
            .into_iter()
            .find_map(|pull| bingo.mark(pull))
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct Bingo(Vec<Board>);

impl Bingo {
    pub fn mark(&mut self, pull: u8) -> Option<isize> {
        self.0
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, board)| {
                board
                    .mark(pull)
                    .map(|(row, col)| {
                        (board.bingo_row(row) || board.bingo_column(col)).then(|| idx)
                    })
                    .flatten()
            })
            .collect_vec()
            .into_iter()
            .rev()
            .find_map(|won| {
                let b = self.0.remove(won);
                self.0.is_empty().then(|| b.sum() * pull as isize)
            })
    }
}

#[cfg(test)]
mod tests {
    crate::tests_for_answer!(super::Answer, {
        example => 1924,
        live => 6594,
    });
}
