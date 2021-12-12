pub mod errors;
pub mod problem;
mod y2021;

pub trait IntoAnswer {
    fn into_answer(self) -> isize;
}

pub trait TryIntoAnswer {
    type Err;

    fn try_into_answer(self) -> std::result::Result<isize, Self::Err>;
}

impl<T: IntoAnswer> TryIntoAnswer for T {
    type Err = std::convert::Infallible;

    fn try_into_answer(self) -> std::result::Result<isize, Self::Err> {
        Ok(self.into_answer())
    }
}

type StringProblem<'a> = problem::Problem<&'a [u8]>;

pub fn solve<'a, S>(s: &'a str) -> errors::Result<isize>
where
    S: TryIntoAnswer + TryFrom<StringProblem<'a>, Error = errors::Error>,
    <S as TryIntoAnswer>::Err: std::error::Error + 'static,
{
    let p = problem::Problem::new(s.as_bytes());
    let s: S = p.try_into()?;
    s.try_into_answer().map_err(errors::Error::from_answer)
}

fn chardigit(c: char) -> u8 {
    const ZERO: u8 = b'0';
    (c as u8) - ZERO
}
