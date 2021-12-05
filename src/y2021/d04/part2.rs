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

use std::io::BufRead;

use crate::{Error as ProblemError, Problem, Solution};

use super::matrix::Board;
use super::Error;

#[derive(Debug)]
struct Answer(super::builder::SolutionBuilder);

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = ProblemError;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl Solution for Answer {
    type Err = Error;

    fn try_into_answer(self) -> Result<isize, Self::Err> {
        let pulls = self.0.pulls;
        let mut boards = Bingo(self.0.boards);
        for pull in pulls {
            if let Some(v) = boards.mark(&pull) {
                return Ok(v);
            }
        }
        Err(Error::InvalidSolution)
    }
}

#[derive(Debug, Default)]
struct Bingo(Vec<Board>);

impl Bingo {
    pub fn mark(&mut self, value: &u8) -> Option<isize> {
        let mut won = Vec::new();
        for (idx, board) in self.0.iter_mut().enumerate() {
            if let Some((row, col)) = board.mark(value) {
                if board.bingo_row(row) || board.bingo_column(col) {
                    won.push(idx);
                }
            }
        }
        for won in won.into_iter().rev() {
            let b = self.0.remove(won);
            if self.0.is_empty() {
                return Some(b.sum() * *value as isize);
            }
        }
        None
    }
}

mod test {
    #[test]
    fn test_example() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/example")).unwrap(),
            1924
        )
    }

    #[test]
    fn test_live() {
        assert_eq!(
            crate::solve::<super::Answer>(include_str!("inputs/live")).unwrap(),
            6594
        )
    }
}
