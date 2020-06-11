use super::*;
use regex::Regex;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct CurlyFormatter {
    pub(crate) prefixes: Option<String>,
    pub(crate) specifier: Option<String>,
    pub(crate) postfixes: Option<String>,
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

    /// Generate a `CurlyFormatter` from a single format segment (one statement between `{{}}`s)
    ///
    /// This function does not validate input, as that is done by // TODO: input parsing
    pub(crate) fn from_segment(format_segment: &str) -> Result<CurlyFormatter, CurlyErrorKind> {
        let mut prefixes: Option<String> = None;
        let mut specifier: Option<String> = None;
        let mut postfixes: Option<String> = None;
        lazy_static! {
            static ref RE: Regex = Regex::new("\\{\\{((([A-Za-z0-9~!@#%^&$*()_+\\-=;'\",.<>/?]+)[:]{1})?)([A-Za-z0-9_\\-=+.]+)(([/]{1}([A-Za-z0-9~!@#%^&$*()_+\\-=;:'\",.<>/?]+))?)\\}\\}").expect("Failed to compile regex");
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
        })
    }

    pub fn default() -> CurlyFormatter {
        CurlyFormatter {
            prefixes: None,
            specifier: None,
            postfixes: None,
        }
    }
}

pub trait CurlyFormattable {
    fn curly_format(&self, formatter: &CurlyFormatter) -> Result<String, CurlyError>;
}

#[cfg(test)]
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

// PLEASE DO NOT IMPLEMENT - Used for post-formatting
pub trait PostFormattable {
    fn curly_post(&self, formatter: &CurlyFormatter) -> Result<String, CurlyError>;
}

impl PostFormattable for String {
    fn curly_post(&self, formatter: &CurlyFormatter) -> Result<String, CurlyError> {
        Ok(String::new())
    }
}
