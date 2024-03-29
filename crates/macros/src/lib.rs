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

pub(crate) mod common;
mod from_bytes;
mod from_iterator;
mod from_lines;
mod roundtrip;
mod try_from_str;
mod unwrap;

macro_rules! derive {
    ($mod:ident, $ty:ident $(, $attr:ident)? $(,)?) => {
        paste::paste! {
            #[proc_macro_derive($ty $(, attributes($attr))?)]
            #[proc_macro_error::proc_macro_error]
            pub fn [<derive_ $mod>](input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                self::$mod::expand(input.into()).into()
            }
        }
    }
}

derive!(from_bytes, FromBytes, from_bytes);
derive!(from_iterator, FromIterator, from_iterator);
derive!(from_lines, FromLines, from_lines);
derive!(try_from_str, TryFromStr);
derive!(unwrap, Unwrap);

#[proc_macro]
#[proc_macro_error::proc_macro_error]
pub fn test_roundtrip(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    self::roundtrip::expand(input.into()).into()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Seek;
    use std::path::PathBuf;

    use proc_macro2::TokenStream;
    use runtime_macros::emulate_attributelike_macro_expansion;
    use runtime_macros::emulate_derive_macro_expansion;
    use runtime_macros::emulate_functionlike_macro_expansion;
    use walkdir::DirEntry;

    type Expander = fn(TokenStream) -> TokenStream;
    type AttrExpander = fn(TokenStream, TokenStream) -> TokenStream;
    type Macro = (&'static str, Expander);
    type AttrMacro = (&'static str, AttrExpander);

    macro_rules! macro_ {
        ($name:ident, $mod:ident) => {
            [
                (
                    concat!("::macros::", stringify!($name)),
                    crate::$mod::expand,
                ),
                (concat!("macros::", stringify!($name)), crate::$mod::expand),
                (stringify!($name), crate::$mod::expand),
            ]
        };
    }

    const FUNCTION_LIKE: &[[Macro; 3]] = &[macro_!(test_roundtrip, roundtrip)];

    const DERIVE: &[[Macro; 3]] = &[
        macro_!(FromBytes, from_bytes),
        macro_!(FromIterator, from_iterator),
        macro_!(FromLines, from_lines),
        macro_!(TryFromStr, try_from_str),
        macro_!(Unwrap, unwrap),
    ];
    const ATTRIBUTE: &[[AttrMacro; 3]] = &[];

    const SRC_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../src");

    fn iter_src_files() -> impl Iterator<Item = PathBuf> {
        walkdir::WalkDir::new(SRC_ROOT)
            .into_iter()
            .map(Result::unwrap)
            .map(DirEntry::into_path)
            .filter(|p| p.extension().map_or(false, |p| p == "rs"))
    }

    fn flatten<const N: usize, T>(arr: &[[T; N]]) -> &[T] {
        unsafe { std::slice::from_raw_parts(arr.as_ptr() as *const T, arr.len() * N) }
    }

    #[test]
    fn code_coverage() {
        // This code doesn't check much. Instead, it does macro expansion at run
        // time to let tarpaulin measure code coverage for the macro.
        let mut once = false;

        for p in iter_src_files() {
            let mut fp = fs::File::open(p).unwrap();

            emulate_functionlike_macro_expansion(fp.try_clone().unwrap(), flatten(FUNCTION_LIKE))
                .unwrap();
            fp.rewind().unwrap();

            emulate_derive_macro_expansion(fp.try_clone().unwrap(), flatten(DERIVE)).unwrap();
            fp.rewind().unwrap();

            emulate_attributelike_macro_expansion(fp.try_clone().unwrap(), flatten(ATTRIBUTE))
                .unwrap();
            fp.rewind().unwrap();

            once = true;
        }

        assert!(once);
    }
}
