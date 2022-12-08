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

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::Parser;

use crate::common::Common;

pub fn expand(tokens: TokenStream) -> TokenStream {
    FromLines::parse
        .parse2(tokens)
        .map(FromLines::into_token_stream)
        .unwrap_or_else(syn::Error::into_compile_error)
}

struct FromLines(Common);

impl FromLines {
    pub fn into_token_stream(self) -> TokenStream {
        let Common {
            attr_type,
            struct_ident,
            struct_generics,
            ..
        } = &self.0;
        let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();
        let from_iter = self.0.impl_from_iterator();
        let try_from_str = self.0.impl_try_from_str();
        quote! {
            #from_iter
            #try_from_str
            impl #impl_generics ::core::str::FromStr for #struct_ident #ty_generics #where_clause {
                type Err = <#attr_type as ::core::str::FromStr>::Err;

                fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::core::str::FromStr>::Err> {
                    s.lines().map(str::parse::<#attr_type>).collect()
                }
            }
        }
    }
}

impl Parse for FromLines {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Common::parse(input, "FromLines", "from_lines").map(Self)
    }
}
