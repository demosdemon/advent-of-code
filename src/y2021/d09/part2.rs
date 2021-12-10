/*
    --- Part Two ---
    Next, you need to find the largest basins so you know what areas are most important
    to avoid.

    A basin is all locations that eventually flow downward to a single low point.
    Therefore, every low point has a basin, although some basins are very small.
    Locations of height 9 do not count as being in any basin, and all other locations
    will always be part of exactly one basin.

    The size of a basin is the number of locations within the basin, including the low
    point. The example above has four basins.

    The top-left basin, size 3:

    **21**99943210
    **3**987894921
    9856789892
    8767896789
    9899965678

    The top-right basin, size 9:

    21999**43210**
    398789**4**9**21**
    985678989**2**
    8767896789
    9899965678

    The middle basin, size 14:

    2199943210
    39**878**94921
    9**85678**9892
    **87678**96789
    9**8**99965678

    The bottom-right basin, size 9:

    2199943210
    3987894921
    9856789**8**92
    876789**678**9
    98999**65678**

    Find the three largest basins and multiply their sizes together. In the above
    example, this is 9 * 14 * 9 = 1134.

    What do you get if you multiply together the sizes of the three largest basins?
*/

use std::io::BufRead;

use crate::errors::Error;
use crate::problem::Problem;
use crate::IntoAnswer;

use super::Ocean;

#[derive(macros::Answer)]
#[answer(example = 1134, live = 920448)]
struct Answer(Ocean);

impl<R: BufRead> TryFrom<Problem<R>> for Answer {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        let mut ocean = Ocean::default();
        ocean.extend(value.collect::<Result<Vec<_>, _>>()?.into_iter());
        Ok(Self(ocean))
    }
}

impl IntoAnswer for Answer {
    fn into_answer(self) -> isize {
        let mut basins = self
            .0
            .basins()
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>();
        basins.sort_unstable();
        basins.reverse();
        basins.into_iter().take(3).fold(1, |a, b| a * b as isize)
    }
}
