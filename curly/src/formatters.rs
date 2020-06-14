use super::*;
use regex::Regex;

#[cfg_attr(test, derive(Eq, PartialEq))]
#[derive(Debug)]
pub struct CurlyFormatter {
    pub(crate) prefixes: Option<String>,
    pub(crate) preflags: Option<Vec<CurlyPreFlags>>,
    pub(crate) specifier: Option<String>,
    pub(crate) postfixes: Option<String>,
    pub(crate) postflags: Option<Vec<CurlyPostFlags>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NumeralSign {
    Plus,  // '+'
    Minus, // '-'
}

#[derive(Debug, Eq, PartialEq)]
pub enum Alignment {
    Left,   // '<'
    Center, // '^'
    Right,  // '>'
}

#[derive(Debug, Eq, PartialEq)]
pub enum CurlyPreFlags {
    // '<' or '^' or '>'
    Align(Option<Alignment>),
    // numeric
    Width(Option<usize>),
    // numeric
    Precision(Option<usize>),
    // '+' or '-'
    NumeralSign(NumeralSign),
    Other(char),
    // '#'
    Alternate,
    // '?'
    Debug,
    // '0'
    Zero,
    // '$'
    Plurality,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CurlyPostFlags {
    // '<' or '^' or '>'
    Align(Option<Alignment>),
    // numeric
    Width(Option<usize>),
    // any non-numeric non-flag character (i.e. not '<', '^', '>', '!', '_', or '-')
    PadChar(Option<char>),
    // '!'
    ToUpper,
    // '_'
    ToLower,
    // '-'
    ToCapital,
}

impl CurlyFormatter {
    pub fn prefixes(&self) -> Option<&str> {
        match &self.prefixes {
            Some(prefixes) => Some(&prefixes),
            None => None,
        }
    }

    pub fn specifier(&self) -> Option<&str> {
        match &self.specifier {
            Some(specifier) => Some(&specifier),
            None => None,
        }
    }

    pub fn to_segment(&self) -> String {
        let prefixes = self
            .prefixes
            .clone()
            .map(|p| format!("{}:", p))
            .unwrap_or(String::new());
        let specifier = self
            .specifier
            .clone()
            .unwrap_or("undefined_specifier".to_string());
        let postfixes = self
            .postfixes
            .clone()
            .map(|p| format!("/{}", p))
            .unwrap_or(String::new());
        format!("{{{{{}{}{}}}}}", prefixes, specifier, postfixes)
    }

    /// Generate a `CurlyFormatter` from a single format segment (one statement between `{{}}`s)
    ///
    /// This function does not validate input YET, as that is done by // TODO: input parsing
    pub fn from_segment(
        format_segment: &str,
        base_row: usize,
        base_col: usize,
    ) -> Result<CurlyFormatter, CurlyErrorKind> {
        let mut prefixes: Option<String> = None;
        let mut specifier: Option<String> = None;
        let mut postfixes: Option<String> = None;
        lazy_static! {
            static ref RE: Regex = Regex::new("\\{\\{((([A-Za-z0-9~!@#%^&$*()_+\\-=;'\",.<>/?]+)[:]{1})?)([A-Za-z0-9_.]+)(([/]{1}([A-Za-z0-9~!@#%^&$*()_+\\-=;:'\",.<>/?]+))?)\\}\\}").expect("Failed to compile regex");
        }
        for cap in RE.captures_iter(format_segment) {
            prefixes = cap.get(3).map(|p| p.as_str().to_string());
            specifier = Some(cap[4].to_string());
            postfixes = cap.get(7).map(|p| p.as_str().to_string());
        }
        Ok(CurlyFormatter {
            prefixes,
            specifier,
            postfixes,
            preflags: None,
            postflags: None,
        })
    }

    pub fn default() -> CurlyFormatter {
        CurlyFormatter {
            prefixes: None,
            specifier: None,
            postfixes: None,
            preflags: None,
            postflags: None,
        }
    }
}

pub trait CurlyFormattable {
    fn curly_format(&self, formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind>;
}

impl CurlyFormattable for String {
    fn curly_format(&self, _formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind> {
        Ok(self.clone())
    }
}

impl CurlyFormattable for str {
    fn curly_format(&self, _formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind> {
        Ok(self.to_string())
    }
}

impl CurlyFormattable for bool {
    fn curly_format(&self, formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind> {
        if let Some(prefixes) = &formatter.prefixes {
            let mut yesno = false;
            let mut invert = false;
            for ch in prefixes.chars() {
                if ch == 'q' || ch == 'Q' {
                    yesno = true;
                } else if ch == '!' {
                    invert = true;
                } else {
                    return Err(CurlyErrorKind::Syntax(CurlyError::from(format!(
                        "Syntax error: invalid prefix character '{}' at '{}'",
                        ch,
                        formatter.to_segment()
                    ))));
                }
            }
            let mut val = self.clone();
            if invert {
                val = !val;
            }
            if yesno {
                return Ok(if val { "yes" } else { "no" }.to_string());
            }
            return Ok(val.to_string());
        }
        Ok(self.to_string())
    }
}

// PLEASE DO NOT IMPLEMENT - Used for post-formatting
pub trait PostFormattable {
    fn curly_post(&self, formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind>;
}

impl PostFormattable for String {
    fn curly_post(&self, formatter: &CurlyFormatter) -> Result<String, CurlyErrorKind> {
        if let Some(postfixes) = &formatter.postfixes {
            let mut result = self.clone();

            result = match postfixes.chars().next().unwrap() {
                '^' => result.to_uppercase(),
                '_' => result.to_lowercase(),
                '!' => result
                    .chars()
                    .enumerate()
                    .map(|(idx, ch)| {
                        if idx == 0 {
                            ch.to_uppercase().to_string()
                        } else {
                            ch.to_string()
                        }
                    })
                    .collect(),
                _ => result,
            };
            Ok(result)
        } else {
            Ok(self.clone())
        }
    }
}

/*#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn from_segment_prefixes() {
        let expected = CurlyFormatter {
            prefixes: Some("prefixes".to_string()),
            specifier: Some("specifier".to_string()),
            postfixes: None,
        };
        let got = CurlyFormatter::from_segment("{{prefixes:specifier}}").unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    pub fn from_segment_specifier_only() {
        let expected = CurlyFormatter {
            prefixes: None,
            specifier: Some("specifier".to_string()),
            postfixes: None,
        };
        let got = CurlyFormatter::from_segment("{{specifier}}").unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    pub fn from_segment_postfixes() {
        let expected = CurlyFormatter {
            prefixes: None,
            specifier: Some("specifier".to_string()),
            postfixes: Some("postfixes".to_string()),
        };
        let got = CurlyFormatter::from_segment("{{specifier/postfixes}}").unwrap();
        assert_eq!(expected, got);
    }
}
*/
// Temporarily disabling these tests while I work on better parsing
