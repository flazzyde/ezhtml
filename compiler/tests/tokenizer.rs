//! Unit tests for the tokenizer.

use ezhtml::tokenizer::tokenize;

#[test]
fn tokenizes_simple_text() {
    let toks = tokenize("title \"Hello\"");
    // Expect: Keyword("title"), String("Hello"), Newline, Eof.
    assert!(toks.iter().any(|t| matches!(&t.token, ezhtml::token::Token::Keyword(_))));
    assert!(toks.iter().any(|t| matches!(&t.token, ezhtml::token::Token::String(s) if s == "Hello")));
}

#[test]
fn emits_indent_and_dedent() {
    let toks = tokenize("section\n    title \"A\"\n");
    let has_indent = toks.iter().any(|t| matches!(t.token, ezhtml::token::Token::Indent));
    let has_dedent = toks.iter().any(|t| matches!(t.token, ezhtml::token::Token::Dedent));
    assert!(has_indent, "expected an INDENT token");
    assert!(has_dedent, "expected a DEDENT token");
}

#[test]
fn comments_are_ignored() {
    let toks = tokenize("# this is a comment\ntitle \"X\"");
    // The first non-newline token should be a keyword (title).
    let first = toks
        .iter()
        .find(|t| !matches!(t.token, ezhtml::token::Token::Newline))
        .unwrap();
    assert!(matches!(first.token, ezhtml::token::Token::Keyword(_)));
}

#[test]
fn handles_multiple_indent_levels() {
    let src = "section\n    row\n        card\n            text \"Hello\"\n";
    let toks = tokenize(src);
    let indents = toks
        .iter()
        .filter(|t| matches!(t.token, ezhtml::token::Token::Indent))
        .count();
    let dedents = toks
        .iter()
        .filter(|t| matches!(t.token, ezhtml::token::Token::Dedent))
        .count();
    assert_eq!(indents, 3);
    assert_eq!(dedents, 3);
}

#[test]
fn directive_is_captured() {
    let toks = tokenize("!lang de\ntitle \"Hallo\"");
    assert!(toks
        .iter()
        .any(|t| matches!(&t.token, ezhtml::token::Token::Directive(s) if s.starts_with("!lang"))));
}

#[test]
fn escapes_quote_inside_string() {
    let toks = tokenize("text \"He said \\\"hi\\\"\"");
    assert!(toks
        .iter()
        .any(|t| matches!(&t.token, ezhtml::token::Token::String(s) if s.contains("\"hi\""))));
}
