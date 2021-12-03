use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalit Bit: {0}")]
    InvalidBit(char),
}
