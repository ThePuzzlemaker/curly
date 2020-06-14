use std::collections::HashMap;

macro_rules! curly_expect {
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
}

pub static ESCAPEABLE_CHARACTERS: [char; 3] = ['\\', '{', '}'];

use super::*;
use formatters::CurlyPreFlags;

pub(crate) enum SyntaxNode {
    DoubleBrace(Vec<SyntaxNode>),
    EscapedCharacter(char),
    OtherCharacter(char),
    Prefixes(Vec<SyntaxNode>),
    Postfixes(Vec<SyntaxNode>),
    Specifier(String),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CurlySpecials {
    EndPrefixes,
    BeginPostfixes,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CurlyAST {
    // Strings (in this case, continuous blocks of characters with no other AST nodes in between) that are not escaped, and not within `{{}}`s
    NonCurly(String),
    // An escaped character
    Escaped(char),
    // AST nodes within double braces
    Braces(Vec<CurlyAST>),
    // A special character indicating a change in context
    Special(CurlySpecials),
    // Specifier prefixes as characters
    Prefixes(Vec<char>),
    // Specifier postfixes as characters
    Postfixes(Vec<char>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CurlyLexemes {
    // Strings (in this case, continuous blocks of characters including newlines with no other lexemes between)
    NonCurly(String),
    // A special character indicating a change in context (':' or `/` in this case, showing the end of a prefix and beginning of a postfix respectively)
    Special(CurlySpecials),
    // A character that is possibly escaped (i.e. the last character was a `\`), separated from a string as it may need to be individally represented
    PossiblyEscaped(char),
    // A backslash, indicating the next character to be found is a lexeme of type PossiblyEscaped
    Backslash,
    // A single opening curly brace
    OpenBrace,
    // A single closing curly brace.
    CloseBrace,
}

// A string turned into a 2-dimensional matrix of characters
// (i.e. a string that is indexed by row and column).
//
// Using this has the disadvantage of making parsing and lexing O(n^2),
// however it makes lexing and parsing errors much more helpful and easy to
// implement.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct CharMatrix {
    pub(crate) rows: HashMap<usize, String>,
}

impl CharMatrix {
    pub(crate) fn new(input: &str) -> CharMatrix {
        let mut matrix = CharMatrix {
            rows: HashMap::new(),
        };
        let mut current_string = String::new();
        let mut row = 0;

        for ch in input.chars() {
            if ch == '\n' {
                matrix.rows.insert(row, current_string.clone());
                row += 1;
                current_string.clear();
            } else if ch == '\r' {
                // We need to make sure carriage returns are ignored.
                continue;
            } else {
                current_string.push(ch);
            }
        }
        if !input.ends_with('\n') {
            matrix.rows.insert(row, current_string);
        }

        matrix
    }

    pub(crate) fn get_row(&self, row: usize) -> Option<&String> {
        self.rows.get(&row)
    }

    pub(crate) fn get_char(&self, row: usize, col: usize) -> Option<char> {
        self.get_row(row)?.chars().nth(col)
    }
}

pub(crate) fn lexer(
    input: &CharMatrix,
    base_row: Option<usize>,
    base_col: Option<usize>,
) -> Result<Vec<CurlyLexemes>, CurlyErrorKind> {
    let row = base_row.unwrap_or(0);
    let col = base_col.unwrap_or(0);
    let mut current_string = String::new();
    let mut should_capture_newline = false;
    let mut lexemes: Vec<CurlyLexemes> = Vec::new();
    let mut last_backslash = false;
    let mut last_row = 0;
    let mut last_col = 0;

    // We need to collect and sort these keys as HashMap iterators/key iterators are arbitrary order, when we need sorted order
    let mut keys: Vec<usize> = input
        .rows
        .keys()
        .into_iter()
        .map(|u| *u)
        .collect::<Vec<usize>>();

    keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for rowidx in keys {
        last_col = 0;
        let row = input.get_row(rowidx).unwrap();
        if row.is_empty() {
            current_string.push('\n');
            should_capture_newline = false;
        }
        for (_chidx, ch) in row.chars().enumerate() {
            last_col = last_col + 1;
            let current_lexeme: CurlyLexemes;
            if last_backslash {
                lexemes.push(CurlyLexemes::PossiblyEscaped(ch));
                last_backslash = false;
                continue;
            }

            if ch == '\\' {
                current_lexeme = CurlyLexemes::Backslash;
                last_backslash = true;
            } else if ch == '{' {
                current_lexeme = CurlyLexemes::OpenBrace;
            } else if ch == '}' {
                current_lexeme = CurlyLexemes::CloseBrace;
            } else if ch == ':' {
                current_lexeme = CurlyLexemes::Special(CurlySpecials::EndPrefixes);
            } else if ch == '/' {
                current_lexeme = CurlyLexemes::Special(CurlySpecials::BeginPostfixes);
            } else {
                should_capture_newline = true;
                current_string.push(ch);
                continue;
            }

            if !current_string.is_empty() {
                lexemes.push(CurlyLexemes::NonCurly(current_string.clone()));
                current_string.clear();
                should_capture_newline = false;
            }
            lexemes.push(current_lexeme);
        }
        if should_capture_newline {
            current_string.push('\n');
        }
        last_row = last_row + 1;
    }
    if !current_string.is_empty() {
        lexemes.push(CurlyLexemes::NonCurly(current_string));
    }
    curly_expect!(explicit !last_backslash, "an escaped character, found end of input", row+last_row, col+last_col);
    Ok(lexemes)
}

pub(crate) fn parser(
    input: HashMap<usize, Vec<CurlyLexemes>>,
    base_row: Option<usize>,
    base_col: Option<usize>,
) -> Result<Vec<CurlyAST>, CurlyErrorKind> {
    let mut ast: Vec<CurlyAST> = Vec::new();
    let base_row = base_row.unwrap_or(0);
    let base_col = base_col.unwrap_or(0);
    let mut cursor = 0usize;
    let mut eoi_valid = true;

    loop {
//        let lexeme = &input[cursor];
    }
    

    Ok(Vec::new())
}

/*
pub(crate) fn parse_to_nodes(
    input: &str,
    base_row: usize,
    base_col: usize,
) -> Result<Vec<SyntaxNode>, CurlyErrorKind> {
    let chars: Vec<char> = input.chars().collect();
    let mut nodes: Vec<SyntaxNode> = Vec::new();
    let mut cursor = 0;
    let mut current_braces: Option<SyntaxNode> = None;

    loop {
        let ch = chars[cursor];
        curly_expect!(explicit cursor < chars.len() && current_braces.is_some(), "a prefix, specifier, postfix, or closing curly braces, found end of string", base_row, base_col, input, cursor + 1);
        if ch == '\\' {
            curly_expect!(explicit cursor + 1 < chars.len(), "an escaped character, found nothing", base_row, base_col, input, cursor + 2);
            let next_ch = chars[cursor + 1];
            curly_expect!(explicit ESCAPEABLE_CHARACTERS.contains(&next_ch), format!("a valid escape character, found '{}'", &next_ch), base_row, base_col, input, cursor + 1);
            nodes.push(SyntaxNode::EscapedCharacter(next_ch));
            cursor += 2;
        } else if ch == '{' {
            curly_expect!(explicit current_braces.is_some(), "a prefix, specifier, postfix, or closing curly braces, found opening curly brace, which is invalid in this context", base_row, base_col, input, cursor + 3);
            if chars[cursor + 1] == '{' {
                current_braces = Some(SyntaxNode::DoubleBrace(Vec::new()));
                cursor += 2;
                continue;
            }
            cursor += 1;
        } else if ch == '}' {
            if chars[cursor + 1] == '}' {}
        }
    }

    Ok(Vec::new())
}
*/

pub(crate) fn parse_prefixes(format_segment: &str) -> Result<Vec<CurlyPreFlags>, CurlyErrorKind> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {

    pub use super::*;

    mod charmatrix {

        pub use super::*;

        #[test]
        fn new() {
            let from_string = "Line 1\nLine 2";
            let mut rows: HashMap<usize, String> = HashMap::new();
            rows.insert(0, "Line 1".to_string());
            rows.insert(1, "Line 2".to_string());
            let expected_matrix = CharMatrix { rows };
            let found_matrix = CharMatrix::new(from_string);
            assert_eq!(expected_matrix, found_matrix);
        }

        #[test]
        fn new_endingnewline() {
            let from_string = "Line 1\nLine 2\n";
            let mut rows: HashMap<usize, String> = HashMap::new();
            rows.insert(0, "Line 1".to_string());
            rows.insert(1, "Line 2".to_string());
            let expected_matrix = CharMatrix { rows };
            let found_matrix = CharMatrix::new(from_string);
            assert_eq!(expected_matrix, found_matrix);
        }

        #[test]
        fn get_row() {
            let matrix = CharMatrix::new("Line 1\nLine 2\n");
            let expected_row = "Line 2".to_string();
            let found_row = matrix.get_row(1).unwrap();
            assert_eq!(expected_row, *found_row);
        }

        #[test]
        fn get_char() {
            let matrix = CharMatrix::new("Line 1\nLine 2\n");
            let expected_ch = '2';
            let found_ch = matrix.get_char(1, 5).unwrap();
            assert_eq!(expected_ch, found_ch);
        }
    }

    mod lexer {

        pub use super::*;

        #[test]
        fn basic_lexing() {
            let from = "String\n {{abc:spec/post}}";
            let expected = vec![
                CurlyLexemes::NonCurly("String\n ".to_string()),
                CurlyLexemes::OpenBrace,
                CurlyLexemes::OpenBrace,
                CurlyLexemes::NonCurly("abc".to_string()),
                CurlyLexemes::Special(CurlySpecials::EndPrefixes),
                CurlyLexemes::NonCurly("spec".to_string()),
                CurlyLexemes::Special(CurlySpecials::BeginPostfixes),
                CurlyLexemes::NonCurly("post".to_string()),
                CurlyLexemes::CloseBrace,
                CurlyLexemes::CloseBrace,
            ];

            let mat = CharMatrix::new(from);
            let found = lexer(&mat, None, None).unwrap();
            assert_eq!(expected, found);
        }

        #[test]
        fn backslash_eoi() {
            let from = "\\";
            let mat = CharMatrix::new(from);
            lexer(&mat, None, None).unwrap_err();
        }

        #[test]
        fn deterministic_row() {
            for _i in 0..25 {
                // As basic_lexing() is where I found this problem, this test is simply a repeated version of it,
                // and instead of copying code, we'll just call it here.
                basic_lexing();
            }
        }
    }
}
