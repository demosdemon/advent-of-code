#![allow(clippy::write_with_newline)]
// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
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
