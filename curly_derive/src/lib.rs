extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::Fields;

#[allow(unused_imports)]
use curly::Provider;

#[proc_macro_derive(Provider)]
pub fn provider_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_provider(&ast)
}

fn impl_provider(ast: &syn::DeriveInput) -> TokenStream {
    let gen;
    if let Data::Struct(data) = &ast.data {
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        let mut matches = quote! {};
        let struct_fields = &data.fields;
        if let Fields::Named(fields) = struct_fields {
            for field in fields.named.iter() {
                if let Some(field_name) = &field.ident {
                    let quoted = quote! {
                        stringify!(#field_name) => Some(self.#field_name.curly_format(formatter).post_format(formatter)),
                    };
                    matches.extend(quoted);
                }
            }
        } else if let Fields::Unnamed(_fields) = struct_fields {
            // This code is currently commented out, as unnamed fields are a giant headache when doing curly formatting.
            // Please use named fields.
            /*for (raw_index, _field) in fields.unnamed.iter().enumerate() {
                let index = syn::Index::from(raw_index);
                let quoted = quote! {
                    stringify!(#index) => self.#index.curly_format(formatter).post_format(formatter),
                };
                matches.extend(quoted);
            }*/
            panic!("Sorry, as of now, Provider derivation does not work on tuple-style structs.")
        }
        gen = quote! {
            use ::curly::formatters::CurlyFormattable;
            use ::curly::formatters::PostFormattable;
            impl #impl_generics ::curly::Provider for #name #ty_generics #where_clause {
                fn provide(&self, formatter: ::curly::formatters::CurlyFormatter, key: &str) -> ::std::result::Result<::std::string::String, ::curly::CurlyErrorKind> {
                    match key {
                        #matches
                        _ => ::std::result::Result::Err(::curly::CurlyErrorKind::Generic(::curly::CurlyError::from("Invalid key".to_string())))
                    }
                }
            }
        };
    } else {
        panic!("Sorry, as of now, Provider derivation only works on structs");
    }
    gen.into()
}
