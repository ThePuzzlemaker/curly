#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive a `Provider` on a struct.
///
/// # Utility Attributes
///
/// ## `#[curly_ignore]`
/// Put this on a struct field to ignore that field.
/// Fields starting with `_` are automatically ignored.
/// In the future, there will be a utility macro to "unignore" those
/// fields.
///
/// ### Example
///
/// ```rs
/// #[macro_use]
/// extern crate curly;
///
/// #[derive(Debug, Provider)]
/// struct SomeStruct {
///     #[curly_ignore]
///     some_internal_value: i16,
///
///     // This value is automatically ignored.
///     _something: u32,
/// }
/// ```
///
/// In this example, the fields `some_internal_value` and `_something` cannot be used for formatting.
///
/// ## `#[curly_rename = "name"]`
/// Put this on a struct field, replacing `"name"` with the field name to rename that field.
///
/// ### Example
///
/// ```rs
/// #[macro_use]
/// extern crate curly;
///
/// #[derive(Debug, Provider)]
/// struct SomeStruct {
///     #[curly_rename = "meaning_of_life"]
///     the_meaning_of_life_the_universe_and_everything: u8
/// }
/// ```
///
/// In this example, the field `the_meaning_of_life_the_universe_and_everything` is renamed to `meaning_of_life`
/// within the provider.
#[proc_macro_derive(Provider, attributes(curly_ignore, curly_rename))]
pub fn provider_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    curly_codegen_impl::derive::provider(input).into()
}
