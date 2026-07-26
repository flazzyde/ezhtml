//! AST & metadata tests.

use ezhtml::*;

#[test]
fn text_node_has_inline_kind() {
    let doc = parser::parse(tokenizer::tokenize("text \"Hello\"")).unwrap();
    assert!(doc.nodes[0].is_inline());
}

#[test]
fn link_with_children_parse() {
    let src = "link \"https://example.com\"\n    text \"Click me\"";
    let doc = parser::parse(tokenizer::tokenize(src)).unwrap();
    match &doc.nodes[0].kind {
        NodeKind::Link { href, children } => {
            assert_eq!(href, "https://example.com");
            assert_eq!(children.len(), 1);
        }
        _ => panic!("expected a link node"),
    }
}

#[test]
fn table_with_rows_keeps_columns() {
    let src = "table\n    headers \"A\", \"B\"\n    rows\n        row_ \"1\", \"2\"\n        row_ \"3\", \"4\"\n";
    let doc = parser::parse(tokenizer::tokenize(src)).unwrap();
    match &doc.nodes[0].kind {
        NodeKind::Table { headers, rows } => {
            assert_eq!(headers, &vec!["A".to_string(), "B".to_string()]);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0], vec!["1".to_string(), "2".to_string()]);
        }
        _ => panic!("expected table node"),
    }
}
