/*
    --- Part Two ---
    Considering every single measurement isn't as useful as you expected: there's just
    too much noise in the data.

    Instead, consider sums of a three-measurement sliding window. Again considering the
    above example:

        199  A
        200  A B
        208  A B C
        210    B C D
        200  E   C D
        207  E F   D
        240  E F G
        269    F G H
        260      G H
        263        H

    Start by comparing the first and second three-measurement windows. The measurements
    in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607.
    The second window is marked B (200, 208, 210); its sum is 618. The sum of
    measurements in the second window is larger than the sum of the first, so this first
    comparison increased.

    Your goal now is to count the number of times the sum of measurements in this
    sliding window increases from the previous sum. So, compare A with B, then compare B
    with C, then C with D, and so on. Stop when there aren't enough measurements left to
    create a new three-measurement sum.

    In the above example, the sum of each three-measurement window is as follows:

        A: 607 (N/A - no previous sum)
        B: 618 (increased)
        C: 618 (no change)
        D: 617 (decreased)
        E: 647 (increased)
        F: 716 (increased)
        G: 769 (increased)
        H: 792 (increased)

    In this example, there are 5 sums that are larger than the previous sum.

    Consider sums of a three-measurement sliding window. How many sums are larger than
    the previous sum?
*/

use std::convert::Infallible;
use std::io::BufRead;

use crate::{Error, Problem, Solution};

#[derive(macros::Answer)]
#[answer(example = 5, live = 1748)]
pub struct Answer(isize);

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .parse_lines(str::parse::<isize>)
                .collect::<Result<Vec<_>, _>>()?
                // a co-worker of mine pointed out that (A + B + C) < (B + C + D) is equivalent
                // to A < D, and thus, we just need a sliding window of 4 elements.
                .windows(4)
                .filter(|s| s[0] < s[3])
                .count() as isize,
        ))
    }
}

impl Solution for Answer {
    type Err = Infallible;

    fn try_into_answer(self) -> Result<isize, Self::Err> {
        Ok(self.0)
    }
}
