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

#[macro_export(crate)]
macro_rules! derive_FromStr_for_FromIterator {
    ($t:ty, $v:ty) => {
        impl ::std::str::FromStr for $t {
            type Err = <$v as ::std::str::FromStr>::Err;
            #[inline]
            fn from_str(
                s: &str,
            ) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
                s.lines().map(str::parse::<$v>).collect()
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_FromIterator {
    ($t:ty, $v:ty) => {
        impl FromIterator<$v> for $t {
            #[inline]
            fn from_iter<T: IntoIterator<Item = $v>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_Extend {
    ($t:ty, $v:ty) => {
        impl Extend<$v> for $t {
            #[inline]
            fn extend<T: IntoIterator<Item = $v>>(&mut self, iter: T) {
                self.0.extend(iter)
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_FromIterator_for_Extend {
    ($t:ty, $v:ty) => {
        impl FromIterator<$v> for $t {
            fn from_iter<T: IntoIterator<Item = $v>>(iter: T) -> Self {
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
        impl Sum<$v> for $t {
            #[inline]
            fn sum<I: Iterator<Item = $v>>(iter: I) -> Self {
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
        impl FromIterator<$v> for $t {
            #[inline]
            fn from_iter<T: IntoIterator<Item = $v>>(iter: T) -> Self {
                iter.into_iter().sum()
            }
        }
    };
}

#[macro_export(crate)]
macro_rules! derive_FromStr_for_nom {
    ($t:ty, $f:path) => {
        impl FromStr for $t {
            type Err = nom::error::Error<String>;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use nom::Finish;
                match ($f)(s).finish() {
                    Ok((_, v)) => Ok(v),
                    Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                        input: input.to_owned(),
                        code,
                    }),
                }
            }
        }
    };
}
