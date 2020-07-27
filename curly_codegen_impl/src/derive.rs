use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

use syn::{Data, Field, Fields, Ident, Lit, Meta};

use super::util::StringParser;

pub fn provider(input: DeriveInput) -> TokenStream {
    let generated;

    let span = Span::call_site();

    let struct_data;
    let struct_name = input.ident;

    let crate_name =
        proc_macro_crate::crate_name("curly").expect("Failed to find curly in `Cargo.toml`");
    let crate_ident = Ident::new(&crate_name, span);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Data::Struct(data) = input.data {
        struct_data = data;
    } else {
        panic!("Deriving a provider only works on structs");
    }

    let struct_fields = &struct_data.fields;
    let struct_fields_named;

    if let Fields::Named(fields) = struct_fields {
        struct_fields_named = fields;
    } else if let Fields::Unit = struct_fields {
        panic!("Deriving a provider on a unit struct does nothing");
    } else {
        panic!("Deriving a provider only works on structs with named fields");
    }

    let module_name = Ident::new(
        &format!("__curly_internal_provider_implement_for_{}", struct_name),
        span,
    );

    let mut matches = quote! {};

    for field in &struct_fields_named.named {
        let field_ident;
        if let Some(ident) = &field.ident {
            field_ident = ident;
        } else {
            unreachable!();
        }

        if !should_provide(&field) {
            continue;
        }

        let field_name = get_provided_name(&field).unwrap_or(field_ident.to_string());

        let quoted = quote! {
            #field_name => self.#field_ident.curly_fmt(context),
        };
        matches.extend(quoted);
    }

    generated = quote! {
        mod #module_name {
            extern crate #crate_ident as curly;

            use curly::formatting::CurlyFmt;

            impl #impl_generics curly::Provider for super::#struct_name #ty_generics
                #where_clause
            {
                fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                    match key {
                        #matches
                        _ => ::std::result::Result::Err(
                            curly::CurlyErrorKind::Generic(
                                curly::CurlyError::from_boxed(
                                    format!("Invalid format specifier `{}`", key)
                                )
                            )
                        )
                    }
                }
            }
        }
    };

    generated
}

fn should_provide(field: &Field) -> bool {
    if let Some(ident) = &field.ident {
        if ident.to_string().starts_with('_') {
            return false;
        }
    }

    for attr in &field.attrs {
        let meta;
        if let Ok(meta_) = attr.parse_meta() {
            meta = meta_;
        } else {
            continue;
        }

        if let Meta::Path(path) = meta {
            return !path.is_ident(&Ident::new("curly_ignore", Span::call_site()));
        }
    }

    true
}

fn get_provided_name(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        let meta;
        if let Ok(meta_) = attr.parse_meta() {
            meta = meta_;
        } else {
            continue;
        }

        if let Meta::NameValue(meta) = meta {
            if !meta
                .path
                .is_ident(&Ident::new("curly_rename", Span::call_site()))
            {
                continue;
            }

            if let Lit::Str(lit) = meta.lit {
                return lit.parse_with(StringParser::new()).ok();
            } else {
                panic!("Invalid literal for `#[curly_rename]`, must be a UTF-8 string literal");
            }
        }
    }

    None
}
