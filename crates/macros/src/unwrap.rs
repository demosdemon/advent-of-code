use convert_case::Case::Snake;
use convert_case::Casing;
use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::Parser;
use syn::Data;
use syn::Error;
use syn::Fields;
use syn::Generics;
use syn::Type;
use syn::Variant;

pub fn expand(tokens: TokenStream) -> TokenStream {
    Unwrap::parse.parse2(tokens).map_or_else(
        |err| err.to_compile_error(),
        |unwrap| unwrap.into_token_stream(),
    )
}

fn snake_ident(ident: &Ident) -> Ident {
    let s = ident.to_string().to_case(Snake);
    Ident::new(&s, ident.span())
}

struct Unwrap {
    ident: Ident,
    generics: Generics,
    variants: Vec<Variant>,
}

impl Parse for Unwrap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = syn::DeriveInput::parse(input)?;

        let ident = derive_input.ident;
        let generics = derive_input.generics;
        let variants = match derive_input.data {
            Data::Enum(data) => Ok(data.variants.into_iter().collect()),
            _ => Err(Error::new(Span::call_site(), "expected enum")),
        }?;

        Ok(Self {
            ident,
            generics,
            variants,
        })
    }
}

impl Unwrap {
    fn into_token_stream(self) -> TokenStream {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let methods = self.variants.iter().map(|v| self.impl_variant(v));
        quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                #(#methods)*
            }
        }
    }

    fn impl_variant(&self, variant: &Variant) -> TokenStream {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => self.impl_methods(
                ident,
                Delimiter::Brace,
                fields
                    .named
                    .iter()
                    .map(|f| (f.ident.as_ref().cloned().unwrap(), &f.ty))
                    .collect(),
            ),
            Fields::Unnamed(fields) => self.impl_methods(
                ident,
                Delimiter::Parenthesis,
                fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(idx, f)| (format_ident!("__field{idx}"), &f.ty))
                    .collect(),
            ),
            Fields::Unit => self.impl_methods(ident, Delimiter::None, vec![]),
        }
    }

    fn impl_methods(
        &self,
        ident: &Ident,
        delimiter: Delimiter,
        fields: Vec<(Ident, &Type)>,
    ) -> TokenStream {
        let num_fields = fields.len();
        macro_rules! maybe_paren {
            ($($tt:tt)*) => {
                if num_fields == 1 {
                    quote!($($tt)*)
                } else {
                    quote!(($($tt)*))
                }
            };
        }

        let snake_ident = snake_ident(ident);

        let recv_ref = quote!(&self);

        let is_enum = UnwrapMethod {
            method_name: format_ident!("is_{}", snake_ident),
            receiver: &recv_ref,
            return_type: &quote!(bool),
            pattern: &match delimiter {
                Delimiter::Parenthesis => quote!(Self::#ident ( .. )),
                Delimiter::Brace => quote!(Self::#ident {  ..  }),
                _ => quote!(Self::#ident),
            },
            expr: &quote!(true),
            default: &quote!(false),
        };

        if fields.is_empty() {
            return is_enum.into_token_stream();
        }

        let receiver = quote!(self);
        let recv_mut = quote!(&mut self);

        let (idents, types): (Vec<_>, Vec<_>) = fields.into_iter().unzip();

        let rt_owned = maybe_paren!(#(#types),*);
        let rt_ref = maybe_paren!(#(&#types),*);
        let rt_mut = maybe_paren!(#(&mut #types),*);

        let pattern = &{
            let idents = quote!(#(#idents),*);
            let group = Group::new(delimiter, idents);
            quote!(Self::#ident #group)
        };

        let expr = &maybe_paren!(#(#idents),*);
        let some_expr = &quote!(Some(#expr));

        let panic_default = quote!(panic!("unexpected variant"));
        let none_default = quote!(None);

        [
            is_enum,
            UnwrapMethod {
                method_name: format_ident!("unwrap_{snake_ident}"),
                receiver: &receiver,
                return_type: &rt_owned,
                pattern,
                expr,
                default: &panic_default,
            },
            UnwrapMethod {
                method_name: format_ident!("unwrap_{snake_ident}_ref"),
                receiver: &recv_ref,
                return_type: &rt_ref,
                pattern,
                expr,
                default: &panic_default,
            },
            UnwrapMethod {
                method_name: format_ident!("unwrap_{snake_ident}_mut"),
                receiver: &recv_mut,
                return_type: &rt_mut,
                pattern,
                expr,
                default: &panic_default,
            },
            UnwrapMethod {
                method_name: format_ident!("as_{snake_ident}"),
                receiver: &receiver,
                return_type: &quote!(Option<#rt_owned>),
                pattern,
                expr: some_expr,
                default: &none_default,
            },
            UnwrapMethod {
                method_name: format_ident!("as_{snake_ident}_ref"),
                receiver: &recv_ref,
                return_type: &quote!(Option<#rt_ref>),
                pattern,
                expr: some_expr,
                default: &none_default,
            },
            UnwrapMethod {
                method_name: format_ident!("as_{snake_ident}_mut"),
                receiver: &recv_mut,
                return_type: &quote!(Option<#rt_mut>),
                pattern,
                expr: some_expr,
                default: &none_default,
            },
        ]
        .into_iter()
        .map(ToTokens::into_token_stream)
        .collect()
    }
}

struct UnwrapMethod<'a, 'b, 'c, 'd, 'e> {
    method_name: Ident,
    receiver: &'a TokenStream,
    return_type: &'b TokenStream,
    pattern: &'c TokenStream,
    expr: &'d TokenStream,
    default: &'e TokenStream,
}

impl<'a, 'b, 'c, 'd, 'e> ToTokens for UnwrapMethod<'a, 'b, 'c, 'd, 'e> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let method_name = &self.method_name;
        let receiver = self.receiver;
        let return_type = self.return_type;
        let pattern = self.pattern;
        let expr = self.expr;
        let default = self.default;
        tokens.extend(quote! {
            pub fn #method_name(#receiver) -> #return_type {
                #[allow(clippy::match_like_matches_macro)]
                match self {
                    #pattern => #expr,
                    _ => #default,
                }
            }
        })
    }
}
