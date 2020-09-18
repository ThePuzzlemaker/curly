use super::prelude::*;
use std::fmt::Debug;
#[derive(Debug, Eq, PartialEq, Clone, Hash, Default)]
pub struct CurlyContext {
    pub custom_flags: Option<String>,
    pub flags: CurlyFlags,
    pub specifier: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct CurlyFlags {
    pub fill: char,
    pub align: Option<Alignment>,
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub sign: Option<Sign>,
    pub sign_aware_zero_pad: bool,
    pub alternate: bool,
    pub number_type: NumberType,
}

impl CurlyContext {
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

impl Default for CurlyFlags {
    fn default() -> Self {
        Self {
            fill: ' ',
            align: None,
            width: None,
            precision: None,
            sign: None,
            sign_aware_zero_pad: false,
            alternate: false,
            number_type: NumberType::Normal,
        }
    }
}

pub trait CurlyFmt {
    fn curly_fmt(&self, context: &CurlyContext) -> CurlyFmtResult;
}
