use std::str::FromStr;

pub mod nom;

pub fn chardigit(c: u8) -> u8 {
    const ZERO: u8 = b'0';
    assert!(c.is_ascii_digit());
    (c as u8) - ZERO
}

pub fn expect_empty_line<S: AsRef<str>>(s: S) -> anyhow::Result<()> {
    let s = s.as_ref();
    if s.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("expected an empty line; got {}", s))
    }
}

pub fn parse_and_solve<I, O, S>(s: &str, f: S) -> Result<O, I::Err>
where
    I: FromStr,
    O: PartialEq,
    S: FnOnce(&I) -> O,
{
    let input = s.parse()?;
    Ok((f)(&input))
}

#[macro_export]
macro_rules! tests_for_problem {
    ($solve:expr, {
        $(
            $test_case:ident => $expected:expr,
        )*
    }) => {
        paste::paste! {
            $(
                #[test]
                fn [<test_ $test_case>]() {
                    let input = include_str!(concat!("inputs/", stringify!($test_case)));
                    let answer = ::aoc::parse_and_solve(input, $solve).unwrap();
                    assert_eq!(answer, $expected);
                }
            )*
        }
    };
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! ordered {
    ($lhs:expr, $rhs:expr) => {{
        let __lhs = ($lhs);
        let __rhs = ($rhs);
        if __rhs < __lhs {
            (__rhs, __lhs)
        } else {
            (__lhs, __rhs)
        }
    }};
}
