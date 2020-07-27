extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[path = "impl.rs"]
mod implement;
use implement::*;

#[proc_macro_derive(Provider, attributes(curly_ignore, curly_rename))]
pub fn provider_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    implement_provider(input).into()
}

// fn impl_provider(ast: &syn::DeriveInput) -> TokenStream {
//     let gen;
//     if let Data::Struct(data) = &ast.data {
//         let name = &ast.ident;
//         let crate_name =
//             proc_macro_crate::crate_name("curly").expect("Failed to find curly in `Cargo.toml`");
//         let crate_name_ident = syn::Ident::new(&crate_name, proc_macro2::Span::call_site());
//         let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
//         let mut matches = quote! {};
//         let struct_fields = &data.fields;
//         if let Fields::Named(fields) = struct_fields {
//             for field in fields.named.iter() {
//                 if let Some(field_name) = &field.ident {
//                     let quoted = quote! {
//                         stringify!(#field_name) => self.#field_name.curly_format(formatter)?,
//                     };
//                     matches.extend(quoted);
//                 }
//             }
//         } else if let Fields::Unnamed(_fields) = struct_fields {
//             panic!("Sorry, Provider derivation does not work on tuple-style structs.")
//         }
//         let span = proc_macro2::Span::call_site();
//         let modname =
//             proc_macro2::Ident::new(&format!("__curly_internal_provider_implfor_{}", name), span);
//         gen = quote! {
//             mod #modname {
//                 extern crate #crate_name_ident as curly;
//                 use curly::formatters::CurlyFormattable;
//                 use curly::formatters::PostFormattable;
//                 impl #impl_generics curly::Provider for super::#name #ty_generics #where_clause {
//                     fn provide(&self, formatter: &curly::formatters::CurlyFormatter, key: &str) -> ::std::result::Result<::std::string::String, curly::CurlyErrorKind> {
//                         match key {
//                             #matches
//                             _ => ::std::result::Result::Err(curly::CurlyErrorKind::Generic(curly::CurlyError::from(format!("Invalid specifier '{}'", key))))
//                         }
//                     }
//                 }
//             };

//         };
//     } else {
//         panic!("Sorry, Provider derivation only works on structs");
//     }
//     gen.into()
// }
