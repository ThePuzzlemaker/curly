#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod formatters;

pub use error::*;

// TODO: actual formatting
#[macro_export]
macro_rules! curly {
    ($format_string:expr, $($argument_name:ident: $argument_type:ty = $argument_value:expr), *, ..$delegate_provider:ident: $delegate_type:ty) => {{
        use ::curly::formatters::CurlyFormattable;
        use ::curly::formatters::PostFormattable;
        struct CurlyArgumentsInternal {
            $(
                $argument_name: $argument_type,
            )*
            delegate_provider: $delegate_type
        }
        impl ::curly::Provider for CurlyArgumentsInternal {
            fn provide (&self, formatter: &::curly::formatters::CurlyFormatter, key: &str) -> Result<::std::string::String, ::curly::CurlyErrorKind> {
                match key {
                    $(
                        stringify!($argument_name) => self.$argument_name.curly_format(formatter)?.curly_post(formatter),
                    )*
                    _ => self.delegate_provider.provide(formatter, key)
                }
            }
        }
        let arguments = CurlyArgumentsInternal {
            $(
                $argument_name: $argument_value,
            )*
            delegate_provider: $delegate_provider
        };
    }};
    ($format_string:expr, $($argument_name:ident: $argument_type:ty = $argument_value:expr), *,) => {{
        use ::curly::formatters::*;
        use ::curly::*;
        use ::curly::formatters::CurlyFormattable;
        use ::curly::formatters::PostFormattable;
        struct CurlyArgumentsInternal {
            $(
                $argument_name: $argument_type,
            )*
        }
        impl ::curly::Provider for CurlyArgumentsInternal {
            fn provide (&self,
                        formatter: &::curly::formatters::CurlyFormatter,
                        key: &str) -> Result<::std::string::String, ::curly::CurlyErrorKind> {
                match key {
                    $(
                        stringify!($argument_name) => self.$argument_name.curly_format(formatter)?.curly_post(formatter),
                    )*
                    _ => ::std::result::Result::Err(::curly::CurlyErrorKind::Generic(::curly::CurlyError::from("Invalid key".to_string())))
                }
            }
        }
        let arguments = CurlyArgumentsInternal {
            $(
                $argument_name: $argument_value,
            )*
        };
    }};
    ($format_string:expr, $($argument_name:ident: $argument_type:ty = $argument_value:expr), *) => {{
        curly!($format_string, $($argument_name: $argument_type = $argument_value, )*)
    }}
}

pub trait Provider {
    fn provide(
        &self,
        formatter: &formatters::CurlyFormatter,
        key: &str,
    ) -> Result<String, CurlyErrorKind>;
}
