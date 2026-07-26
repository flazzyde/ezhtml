//! # EZHTML Compiler – Public API
//!
//! The crate exposes a single high-level entry point, [`compile`], that runs
//! the full pipeline (tokenize → parse → validate → emit) on a source string
//! and returns either HTML or a structured error report.
//!
//! ```rust
//! use ezhtml::{compile, CompileOptions};
//!
//! let source = r#"title "Hello, World""#;
//! let html = compile(source, &CompileOptions::default()).unwrap();
//! assert!(html.contains("<title>Hello, World</title>"));
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod ast;
pub mod cli;
pub mod config;
pub mod emitter;
pub mod error;
pub mod format;
pub mod parser;
pub mod project;
pub mod token;
pub mod tokenizer;
pub mod validator;

pub use crate::ast::{Node, NodeKind};
pub use crate::config::{CompileOptions, Metadata, PageSettings, SiteConfig};
pub use crate::error::{CompileError, CompileReport, Severity, SourceSpan};
pub use crate::project::ProjectFile;

/// Compile an `.ezhtml` source string into HTML5.
///
/// Returns the rendered HTML on success or a [`CompileReport`] containing all
/// collected diagnostics otherwise.
pub fn compile(source: &str, options: &CompileOptions) -> Result<String, CompileReport> {
    let tokens = tokenizer::tokenize(source);
    let ast = parser::parse(tokens)?;
    validator::validate(&ast, options);
    let html = emitter::emit(&ast, options);
    Ok(html)
}

/// Compile with diagnostics – returns both the rendered HTML and the
/// merged validation report. Parse errors are folded into the report so
/// the caller can decide whether to abort.
pub fn compile_with_report(
    source: &str,
    options: &CompileOptions,
) -> (String, CompileReport) {
    let tokens = tokenizer::tokenize(source);
    let mut report = CompileReport::new();
    let ast = match parser::parse(tokens) {
        Ok(doc) => doc,
        Err(r) => {
            report.diagnostics.extend(r.diagnostics);
            Document::default()
        }
    };
    let validation = validator::validate(&ast, options);
    report.diagnostics.extend(validation.diagnostics);
    let html = emitter::emit(&ast, options);
    (html, report)
}

/// Compile with diagnostics – returns both the rendered HTML and the
/// merged validation report. Parse errors are folded into the report so
/// the caller can decide whether to abort.
pub fn compile_with_report(
    source: &str,
    options: &CompileOptions,
) -> (String, CompileReport) {
    let tokens = tokenizer::tokenize(source);
    let mut report = CompileReport::new();
    let ast = match parser::parse(tokens) {
        Ok(doc) => doc,
        Err(r) => {
            report.diagnostics.extend(r.diagnostics);
            Document::default()
        }
    };
    let validation = validator::validate(&ast, options);
    report.diagnostics.extend(validation.diagnostics);
    let html = emitter::emit(&ast, options);
    (html, report)
}

/// Compile a file from disk, discovering adjacent `project.ez` files to
/// enrich the [`Metadata`] used for `<head>` generation.
pub fn compile_file(path: &std::path::Path) -> Result<String, CompileReport> {
    let source = std::fs::read_to_string(path).map_err(|e| {
        CompileReport::from(CompileError::Io {
            message: e.to_string(),
            span: SourceSpan::point(0, 0),
        })
    })?;
    let options = crate::project::load_options_from_dir(
        path.parent().unwrap_or_else(|| std::path::Path::new(".")),
    );
    compile(&source, &options)
}
