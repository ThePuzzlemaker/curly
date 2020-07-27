use super::*;
use std::fmt::{Alignment, Debug};
#[derive(Debug)]
pub struct CurlyContext {
    pub(crate) custom_flags: Option<String>,
    pub(crate) flags: CurlyFlags,
    pub(crate) specifier: Option<String>,
}

#[derive(Debug)]
pub enum NumberType {
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
    Normal,
}

#[derive(Debug)]
pub struct CurlyFlags {
    pub fill: char,
    pub align: Option<Alignment>,
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub sign_plus: bool,
    pub sign_minus: bool,
    pub sign_aware_zero_pad: bool,
    pub alternate: bool,
    pub debug: bool,
    pub number_type: NumberType,
}

impl CurlyContext {
    pub fn specifier(&self) -> Option<&str> {
        match &self.specifier {
            Some(specifier) => Some(&specifier),
            None => None,
        }
    }

    pub fn custom_flags(&self) -> Option<&str> {
        match &self.custom_flags {
            Some(custom_flags) => Some(&custom_flags),
            None => None,
        }
    }

    pub fn flags(&self) -> &CurlyFlags {
        &self.flags
    }

    /// Generate a `CurlyContext` from a single format segment (one statement between `{}`s)
    ///
    /// This function does not validate input YET, as that is done by // TODO: input parsing
    pub fn from_segment(
        _format_segment: &str,
        _base_row: usize,
        _base_col: usize,
    ) -> CurlyResult<Self> {
        Ok(Self::default())
    }
}

impl Default for CurlyContext {
    fn default() -> Self {
        Self {
            specifier: None,
            flags: CurlyFlags::default(),
            custom_flags: None,
        }
    }
}

impl Default for CurlyFlags {
    fn default() -> Self {
        Self {
            fill: ' ',
            align: None,
            width: None,
            precision: None,
            sign_plus: false,
            sign_minus: false,
            sign_aware_zero_pad: false,
            alternate: false,
            debug: false,
            number_type: NumberType::Normal,
        }
    }
}

pub trait Curly {
    fn curly(&self, context: &CurlyContext) -> CurlyFmtResult;
}

pub trait CurlyFmt {
    fn curly_fmt(&self, context: &CurlyContext) -> CurlyFmtResult;
}

pub trait CurlyDebug {
    fn curly_debug(&self, context: &CurlyContext) -> CurlyFmtResult;
}
