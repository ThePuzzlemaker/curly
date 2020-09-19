use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DeriveInput;

use syn::{Data, Field, Fields, Ident, Lit, Meta};

use super::util::StringParser;

/// Derive a Provider
pub fn provider(input: DeriveInput) -> TokenStream {
    let generated;

    let span = Span::call_site();

    let struct_data;
    let struct_name = input.ident;

    let crate_name = proc_macro_crate::crate_name("curly").unwrap_or_else(|_| "curly".to_string());
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

    if struct_fields_named.named.is_empty() {
        panic!("Deriving a provider on a struct with no fields does nothing");
    }

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

        let field_name = get_provided_name(&field).unwrap_or_else(|| field_ident.to_string());

        let quoted = quote! {
            #field_name => self.#field_ident.curly_fmt(context),
        };
        matches.extend(quoted);
    }

    generated = quote! {
        #[doc(hidden)] mod #module_name {
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
                                    format!("Invalid format specifier `{}`.", key)
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

/// Whether or not a provider should provide this field.
/// Returns false if the field name starts with `_`, or
/// if the field is annotated with `#[curly_ignore]`.
/// Otherwise, true is returned.
fn should_provide(field: &Field) -> bool {
    if let Some(ident) = &field.ident {
        if ident.to_string().starts_with('_') {
            return false;
        }
    }

    for attr in &field.attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Meta::Path(path) = meta {
                return !path.is_ident(&Ident::new("curly_ignore", Span::call_site()));
            }
        }
    }

    true
}

/// Get the provided name of a field if
/// `#[curly_rename = "new_name"]` is specified.
fn get_provided_name(field: &Field) -> Option<String> {
    for attr in &field.attrs {
        if let Ok(meta) = attr.parse_meta() {
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
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;
    use quote::quote;
    use syn::parse::Parser;
    use syn::Field;

    #[test]
    fn should_not_provide_prefix() {
        let field: Field =
            Parser::parse_str(Field::parse_named, "_should_be_ignored: String").unwrap();
        assert!(!should_provide(&field));
    }

    #[test]
    fn should_not_provide_explicit() {
        let field: Field = Parser::parse_str(
            Field::parse_named,
            "#[curly_ignore] should_be_ignored: String",
        )
        .unwrap();
        assert!(!should_provide(&field));
    }

    #[test]
    fn should_provide_field() {
        let field: Field =
            Parser::parse_str(Field::parse_named, "should_be_provided: String").unwrap();
        assert!(should_provide(&field));
    }

    #[test]
    fn rename_field() {
        let field: Field = Parser::parse_str(
            Field::parse_named,
            "#[curly_rename = \"renamed\"] some_name: String",
        )
        .unwrap();
        assert_eq!(get_provided_name(&field), Some(String::from("renamed")));
    }

    #[test]
    fn dont_rename_field() {
        let field: Field = Parser::parse_str(Field::parse_named, "some_name: String").unwrap();
        assert!(get_provided_name(&field).is_none());
    }

    #[test]
    #[should_panic(
        expected = "Invalid literal for `#[curly_rename]`, must be a UTF-8 string literal"
    )]
    fn panics_on_invalid_literal_for_rename() {
        let field: Field =
            Parser::parse_str(Field::parse_named, "#[curly_rename = 5] some_name: String").unwrap();
        get_provided_name(&field);
    }

    #[test]
    #[should_panic(expected = "Deriving a provider on a unit struct does nothing")]
    fn panics_on_deriving_for_unit_struct() {
        let input: DeriveInput = syn::parse_str("struct TestStruct;").unwrap();
        provider(input);
    }

    #[test]
    #[should_panic(expected = "Deriving a provider on a struct with no fields does nothing")]
    fn panics_on_deriving_for_empty_struct() {
        let input: DeriveInput = syn::parse_str("struct TestStruct {}").unwrap();
        provider(input);
    }

    #[test]
    #[should_panic(expected = "Deriving a provider only works on structs with named fields")]
    fn panics_on_deriving_for_unnamed_fields() {
        let input: DeriveInput = syn::parse_str("struct TestStruct(String);").unwrap();
        provider(input);
    }

    #[test]
    #[should_panic(expected = "Deriving a provider only works on structs")]
    fn panics_on_deriving_for_non_structs() {
        let input: DeriveInput = syn::parse_str("enum NonStruct { Var1, Var2 }").unwrap();
        provider(input);
    }

    #[test]
    fn derive_single() {
        let input: DeriveInput = syn::parse_str(
            r#"
                struct TestDerive {
                    some_field: String
                }
            "#,
        )
        .unwrap();

        let found = provider(input).to_string();

        let expected = quote! {
            #[doc(hidden)] mod __curly_internal_provider_implement_for_TestDerive {
                extern crate curly as curly;

                use curly::formatting::CurlyFmt;

                impl curly::Provider for super::TestDerive {
                    fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                        match key {
                            "some_field" => self.some_field.curly_fmt(context),
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
        }.to_string();

        assert_eq!(found, expected);
    }

    #[test]
    fn derive_multiple() {
        let input: DeriveInput = syn::parse_str(
            r#"
                struct TestDerive {
                    some_field: String,
                    other_field: String,
                }
            "#,
        )
        .unwrap();

        let found = provider(input).to_string();

        let expected = quote! {
            #[doc(hidden)] mod __curly_internal_provider_implement_for_TestDerive {
                extern crate curly as curly;

                use curly::formatting::CurlyFmt;

                impl curly::Provider for super::TestDerive {
                    fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                        match key {
                            "some_field" => self.some_field.curly_fmt(context),
                            "other_field" => self.other_field.curly_fmt(context),
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
        }.to_string();

        assert_eq!(found, expected);
    }

    #[test]
    fn derive_ignore() {
        let input: DeriveInput = syn::parse_str(
            r#"
                struct TestDerive {
                    some_field: String,
                    _ignored: String,
                    #[curly_ignore]
                    ignored: String
                }
            "#,
        )
        .unwrap();

        let found = provider(input).to_string();

        let expected = quote! {
            #[doc(hidden)] mod __curly_internal_provider_implement_for_TestDerive {
                extern crate curly as curly;

                use curly::formatting::CurlyFmt;

                impl curly::Provider for super::TestDerive {
                    fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                        match key {
                            "some_field" => self.some_field.curly_fmt(context),
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
        }.to_string();

        assert_eq!(found, expected);
    }

    #[test]
    fn derive_rename() {
        let input: DeriveInput = syn::parse_str(
            r#"
                struct TestDerive {
                    #[curly_rename = "other_field"]
                    some_field: String
                }
            "#,
        )
        .unwrap();

        let found = provider(input).to_string();

        let expected = quote! {
            #[doc(hidden)] mod __curly_internal_provider_implement_for_TestDerive {
                extern crate curly as curly;

                use curly::formatting::CurlyFmt;

                impl curly::Provider for super::TestDerive {
                    fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                        match key {
                            "other_field" => self.some_field.curly_fmt(context),
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
        }.to_string();

        assert_eq!(found, expected);
    }

    #[test]
    fn derive_generics() {
        let input: DeriveInput = syn::parse_str(
            r#"
                struct TestDerive<T> where T: PartialEq {
                    some_field: String
                }
            "#,
        )
        .unwrap();

        let found = provider(input).to_string();

        let expected = quote! {
            #[doc(hidden)] mod __curly_internal_provider_implement_for_TestDerive {
                extern crate curly as curly;

                use curly::formatting::CurlyFmt;

                impl<T> curly::Provider for super::TestDerive<T>
                    where T: PartialEq
                {
                    fn provide(&self, context: &curly::formatting::CurlyContext, key: &str) -> curly::CurlyFmtResult {
                        match key {
                            "some_field" => self.some_field.curly_fmt(context),
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
        }.to_string();

        assert_eq!(found, expected);
    }
}
