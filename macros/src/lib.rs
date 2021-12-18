use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::ext::IdentExt;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, ReturnType, Type};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn problem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let item_attrs = &item.attrs;
    let item_vis = &item.vis;
    let signature = &item.sig;
    let struct_ident = {
        let name = signature.ident.unraw().to_string().to_upper_camel_case();
        Ident::new(&name, signature.ident.span())
    };
    let generics = &signature.generics;
    let where_clause = generics.where_clause.as_ref();
    let input = {
        let mut iter = signature.inputs.iter();
        let input = iter
            .next()
            .unwrap_or_else(|| abort!(signature, "expected one input paramter"));
        if let Some(next) = iter.next() {
            abort!(next, "expected only one input parameter");
        }
        match input {
            FnArg::Typed(arg) => arg,
            arg => abort!(arg, "expected a typed input argument"),
        }
    };
    let input_name = match &*input.pat {
        Pat::Ident(ident) => ident,
        pat => abort!(pat, "expected an ident"),
    };
    let input_type = match &*input.ty {
        Type::Reference(ty) => {
            if let Some(ref lt) = ty.lifetime {
                abort!(lt, "expected an anonymous lifetime");
            }
            if let Some(ref t) = ty.mutability {
                abort!(t, "input type is not mutable");
            }
            &*ty.elem
        }
        ty => abort!(ty, "expected a type reference"),
    };
    let output_type = match &signature.output {
        ReturnType::Type(_, t) => &**t,
        t => abort!(t, "must use an explicitly defined return type"),
    };
    let body = &*item.block;

    let rv = quote! {
        #(#item_attrs)*
        struct #struct_ident;

        #item_vis
        impl crate::Problem for #struct_ident {
            type Input = #input_type;

            type Output = #output_type;

            fn solve #generics (#input_name: &#input_type) -> #output_type #where_clause
            #body
        }
    };

    rv.into()
}
