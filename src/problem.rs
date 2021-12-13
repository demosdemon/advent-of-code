use iterator_ext::IteratorExt;

use crate::errors::{Error, Result};

pub struct Problem<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Problem<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn slice(&self) -> &'_ str {
        let pos = self.pos.clamp(0, self.input.len());
        &self.input[pos..]
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn expect_map_line<F, V, E>(&mut self, sep: &'a str, f: F) -> Result<Vec<V>>
    where
        F: FnMut(&str) -> std::result::Result<V, E> + 'a,
        E: std::error::Error + 'static,
    {
        self.map_line(sep, f).ok_or(Error::UnexpectedEndOfInput)?
    }

    pub fn expect_parse_line<F, V, E>(&mut self, f: F) -> Result<V>
    where
        F: FnOnce(&str) -> std::result::Result<V, E> + 'a,
        E: std::error::Error + 'static,
    {
        self.parse_line(f).ok_or(Error::UnexpectedEndOfInput)?
    }

    pub fn expect_take_line(&mut self) -> Result<&'_ str> {
        self.take_line().ok_or(Error::UnexpectedEndOfInput)
    }

    pub fn expect_empty_line(&mut self) -> Result<()> {
        match self.expect_take_line() {
            Ok(v) => Error::from_empty_line(v.to_owned()),
            Err(v) => Err(v),
        }
    }

    pub fn map_line<F, V, E>(&mut self, sep: &'a str, f: F) -> Option<Result<Vec<V>>>
    where
        F: FnMut(&str) -> std::result::Result<V, E> + 'a,
        E: std::error::Error + 'static,
    {
        self.parse_line(move |v| v.split(sep).map(f).collect())
    }

    pub fn parse_line<F, V, E>(&mut self, f: F) -> Option<Result<V>>
    where
        F: FnOnce(&str) -> std::result::Result<V, E> + 'a,
        E: std::error::Error + 'static,
    {
        self.take_line().map(|v| (f)(v).map_err(Error::from_parse))
    }

    pub fn parse_lines<F, V, E>(&'a self, f: F) -> impl Iterator<Item = Result<V>> + '_
    where
        F: FnMut(&'a str) -> std::result::Result<V, E> + 'a,
        E: std::error::Error + 'static,
    {
        self.slice()
            .lines()
            .map(f)
            .map_err(|e| Error::from_parse(e))
    }

    pub fn take_line(&mut self) -> Option<&'_ str> {
        self.next_line().map(|eol| {
            let mut s = &self.input[self.pos..self.pos + eol];
            self.pos += eol;
            if s.ends_with('\n') {
                s = &s[..s.len() - 1];
                if s.ends_with('\r') {
                    s = &s[..s.len() - 1];
                }
            }
            s
        })
    }

    fn next_line(&self) -> Option<usize> {
        let s = self.slice();
        (!s.is_empty())
            .then(|| {
                s.find('\n')
                    .map(|c| c + 1)
                    .or_else(|| Some(self.slice().len()))
            })
            .flatten()
    }
}
