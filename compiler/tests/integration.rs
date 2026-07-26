//! End-to-end integration tests for the EZHTML compiler.

use ezhtml::{compile, CompileOptions};

fn opts() -> CompileOptions {
    CompileOptions::default()
}

#[test]
fn empty_document_renders_scaffold() {
    let html = compile("", &opts()).unwrap();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<html lang=\"en\">"));
    assert!(html.contains("<head>"));
    assert!(html.contains("<body>"));
}

#[test]
fn title_becomes_h1() {
    let html = compile("title \"Hello\"", &opts()).unwrap();
    assert!(html.contains("<h1 class=\"title\">Hello</h1>"));
}

#[test]
fn subtitle_is_h2() {
    let html = compile("subtitle \"World\"", &opts()).unwrap();
    assert!(html.contains("<h2>World</h2>"));
}

#[test]
fn text_is_paragraph() {
    let html = compile("text \"A paragraph\"", &opts()).unwrap();
    assert!(html.contains("<p>A paragraph</p>"));
}

#[test]
fn button_has_label_and_href() {
    let html = compile("button \"Go\" \"https://example.com\"", &opts()).unwrap();
    assert!(html.contains("class=\"btn btn-primary\""));
    assert!(html.contains("href=\"https://example.com\""));
    assert!(html.contains("Go"));
}

#[test]
fn image_emits_alt() {
    let html = compile("image \"./logo.png\" \"Company logo\"", &opts()).unwrap();
    assert!(html.contains("src=\"./logo.png\""));
    assert!(html.contains("alt=\"Company logo\""));
}

#[test]
fn section_creates_hierarchy() {
    let src = "section\n    title \"Section title\"\n    text \"Inner text\"\n";
    let html = compile(src, &opts()).unwrap();
    assert!(html.contains("<section>"));
    assert!(html.contains("<h1 class=\"title\">Section title</h1>"));
    assert!(html.contains("<p>Inner text</p>"));
    assert!(html.contains("</section>"));
}

#[test]
fn list_wraps_items() {
    let src = "list\n    item\n        text \"One\"\n    item\n        text \"Two\"\n";
    let html = compile(src, &opts()).unwrap();
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
    assert!(html.contains("Two"));
}

#[test]
fn table_emits_thead_and_tbody() {
    let src = "table\n    headers \"Name\", \"Age\"\n    rows\n        row_ \"Alice\", \"30\"\n        row_ \"Bob\", \"42\"\n";
    let html = compile(src, &opts()).unwrap();
    assert!(html.contains("<table>"));
    assert!(html.contains("<thead>"));
    assert!(html.contains("<th>Name</th>"));
    assert!(html.contains("<th>Age</th>"));
    assert!(html.contains("<tbody>"));
    assert!(html.contains("Alice"));
    assert!(html.contains("Bob"));
}

#[test]
fn html_special_chars_are_escaped() {
    let html = compile("text \"<script>alert('xss')</script>\"", &opts()).unwrap();
    assert!(!html.contains("<script>"));
    assert!(html.contains("&lt;script&gt;"));
}

#[test]
fn metadata_overrides_title() {
    let mut opts2 = opts();
    opts2.metadata.title = "Custom".to_string();
    let html = compile("text \"Body only\"", &opts2).unwrap();
    assert!(html.contains("<title>Custom</title>"));
}

#[test]
fn metadata_keywords_are_emitted() {
    let mut opts2 = opts();
    opts2.metadata.keywords = vec!["rust".to_string(), "compiler".to_string()];
    let html = compile("", &opts2).unwrap();
    assert!(html.contains("name=\"keywords\""));
    assert!(html.contains("rust, compiler"));
}

#[test]
fn opengraph_is_present() {
    let html = compile("", &opts()).unwrap();
    assert!(html.contains("property=\"og:title\""));
    assert!(html.contains("property=\"og:type\""));
    assert!(html.contains("twitter:card"));
}

#[test]
fn divider_becomes_hr() {
    let html = compile("divider", &opts()).unwrap();
    assert!(html.contains("<hr>"));
}

#[test]
fn unknown_element_still_produces_html() {
    // Unknown elements are warnings – the compiler must still emit HTML for
    // the surrounding valid content.
    let html = compile("text \"ok\"\nnot_an_element \"foo\"", &opts());
    assert!(html.is_ok(), "compile should succeed despite the unknown element");
    let html = html.unwrap();
    assert!(html.contains("ok"));
}

#[test]
fn nested_row_and_card() {
    let src = "row\n    card\n        title \"Card\"\n        text \"Body\"\n";
    let html = compile(src, &opts()).unwrap();
    assert!(html.contains("<div class=\"row\">"));
    assert!(html.contains("<article class=\"card\">"));
    assert!(html.contains("Card"));
}

#[test]
fn form_input_types() {
    let src = "section\n    input \"name\"\n    email \"mail\"\n    password \"pwd\"\n    checkbox \"agree\" \"I agree\"\n    textarea \"msg\"\n";
    let html = compile(src, &opts()).unwrap();
    assert!(html.contains("type=\"text\""));
    assert!(html.contains("type=\"email\""));
    assert!(html.contains("type=\"password\""));
    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("textarea"));
}
