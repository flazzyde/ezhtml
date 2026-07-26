//! Parser – turns tokens into an [`ast::Document`].
//!
//! The parser consumes INDENT/DEDENT tokens to build a proper tree, then
//! dispatches on each `Keyword` to create the matching AST kind.

use crate::ast::{Document, Node, NodeKind};
use crate::error::{CompileError, CompileReport};
use crate::token::{Keyword, SpannedToken, Token};

/// Parse the tokens produced by [`crate::tokenizer::tokenize`].
pub fn parse(tokens: Vec<SpannedToken>) -> Result<Document, CompileReport> {
    let mut report = CompileReport::new();
    let mut parser = ParserState {
        tokens,
        pos: 0,
        report: &mut report,
    };
    let mut doc = Document::new();

    while !parser.is_eof() {
        // Skip stray newlines / dedents at the top level.
        parser.skip_newlines();
        if parser.is_eof() {
            break;
        }
        match parser.peek().token.clone() {
            Token::Keyword(kw) => {
                parser.advance();
                match parse_keyword(&mut parser, kw) {
                    Ok(node) => doc.push(node),
                    Err(e) => parser.report.push_error(e),
                }
            }
            Token::Identifier(name) | Token::Bare(name) => {
                let span = parser.peek().span;
                parser.advance();
                parser.report.push(crate::error::Diagnostic {
                    severity: crate::error::Severity::Warning,
                    message: format!("unknown element `{}` (emitted as <div>)", name),
                    span,
                    code: Some("W0200".into()),
                });
                doc.push(Node::new(
                    NodeKind::Html(format!(
                        "<div data-unknown=\"{}\"></div>",
                        html_escape_attr(&name)
                    )),
                    span,
                ));
            }
            Token::Directive(text) => {
                parser.advance();
                doc.push(Node::new(
                    NodeKind::Html(format!("<!-- directive: {} -->", text)),
                    parser.peek().span,
                ));
            }
            Token::Eof => break,
            _ => {
                parser.advance();
            }
        }
    }

    if report.has_errors() {
        Err(report)
    } else {
        Ok(doc)
    }
}

struct ParserState<'a> {
    tokens: Vec<SpannedToken>,
    pos: usize,
    report: &'a mut CompileReport,
}

impl<'a> ParserState<'a> {
    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len().saturating_sub(1))]
    }

    fn advance(&mut self) -> SpannedToken {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek().token, Token::Eof) || self.pos >= self.tokens.len()
    }

    fn skip_newlines(&mut self) {
        while !self.is_eof() {
            match &self.peek().token {
                Token::Newline | Token::Dedent => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn expect_string(&mut self, ctx: &str) -> Option<String> {
        match &self.peek().token {
            Token::String(_) => {
                let t = self.advance();
                if let Token::String(s) = t.token {
                    Some(s)
                } else {
                    None
                }
            }
            Token::Bare(_) => {
                let t = self.advance();
                if let Token::Bare(s) = t.token {
                    Some(s)
                } else {
                    None
                }
            }
            _ => {
                self.report.push_error(CompileError::MissingArgument {
                    name: ctx.to_string(),
                    span: self.peek().span,
                });
                None
            }
        }
    }

    fn expect_keyword(&mut self, kw: Keyword, ctx: &str) -> Result<(), CompileError> {
        match &self.peek().token {
            Token::Keyword(k) if *k == kw => {
                self.advance();
                Ok(())
            }
            _ => Err(CompileError::Parse {
                message: format!("expected `{}` block in `{}`", keyword_str(kw), ctx),
                span: self.peek().span,
            }),
        }
    }

    /// Parse a list of children up to (but not consuming) a DEDENT or EOF.
    fn parse_block(&mut self) -> Result<Vec<Node>, CompileError> {
        let mut children = Vec::new();
        self.skip_newlines();
        while !self.is_eof() {
            match &self.peek().token {
                Token::Dedent | Token::Eof => break,
                Token::Newline => {
                    self.advance();
                    self.skip_newlines();
                }
                Token::Keyword(_) => {
                    let kw = if let Token::Keyword(k) = self.peek().token {
                        k
                    } else {
                        break;
                    };
                    let span = self.peek().span;
                    self.advance();
                    match parse_keyword(self, kw) {
                        Ok(node) => children.push(node),
                        Err(e) => {
                            self.report.push_error(e);
                            return Err(CompileError::Parse {
                                message: "aborting block".to_string(),
                                span,
                            });
                        }
                    }
                    self.skip_newlines();
                }
                Token::Identifier(name) => {
                    let span = self.peek().span;
                    self.advance();
                    self.report.push(crate::error::Diagnostic {
                        severity: crate::error::Severity::Warning,
                        message: format!(
                            "unknown element `{}` inside block (emitted as <div>)",
                            name
                        ),
                        span,
                        code: Some("W0201".into()),
                    });
                    let escaped = html_escape_attr(&name);
                    children.push(Node::new(
                        NodeKind::Html(format!("<div data-unknown=\"{}\"></div>", escaped)),
                        span,
                    ));
                }
                _ => {
                    self.advance();
                }
            }
        }
        Ok(children)
    }
}

fn html_escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn keyword_str(kw: Keyword) -> &'static str {
    match kw {
        Keyword::Title => "title",
        Keyword::Subtitle => "subtitle",
        Keyword::Text => "text",
        Keyword::Button => "button",
        Keyword::Image => "image",
        Keyword::Video => "video",
        Keyword::Link => "link",
        Keyword::Header => "header",
        Keyword::Footer => "footer",
        Keyword::Navbar => "navbar",
        Keyword::Section => "section",
        Keyword::Container => "container",
        Keyword::Row => "row",
        Keyword::Column => "column",
        Keyword::Card => "card",
        Keyword::List => "list",
        Keyword::Item => "item",
        Keyword::Table => "table",
        Keyword::Row_ => "row_",
        Keyword::Input => "input",
        Keyword::Email => "email",
        Keyword::Password => "password",
        Keyword::Checkbox => "checkbox",
        Keyword::Radio => "radio",
        Keyword::Textarea => "textarea",
        Keyword::Code => "code",
        Keyword::Quote => "quote",
        Keyword::Divider => "divider",
        Keyword::Space => "space",
        Keyword::Icon => "icon",
        Keyword::Html => "html",
        Keyword::Headers => "headers",
        Keyword::Rows => "rows",
        Keyword::Cell => "cell",
    }
}

fn parse_keyword(parser: &mut ParserState, kw: Keyword) -> Result<Node, CompileError> {
    let span = parser.peek().span;
    parser.skip_newlines();

    match kw {
        Keyword::Title => {
            let s = parser.expect_string("title").unwrap_or_default();
            Ok(Node::new(NodeKind::Title(s), span))
        }
        Keyword::Subtitle => {
            let s = parser.expect_string("subtitle").unwrap_or_default();
            Ok(Node::new(NodeKind::Subtitle(s), span))
        }
        Keyword::Text => {
            let s = parser.expect_string("text").unwrap_or_default();
            Ok(Node::new(NodeKind::Text(s), span))
        }
        Keyword::Button => {
            let label = parser.expect_string("button label").unwrap_or_default();
            let href = parser.expect_string("button href").unwrap_or_default();
            Ok(Node::new(NodeKind::Button { label, href }, span))
        }
        Keyword::Image => {
            let src = parser.expect_string("image src").unwrap_or_default();
            let alt = parser.expect_string("image alt").unwrap_or_default();
            Ok(Node::new(NodeKind::Image { src, alt }, span))
        }
        Keyword::Video => {
            let s = parser.expect_string("video src").unwrap_or_default();
            Ok(Node::new(NodeKind::Video(s), span))
        }
        Keyword::Quote => {
            let text = parser.expect_string("quote text").unwrap_or_default();
            let cite = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("quote cite"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Quote { text, cite }, span))
        }
        Keyword::Icon => {
            let s = parser.expect_string("icon name").unwrap_or_default();
            Ok(Node::new(NodeKind::Icon(s), span))
        }
        Keyword::Divider => Ok(Node::new(NodeKind::Divider, span)),
        Keyword::Space => Ok(Node::new(NodeKind::Space, span)),
        Keyword::Code => {
            let content = parser.expect_string("code content").unwrap_or_default();
            let language = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("code language"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Code { content, language }, span))
        }
        Keyword::Html => {
            let raw = parser.expect_string("html content").unwrap_or_default();
            Ok(Node::new(NodeKind::Html(raw), span))
        }
        Keyword::Input => {
            let name = parser.expect_string("input name").unwrap_or_default();
            let placeholder = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("input placeholder"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Input { name, placeholder }, span))
        }
        Keyword::Email => {
            let name = parser.expect_string("email name").unwrap_or_default();
            let placeholder = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("email placeholder"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Email { name, placeholder }, span))
        }
        Keyword::Password => {
            let name = parser.expect_string("password name").unwrap_or_default();
            let placeholder = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("password placeholder"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Password { name, placeholder }, span))
        }
        Keyword::Checkbox => {
            let name = parser.expect_string("checkbox name").unwrap_or_default();
            let label = parser.expect_string("checkbox label").unwrap_or_default();
            Ok(Node::new(NodeKind::Checkbox { name, label }, span))
        }
        Keyword::Radio => {
            let name = parser.expect_string("radio name").unwrap_or_default();
            let value = parser.expect_string("radio value").unwrap_or_default();
            let label = parser.expect_string("radio label").unwrap_or_default();
            Ok(Node::new(NodeKind::Radio { name, value, label }, span))
        }
        Keyword::Textarea => {
            let name = parser.expect_string("textarea name").unwrap_or_default();
            let placeholder = match &parser.peek().token {
                Token::String(_) | Token::Bare(_) => parser.expect_string("textarea placeholder"),
                _ => None,
            };
            Ok(Node::new(NodeKind::Textarea { name, placeholder }, span))
        }

        Keyword::Link => {
            let mut href = String::new();
            parser.skip_newlines();
            if let Token::String(_) | Token::Bare(_) = parser.peek().token {
                href = parser.expect_string("link href").unwrap_or_default();
            }
            let children = parser.parse_block()?;
            Ok(Node::new(NodeKind::Link { href, children }, span))
        }

        Keyword::Header
        | Keyword::Footer
        | Keyword::Navbar
        | Keyword::Section
        | Keyword::Container
        | Keyword::Row
        | Keyword::Column
        | Keyword::Card
        | Keyword::List
        | Keyword::Item => {
            let children = parser.parse_block()?;
            let kind = match kw {
                Keyword::Header => NodeKind::Header(children),
                Keyword::Footer => NodeKind::Footer(children),
                Keyword::Navbar => NodeKind::Navbar(children),
                Keyword::Section => NodeKind::Section(children),
                Keyword::Container => NodeKind::Container(children),
                Keyword::Row => NodeKind::Row(children),
                Keyword::Column => NodeKind::Column(children),
                Keyword::Card => NodeKind::Card(children),
                Keyword::List => {
                    let items = if children.iter().all(|c| matches!(c.kind, NodeKind::Item(_))) {
                        children
                    } else {
                        vec![Node::new(NodeKind::Item(children), span)]
                    };
                    NodeKind::List(items)
                }
                Keyword::Item => NodeKind::Item(children),
                _ => unreachable!(),
            };
            Ok(Node::new(kind, span))
        }

        Keyword::Table => {
            parser.expect_keyword(Keyword::Headers, "table")?;
            parser.skip_newlines();
            let headers = parse_string_block(parser)?;
            parser.expect_keyword(Keyword::Rows, "table")?;
            parser.skip_newlines();
            let rows = parse_row_block(parser)?;
            Ok(Node::new(NodeKind::Table { headers, rows }, span))
        }
        Keyword::Headers | Keyword::Rows | Keyword::Row_ | Keyword::Cell => Err(CompileError::Parse {
            message: format!("`{}` can only appear inside a table block", keyword_str(kw)),
            span,
        }),
    }
}

fn parse_string_block(parser: &mut ParserState) -> Result<Vec<String>, CompileError> {
    let mut out = Vec::new();
    while !parser.is_eof() {
        match &parser.peek().token {
            Token::String(_) | Token::Bare(_) => {
                let t = parser.advance();
                let s = match t.token {
                    Token::String(s) => s,
                    Token::Bare(s) => s,
                    _ => unreachable!(),
                };
                out.push(s);
            }
            Token::Dedent | Token::Eof => break,
            Token::Newline => {
                parser.advance();
                if matches!(parser.peek().token, Token::Dedent | Token::Eof) {
                    break;
                }
            }
            _ => {
                parser.advance();
            }
        }
    }
    Ok(out)
}

fn parse_row_block(parser: &mut ParserState) -> Result<Vec<Vec<String>>, CompileError> {
    let mut rows = Vec::new();
    let mut current: Option<Vec<String>> = None;
    while !parser.is_eof() {
        match &parser.peek().token {
            Token::Keyword(Keyword::Row_) => {
                if let Some(row) = current.take() {
                    rows.push(row);
                }
                parser.advance();
                current = Some(Vec::new());
            }
            Token::String(_) | Token::Bare(_) => {
                let t = parser.advance();
                let s = match t.token {
                    Token::String(s) => s,
                    Token::Bare(s) => s,
                    _ => unreachable!(),
                };
                current.get_or_insert_with(Vec::new).push(s);
            }
            Token::Dedent | Token::Eof => break,
            Token::Newline => {
                parser.advance();
                if matches!(parser.peek().token, Token::Dedent | Token::Eof) {
                    break;
                }
            }
            _ => {
                parser.advance();
            }
        }
    }
    if let Some(row) = current.take() {
        rows.push(row);
    }
    Ok(rows)
}
