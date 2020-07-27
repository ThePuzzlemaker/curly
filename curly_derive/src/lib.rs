#![warn(clippy::pedantic)]
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Provider, attributes(curly_ignore, curly_rename))]
pub fn provider_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    curly_codegen_impl::derive::provider(input).into()
}