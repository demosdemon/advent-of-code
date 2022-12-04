mod from_iterator;
mod from_lines;
mod roundtrip;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

use self::from_iterator::FromIterator;
use self::from_lines::FromLines;
use self::roundtrip::Roundtrip;

#[proc_macro]
#[proc_macro_error]
pub fn test_roundtrip(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Roundtrip)
        .into_token_stream()
        .into()
}

#[proc_macro_derive(FromIterator, attributes(from_iterator))]
#[proc_macro_error]
pub fn derive_from_iterator(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as FromIterator)
        .into_token_stream()
        .into()
}

#[proc_macro_derive(FromLines, attributes(from_lines))]
#[proc_macro_error]
pub fn derive_from_lines(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as FromLines)
        .into_token_stream()
        .into()
}
