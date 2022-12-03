use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Ident;
use syn::Token;
use syn::TypePath;

struct Roundtrip {
    ty: TypePath,

    #[allow(unused)]
    comma: Token![,],

    lits: Punctuated<Expr, Token![,]>,
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

#[proc_macro]
#[proc_macro_error]
pub fn test_roundtrip(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Roundtrip);

    let ty = &input.ty;
    let ty_ident = ty
        .path
        .segments
        .last()
        .unwrap()
        .ident
        .unraw()
        .to_string()
        .to_snake_case();
    let num_tests = input.lits.len();
    let zpad = (num_tests as f32).log10().ceil() as usize;

    let tests = input
        .lits
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
        .collect::<Vec<_>>();

    let rv = quote!(#( #tests )*);
    rv.into()
}
