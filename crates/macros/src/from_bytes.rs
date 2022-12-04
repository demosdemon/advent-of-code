use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

use crate::common::Common;

pub struct FromBytes(Common);

impl FromBytes {
    pub fn into_token_stream(self) -> TokenStream {
        let Common {
            attr_type,
            struct_ident,
            struct_generics,
            ..
        } = &self.0;
        let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();
        let from_iter = self.0.impl_from_iterator();
        quote! {
            #from_iter
            impl #impl_generics ::core::str::FromStr for #struct_ident #ty_generics #where_clause {
                type Err = <#attr_type as ::core::convert::TryFrom<u8>>::Error;

                fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::core::str::FromStr>::Err> {
                    s.bytes().map(::core::convert::TryInto::try_into).collect()
                }
            }
        }
    }
}

impl Parse for FromBytes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Common::parse(input, "FromBytes", "from_bytes").map(Self)
    }
}
