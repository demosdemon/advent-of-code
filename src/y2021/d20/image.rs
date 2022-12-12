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

use std::ops::AddAssign;
use std::ops::Index;
use std::ops::Not;

use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, derive_more::Display, macros::Unwrap)]
pub(super) enum Pixel {
    #[display(fmt = ".")]
    Dim,

    #[display(fmt = "#")]
    Lit,
}

impl Not for Pixel {
    type Output = Pixel;

    fn not(self) -> Self::Output {
        match self {
            Pixel::Dim => Pixel::Lit,
            Pixel::Lit => Pixel::Dim,
        }
    }
}

impl<'a> AddAssign<&'a Pixel> for usize {
    fn add_assign(&mut self, rhs: &'a Pixel) {
        *self <<= 1;
        if rhs.is_lit() {
            *self += 1;
        }
    }
}

pub(super) struct AlgorithmIndex<'image>([&'image Pixel; super::ALGORITHM_BITS]);

impl<'index, 'image> From<&'index AlgorithmIndex<'image>> for usize {
    fn from(idx: &'index AlgorithmIndex<'image>) -> usize {
        idx.0.iter().fold(0, |mut lhs, rhs| {
            lhs += *rhs;
            lhs
        })
    }
}

#[derive(derive_more::Constructor)]
pub(super) struct Algorithm([Pixel; super::ALGORITHM_PIXELS]);

impl<'index, 'image> Index<&'index AlgorithmIndex<'image>> for Algorithm {
    type Output = Pixel;

    fn index(&self, index: &'index AlgorithmIndex<'image>) -> &Self::Output {
        let idx: usize = index.into();
        // safety: the compiler constants maintain safety
        debug_assert!(idx < super::ALGORITHM_PIXELS);
        unsafe { self.0.get_unchecked(idx) }
    }
}

pub(super) struct Image {
    width: usize,
    pixels: Vec<Pixel>,
    outer: Pixel,
}

impl Image {
    pub fn new(v: Vec<Vec<Pixel>>) -> Self {
        Image {
            width: v[0].len(),
            pixels: v.into_iter().flatten().collect(),
            outer: Pixel::Dim,
        }
    }

    pub fn count(&self) -> usize {
        self.pixels.iter().filter(|p| p.is_lit()).count()
    }

    pub fn transform(&self, algo: &Algorithm) -> Image {
        let pixels = self
            .pixels
            .iter()
            .enumerate()
            .map(|(idx, _)| self.index(idx))
            .map(|idx| &algo[&idx])
            .cloned()
            .collect::<Vec<_>>();
        let outer = pixels[0];
        Image {
            width: self.width,
            pixels,
            outer,
        }
    }

    pub fn pad(&self, units: usize) -> Image {
        let dos_units = 2 * units;
        let new_width = self.width + dos_units;
        let new_depth = self.depth() + dos_units;
        let mut new_pixels = Vec::with_capacity(new_width * new_depth);
        new_pixels.extend(vec![self.outer; new_width * units]);
        for row in 0..self.depth() {
            new_pixels.extend(vec![self.outer; units]);
            new_pixels.extend(self.iter_row(row).cloned());
            new_pixels.extend(vec![self.outer; units]);
        }
        new_pixels.extend(vec![self.outer; new_width * units]);
        Image {
            width: new_width,
            pixels: new_pixels,
            outer: self.outer,
        }
    }

    fn depth(&self) -> usize {
        self.pixels.len() / self.width
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, (x, y): (usize, usize)) -> usize {
        (y * self.width) + x
    }

    fn visible(&self, x: isize, y: isize) -> bool {
        (0..self.width as isize).contains(&x) && (0..self.depth() as isize).contains(&y)
    }

    fn index(&self, center: usize) -> AlgorithmIndex {
        let av = self
            .iter_surrounding(center)
            .collect::<ArrayVec<_, { super::ALGORITHM_BITS }>>();
        // safety: compiler constants ensure safety
        let pixels = unsafe { av.into_inner_unchecked() };
        AlgorithmIndex(pixels)
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &'_ Pixel> + ExactSizeIterator + '_ {
        self.pixels.iter().skip(self.width * row).take(self.width)
    }

    fn iter_surrounding(
        &self,
        idx: usize,
    ) -> impl Iterator<Item = &'_ Pixel> + ExactSizeIterator + '_ {
        let (x, y) = self.idx_to_pos(idx);
        super::ALGORITHM_DX_DY
            .iter()
            .map(move |&(dx, dy)| (x as isize + dx, y as isize + dy))
            .map(|(x, y)| {
                if self.visible(x, y) {
                    let idx = self.pos_to_idx((x as usize, y as usize));
                    &self.pixels[idx]
                } else {
                    &self.outer
                }
            })
    }
}
