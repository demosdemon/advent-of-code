// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod matrix;
pub mod nom;

pub fn chardigit(c: u8) -> u8 {
    const ZERO: u8 = b'0';
    assert!(c.is_ascii_digit());
    c - ZERO
}

pub fn expect_empty_line<S: AsRef<str>>(s: S) -> anyhow::Result<()> {
    let s = s.as_ref();
    if s.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("expected an empty line; got {}", s))
    }
}

pub fn parse_and_solve<'a, I, O>(s: &'a str, f: fn(I) -> O) -> Result<O, I::Error>
where
    I: TryFrom<&'a str>,
    O: PartialEq,
{
    s.try_into().map(f)
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
