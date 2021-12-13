mod part1;
mod part2;

mod ocean;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("expected hyphen delimiter; got {0}")]
    Hyphen(String),

    #[error("missing start node")]
    Start,

    #[error("missing end node")]
    End,
}
