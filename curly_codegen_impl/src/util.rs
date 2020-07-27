use proc_macro2::TokenStream;

/// Parses a TokenStream into a String.
///
/// This is useful for literals like `syn::Lit::Str(Str)`, where
/// `Str::parse_with` or similar methods are used to parse a
/// literal.
///
/// Note that with `quote!` syntax might not be the exact
/// same as the syntax that is passed in.
///
/// ```
/// # use curly_codegen_impl::util::StringParser;
/// # use quote::quote;
/// # use syn::parse::Parser;
/// let tokens = quote! {
///     struct Test { }
/// };
/// assert_eq!(StringParser::new().parse2(tokens).unwrap(), "struct Test { }");
/// ```
pub struct StringParser {}

impl Default for StringParser {
    fn default() -> Self {
        Self {}
    }
}

impl StringParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl syn::parse::Parser for StringParser {
    type Output = String;

    fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
        Ok(tokens.to_string())
    }
}