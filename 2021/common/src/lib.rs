use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Input file to parse.
    path: OsString,
}

fn map_line<T>(line: std::io::Result<String>) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    line.context("reading line")?
        .parse()
        .context("parsing line")
}

pub fn read<T, V>() -> Result<V>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
    V: FromIterator<T>,
{
    let args = Args::from_args();

    println!("opening {:?}", args.path);

    let file = File::open(args.path).context("opening path for reading")?;

    BufReader::new(file).lines().map(map_line).collect()
}
