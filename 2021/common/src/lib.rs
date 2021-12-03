use std::ffi::OsString;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use structopt::StructOpt;
use thiserror::Error;

pub trait IntoAnswer {
    fn into_answer(self) -> isize;
}

#[derive(StructOpt)]
struct Args {
    /// Input file to parse.
    path: OsString,
}

#[derive(Debug, Error)]
pub enum ParseError<T: FromStr>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    IOError(#[from] ::std::io::Error),

    ParseError(<T as FromStr>::Err),
}

pub fn map_line<T>(line: std::io::Result<String>) -> Result<T, ParseError<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line?.parse().map_err(ParseError::ParseError)
}

pub fn read<T, V>() -> Result<isize, ParseError<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    V: FromIterator<T> + IntoAnswer,
{
    let args = Args::from_args();

    println!("opening {:?}", args.path);

    let file = File::open(args.path)?;

    Ok(BufReader::new(file)
        .lines()
        .map(map_line)
        .collect::<Result<V, _>>()?
        .into_answer())
}

pub fn test<T, V>(input: &str) -> Result<isize, <T as FromStr>::Err>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    V: FromIterator<T> + IntoAnswer,
{
    Ok(input
        .lines()
        .map(|l| l.parse::<T>())
        .collect::<Result<V, <T as FromStr>::Err>>()?
        .into_answer())
}
