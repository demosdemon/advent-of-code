use derive_builder::Builder;
use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site, OptionExt};
use quote::quote;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::{DataStruct, DeriveInput, Lit, LitStr, Meta, MetaList, NestedMeta};

#[derive(Builder)]
#[builder(pattern = "owned")]
struct AnswerAttribute {
    #[builder(setter(into, strip_option), default)]
    example_input: Option<Lit>,

    example_output: Lit,

    #[builder(setter(into, strip_option), default)]
    live_input: Option<Lit>,

    #[builder(setter(into, strip_option), default)]
    live_output: Option<Lit>,
}

impl AnswerAttribute {
    fn from_meta_list(ml: MetaList) -> Self {
        let mut builder = AnswerAttributeBuilder::default();
        for nested_meta in ml.nested {
            match nested_meta {
                NestedMeta::Meta(m) => match m {
                    Meta::NameValue(nv) => match nv.path.get_ident() {
                        Some(ident) => match ident.to_string().as_ref() {
                            "example" => builder = builder.example_output(nv.lit),
                            "example_input" => builder = builder.example_input(nv.lit),
                            "live" => builder = builder.live_output(nv.lit),
                            "live_input" => builder = builder.live_input(nv.lit),
                            _ => abort!(
                                ident.span(),
                                "expected on of example, example_input, live, live_input"
                            ),
                        },
                        None => abort!(nv.span(), "expected an ident"),
                    },
                    _ => abort!(m.span(), "only a name-value pair expected here"),
                },
                NestedMeta::Lit(l) => abort!(l.span(), "literal was not expected here"),
            }
        }
        match builder.build() {
            Ok(v) => v,
            Err(e) => abort_call_site!(e),
        }
    }
}

#[derive(derive_more::Constructor)]
pub struct Answer<'derive> {
    input: &'derive DeriveInput,

    #[allow(unused)]
    sdata: &'derive DataStruct,
}

impl<'derive> Answer<'derive> {
    fn test_module_ident(&self) -> Ident {
        let name = self.input.ident.unraw().to_string().to_snake_case();
        let name = format!("test_{}", name);
        Ident::new(&name, self.input.ident.span())
    }

    fn get_attribute(&self) -> Option<AnswerAttribute> {
        let meta = self
            .input
            .attrs
            .iter()
            .filter(|m| m.path.is_ident("answer"))
            .map(|m| m.parse_meta().unwrap_or_else(|err| abort!(m.span(), err)))
            .next()?;

        let span = meta.span();

        let list = match meta {
            Meta::Path(_) => abort!(span, "empty attribute is not allowed."),
            Meta::List(list) => list,
            Meta::NameValue(_) => {
                abort!(span, "attribute does not support name-value format here.")
            }
        };

        Some(AnswerAttribute::from_meta_list(list))
    }

    pub fn build(self) -> TokenStream {
        let struct_name = &self.input.ident;
        let module = self.test_module_ident();
        let attr = self
            .get_attribute()
            .expect_or_abort("need an #[answer(...)] attribute");

        let example_input = attr
            .example_input
            .unwrap_or_else(|| Lit::Str(LitStr::new("inputs/example", Span::call_site())));

        let example_output = attr.example_output;

        let live_input = attr
            .live_input
            .unwrap_or_else(|| Lit::Str(LitStr::new("inputs/live", Span::call_site())));

        let live_output = attr.live_output;

        let example_impl = quote! {
            #[test]
            fn test_example() {
                assert_eq!(
                    crate::solve::<super::#struct_name>(include_str!(#example_input)).unwrap(),
                    #example_output,
                );
            }
        };

        let live_impl = match live_output {
            Some(lit) => {
                quote! {
                    #[test]
                    fn test_live() {
                        assert_eq!(
                            crate::solve::<super::#struct_name>(include_str!(#live_input)).unwrap(),
                            #lit,
                        );
                    }
                }
            }
            None => quote! {},
        };

        quote! {
            #[cfg(test)]
            mod #module {
                #example_impl
                #live_impl
            }
        }
    }
}
