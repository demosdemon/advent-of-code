use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Input file to parse.
    path: OsString,
}

fn map_line<E>(line: std::result::Result<String, E>) -> Result<isize>
where
    E: std::error::Error + Sync + Send + 'static,
{
    line.context("reading line")?
        .parse()
        .context("parsing line into isize")
}

pub fn read() -> Result<Vec<isize>> {
    let args = Args::from_args();

    println!("opening {:?}", args.path);

    let file = File::open(args.path).context("opening path for reading")?;

    BufReader::new(file).lines().map(map_line).collect()
}
