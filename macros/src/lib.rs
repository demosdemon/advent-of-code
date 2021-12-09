mod answer;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Answer, attributes(answer))]
#[proc_macro_error]
pub fn derive_from_problem(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(ref data) => answer::AnswerBuilderBuilder::default()
            .input(&input)
            .sdata(data)
            .build()
            .unwrap()
            .build()
            .into(),
        _ => abort_call_site!("Deriving From<Problem<_>> is only supported on structs."),
    }
}
