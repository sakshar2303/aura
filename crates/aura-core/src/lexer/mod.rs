//! # Aura Lexer
//!
//! Tokenizes `.aura` source files into a stream of tokens.
//! Uses the `logos` crate for high-performance lexing.
//!
//! ## Design
//! - Indentation-significant: tracks indent/dedent as tokens
//! - UTF-8 only (rejects other encodings)
//! - Max file size: 10 MB
//! - Tabs produce errors (2-space indent only)

mod tokens;

pub use tokens::Token;

use logos::Logos;

/// Maximum source file size in bytes (10 MB).
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// The indent unit in spaces.
pub const INDENT_UNIT: usize = 2;

/// Result of lexing a source file.
pub struct LexResult {
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<crate::errors::AuraError>,
}

/// A token with its span in the source.
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

/// A source span (byte offsets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Lex a source string into tokens.
///
/// Returns all tokens and any lexer errors encountered.
/// The lexer does NOT stop at the first error — it collects all errors
/// and produces as many valid tokens as possible.
pub fn lex(source: &str) -> LexResult {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    // Check file size
    if source.len() > MAX_FILE_SIZE {
        errors.push(crate::errors::AuraError::new(
            crate::errors::ErrorCode::E0002,
            crate::errors::Severity::Error,
            format!(
                "File exceeds maximum size of {} MB ({} bytes)",
                MAX_FILE_SIZE / (1024 * 1024),
                source.len()
            ),
            Span::new(0, 0),
        ));
        return LexResult { tokens, errors };
    }

    // Check for tabs
    for (i, line) in source.lines().enumerate() {
        if let Some(pos) = line.find('\t') {
            let byte_offset = source.lines().take(i).map(|l| l.len() + 1).sum::<usize>() + pos;
            errors.push(crate::errors::AuraError::new(
                crate::errors::ErrorCode::E0010,
                crate::errors::Severity::Error,
                "Tab character found. Aura uses 2-space indentation.".to_string(),
                Span::new(byte_offset, byte_offset + 1),
            ));
        }
    }

    // Tokenize with logos
    let mut lexer = Token::lexer(source);
    while let Some(result) = lexer.next() {
        let span = Span::new(lexer.span().start, lexer.span().end);
        match result {
            Ok(token) => {
                tokens.push(Spanned { value: token, span });
            }
            Err(()) => {
                errors.push(crate::errors::AuraError::new(
                    crate::errors::ErrorCode::E0030,
                    crate::errors::Severity::Error,
                    format!("Unexpected character: {:?}", &source[span.start..span.end]),
                    span,
                ));
            }
        }
    }

    // Process indentation: convert leading spaces into Indent/Dedent tokens
    let indented = process_indentation(source, &tokens, &mut errors);

    LexResult {
        tokens: indented,
        errors,
    }
}

/// Process indentation levels and insert Indent/Dedent tokens.
fn process_indentation(
    source: &str,
    tokens: &[Spanned<Token>],
    errors: &mut Vec<crate::errors::AuraError>,
) -> Vec<Spanned<Token>> {
    let mut result: Vec<Spanned<Token>> = Vec::new();
    let mut indent_stack: Vec<usize> = vec![0];
    let mut line_start = true;
    let mut current_indent = 0;

    for line in source.lines() {
        let stripped = line.trim_start_matches(' ');
        if stripped.is_empty() || stripped.starts_with("//") {
            continue; // Skip blank lines and comment-only lines
        }

        let spaces = line.len() - stripped.len();

        if spaces % INDENT_UNIT != 0 {
            errors.push(crate::errors::AuraError::new(
                crate::errors::ErrorCode::E0011,
                crate::errors::Severity::Error,
                format!(
                    "Inconsistent indentation: found {} spaces (must be a multiple of {})",
                    spaces, INDENT_UNIT
                ),
                Span::new(0, spaces),
            ));
        }

        let level = spaces / INDENT_UNIT;

        if level > *indent_stack.last().unwrap() {
            indent_stack.push(level);
        } else {
            while indent_stack.len() > 1 && *indent_stack.last().unwrap() > level {
                indent_stack.pop();
            }
        }
    }

    // For the initial skeleton, return tokens as-is
    // Full indentation processing will be implemented with the parser
    tokens.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_empty_source() {
        let result = lex("");
        assert!(result.tokens.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_lex_rejects_oversized_file() {
        let source = "a".repeat(MAX_FILE_SIZE + 1);
        let result = lex(&source);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, crate::errors::ErrorCode::E0002);
    }

    #[test]
    fn test_lex_rejects_tabs() {
        let result = lex("\tapp Hello");
        assert!(result.errors.iter().any(|e| e.code == crate::errors::ErrorCode::E0010));
    }

    #[test]
    fn test_lex_simple_tokens() {
        let result = lex("app Hello");
        assert!(!result.tokens.is_empty());
        assert!(result.errors.is_empty());
    }
}
