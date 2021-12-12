use std::io::BufRead;

use crate::errors::{Error, Result};

#[derive(derive_more::Constructor)]
pub struct Problem<R: BufRead>(R);

impl<R: BufRead> Problem<R> {
    pub fn expect_map_line<F, V, E>(&mut self, sep: &str, f: F) -> Result<Vec<V>>
    where
        F: FnMut(&str) -> std::result::Result<V, E>,
        E: std::error::Error + 'static,
    {
        self.map_line(sep, f).ok_or(Error::UnexpectedEndOfInput)?
    }

    pub fn expect_parse_line<F, V, E>(&mut self, f: F) -> Result<V>
    where
        F: FnOnce(&str) -> std::result::Result<V, E>,
        E: std::error::Error + 'static,
    {
        self.parse_line(f).ok_or(Error::UnexpectedEndOfInput)?
    }

    pub fn expect_take_line(&mut self) -> Result<String> {
        self.take_line().ok_or(Error::UnexpectedEndOfInput)?
    }

    pub fn expect_empty_line(&mut self) -> Result<()> {
        match self.expect_take_line() {
            Ok(v) => Error::from_empty_line(v),
            Err(v) => Err(v),
        }
    }

    pub fn map_line<F, V, E>(&mut self, sep: &str, f: F) -> Option<Result<Vec<V>>>
    where
        F: FnMut(&str) -> std::result::Result<V, E>,
        E: std::error::Error + 'static,
    {
        self.parse_line(|v| v.split(sep).map(f).collect())
    }

    pub fn parse_line<F, V, E>(&mut self, f: F) -> Option<Result<V>>
    where
        F: FnOnce(&str) -> std::result::Result<V, E>,
        E: std::error::Error + 'static,
    {
        match self.take_line() {
            None => None,
            Some(Ok(v)) => Some((f)(&v).map_err(Error::from_parse)),
            Some(Err(v)) => Some(Err(v)),
        }
    }

    pub fn parse_lines<F, V, E>(self, f: F) -> impl Iterator<Item = Result<V>>
    where
        F: FnMut(&str) -> std::result::Result<V, E>,
        E: std::error::Error + 'static,
    {
        ParseLines {
            problem: self,
            parser: f,
        }
    }

    pub fn take_line(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(v) => Some(Err(v)),
        }
    }

    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        match self.0.read_line(buf).map_err(Error::IO) {
            Ok(0) => Ok(0),
            Ok(v) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
}

impl<R: BufRead> Iterator for Problem<R> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_line()
    }
}

struct ParseLines<R, F>
where
    R: BufRead,
{
    problem: Problem<R>,
    parser: F,
}

impl<R, F, V, E> Iterator for ParseLines<R, F>
where
    R: BufRead,
    F: FnMut(&str) -> std::result::Result<V, E>,
    E: std::error::Error + 'static,
{
    type Item = Result<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.problem.parse_line(&mut self.parser)
    }
}
