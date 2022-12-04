use super::board::Board;
use super::line::Line;

#[derive(Debug, derive_more::IntoIterator, macros::FromIterator)]
#[into_iterator(owned, ref)]
#[from_iterator(Line)]
pub(super) struct SolutionBuilder(Vec<Line>);

::aoc::derive_FromStr_for_FromIterator!(SolutionBuilder, Line);

impl SolutionBuilder {
    pub fn board<F>(&self, predicate: F) -> Board
    where
        F: FnMut(&&Line) -> bool,
    {
        self.into_iter().filter(predicate).cloned().collect()
    }
}
