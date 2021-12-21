mod part1;
mod part2;

mod image;
mod parser;

use image::{Algorithm, Image, Pixel};

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

struct Input {
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

crate::derive_FromStr_for_nom!(Input, parser::input);
