/* macro_rules! curly_expect {
    ($result:ident, $expected:expr, $message:expr, $row:expr, $col:expr) => {
        if !($expected) {
            $result = Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(
                format!(
                    "Expected {} at {}:{}",
                    $message, $row, $col, $segment, $relcol
                ),
            )))
        }
    };
    (explicit $expected:expr, $message:expr, $row:expr, $col:expr) => {
        if !($expected) {
            return Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(
                format!("Expected {} at {}:{}", $message, $row, $col),
            )));
        }
    };
    (implicit $expected:expr, $message:expr, $row:expr, $col:expr) => {
        if !($expected) {
            Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(
                format!(
                    "Expected {} at {}:{} (error is within segment '{}' at {})",
                    $message, $row, $col
                ),
            )))
        }
    };
} */
