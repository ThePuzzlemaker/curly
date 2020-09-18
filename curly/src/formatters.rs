use crate::prelude::*;

impl CurlyFmt for String {
    fn curly_fmt(&self, _context: &CurlyContext) -> CurlyFmtResult {
        Ok(self.clone())
    }
}

impl CurlyFmt for str {
    fn curly_fmt(&self, _context: &CurlyContext) -> CurlyFmtResult {
        Ok(self.to_string())
    }
}
