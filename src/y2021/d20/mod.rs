// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
