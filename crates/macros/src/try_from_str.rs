use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::Parser;
use syn::DeriveInput;
use syn::Generics;
use syn::Ident;

pub fn expand(tokens: TokenStream) -> TokenStream {
    TryFromStr::parse
        .parse2(tokens)
        .map(TryFromStr::into_token_stream)
        .unwrap_or_else(syn::Error::into_compile_error)
}

struct TryFromStr {
    pub struct_ident: Ident,
    pub struct_generics: Generics,
}

impl TryFromStr {
    pub fn into_token_stream(self) -> TokenStream {
        let Self {
            struct_ident,
            struct_generics,
        } = self;
        let (_, ty_generics, where_clause) = struct_generics.split_for_impl();
        let mut trait_generics = struct_generics.clone();
        trait_generics
            .params
            .insert(0, syn::parse_quote!('__try_from));
        let (impl_generics, _, _) = trait_generics.split_for_impl();

        quote! {
            impl #impl_generics ::core::convert::TryFrom<&'__try_from str> for #struct_ident #ty_generics #where_clause {
                type Error = <Self as ::core::str::FromStr>::Err;

                fn try_from(s: &'__try_from str) -> ::core::result::Result<Self, <Self as ::core::convert::TryFrom<&'__try_from str>>::Error> {
                    s.parse()
                }
            }
        }
    }
}

impl Parse for TryFromStr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = DeriveInput::parse(input)?;
        Ok(Self {
            struct_ident: derive_input.ident,
            struct_generics: derive_input.generics,
        })
    }
}
