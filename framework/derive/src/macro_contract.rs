use super::contract_impl::contract_implementation;
use crate::{
    parse::parse_contract_trait, preprocessing::trait_preprocessing, validate::validate_contract,
};

// #[derive(Debug)]
// pub struct X {
//     pub field: Vec<syn::MetaNameValue>,
// }

// impl syn::parse::Parse for X {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         Result::Ok(X { field: Vec::new() })
//     }
// }

pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let new_input = trait_preprocessing(input);

    // println!("process_contract args are: {:#?}", args);
    let args_input = parse_macro_input!(args as syn::MetaList);

    // println!("process_contract args_input are: {:#?}", args_input);
    let proc_input = &parse_macro_input!(new_input as syn::ItemTrait);
    let contract = parse_contract_trait(args_input, proc_input);
    validate_contract(&contract);

    let contract_impl = contract_implementation(&contract, true);

    proc_macro::TokenStream::from(contract_impl)
}
