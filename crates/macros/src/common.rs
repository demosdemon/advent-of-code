use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Field;
use syn::Fields;
use syn::Generics;
use syn::Ident;
use syn::Result;
use syn::Type;

struct CommonAttr {
    ty: Type,
    skip_from_iter: bool,
}

impl CommonAttr {
    fn from_input(name: &str, input: &DeriveInput) -> Result<Self> {
        input
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident(name))
            .ok_or_else(|| {
                Error::new(
                    input.ident.span(),
                    format!("missing #[{name}(...)] attribute"),
                )
            })?
            .parse_args()
    }
}

impl Parse for CommonAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        let comma = input.parse::<Option<Comma>>()?;
        let skip_from_iter = match &comma {
            Some(_) => match input.parse::<Ident>() {
                Ok(ident) if ident == "skip_from_iter" => Ok(true),
                Ok(ident) => Err(Error::new(ident.span(), "expected `skip_from_iter`")),
                Err(err) => Err(err),
            },
            None => Ok(false),
        }?;

        Ok(Self { ty, skip_from_iter })
    }
}

pub struct Common {
    pub attr_type: Type,
    pub skip_from_iter: bool,
    pub struct_ident: Ident,
    pub struct_generics: Generics,
    pub field_ident: Option<Ident>,
    pub field_type: Type,
}

impl Common {
    pub fn parse(input: ParseStream, trait_name: &str, attr_name: &str) -> Result<Self> {
        let derive_input = DeriveInput::parse(input)?;
        let attr = CommonAttr::from_input(attr_name, &derive_input)?;
        let field = field(trait_name, &derive_input)?;
        Ok(Self {
            attr_type: attr.ty,
            skip_from_iter: attr.skip_from_iter,
            struct_ident: derive_input.ident,
            struct_generics: derive_input.generics,
            field_ident: field.ident,
            field_type: field.ty,
        })
    }

    pub fn impl_try_from_str(&self) -> TokenStream {
        let Self {
            struct_ident,
            struct_generics,
            ..
        } = &self;
        let (_, ty_generics, where_clause) = struct_generics.split_for_impl();
        let mut try_from_generics = struct_generics.clone();
        try_from_generics
            .params
            .insert(0, syn::parse_quote!('__try_from));
        let (try_from_impl_generics, _, _) = try_from_generics.split_for_impl();
        quote! {
            impl #try_from_impl_generics ::core::convert::TryFrom<&'__try_from str> for #struct_ident #ty_generics #where_clause {
                type Error = <#struct_ident as ::core::str::FromStr>::Err;

                fn try_from(s: &'__try_from str) -> ::core::result::Result<Self, <Self as ::core::convert::TryFrom<&'__try_from str>>::Error> {
                    s.parse()
                }
            }
        }
    }

    pub fn impl_from_iterator(&self) -> TokenStream {
        if self.skip_from_iter {
            return quote! {};
        }

        let Self {
            attr_type,
            skip_from_iter: _,
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

fn field(trait_name: &str, derive_input: &DeriveInput) -> Result<Field> {
    match &derive_input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => exactly_one_field(trait_name, &fields.named),
            Fields::Unnamed(fields) => exactly_one_field(trait_name, &fields.unnamed),
            Fields::Unit => Err(Error::new(
                derive_input.ident.span(),
                format!("cannot #[derive({trait_name})] for unit structs"),
            )),
        },
        Data::Enum(data) => Err(Error::new(
            data.enum_token.span,
            format!("cannot #[derive({trait_name})] for enums"),
        )),
        Data::Union(data) => Err(Error::new(
            data.union_token.span,
            format!("cannot #[derive({trait_name})] for unions"),
        )),
    }
}

fn exactly_one_field(trait_name: &str, fields: &Punctuated<Field, Comma>) -> Result<Field> {
    let mut iter = fields.into_iter();
    let f1 = iter.next();
    let f2 = iter.next();
    match (f1, f2) {
        (Some(field), None) => Ok(field.clone()),
        (_, Some(field)) => Err(Error::new_spanned(
            field,
            format!("can only #[derive({trait_name}] for structs with a single field"),
        )),
        (None, None) => Err(Error::new_spanned(
            fields,
            format!("can only #[derive({trait_name}] for structs with a single field"),
        )),
    }
}
