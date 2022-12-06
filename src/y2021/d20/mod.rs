pub(crate) mod part1;
pub(crate) mod part2;

pub(crate) mod image;
pub(crate) mod parser;

use image::Algorithm;
use image::Image;
use image::Pixel;

const ALGORITHM_BITS: usize = 9;
const ALGORITHM_PIXELS: usize = 1 << ALGORITHM_BITS;
const ALGORITHM_DX_DY: [(isize, isize); ALGORITHM_BITS] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(macros::TryFromStr)]
pub struct Input {
    algo: Algorithm,
    image: Image,
}

impl Input {
    pub fn fold(&self, count: usize) -> usize {
        (0..count)
            .fold(self.image.pad(0), |img, _| img.pad(2).transform(&self.algo))
            .count()
    }
}

::aoc::derive_FromStr_for_nom!(Input, parser::input);
