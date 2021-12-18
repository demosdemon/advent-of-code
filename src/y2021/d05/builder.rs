use super::board::Board;
use super::line::Line;

#[derive(Debug, derive_more::IntoIterator)]
#[into_iterator(owned, ref)]
pub(super) struct SolutionBuilder(Vec<Line>);

crate::derive_FromIterator!(SolutionBuilder, Line);
crate::derive_FromStr_for_FromIterator!(SolutionBuilder, Line);

impl SolutionBuilder {
    pub fn board<F>(&self, predicate: F) -> Board
    where
        F: FnMut(&&Line) -> bool,
    {
        self.into_iter().filter(predicate).cloned().collect()
    }
}
