//! Tokenizer (lexer) for `.ezhtml` source files.
//!
//! The tokenizer produces INDENT/DEDENT tokens to make the indentation
//! grammar explicit. Inspired by Python's `tokenize` module but tailored
//! for EZHTML's keyword-driven syntax.

use crate::error::{CompileError, SourceSpan};
use crate::token::{Keyword, SpannedToken, Token};

/// Tokenise a full EZHTML source string.
///
/// Errors are returned as a single `CompileError::Lex` with the offending
/// span. The tokenizer is forgiving – unknown characters are emitted as
/// `Bare` tokens rather than aborting the whole stream.
pub fn tokenize(source: &str) -> Vec<SpannedToken> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut col = 1usize;
    let mut at_line_start = true;
    let mut indent_stack: Vec<usize> = vec![0];

    while i < bytes.len() {
        let c = bytes[i];

        // ---- Whitespace at the start of a line → indentation ----
        if at_line_start {
            let mut indent = 0usize;
            while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
                // Count tabs as 4 spaces, the source of all alignment evils.
                indent += if bytes[i] == b'\t' { 4 } else { 1 };
                i += 1;
                col += 1;
            }
            at_line_start = false;

            // Skip blank lines and comment lines entirely (they don't
            // affect the indent stack).
            if i < bytes.len() && (bytes[i] == b'\n' || bytes[i] == b'#') {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'\n' {
                    i += 1;
                    line += 1;
                    col = 1;
                    at_line_start = true;
                }
                continue;
            }

            // EOF – close all open indents.
            if i >= bytes.len() {
                while indent_stack.len() > 1 {
                    indent_stack.pop();
                    tokens.push(SpannedToken::new(
                        Token::Dedent,
                        SourceSpan::point(line, col),
                    ));
                }
                tokens.push(SpannedToken::new(Token::Eof, SourceSpan::point(line, col)));
                return tokens;
            }

            // Compare with the top of the indent stack.
            let top = *indent_stack.last().unwrap_or(&0);
            if indent > top {
                // We require indent to be a multiple of 2 (or 4). Pick up
                // the unit from the first non-zero indentation in the file.
                let unit = if top == 0 {
                    indent
                } else {
                    indent - top
                };
                if top == 0 {
                    indent_stack.push(indent);
                    tokens.push(SpannedToken::new(
                        Token::Indent,
                        SourceSpan::point(line, col),
                    ));
                } else if unit == 2 || unit == 4 {
                    indent_stack.push(indent);
                    tokens.push(SpannedToken::new(
                        Token::Indent,
                        SourceSpan::point(line, col),
                    ));
                } else {
                    tokens.push(SpannedToken::new(
                        Token::Bare(format!("<<INDENT-ERROR:{}>>", indent)),
                        SourceSpan::point(line, col),
                    ));
                }
            } else if indent < top {
                while indent_stack.len() > 1 && *indent_stack.last().unwrap() > indent {
                    indent_stack.pop();
                    tokens.push(SpannedToken::new(
                        Token::Dedent,
                        SourceSpan::point(line, col),
                    ));
                }
                if *indent_stack.last().unwrap_or(&0) != indent {
                    // Don't fail loudly – emit a marker and keep going so
                    // the user still gets *some* HTML output.
                    tokens.push(SpannedToken::new(
                        Token::Bare(format!("<<INDENT-MISMATCH:{}>>", indent)),
                        SourceSpan::point(line, col),
                    ));
                }
            }
        }

        // ---- Skip inner whitespace (between tokens on the same line) ----
        if i < bytes.len() && bytes[i] == b' ' {
            i += 1;
            col += 1;
            continue;
        }
        if i < bytes.len() && bytes[i] == b'\t' {
            i += 1;
            col += 1;
            continue;
        }

        // ---- Directives: `!lang de`, `!doctype html5` ----
        if c == b'!' {
            let start = i;
            let start_col = col;
            i += 1;
            col += 1;
            while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'#' {
                i += 1;
                col += 1;
            }
            let directive = std::str::from_utf8(&bytes[start..i])
                .unwrap_or("")
                .trim()
                .to_string();
            tokens.push(SpannedToken::new(
                Token::Directive(directive),
                SourceSpan::range(line, start_col, start, i - start + 1),
            ));
            continue;
        }

        // ---- Newline ----
        if c == b'\n' {
            i += 1;
            line += 1;
            col = 1;
            at_line_start = true;
            tokens.push(SpannedToken::new(Token::Newline, SourceSpan::point(line, col)));
            continue;
        }

        // ---- Quoted string ----
        if c == b'"' || c == b'\'' {
            let quote = c;
            let start = i;
            let start_col = col;
            i += 1;
            col += 1;
            let mut buf = String::new();
            while i < bytes.len() && bytes[i] != quote {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    let esc = bytes[i + 1];
                    buf.push(match esc {
                        b'n' => '\n',
                        b't' => '\t',
                        b'r' => '\r',
                        b'\\' => '\\',
                        b'"' => '"',
                        b'\'' => '\'',
                        _ => esc as char,
                    });
                    i += 2;
                    col += 2;
                    continue;
                }
                if bytes[i] == b'\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
                buf.push(bytes[i] as char);
                i += 1;
            }
            if i >= bytes.len() {
                // Unterminated string – emit what we have but mark it.
                tokens.push(SpannedToken::new(
                    Token::Bare(format!("<<UNTERMINATED-STRING:{}>>", buf)),
                    SourceSpan::range(line, start_col, start, i - start),
                ));
            } else {
                i += 1; // closing quote
                col += 1;
                tokens.push(SpannedToken::new(
                    Token::String(buf),
                    SourceSpan::range(line, start_col, start, i - start),
                ));
            }
            continue;
        }

        // ---- Hash comment to end of line ----
        if c == b'#' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        // ---- Bare word: keyword or identifier ----
        let start = i;
        let start_col = col;
        while i < bytes.len()
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'"'
            && bytes[i] != b'\''
            && bytes[i] != b'#'
        {
            i += 1;
            col += 1;
        }
        let word = std::str::from_utf8(&bytes[start..i])
            .unwrap_or("")
            .to_string();
        if word.is_empty() {
            // Unknown character – skip it gracefully.
            i += 1;
            col += 1;
            continue;
        }
        let token = if let Some(kw) = Keyword::from_str(&word) {
            Token::Keyword(kw)
        } else {
            Token::Identifier(word)
        };
        tokens.push(SpannedToken::new(token, SourceSpan::range(line, start_col, start, word.len())));
    }

    // Final close-out.
    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(SpannedToken::new(Token::Dedent, SourceSpan::point(line, col)));
    }
    tokens.push(SpannedToken::new(Token::Eof, SourceSpan::point(line, col)));
    tokens
}

/// Helper used by CLI for clearer error messages.
pub fn tokenize_strict(source: &str) -> Result<Vec<SpannedToken>, CompileError> {
    let tokens = tokenize(source);
    for t in &tokens {
        if let Token::Bare(s) = &t.token {
            if s.contains("<<") && s.contains(">>") {
                return Err(CompileError::Lex {
                    message: s.clone(),
                    span: t.span,
                });
            }
        }
    }
    Ok(tokens)
}
