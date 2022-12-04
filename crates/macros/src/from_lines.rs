use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

use crate::common::Common;

pub struct FromLines(Common);

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
        quote! {
            #from_iter

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
