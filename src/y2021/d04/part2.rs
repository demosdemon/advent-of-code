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

use super::matrix::Board;

pub fn solve(input: &super::Game) -> usize {
    let input = input.to_owned();
    let mut bingo = Bingo(input.boards);
    input
        .pulls
        .into_iter()
        .find_map(|pull| bingo.mark(pull))
        .unwrap()
}

#[derive(Debug, Default)]
struct Bingo(Vec<Board>);

impl Bingo {
    pub fn mark(&mut self, pull: u8) -> Option<usize> {
        self.0
            .iter_mut()
            .enumerate()
            .filter_map(|(idx, board)| {
                board.mark(pull).and_then(|pos| {
                    (board.bingo_row(pos.x) || board.bingo_column(pos.y)).then(|| idx)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .find_map(|won| {
                let b = self.0.remove(won);
                self.0.is_empty().then(|| b.sum() * pull as usize)
            })
    }
}

#[cfg(test)]
mod tests {
    ::aoc::tests_for_problem!(super::solve, {
        example => 1924,
        live => 6594,
    });
}
