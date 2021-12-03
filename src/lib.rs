use std::ffi::OsString;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<P, C>
where
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    #[error("an error occured while reading from source: {0}")]
    IO(#[from] std::io::Error),

    #[error("an error occured while parsing the input line '{0}': {1}")]
    Parse(String, #[source] P::Err),

    #[error("an error occured while generating the answer: {0}")]
    Answer(#[source] C::Err),
}

pub trait TryIntoAnswer {
    type Err;

    fn try_into_answer(self) -> Result<isize, Self::Err>;
}

pub trait IntoAnswer {
    fn into_answer(self) -> isize;
}

impl<T: IntoAnswer> TryIntoAnswer for T {
    type Err = &'static dyn std::error::Error;

    fn try_into_answer(self) -> Result<isize, Self::Err> {
        Ok(self.into_answer())
    }
}

impl<P, C> TryIntoAnswer for Result<C, Error<P, C>>
where
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    type Err = Error<P, C>;

    fn try_into_answer(self) -> Result<isize, Error<P, C>> {
        match self {
            Ok(v) => v.try_into_answer().map_err(Error::Answer),
            Err(e) => Err(e),
        }
    }
}

fn map_line<P, C>(line: std::io::Result<String>) -> Result<P, Error<P, C>>
where
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    match line {
        Ok(line) => match line.parse() {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Parse(line, e)),
        },
        Err(e) => Err(Error::IO(e)),
    }
}

pub fn answer_from_read<R, P, C>(read: R) -> Result<isize, Error<P, C>>
where
    R: Read,
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: FromIterator<P> + TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    BufReader::new(read)
        .lines()
        .map(map_line)
        .collect::<Result<C, Error<P, C>>>()
        .try_into_answer()
}

#[derive(StructOpt)]
struct Args {
    /// Input file to parse.
    path: OsString,
}

pub fn read<P, C>() -> Result<isize, Error<P, C>>
where
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: FromIterator<P> + TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    let args = Args::from_args();
    println!("opening {:?}", args.path);
    answer_from_read(File::open(args.path)?)
}

pub fn test<P, C>(input: &str) -> Result<isize, Error<P, C>>
where
    P: FromStr,
    <P as FromStr>::Err: std::error::Error + 'static,
    C: FromIterator<P> + TryIntoAnswer,
    <C as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    answer_from_read(input.as_bytes())
}
