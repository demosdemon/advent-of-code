use super::board::Board;
use super::line::Line;

#[derive(Debug, derive_more::IntoIterator, macros::FromLines)]
#[into_iterator(owned, ref)]
#[from_lines(Line)]
pub(super) struct SolutionBuilder(Vec<Line>);

impl SolutionBuilder {
    pub fn board<F>(&self, predicate: F) -> Board
    where
        F: FnMut(&&Line) -> bool,
    {
        self.into_iter().filter(predicate).cloned().collect()
    }
}
