pub mod nom;

mod y2021;

fn chardigit(c: char) -> u8 {
    const ZERO: u8 = b'0';
    assert!(c.is_ascii_digit());
    (c as u8) - ZERO
}

fn expect_empty_line<S: AsRef<str>>(s: S) -> anyhow::Result<()> {
    let s = s.as_ref();
    if s.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("expected an empty line; got {}", s))
    }
}

pub trait Problem {
    type Input: ::core::str::FromStr;

    type Output: ::core::cmp::PartialEq;

    fn solve(input: &<Self as Problem>::Input) -> <Self as Problem>::Output;

    fn parse_and_solve(
        input: &str,
    ) -> ::core::result::Result<
        <Self as Problem>::Output,
        <<Self as Problem>::Input as ::core::str::FromStr>::Err,
    > {
        let input = input.parse()?;
        Ok(Self::solve(&input))
    }
}

#[cfg(test)]
macro_rules! tests_for_problem {
    ($t:ty, {
        $(
            $test_case:ident => $expected:expr,
        )*
    }) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_ $test_case>]() {
                    let input = include_str!(concat!("inputs/", stringify!($test_case)));
                    let answer = <$t as $crate::Problem>::parse_and_solve(input).unwrap();
                    assert_eq!(answer, $expected);
                }
            )*
        }
    };
}

macro_rules! derive_FromStr_for_FromIterator {
    ($t:ty, $v:ty) => {
        impl ::core::str::FromStr for $t {
            type Err = <$v as ::core::str::FromStr>::Err;
            #[inline]
            fn from_str(
                s: &str,
            ) -> ::core::result::Result<Self, <Self as ::core::str::FromStr>::Err> {
                s.lines().map(str::parse::<$v>).collect()
            }
        }
    };
}

macro_rules! derive_FromIterator {
    ($t:ty, $v:ty) => {
        impl ::core::iter::FromIterator<$v> for $t {
            #[inline]
            fn from_iter<T: ::core::iter::IntoIterator<Item = $v>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
}

macro_rules! derive_FromStr_for_nom {
    ($t:ty, $f:path) => {
        impl ::core::str::FromStr for $t {
            type Err = ::nom::error::Error<String>;
            #[inline]
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match ::nom::Finish::finish(($f)(s)) {
                    Ok((_, v)) => Ok(v),
                    Err(::nom::error::Error { input, code }) => Err(::nom::error::Error {
                        input: input.to_owned(),
                        code,
                    }),
                }
            }
        }
    };
}

pub(crate) use derive_FromIterator;
pub(crate) use derive_FromStr_for_FromIterator;
pub(crate) use derive_FromStr_for_nom;
#[cfg(test)]
pub(crate) use tests_for_problem;
