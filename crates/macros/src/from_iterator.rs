use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::parse::Parser;

use crate::common::Common;

pub fn expand(tokens: TokenStream) -> TokenStream {
    FromIterator::parse
        .parse2(tokens)
        .map(FromIterator::into_token_stream)
        .unwrap_or_else(syn::Error::into_compile_error)
}

struct FromIterator(Common);

impl FromIterator {
    pub fn into_token_stream(self) -> TokenStream {
        self.0.impl_from_iterator()
    }
}

impl Parse for FromIterator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Common::parse(input, "FromIterator", "from_iterator").map(Self)
    }
}
