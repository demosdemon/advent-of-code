pub mod errors;
pub mod problem;

mod y2021;

pub use errors::Error;
pub use problem::Problem;

pub trait ParseProblem {
    type Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait IntoAnswer {
    type Output: PartialEq;

    fn into_answer(self) -> Self::Output;
}

pub fn solve<S>(s: &str) -> Result<S::Output, S::Error>
where
    S: ParseProblem + IntoAnswer,
{
    let mut p = problem::Problem::new(s);
    let s = S::parse_problem(&mut p)?;
    Ok(s.into_answer())
}

fn chardigit(c: char) -> u8 {
    const ZERO: u8 = b'0';
    assert!(c.is_ascii_digit());
    (c as u8) - ZERO
}
