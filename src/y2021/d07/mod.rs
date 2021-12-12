pub mod part1;
pub mod part2;

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
struct Ocean(Vec<isize>);

impl Ocean {
    pub fn solve<F>(self, mut cost: F) -> isize
    where
        F: FnMut(isize, isize) -> isize,
    {
        self.range()
            .map(|a| (&self).into_iter().map(|&b| (cost)(a, b)).sum())
            .min()
            .unwrap_or_default()
    }

    pub fn range(&self) -> std::ops::RangeInclusive<isize> {
        self.min()..=self.max()
    }

    pub fn min(&self) -> isize {
        self.into_iter().min().unwrap_or(&0).to_owned()
    }

    pub fn max(&self) -> isize {
        self.into_iter().max().unwrap_or(&0).to_owned()
    }
}
