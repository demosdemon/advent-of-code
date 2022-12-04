mod parser;
pub(crate) mod part1;
pub(crate) mod part2;

use std::ops::RangeInclusive;

struct Assignment(RangeInclusive<u8>);

impl Assignment {
    fn contains(&self, rhs: &Self) -> bool {
        self.0.contains(rhs.0.start()) && self.0.contains(rhs.0.end())
    }

    fn overlaps(&self, rhs: &Self) -> bool {
        self.0.contains(rhs.0.start()) || self.0.contains(rhs.0.end())
    }
}

impl From<(u8, u8)> for Assignment {
    fn from((start, end): (u8, u8)) -> Self {
        Self(start..=end)
    }
}

struct AssignmentPair(Assignment, Assignment);

::aoc::derive_FromStr_for_nom!(AssignmentPair, parser::assignment_pair);

impl From<((u8, u8), (u8, u8))> for AssignmentPair {
    fn from(((start1, end1), (start2, end2)): ((u8, u8), (u8, u8))) -> Self {
        Self(
            Assignment::from((start1, end1)),
            Assignment::from((start2, end2)),
        )
    }
}

impl AssignmentPair {
    /// The pair is inclusive if either assignment range is wholly contained by
    /// the other.
    fn is_inclusive(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    /// The pair overlaps if any part of the range of one assignment is
    /// contained by the other.
    fn is_overlapping(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

struct Assignments(Vec<AssignmentPair>);

::aoc::derive_FromIterator!(Assignments, AssignmentPair);
::aoc::derive_FromStr_for_FromIterator!(Assignments, AssignmentPair);
