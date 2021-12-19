use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{
    ext::IdentExt, parse::Parse, parse_macro_input, punctuated::Punctuated, FnArg, ItemFn, LitStr,
    Pat, ReturnType, Token, Type, TypePath,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn problem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);
    let item_attrs = &item.attrs;
    let item_vis = &item.vis;
    let signature = &item.sig;
    let struct_ident = {
        let i = &signature.ident;
        Ident::new(&i.unraw().to_string().to_upper_camel_case(), i.span())
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
        #item_vis
        struct #struct_ident;

        impl crate::Problem for #struct_ident {
            type Input = #input_type;

            type Output = #output_type;

            fn solve #generics (#input_name: &#input_type) -> #output_type #where_clause
            #body
        }
    };

    rv.into()
}

struct Roundtrip {
    ty: TypePath,
    #[allow(unused)]
    comma: Token![,],
    lits: Punctuated<LitStr, Token![,]>,
}

impl Parse for Roundtrip {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            lits: input.parse_terminated(<LitStr as Parse>::parse)?,
        })
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn test_roundtrip(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Roundtrip);

    let ty = &input.ty;
    let ty_ident = ty
        .path
        .segments
        .last()
        .unwrap()
        .ident
        .unraw()
        .to_string()
        .to_snake_case();
    let num_tests = input.lits.len();
    let zpad = (num_tests as f32).log10().ceil() as usize;

    let tests = input
        .lits
        .iter()
        .enumerate()
        .map(|(idx, lit)| {
            let test_fn_ident = {
                let s = format!("test_{}_{:0zpad$}", ty_ident, idx, zpad = zpad);
                Ident::new(&s, lit.span())
            };
            quote! {
                #[test]
                fn #test_fn_ident() {
                    const INPUT: &str = #lit;
                    let parsed: #ty = INPUT.parse().unwrap();
                    let output = parsed.to_string();
                    assert_eq!(INPUT, output);
                }
            }
        })
        .collect::<Vec<_>>();

    let rv = quote!(#( #tests )*);
    rv.into()
}
