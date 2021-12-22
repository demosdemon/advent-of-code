use structopt::StructOpt;

pub mod y2021;

#[derive(StructOpt)]
struct App {
    #[structopt(short, long)]
    benchmark: bool,
    // #[structopt(subcommand)]
    // command: Command,
}

fn main() {
    println!("Hello, world");
}
