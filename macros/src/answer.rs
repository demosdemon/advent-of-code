use std::collections::HashMap;

use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort, OptionExt};
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::{DataStruct, DeriveInput, Lit, LitStr, Meta, MetaList, NestedMeta};

struct AnswerAttribute {
    cases: HashMap<Ident, Lit>,
}

impl AnswerAttribute {
    fn from_meta_list(ml: MetaList) -> Self {
        let mut cases = HashMap::new();
        for nested_meta in ml.nested {
            match nested_meta {
                NestedMeta::Meta(m) => match m {
                    Meta::NameValue(nv) => match nv.path.get_ident() {
                        Some(ident) => {
                            cases.insert(ident.to_owned(), nv.lit);
                        }
                        None => abort!(nv.span(), "expected an ident"),
                    },
                    _ => abort!(m.span(), "only a name-value pair expected here"),
                },
                NestedMeta::Lit(l) => abort!(l.span(), "literal was not expected here"),
            }
        }
        Self { cases }
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

        let cases = attr.cases.into_iter().map(|(name, value)| {
            let test_fn_ident = format_ident!("test_{}", name);
            let filename = Lit::Str(LitStr::new(
                format!("inputs/{}", name).as_str(),
                name.span(),
            ));
            quote! {
                #[test]
                fn #test_fn_ident() {
                    assert_eq!(
                        crate::solve::<super::#struct_name>(include_str!(#filename)).unwrap(),
                        #value,
                    );
                }
            }
        });

        quote! {
            #[cfg(test)]
            mod #module {
                #( #cases )*
            }
        }
    }
}
