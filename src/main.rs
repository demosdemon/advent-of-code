#![allow(clippy::write_with_newline)]
#![cfg_attr(not(test), allow(unused))]

#[cfg(any(test, feature = "y2021"))]
mod y2021;

#[cfg(any(test, feature = "y2021"))]
mod y2022;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    subcmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Benchmark(InnerCommand),
    Run(InnerCommand),
}

#[derive(Debug, Args)]
struct InnerCommand {
    #[command(subcommand)]
    year: Option<Year>,
}

#[derive(Debug, Subcommand)]
enum Year {
    Y2021(InnerYear),
    Y2022(InnerYear),
}

#[derive(Debug, Args)]
struct InnerYear {
    #[command(subcommand)]
    day: Option<Day>,
}

#[derive(Debug, Subcommand)]
enum Day {
    D01(InnerDay),
    D02(InnerDay),
    D03(InnerDay),
    D04(InnerDay),
    D05(InnerDay),
    D06(InnerDay),
    D07(InnerDay),
    D08(InnerDay),
    D09(InnerDay),
    D10(InnerDay),
    D11(InnerDay),
    D12(InnerDay),
    D13(InnerDay),
    D14(InnerDay),
    D15(InnerDay),
    D16(InnerDay),
    D17(InnerDay),
    D18(InnerDay),
    D19(InnerDay),
    D20(InnerDay),
    D21(InnerDay),
    D22(InnerDay),
    D23(InnerDay),
    D24(InnerDay),
    D25(InnerDay),
}

#[derive(Debug, Args)]
struct InnerDay {
    #[command(subcommand)]
    part: Option<Part>,
}

#[derive(Debug, Subcommand)]
enum Part {
    P1,
    P2,
}

fn main() {
    let cli = Cli::parse();
    eprintln!("{cli:#?}");
}
