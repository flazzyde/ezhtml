//! Tokens produced by the [`tokenizer`].
//!
//! The tokenizer is responsible for turning raw characters into a stream
//! of [`Token`]s that the [`parser`] then consumes.

use crate::error::SourceSpan;

/// A token in the EZHTML source.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A keyword that opens a block, e.g. `title`, `button`, `section`.
    Keyword(Keyword),

    /// An identifier that's not a known keyword. Treated as `html` content
    /// by the parser (caller decides).
    Identifier(String),

    /// A quoted string literal.
    String(String),

    /// An unquoted bare string (until whitespace).
    Bare(String),

    /// Indentation increased (relative to the previous line).
    Indent,

    /// Indentation decreased (relative to the previous line).
    Dedent,

    /// Newline (end of line).
    Newline,

    /// A `!` directive, e.g. `!doctype html5` or `!lang de`.
    Directive(String),

    /// End of input.
    Eof,
}

/// Built-in EZHTML keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    /// `<title>` – page title.
    Title,
    /// `<h2>` – sub title.
    Subtitle,
    /// `<p>` – paragraph.
    Text,
    /// `<a class="btn">` – styled link button.
    Button,
    /// `<img>`.
    Image,
    /// `<video>`.
    Video,
    /// `<a>`.
    Link,
    /// `<header>`.
    Header,
    /// `<footer>`.
    Footer,
    /// `<nav>`.
    Navbar,
    /// `<section>`.
    Section,
    /// `<div class="container">`.
    Container,
    /// `<div class="row">` – flex/grid wrapper.
    Row,
    /// `<div class="col">`.
    Column,
    /// `<article class="card">`.
    Card,
    /// `<ul>`.
    List,
    /// `<li>`.
    Item,
    /// `<table>` – followed by `headers` and `rows` blocks.
    Table,
    /// `<tr>` – table row literal.
    Row_,
    /// `<input type="...">`.
    Input,
    /// `<input type="email">`.
    Email,
    /// `<input type="password">`.
    Password,
    /// `<input type="checkbox">`.
    Checkbox,
    /// `<input type="radio">`.
    Radio,
    /// `<textarea>`.
    Textarea,
    /// `<pre><code>`.
    Code,
    /// `<blockquote>`.
    Quote,
    /// `<hr>`.
    Divider,
    /// Empty paragraph for spacing.
    Space,
    /// `<i class="icon icon-…">` placeholder.
    Icon,
    /// Raw HTML pass-through.
    Html,
    /// `headers` – sub-block of `table`.
    Headers,
    /// `rows` – sub-block of `table`.
    Rows,
    /// `cell` – table cell.
    Cell,
}

impl Keyword {
    /// Convert a string to a [`Keyword`] if it matches.
    pub fn from_str(s: &str) -> Option<Self> {
        use Keyword::*;
        Some(match s {
            "title" => Title,
            "subtitle" => Subtitle,
            "text" => Text,
            "button" => Button,
            "image" => Image,
            "video" => Video,
            "link" => Link,
            "header" => Header,
            "footer" => Footer,
            "navbar" => Navbar,
            "section" => Section,
            "container" => Container,
            "row" => Row,
            "column" => Column,
            "card" => Card,
            "list" => List,
            "item" => Item,
            "table" => Table,
            "row_" => Row_,
            "input" => Input,
            "email" => Email,
            "password" => Password,
            "checkbox" => Checkbox,
            "radio" => Radio,
            "textarea" => Textarea,
            "code" => Code,
            "quote" => Quote,
            "divider" => Divider,
            "space" => Space,
            "icon" => Icon,
            "html" => Html,
            "headers" => Headers,
            "rows" => Rows,
            "cell" => Cell,
            _ => return None,
        })
    }

    /// Canonical HTML tag name this keyword emits.
    pub fn html_tag(&self) -> &'static str {
        use Keyword::*;
        match self {
            Title => "title",
            Subtitle => "h2",
            Text => "p",
            Button => "a",
            Image => "img",
            Video => "video",
            Link => "a",
            Header => "header",
            Footer => "footer",
            Navbar => "nav",
            Section => "section",
            Container => "div",
            Row => "div",
            Column => "div",
            Card => "article",
            List => "ul",
            Item => "li",
            Table => "table",
            Row_ => "tr",
            Input | Email | Password | Checkbox | Radio => "input",
            Textarea => "textarea",
            Code => "pre",
            Quote => "blockquote",
            Divider => "hr",
            Space => "p",
            Icon => "i",
            Html => "div",
            Headers => "thead",
            Rows => "tbody",
            Cell => "td",
        }
    }
}

/// A token plus its location in the source.
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    /// The token.
    pub token: Token,
    /// Where the token was found.
    pub span: SourceSpan,
}

impl SpannedToken {
    /// Construct a new spanned token.
    pub fn new(token: Token, span: SourceSpan) -> Self {
        Self { token, span }
    }
}
