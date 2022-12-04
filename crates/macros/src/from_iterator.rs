use proc_macro2::TokenStream;
use syn::parse::Parse;

use crate::common::Common;

pub struct FromIterator(Common);

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
