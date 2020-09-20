#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

/// `#[derive(Provider)]` codegen
#[cfg(feature = "derive")]
pub mod derive;

/// Codegen utilities
pub mod util;
