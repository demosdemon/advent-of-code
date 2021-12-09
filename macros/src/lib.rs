mod answer;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Answer, attributes(answer))]
#[proc_macro_error]
pub fn derive_from_problem(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(sdata) => answer::Answer::new(&input, sdata).build().into(),
        _ => abort_call_site!("Problem is only supported on structs."),
    }
}
