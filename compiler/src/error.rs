//! Error types, spans and aggregated diagnostic reports.

use std::fmt;
use thiserror::Error;

/// A position inside a source file – `line` is 1-based, `column` is 1-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceSpan {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
    /// Byte offset within the source.
    pub offset: usize,
    /// Length of the offending region in bytes.
    pub length: usize,
}

impl SourceSpan {
    /// Construct a span that points at a single character.
    pub fn point(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            offset: 0,
            length: 1,
        }
    }

    /// Construct a span covering a byte range.
    pub fn range(line: usize, column: usize, offset: usize, length: usize) -> Self {
        Self {
            line,
            column,
            offset,
            length,
        }
    }
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Information – useful but not a problem.
    Info,
    /// Warning – possibly incorrect, but compilation continues.
    Warning,
    /// Error – compilation must fail.
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "info"),
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
        }
    }
}

/// A single diagnostic produced by the compiler.
#[derive(Debug, Clone, Error)]
#[error("{severity}: {message} ({span})")]
pub struct Diagnostic {
    /// Severity of this diagnostic.
    pub severity: Severity,
    /// Human-readable message.
    pub message: String,
    /// Position in the source file.
    pub span: SourceSpan,
    /// Optional rule code, e.g. `"E0301"` or `"W0102"`.
    pub code: Option<String>,
}

/// All errors that can be returned by the compiler pipeline.
#[derive(Debug, Clone, Error)]
pub enum CompileError {
    /// Unexpected end-of-file while parsing.
    #[error("unexpected end of file: {message}")]
    UnexpectedEof {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// Unknown element name.
    #[error("unknown element `{name}`")]
    UnknownElement {
        /// The element name that was not recognised.
        name: String,
        /// Position.
        span: SourceSpan,
    },

    /// Wrong number of arguments for an element.
    #[error("element `{name}` expected {expected} argument(s), got {actual}")]
    WrongArgumentCount {
        /// Element name.
        name: String,
        /// Expected count.
        expected: usize,
        /// Actual count.
        actual: usize,
        /// Position.
        span: SourceSpan,
    },

    /// A required argument was missing.
    #[error("missing required argument for `{name}`")]
    MissingArgument {
        /// Element name.
        name: String,
        /// Position.
        span: SourceSpan,
    },

    /// Lexical / tokeniser error.
    #[error("lex error: {message}")]
    Lex {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// Parsing error.
    #[error("parse error: {message}")]
    Parse {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// Invalid indentation.
    #[error("invalid indentation: {message}")]
    Indentation {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// I/O error (e.g. file not found).
    #[error("I/O error: {message}")]
    Io {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// Invalid UTF-8.
    #[error("invalid UTF-8: {message}")]
    Encoding {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },

    /// Validation issue caught by the validator pass.
    #[error("validation issue: {message}")]
    Validation {
        /// Explanation.
        message: String,
        /// Position.
        span: SourceSpan,
    },
}

impl CompileError {
    /// Convert this [`CompileError`] into a [`Diagnostic`].
    pub fn to_diagnostic(&self) -> Diagnostic {
        let span = match self {
            CompileError::UnexpectedEof { span, .. }
            | CompileError::UnknownElement { span, .. }
            | CompileError::WrongArgumentCount { span, .. }
            | CompileError::MissingArgument { span, .. }
            | CompileError::Lex { span, .. }
            | CompileError::Parse { span, .. }
            | CompileError::Indentation { span, .. }
            | CompileError::Io { span, .. }
            | CompileError::Encoding { span, .. }
            | CompileError::Validation { span, .. } => *span,
        };
        let severity = match self {
            CompileError::Validation { .. } => Severity::Warning,
            _ => Severity::Error,
        };
        Diagnostic {
            severity,
            message: self.to_string(),
            span,
            code: None,
        }
    }
}

/// Aggregated diagnostic report returned by the compiler.
#[derive(Debug, Clone, Default)]
pub struct CompileReport {
    /// Collected diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

impl CompileReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a diagnostic.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Push an error converted from a [`CompileError`].
    pub fn push_error(&mut self, error: CompileError) {
        self.diagnostics.push(error.to_diagnostic());
    }

    /// Returns `true` if any error is present.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    /// Returns `true` if any warnings are present.
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    }

    /// Pretty-print the report to a buffer.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for d in &self.diagnostics {
            out.push_str(&format!(
                "{} [{}] {}\n",
                d.severity,
                d.span,
                d.message
            ));
        }
        if self.diagnostics.is_empty() {
            out.push_str("No issues.\n");
        }
        out
    }
}

impl fmt::Display for CompileReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

impl From<CompileError> for CompileReport {
    fn from(error: CompileError) -> Self {
        let mut r = Self::new();
        r.push_error(error);
        r
    }
}

impl std::error::Error for CompileReport {}

/// Result alias used throughout the compiler pipeline.
pub type CompileResult<T> = Result<T, CompileError>;
