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

use convert_case::Case::Snake;
use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Ident;
use syn::Token;
use syn::TypePath;

pub fn expand(tokens: TokenStream) -> TokenStream {
    Roundtrip::parse
        .parse2(tokens)
        .map(Roundtrip::into_token_stream)
        .unwrap_or_else(syn::Error::into_compile_error)
}

struct Roundtrip {
    ty: TypePath,

    #[allow(unused)]
    comma: Token![,],

    lits: Punctuated<Expr, Token![,]>,
}

impl Roundtrip {
    pub fn into_token_stream(self) -> TokenStream {
        let ty = &self.ty;
        let ty_ident = ty
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .unraw()
            .to_string()
            .to_case(Snake);
        let num_tests = self.lits.len();
        let zpad = (num_tests as f32).log10().ceil() as usize;

        self.lits
            .iter()
            .enumerate()
            .map(|(idx, lit)| {
                let test_fn_ident = {
                    let s = format!("test_roundtrip_{ty_ident}_{idx:0zpad$}");
                    Ident::new(&s, lit.span())
                };
                quote! {
                    #[test]
                    fn #test_fn_ident() {
                        const INPUT: &str = #lit;
                        let parsed: #ty = INPUT.parse().unwrap();
                        let output = parsed.to_string();
                        assert_eq!(INPUT, output);
                    }
                }
            })
            .collect()
    }
}

impl Parse for Roundtrip {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            lits: input.parse_terminated(<Expr as Parse>::parse)?,
        })
    }
}
