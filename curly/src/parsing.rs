macro_rules! curly_expect {
  ($result:ident, $expected:expr, $message:expr, $row:expr, $col:expr, $segment:expr, $relrow:expr, $relcol:expr) => {
    if !($expected) {
      $result = Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(format!("Expected {} at {}:{} (error is within segment '{}' at {}:{})", $message, $row, $col, $segment, $relrow, $relcol))));
    }
  };
  (explicit $expected:expr, $message:expr, $row:expr, $col:expr, $segment:expr, $relrow:expr, $relcol:expr) => {
    if !($expected) {  
      return Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(format!("Expected {} at {}:{} (error is within segment '{}' at {}:{})", $message, $row, $col, $segment, $relrow, $relcol))));
    }
  };
  (implicit $expected:expr, $message:expr, $row:expr, $col:expr, $segment:expr, $relrow:expr, $relcol:expr) => {
      if !($expected) {
        Err($crate::CurlyErrorKind::Syntax($crate::CurlyError::from(format!("Expected {} at {}:{} (error is within segment '{}' at {}:{})", $message, $row, $col, $segment, $relrow, $relcol))))
      }
   }
}

static ESCAPEABLE_CHARACTERS: [char; 3] = ['\\', '{', '}'];