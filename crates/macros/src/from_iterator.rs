use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;

pub struct FromIterator {
    attr_type: syn::Type,
    struct_ident: syn::Ident,
    struct_generics: syn::Generics,
    field_ident: Option<syn::Ident>,
    field_type: syn::Type,
}

impl FromIterator {
    pub fn into_token_stream(self) -> TokenStream {
        let FromIterator {
            attr_type,
            struct_ident,
            struct_generics,
            field_ident,
            field_type,
        } = self;

        let (impl_generics, ty_generics, where_clause) = struct_generics.split_for_impl();

        let impl_ = quote! {
            <#field_type>::from_iter(iter)
        };

        let ctor = if let Some(ident) = field_ident {
            quote! {
                Self {
                    #ident: #impl_,
                }
            }
        } else {
            quote! {
                Self(#impl_)
            }
        };

        quote! {
            impl #impl_generics ::core::iter::FromIterator<#attr_type> for #struct_ident #ty_generics #where_clause {
                fn from_iter<__IntoIterator: ::core::iter::IntoIterator<Item = #attr_type>>(iter: __IntoIterator) -> Self {
                    #ctor
                }
            }
        }
    }
}

impl Parse for FromIterator {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = syn::DeriveInput::parse(input)?;
        let struct_ident = derive_input.ident;
        let struct_generics = derive_input.generics;

        let attr_type = derive_input
            .attrs
            .into_iter()
            .find_map(|attr| {
                if attr.path.is_ident("from_iterator") {
                    Some(attr.parse_args())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                syn::Error::new(struct_ident.span(), "missing #[from_iterator] attribute")
            })??;

        let field = match derive_input.data {
            syn::Data::Struct(data) => match data.fields {
                syn::Fields::Named(f) => exactly_one(f.named),
                syn::Fields::Unnamed(f) => exactly_one(f.unnamed),
                syn::Fields::Unit => {
                    return Err(syn::Error::new(
                        struct_ident.span(),
                        "Cannot derive FromIterator for unit struct",
                    ))
                }
            },
            syn::Data::Enum(data) => {
                return Err(syn::Error::new(
                    data.enum_token.span,
                    "FromIterator cannot be derived for enums",
                ))
            }
            syn::Data::Union(data) => {
                return Err(syn::Error::new(
                    data.union_token.span,
                    "FromIterator cannot be derived for unions",
                ))
            }
        }
        .ok_or_else(|| {
            syn::Error::new(
                struct_ident.span(),
                "Can only derive FromIterator for structs with a single field",
            )
        })?;

        let field_ident = field.ident;

        Ok(Self {
            attr_type,
            struct_ident,
            struct_generics,
            field_ident,
            field_type: field.ty,
        })
    }
}

fn exactly_one<T, I: IntoIterator<Item = T>>(iter: I) -> Option<T> {
    let mut iter = iter.into_iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        None
    } else {
        Some(first)
    }
}
