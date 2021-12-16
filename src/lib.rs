mod y2021;

use std::str::FromStr;

pub trait IntoAnswer {
    type Output: PartialEq;

    fn into_answer(self) -> Self::Output;
}

pub fn solve<S>(s: &str) -> Result<S::Output, S::Err>
where
    S: FromStr + IntoAnswer,
{
    let s = s.parse::<S>()?;
    Ok(s.into_answer())
}

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

#[macro_export(crate)]
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
                    assert_eq!(crate::solve::<$t>(input).unwrap(), $expected);
                }
            )*
        }
    };
}

#[macro_export(crate)]
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

#[macro_export(crate)]
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

#[macro_export(crate)]
macro_rules! derive_Extend {
    ($t:ty, $v:ty) => {
        impl ::core::iter::Extend<$v> for $t {
            #[inline]
            fn extend<T: ::core::iter::IntoIterator<Item = $v>>(&mut self, iter: T) {
                self.0.extend(iter)
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_FromIterator_for_Extend {
    ($t:ty, $v:ty) => {
        impl ::core::iter::FromIterator<$v> for $t {
            fn from_iter<T: ::core::iter::IntoIterator<Item = $v>>(iter: T) -> Self {
                let mut v = Self::default();
                v.extend(iter);
                v
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_Sum_for_AddAssign {
    ($t:ty, $v:ty) => {
        impl ::core::iter::Sum<$v> for $t {
            #[inline]
            fn sum<I: ::core::iter::IntoIterator<Item = $v>>(iter: I) -> Self {
                let mut new = Self::default();
                for dir in iter {
                    new += dir
                }
                new
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_FromIterator_for_Sum {
    ($t:ty, $v:ty) => {
        impl ::core::iter::FromIterator<$v> for $t {
            #[inline]
            fn from_iter<T: ::core::iter::IntoIterator<Item = $v>>(iter: T) -> Self {
                iter.into_iter().sum()
            }
        }
    };
}

#[macro_export(crate)]
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
