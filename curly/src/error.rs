#[derive(Debug)]
pub enum CurlyErrorKind {
    Generic(CurlyError),
    Syntax(CurlyError),
}

impl std::fmt::Display for CurlyErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CurlyErrorKind::Generic(e) => e.fmt(fmt),
            CurlyErrorKind::Syntax(e) => e.fmt(fmt),
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
    pub fn from(msg: String) -> CurlyError {
        CurlyError { msg }
    }
}
