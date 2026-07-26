//! HTML Emitter – walks the AST and produces pretty-printed HTML5.
//!
//! The emitter is responsible for the entire `<!DOCTYPE html><html>...</html>`
//! scaffold, including `<head>`, meta tags, OpenGraph and Twitter Cards.

use crate::ast::{Document, NodeKind};
use crate::config::{CompileOptions, TextDirection};

/// Generate HTML5 from a [`Document`] using the supplied options.
pub fn emit(doc: &Document, options: &CompileOptions) -> String {
    let mut out = String::new();
    let metadata = compute_metadata(doc, options);

    out.push_str("<!DOCTYPE html>\n");
    let dir_attr = match options.page_settings.text_direction {
        TextDirection::Rtl => " dir=\"rtl\"",
        TextDirection::Ltr => "",
    };
    out.push_str(&format!(
        "<html lang=\"{}\"{}>\n",
        html_escape(&metadata.language),
        dir_attr
    ));
    out.push_str("<head>\n");
    out.push_str("  <meta charset=\"utf-8\">\n");
    out.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str(&format!(
        "  <title>{}</title>\n",
        html_escape(&metadata.title)
    ));

    if let Some(desc) = &metadata.description {
        out.push_str(&format!(
            "  <meta name=\"description\" content=\"{}\">\n",
            html_escape(desc)
        ));
    }
    if let Some(author) = &metadata.author {
        out.push_str(&format!(
            "  <meta name=\"author\" content=\"{}\">\n",
            html_escape(author)
        ));
    }
    if !metadata.keywords.is_empty() {
        out.push_str(&format!(
            "  <meta name=\"keywords\" content=\"{}\">\n",
            html_escape(&metadata.keywords.join(", "))
        ));
    }
    if let Some(color) = &metadata.theme_color {
        out.push_str(&format!(
            "  <meta name=\"theme-color\" content=\"{}\">\n",
            html_escape(color)
        ));
    }
    if let Some(favicon) = &metadata.favicon {
        out.push_str(&format!(
            "  <link rel=\"icon\" href=\"{}\" type=\"image/x-icon\">\n",
            html_escape(favicon)
        ));
    }
    if let Some(manifest) = &metadata.manifest {
        out.push_str(&format!(
            "  <link rel=\"manifest\" href=\"{}\">\n",
            html_escape(manifest)
        ));
    }
    // OpenGraph
    out.push_str(&format!(
        "  <meta property=\"og:title\" content=\"{}\">\n",
        html_escape(&metadata.title)
    ));
    if let Some(desc) = &metadata.description {
        out.push_str(&format!(
            "  <meta property=\"og:description\" content=\"{}\">\n",
            html_escape(desc)
        ));
    }
    out.push_str("  <meta property=\"og:type\" content=\"website\">\n");
    if let Some(url) = &metadata.url {
        out.push_str(&format!(
            "  <meta property=\"og:url\" content=\"{}\">\n",
            html_escape(url)
        ));
    }
    // Twitter
    out.push_str("  <meta name=\"twitter:card\" content=\"summary_large_image\">\n");
    out.push_str(&format!(
        "  <meta name=\"twitter:title\" content=\"{}\">\n",
        html_escape(&metadata.title)
    ));
    if let Some(desc) = &metadata.description {
        out.push_str(&format!(
            "  <meta name=\"twitter:description\" content=\"{}\">\n",
            html_escape(desc)
        ));
    }
    if let Some(image) = &metadata.og_image {
        out.push_str(&format!(
            "  <meta property=\"og:image\" content=\"{}\">\n",
            html_escape(image)
        ));
        out.push_str(&format!(
            "  <meta name=\"twitter:image\" content=\"{}\">\n",
            html_escape(image)
        ));
    }

    out.push_str("</head>\n");
    out.push_str("<body>\n");

    for node in &doc.nodes {
        emit_node(node, &mut out, 1);
    }

    out.push_str("</body>\n");
    out.push_str("</html>\n");
    out
}

fn emit_node(node: &crate::ast::Node, out: &mut String, indent: usize) {
    let pad = "  ".repeat(indent);
    match &node.kind {
        NodeKind::Title(s) => {
            emit_html(out, &pad, "h1", &[("class", "title")], |o| {
                o.push_str(&html_escape(s));
                Ok(())
            })
            .ok();
        }
        NodeKind::Subtitle(s) => {
            emit_html(out, &pad, "h2", &[], |o| {
                o.push_str(&html_escape(s));
                Ok(())
            })
            .ok();
        }
        NodeKind::Text(s) => {
            emit_html(out, &pad, "p", &[], |o| {
                o.push_str(&html_escape(s));
                Ok(())
            })
            .ok();
        }
        NodeKind::Button { label, href } => {
            emit_html(
                out,
                &pad,
                "a",
                &[("class", "btn btn-primary"), ("href", href.as_str())],
                |o| {
                    o.push_str(&html_escape(label));
                    Ok(())
                },
            )
            .ok();
        }
        NodeKind::Image { src, alt } => {
            out.push_str(&format!(
                "{}<img src=\"{}\" alt=\"{}\">\n",
                pad,
                html_escape(src),
                html_escape(alt)
            ));
        }
        NodeKind::Video(src) => {
            out.push_str(&format!(
                "{}<video controls src=\"{}\"></video>\n",
                pad,
                html_escape(src)
            ));
        }
        NodeKind::Link { href, children } => {
            if children.is_empty() {
                emit_html(out, &pad, "a", &[("href", href.as_str())], |o| Ok(())).ok();
            } else {
                emit_html(out, &pad, "a", &[("href", href.as_str())], |o| {
                    for (i, c) in children.iter().enumerate() {
                        if i > 0 {
                            o.push('\n');
                        }
                        emit_node(c, o, indent + 1);
                    }
                    Ok(())
                })
                .ok();
            }
        }
        NodeKind::Header(children) => emit_block(out, "header", &[], children, indent),
        NodeKind::Footer(children) => emit_block(out, "footer", &[], children, indent),
        NodeKind::Navbar(children) => {
            emit_block(out, "nav", &[("class", "navbar")], children, indent)
        }
        NodeKind::Section(children) => emit_block(out, "section", &[], children, indent),
        NodeKind::Container(children) => {
            emit_block(out, "div", &[("class", "container")], children, indent)
        }
        NodeKind::Row(children) => emit_block(out, "div", &[("class", "row")], children, indent),
        NodeKind::Column(children) => {
            emit_block(out, "div", &[("class", "col")], children, indent)
        }
        NodeKind::Card(children) => {
            emit_block(out, "article", &[("class", "card")], children, indent)
        }
        NodeKind::List(children) => {
            if children.is_empty() {
                emit_html(out, &pad, "ul", &[], |o| Ok(())).ok();
                return;
            }
            emit_html(out, &pad, "ul", &[], |o| {
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        o.push('\n');
                    }
                    emit_node(c, o, indent + 1);
                }
                Ok(())
            })
            .ok();
        }
        NodeKind::Item(children) => {
            if children.is_empty() {
                emit_html(out, &pad, "li", &[], |o| Ok(())).ok();
                return;
            }
            emit_html(out, &pad, "li", &[], |o| {
                for (i, c) in children.iter().enumerate() {
                    if i > 0 {
                        o.push('\n');
                    }
                    emit_node(c, o, indent + 1);
                }
                Ok(())
            })
            .ok();
        }
        NodeKind::Table { headers, rows } => emit_table(out, indent, headers, rows),
        NodeKind::Input { name, placeholder } => {
            out.push_str(&format!(
                "{}<input type=\"text\" name=\"{}\" placeholder=\"{}\">\n",
                pad,
                html_escape(name),
                html_escape(placeholder.as_deref().unwrap_or(""))
            ));
        }
        NodeKind::Email { name, placeholder } => {
            out.push_str(&format!(
                "{}<input type=\"email\" name=\"{}\" placeholder=\"{}\">\n",
                pad,
                html_escape(name),
                html_escape(placeholder.as_deref().unwrap_or(""))
            ));
        }
        NodeKind::Password { name, placeholder } => {
            out.push_str(&format!(
                "{}<input type=\"password\" name=\"{}\" placeholder=\"{}\">\n",
                pad,
                html_escape(name),
                html_escape(placeholder.as_deref().unwrap_or(""))
            ));
        }
        NodeKind::Checkbox { name, label } => {
            out.push_str(&format!(
                "{}<label class=\"checkbox\"><input type=\"checkbox\" name=\"{}\"> {}</label>\n",
                pad,
                html_escape(name),
                html_escape(label)
            ));
        }
        NodeKind::Radio { name, value, label } => {
            out.push_str(&format!(
                "{}<label class=\"radio\"><input type=\"radio\" name=\"{}\" value=\"{}\"> {}</label>\n",
                pad,
                html_escape(name),
                html_escape(value),
                html_escape(label)
            ));
        }
        NodeKind::Textarea { name, placeholder } => {
            out.push_str(&format!(
                "{}<textarea name=\"{}\" placeholder=\"{}\"></textarea>\n",
                pad,
                html_escape(name),
                html_escape(placeholder.as_deref().unwrap_or(""))
            ));
        }
        NodeKind::Code { content, language } => {
            let lang_attr = language
                .as_deref()
                .map(|l| format!(" class=\"language-{}\"", html_escape(l)))
                .unwrap_or_default();
            out.push_str(&format!(
                "{}<pre><code{}>{}</code></pre>\n",
                pad,
                lang_attr,
                html_escape(content)
            ));
        }
        NodeKind::Quote { text, cite } => {
            let cite_attr = cite
                .as_deref()
                .map(|c| format!(" cite=\"{}\"", html_escape(c)))
                .unwrap_or_default();
            out.push_str(&format!(
                "{}<blockquote{}>{}</blockquote>\n",
                pad,
                cite_attr,
                html_escape(text)
            ));
        }
        NodeKind::Divider => {
            out.push_str(&format!("{}<hr>\n", pad));
        }
        NodeKind::Space => {
            out.push_str(&format!("{}<p class=\"space\">&nbsp;</p>\n", pad));
        }
        NodeKind::Icon(name) => {
            out.push_str(&format!(
                "{}<i class=\"icon icon-{}\" aria-hidden=\"true\"></i>\n",
                pad,
                html_escape(name)
            ));
        }
        NodeKind::Html(raw) => {
            out.push_str(&format!("{}<!-- raw html -->\n", pad));
            out.push_str(raw);
            if !raw.ends_with('\n') {
                out.push('\n');
            }
        }
    }
}

fn emit_block(
    out: &mut String,
    tag: &str,
    attrs: &[(&str, &str)],
    children: &[crate::ast::Node],
    indent: usize,
) {
    if children.is_empty() {
        emit_html(out, &"  ".repeat(indent), tag, attrs, |o| Ok(())).ok();
        return;
    }
    emit_html(out, &"  ".repeat(indent), tag, attrs, |o| {
        for (i, c) in children.iter().enumerate() {
            if i > 0 {
                o.push('\n');
            }
            emit_node(c, o, indent + 1);
        }
        Ok(())
    })
    .ok();
}

fn emit_table(out: &mut String, indent: usize, headers: &[String], rows: &[Vec<String>]) {
    let pad = "  ".repeat(indent);
    out.push_str(&format!("{}<table>\n", pad));
    out.push_str(&format!("{}  <thead>\n", pad));
    out.push_str(&format!("{}    <tr>\n", pad));
    for h in headers {
        out.push_str(&format!("{}      <th>{}</th>\n", pad, html_escape(h)));
    }
    out.push_str(&format!("{}    </tr>\n", pad));
    out.push_str(&format!("{}  </thead>\n", pad));
    out.push_str(&format!("{}  <tbody>\n", pad));
    for row in rows {
        out.push_str(&format!("{}    <tr>\n", pad));
        for cell in row {
            out.push_str(&format!("{}      <td>{}</td>\n", pad, html_escape(cell)));
        }
        out.push_str(&format!("{}    </tr>\n", pad));
    }
    out.push_str(&format!("{}  </tbody>\n", pad));
    out.push_str(&format!("{}</table>\n", pad));
}

type Result<T> = std::result::Result<T, std::convert::Infallible>;

fn emit_html<F>(
    out: &mut String,
    pad: &str,
    tag: &str,
    attrs: &[(&str, &str)],
    body: F,
) -> Result<()>
where
    F: FnOnce(&mut String) -> Result<()>,
{
    out.push_str(pad);
    out.push('<');
    out.push_str(tag);
    for (k, v) in attrs {
        if v.is_empty() {
            continue;
        }
        out.push(' ');
        out.push_str(k);
        out.push_str("=\"");
        out.push_str(&html_escape(v));
        out.push('"');
    }
    out.push('>');
    body(out)?;
    out.push_str(&format!("</{}>\n", tag));
    Ok(())
}

/// Escape `&`, `<`, `>` and `"`.
pub fn html_escape(s: &str) -> String {
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

fn compute_metadata<'a>(doc: &'a Document, options: &'a CompileOptions) -> MetadataView<'a> {
    let mut title = options.metadata.title.clone();
    let mut description = options.metadata.description.clone();
    for n in &doc.nodes {
        if title.is_empty() {
            if let NodeKind::Title(s) = &n.kind {
                title = s.clone();
            }
        }
        if description.is_none() {
            if let NodeKind::Text(s) = &n.kind {
                description = Some(s.clone());
            }
        }
    }
    MetadataView {
        title,
        description,
        author: options.metadata.author.as_ref(),
        keywords: &options.metadata.keywords,
        theme_color: options.metadata.theme_color.as_ref(),
        favicon: options.metadata.favicon.as_ref(),
        manifest: options.metadata.manifest.as_ref(),
        og_image: options.metadata.og_image.as_ref(),
        url: options.metadata.url.as_ref(),
        language: &options.page_settings.language,
    }
}

struct MetadataView<'a> {
    title: String,
    description: Option<String>,
    author: Option<&'a String>,
    keywords: &'a Vec<String>,
    theme_color: Option<&'a String>,
    favicon: Option<&'a String>,
    manifest: Option<&'a String>,
    og_image: Option<&'a String>,
    url: Option<&'a String>,
    language: &'a String,
}
