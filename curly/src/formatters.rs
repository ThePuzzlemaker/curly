
use crate::minimum::*;

use std::fmt::Debug;
use NumberType::*;

use crate::curly_unreachable;

impl<T: CurlyDebug + CurlyFmt> Curly for T {
    fn curly(&self, context: &CurlyContext) -> CurlyFmtResult {
        if context.flags.debug {
            self.curly_debug(context)
        } else {
            self.curly_fmt(context)
        }
    }
}

impl<T: Debug> CurlyDebug for T {
    fn curly_debug(&self, context: &CurlyContext) -> CurlyFmtResult {
        // FIXME: Make sure parser only allows upper and lower hex for debug as that's what regular debug does

        // Yes, I know this is an absolute abomination. However, it makes it INCREDIBLY easy to implement.
        let f = &context.flags;
        if f.alternate {
            if f.sign_plus {
                return match &f.number_type {
                    Normal => Ok(format!("{:+#?}", self)),
                    UpperHex => Ok(format!("{:+#X?}", self)),
                    LowerHex => Ok(format!("{:+#x?}", self)),
                    _ => curly_unreachable!(),
                };
            } else if f.sign_minus {
                return match &f.number_type {
                    Normal => Ok(format!("{:-#?}", self)),
                    UpperHex => Ok(format!("{:-#X?}", self)),
                    LowerHex => Ok(format!("{:-#x?}", self)),
                    _ => curly_unreachable!(),
                };
            } else {
                return match &f.number_type {
                    Normal => Ok(format!("{:#?}", self)),
                    UpperHex => Ok(format!("{:#X?}", self)),
                    LowerHex => Ok(format!("{:#x?}", self)),
                    _ => curly_unreachable!(),
                };
            }
        } else {
            if f.sign_plus {
                return match &f.number_type {
                    Normal => Ok(format!("{:+?}", self)),
                    UpperHex => Ok(format!("{:+X?}", self)),
                    LowerHex => Ok(format!("{:+x?}", self)),
                    _ => curly_unreachable!(),
                };
            } else if f.sign_minus {
                return match &f.number_type {
                    Normal => Ok(format!("{:-?}", self)),
                    UpperHex => Ok(format!("{:-X?}", self)),
                    LowerHex => Ok(format!("{:-x?}", self)),
                    _ => curly_unreachable!(),
                };
            } else {
                return match &f.number_type {
                    Normal => Ok(format!("{:?}", self)),
                    UpperHex => Ok(format!("{:X?}", self)),
                    LowerHex => Ok(format!("{:x?}", self)),
                    _ => curly_unreachable!(),
                };
            }
        }
    }
}
