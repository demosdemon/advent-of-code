use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Ident;
use syn::Token;
use syn::TypePath;

pub struct Roundtrip {
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
            .to_snake_case();
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
