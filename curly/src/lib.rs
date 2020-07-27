#![warn(clippy::pedantic)]

pub mod error;
pub mod formatters;
pub mod formatting;
pub mod parsing;

pub use error::*;

#[macro_export]
macro_rules! curly_unreachable {
    () => {
        Err(CurlyErrorKind::Internal(CurlyError::from_boxed(
            "Unreachable!".to_string(),
        )))
    };
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
                    _ => ::std::result::Result::Err($crate::CurlyErrorKind::Generic($crate::CurlyError::from_boxed(format!("Invalid specifier '{}'", key))))
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
    fn provide(&self, context: &formatting::CurlyContext, key: &str) -> CurlyFmtResult;
}

pub type CurlyResult<T> = Result<T, CurlyErrorKind>;
pub type CurlyFmtResult = CurlyResult<String>;

// Minimum + all implementations for primitives and std types according to
// their code and/or documentations
pub mod prelude {

    pub use crate::minimum::*;
}

// The minimum required to get curly working, not including any
// CurlyFmt implementations for primitives and std types
pub mod minimum {

    pub use crate::error::*;
    pub use crate::formatting::*;
    pub use crate::{CurlyFmtResult, CurlyResult};
}
