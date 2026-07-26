//! Validator – runs additional checks on the AST that the parser cannot
//! express.
//!
//! The validator never *rejects* the program; instead it accumulates
//! warnings into a [`CompileReport`] so authors see issues alongside any
//! error output.

use crate::ast::{Document, NodeKind};
use crate::config::CompileOptions;
use crate::error::{CompileReport, Severity, SourceSpan};

/// Validate the document and produce a [`CompileReport`].
pub fn validate(doc: &Document, options: &CompileOptions) -> CompileReport {
    let mut report = CompileReport::new();
    let mut has_title = false;

    for node in &doc.nodes {
        validate_node(node, &mut report, &mut has_title);
    }

    if !has_title {
        report.push(crate::error::Diagnostic {
            severity: Severity::Warning,
            message: "document has no `title` element – metadata fallback will be used".into(),
            span: SourceSpan::point(1, 1),
            code: Some("W0001".into()),
        });
    }

    if options.metadata.description.is_none() {
        report.push(crate::error::Diagnostic {
            severity: Severity::Info,
            message: "no `description` set – using first paragraph as fallback".into(),
            span: SourceSpan::point(1, 1),
            code: Some("I0001".into()),
        });
    }

    report
}

fn validate_node(
    node: &crate::ast::Node,
    report: &mut CompileReport,
    has_title: &mut bool,
) {
    if matches!(node.kind, NodeKind::Title(_)) {
        *has_title = true;
    }

    match &node.kind {
        NodeKind::Image { src, alt } => {
            if alt.trim().is_empty() {
                report.push(crate::error::Diagnostic {
                    severity: Severity::Warning,
                    message: format!("image `{}` is missing alt text (accessibility)", src),
                    span: node.span,
                    code: Some("W0101".into()),
                });
            }
            if src.is_empty() {
                report.push(crate::error::Diagnostic {
                    severity: Severity::Warning,
                    message: "image has empty `src` attribute".into(),
                    span: node.span,
                    code: Some("W0102".into()),
                });
            }
        }
        NodeKind::Link { href, children } => {
            if href.is_empty() {
                report.push(crate::error::Diagnostic {
                    severity: Severity::Warning,
                    message: "link has empty `href`".into(),
                    span: node.span,
                    code: Some("W0103".into()),
                });
            }
            for child in children {
                validate_node(child, report, has_title);
            }
        }
        NodeKind::Table { headers, rows } => {
            let cols = headers.len();
            for (i, row) in rows.iter().enumerate() {
                if row.len() != cols {
                    report.push(crate::error::Diagnostic {
                        severity: Severity::Warning,
                        message: format!(
                            "table row {} has {} cells, expected {}",
                            i + 1,
                            row.len(),
                            cols
                        ),
                        span: node.span,
                        code: Some("W0104".into()),
                    });
                }
            }
        }
        NodeKind::Html(raw) => {
            // Soft warning on unsanitised html passthrough.
            if raw.contains("<script") {
                report.push(crate::error::Diagnostic {
                    severity: Severity::Warning,
                    message: "raw `html` block contains <script> – prefer a plain element".into(),
                    span: node.span,
                    code: Some("W0105".into()),
                });
            }
        }
        _ => {
            // Generic child recursion.
            let children: Option<&Vec<crate::ast::Node>> = match &node.kind {
                NodeKind::Header(c) | NodeKind::Footer(c) | NodeKind::Navbar(c)
                | NodeKind::Section(c) | NodeKind::Container(c) | NodeKind::Row(c)
                | NodeKind::Column(c) | NodeKind::Card(c) | NodeKind::List(c)
                | NodeKind::Item(c) => Some(c),
                _ => None,
            };
            if let Some(c) = children {
                for child in c {
                    validate_node(child, report, has_title);
                }
            }
        }
    }
}
