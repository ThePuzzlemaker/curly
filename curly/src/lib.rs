#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod errors;
pub mod formatters;
pub mod formatting;
pub mod parsing;

pub use errors::*;

/// Re-export Provider derivation with `#[cfg(feature = "derive")]`
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate curly_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use curly_derive::*;

mod internal_macros {
    #[macro_export]
    macro_rules! curly_unreachable {
        () => {
            Err(CurlyErrorKind::Internal(CurlyError::from_boxed(
                "Unreachable!".to_string(),
            )))
        };
    }
}

// TODO: actual formatting
#[macro_export]
macro_rules! curly {
    ($format_string:expr, $($argument_name:ident: $argument_type:ty = $argument_value:expr), *, ..$delegate_provider:ident: $delegate_type:ty) => {{
        use $crate::formatting::CurlyFmt;
        use $crate::CurlyFmtResult;
        struct CurlyArgumentsInternal {
            $(
                $argument_name: $argument_type,
            )*
            delegate_provider: $delegate_type
        }
        impl $crate::Provider for CurlyArgumentsInternal {
            fn provide (&self, context: &$crate::formatting::CurlyContext, key: &str) -> CurlyFmtResult {
                match key {
                    $(
                        stringify!($argument_name) => self.$argument_name.curly_fmt(context),
                    )*
                    _ => self.delegate_provider.provide(context, key)
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
        use $crate::formatting::CurlyFmt;
        use $crate::CurlyFmtResult;
        struct CurlyArgumentsInternal {
            $(
                $argument_name: $argument_type,
            )*
        }
        impl $crate::Provider for CurlyArgumentsInternal {
            fn provide (&self,
                        context: &$crate::formatting::CurlyContext,
                        key: &str) -> CurlyFmtResult {
                match key {
                    $(
                        stringify!($argument_name) => self.$argument_name.curly_fmt(context),
                    )*
                    _ => ::std::result::Result::Err($crate::CurlyErrorKind::Generic($crate::CurlyError::from_boxed(format!("Invalid format specifier `{}`.", key))))
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

/// A trait that allows implementing structs to provide formatted objects to print.
pub trait Provider {
    /// Provide the formatted result of object `key` in this struct with formatting context `context`.
    ///
    /// When possible, please use [`curly_derive`](curly_derive) (or the `derive` feature) to do this for you.
    /// Otherwise, code breaks and gets all messy.
    ///
    /// # Errors
    ///
    /// Type [`CurlyErrorKind::Generic`](errors::CurlyErrorKind::Generic), Message: ``Invalid format specifier: `<KEY>`.``:
    /// Key `<KEY>` was not found within this struct.
    ///
    /// All other errors are from formatting objects within this struct.
    fn provide(&self, context: &formatting::CurlyContext, key: &str) -> CurlyFmtResult;
}

/// A [`Result<T, E>`](std::result::Result) with [`E=CurlyErrorKind`](curly::errors::CurlyErrorKind), genericized over `T`
pub type CurlyResult<T> = Result<T, CurlyErrorKind>;
/// A [`Result<T, E>`](std::result::Result) with [`E=CurlyErrorKind`](curly::errors::CurlyErrorKind), and [`T=String`](curly::errors::CurlyErrorKind),
/// intended for use with formatting (hence the name).
pub type CurlyFmtResult = CurlyResult<String>;

/// Required modules and imports for Curly.
pub mod prelude {

    pub use crate::errors::*;
    pub use crate::formatting::*;
    pub use crate::{CurlyFmtResult, CurlyResult};
}
