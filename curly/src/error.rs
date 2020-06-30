use std::error::Error;
#[derive(Debug)]
pub enum CurlyErrorKind {
    Generic(Box<dyn Error>),
    Syntax(Box<dyn Error>),
    Internal(Box<dyn Error>),
}

impl std::fmt::Display for CurlyErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CurlyErrorKind::Generic(e) => {
                fmt.write_str("Error: ")?;
                e.fmt(fmt)
            }
            CurlyErrorKind::Syntax(e) => {
                fmt.write_str("Syntax Error: ")?;
                e.fmt(fmt)
            }
            CurlyErrorKind::Internal(e) => {
                fmt.write_str("Internal Error: ")?;
                e.fmt(fmt)
            }
        }
    }
}

#[derive(Debug)]
pub struct CurlyError {
    msg: String,
}

impl std::fmt::Display for CurlyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.msg)
    }
}

impl std::error::Error for CurlyError {}

impl CurlyError {
    pub fn from(msg: String) -> Self {
        CurlyError { msg }
    }

    pub fn from_boxed(msg: String) -> Box<Self> {
        Box::new(CurlyError { msg })
    }
}
