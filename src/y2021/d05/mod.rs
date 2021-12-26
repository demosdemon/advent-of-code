pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod board;
pub(crate) mod builder;
pub(crate) mod line;
pub(crate) mod parser;

use euclid::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct D05;

type Coordinate = Point2D<usize, D05>;
