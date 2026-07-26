//! AST (Abstract Syntax Tree) produced by the [`parser`].
//!
//! Every element of the EZHTML language is represented as a [`Node`].

use crate::error::SourceSpan;
use crate::token::Keyword;

/// A parsed EZHTML document, modelled as a tree of [`Node`]s.
#[derive(Debug, Clone, Default)]
pub struct Document {
    /// Top-level nodes of the document.
    pub nodes: Vec<Node>,
}

impl Document {
    /// Create an empty document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a node.
    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

/// A single node in the AST.
#[derive(Debug, Clone)]
pub struct Node {
    /// What kind of node this is.
    pub kind: NodeKind,
    /// Where this node came from.
    pub span: SourceSpan,
}

/// The kind of a node – everything the emitter knows how to render.
#[derive(Debug, Clone)]
pub enum NodeKind {
    // ----- Head elements -----
    /// Page title (also `<html lang>` shortcut).
    Title(String),
    /// Sub-title (`<h2>`).
    Subtitle(String),
    /// `<p>` text.
    Text(String),

    // ----- Media / Links -----
    /// `<a class="btn">`
    Button {
        /// Visible label.
        label: String,
        /// Target URL.
        href: String,
    },
    /// `<img>` element.
    Image {
        /// Image source URL.
        src: String,
        /// Alt text (required).
        alt: String,
    },
    /// `<video src="…">`.
    Video(String),
    /// `<a href="…">`.
    Link {
        /// Target URL.
        href: String,
        /// Inner content.
        children: Vec<Node>,
    },

    // ----- Containers -----
    /// `<header>`.
    Header(Vec<Node>),
    /// `<footer>`.
    Footer(Vec<Node>),
    /// `<nav>`.
    Navbar(Vec<Node>),
    /// `<section>`.
    Section(Vec<Node>),
    /// `<div class="container">`.
    Container(Vec<Node>),
    /// `<div class="row">`.
    Row(Vec<Node>),
    /// `<div class="col">`.
    Column(Vec<Node>),
    /// `<article class="card">`.
    Card(Vec<Node>),

    // ----- Lists -----
    /// `<ul>` – empty by design, contains Items.
    List(Vec<Node>),
    /// `<li>`.
    Item(Vec<Node>),

    // ----- Tables -----
    /// `<table>` with explicit headers + rows.
    Table {
        /// Column headers.
        headers: Vec<String>,
        /// Data rows.
        rows: Vec<Vec<String>>,
    },

    // ----- Forms -----
    /// `<input type="text">`.
    Input {
        /// Input name.
        name: String,
        /// Placeholder text.
        placeholder: Option<String>,
    },
    /// `<input type="email">`.
    Email {
        /// Input name.
        name: String,
        /// Placeholder text.
        placeholder: Option<String>,
    },
    /// `<input type="password">`.
    Password {
        /// Input name.
        name: String,
        /// Placeholder text.
        placeholder: Option<String>,
    },
    /// `<input type="checkbox">`.
    Checkbox {
        /// Input name.
        name: String,
        /// Visible label.
        label: String,
    },
    /// `<input type="radio">` (group).
    Radio {
        /// Group name.
        name: String,
        /// Selected value.
        value: String,
        /// Visible label.
        label: String,
    },
    /// `<textarea>`.
    Textarea {
        /// Textarea name.
        name: String,
        /// Placeholder text.
        placeholder: Option<String>,
    },

    // ----- Misc -----
    /// `<pre><code>`.
    Code {
        /// Source code.
        content: String,
        /// Optional language hint.
        language: Option<String>,
    },
    /// `<blockquote>`.
    Quote {
        /// Quote body.
        text: String,
        /// Optional attribution / source URL.
        cite: Option<String>,
    },
    /// `<hr>`.
    Divider,
    /// Empty paragraph for vertical whitespace.
    Space,
    /// Placeholder for an icon font glyph.
    Icon(String),
    /// Raw HTML pass-through (escaped *only* on the dispatcher side).
    Html(String),
}

impl Node {
    /// Create a new node at the given span.
    pub fn new(kind: NodeKind, span: SourceSpan) -> Self {
        Self { kind, span }
    }

    /// The trimmed textual content of this node, if any.
    pub fn text(&self) -> Option<&str> {
        match &self.kind {
            NodeKind::Title(s)
            | NodeKind::Subtitle(s)
            | NodeKind::Text(s)
            | NodeKind::Video(s)
            | NodeKind::Quote { text: s, .. }
            | NodeKind::Icon(s)
            | NodeKind::Html(s) => Some(s),
            _ => None,
        }
    }

    /// Does this node allow textual-only content (used by heuristics)?
    pub fn is_inline(&self) -> bool {
        matches!(
            self.kind,
            NodeKind::Title(_)
                | NodeKind::Subtitle(_)
                | NodeKind::Text(_)
                | NodeKind::Code { .. }
                | NodeKind::Quote { .. }
                | NodeKind::Icon(_)
                | NodeKind::Html(_)
                | NodeKind::Button { .. }
                | NodeKind::Link { .. }
        )
    }

    /// What keyword opened this node.
    pub fn keyword(&self) -> Keyword {
        match &self.kind {
            NodeKind::Title(_) => Keyword::Title,
            NodeKind::Subtitle(_) => Keyword::Subtitle,
            NodeKind::Text(_) => Keyword::Text,
            NodeKind::Button { .. } => Keyword::Button,
            NodeKind::Image { .. } => Keyword::Image,
            NodeKind::Video(_) => Keyword::Video,
            NodeKind::Link { .. } => Keyword::Link,
            NodeKind::Header(_) => Keyword::Header,
            NodeKind::Footer(_) => Keyword::Footer,
            NodeKind::Navbar(_) => Keyword::Navbar,
            NodeKind::Section(_) => Keyword::Section,
            NodeKind::Container(_) => Keyword::Container,
            NodeKind::Row(_) => Keyword::Row,
            NodeKind::Column(_) => Keyword::Column,
            NodeKind::Card(_) => Keyword::Card,
            NodeKind::List(_) => Keyword::List,
            NodeKind::Item(_) => Keyword::Item,
            NodeKind::Table { .. } => Keyword::Table,
            NodeKind::Input { .. } => Keyword::Input,
            NodeKind::Email { .. } => Keyword::Email,
            NodeKind::Password { .. } => Keyword::Password,
            NodeKind::Checkbox { .. } => Keyword::Checkbox,
            NodeKind::Radio { .. } => Keyword::Radio,
            NodeKind::Textarea { .. } => Keyword::Textarea,
            NodeKind::Code { .. } => Keyword::Code,
            NodeKind::Quote { .. } => Keyword::Quote,
            NodeKind::Divider => Keyword::Divider,
            NodeKind::Space => Keyword::Space,
            NodeKind::Icon(_) => Keyword::Icon,
            NodeKind::Html(_) => Keyword::Html,
        }
    }
}
