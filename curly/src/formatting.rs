use super::prelude::*;
use std::fmt::Debug;

/// The formatting context for a single format segment.
/// This includes things such as custom flags, default flags, and
/// the actual specifier.
#[derive(Debug, Eq, PartialEq, Clone, Hash, Default)]
pub struct CurlyContext {
    /// Custom flags (remaining flags that are unparsed by Curly
    /// so that they can be used for custom formatting behaviours.
    pub custom_flags: Option<String>,
    /// Flags that are parsed by Curly for things such as padding,
    /// format type, etc...
    pub flags: CurlyFlags,
    /// The specifier for this format segment.
    pub specifier: Option<String>,
}

/// The number display type for a single format segment.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum NumberType {
    /// Octal (`o`) ⇒ [`Octal`](std::fmt::Octal)
    Octal,
    /// Lowercase hexadecimal (`x`) ⇒ [`LowerHex`](std::fmt::LowerHex)
    LowerHex,
    /// Uppercase hexadecimal (`X`) ⇒ [`UpperHex`](std::fmt::UpperHex)
    UpperHex,
    /// Show as location in memory, usually as hex (`p`) ⇒ [`Pointer`](std::fmt::Pointer)
    Pointer,
    /// Binary (`b`) ⇒ [`Binary`](std::fmt::Binary)
    Binary,
    /// Lowercase scientific notation (`e`) ⇒ [`LowerExp`](std::fmt::LowerExp)
    LowerExp,
    /// Uppercase scientific notation (`E`) ⇒ [`UpperExp`](std::fmt::UpperExp)
    UpperExp,
    /// Normal number types
    Normal,
}

/// The sign for a single format segment (`+` or `-`). `-` is currently unimplemented, but it may be used in the future.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Sign {
    /// Always print sign
    Plus,
    /// Unused, but may be used in the future with [`format!`](std::format!), so it's reserved here.
    Minus,
}

/// The padding alignment for a single format segment.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Alignment {
    /// Left-aligned padding (`<`)
    Left,
    /// Right-aligned padding (`>`)
    Right,
    /// Center-aligned padding (`^`)
    Center,
}

/// Flags for a single format segment, e.g. padding,
/// precision, number type, etc...
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct CurlyFlags {
    /// The padding fill character (default: `' '`)
    pub fill: char,
    /// The padding alignment
    pub align: Option<Alignment>,
    /// The padding width
    pub width: Option<usize>,
    /// The float precision
    pub precision: Option<usize>,
    /// The decimal sign
    pub sign: Option<Sign>,
    /// Whether the zero (`0`) flag is set or not
    pub sign_aware_zero_pad: bool,
    /// Whether the alternate (`#`) flag is set or not
    pub alternate: bool,
    /// The type of number (Normal, Octal, Lower/Upper Hex, etc...)
    pub number_type: NumberType,
}

impl CurlyContext {
    /// Generate a `CurlyContext` from a single format segment (one statement between `{}`s)
    ///
    /// This function does not validate input YET, as that is done by // TODO: input parsing
    /// # Errors
    ///
    /// TODO: Once we get this actually working
    pub fn from_segment(
        _format_segment: &str,
        _base_row: usize,
        _base_col: usize,
    ) -> CurlyResult<Self> {
        unimplemented!();
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

/// A formatter trait for Curly, like [`std`](std)'s [`Display`](std::fmt::Display)
pub trait CurlyFmt {
    /// Formats `&self` to a `String` using the `CurlyContext` as a reference.
    ///
    /// # Errors
    ///
    /// There can be any error that is returned with the formatting of a single format segment.
    /// These will be passed up to the end result of the `curly!` or derivative macros.
    fn curly_fmt(&self, context: &CurlyContext) -> CurlyFmtResult;
}
