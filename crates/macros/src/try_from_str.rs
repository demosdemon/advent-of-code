use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::DeriveInput;
use syn::Generics;
use syn::Ident;

pub struct TryFromStr {
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
                type Error = <#struct_ident #ty_generics as ::core::str::FromStr>::Err;

                #[inline]
                fn try_from(s: &str) -> ::core::result::Result<Self, Self::Error> {
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
