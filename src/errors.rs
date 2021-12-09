pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("an error occurred reading from input: {0}")]
    IO(#[from] std::io::Error),

    #[error("an error ocurred parsing the input: {0}")]
    Parse(#[source] Box<dyn std::error::Error>),

    #[error("the input ended unexpectedly")]
    UnexpectedEndOfInput,

    #[error("expected an empty line but found {0:?}")]
    ExpectedEmptyLine(String),

    #[error("an error occurred while generating the solution: {0}")]
    IntoSolution(#[source] Box<dyn std::error::Error>),

    #[error("an error occurred while generating the answer from the solution: {0}")]
    IntoAnswer(#[source] Box<dyn std::error::Error>),
}

impl Error {
    pub fn from_parse<E: std::error::Error + 'static>(e: E) -> Self {
        Self::Parse(Box::new(e))
    }

    pub fn from_solution<E: std::error::Error + 'static>(e: E) -> Self {
        Self::IntoSolution(Box::new(e))
    }

    pub fn from_answer<E: std::error::Error + 'static>(e: E) -> Self {
        Self::IntoAnswer(Box::new(e))
    }

    pub fn from_empty_line(s: String) -> Result<()> {
        if s.is_empty() {
            Ok(())
        } else {
            Err(Self::ExpectedEmptyLine(s))
        }
    }
}
