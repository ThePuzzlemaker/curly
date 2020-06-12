#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod formatters;

pub use error::*;

pub trait Provider {
    fn provide(
        &self,
        formatter: &formatters::CurlyFormatter,
        key: &str,
    ) -> Result<String, CurlyErrorKind>;
    fn delegate(
        &self,
        provider: Box<dyn Provider>,
        formatter: &formatters::CurlyFormatter,
        key: &str,
    ) -> Result<String, CurlyErrorKind> {
        provider.provide(formatter, key)
    }
}
