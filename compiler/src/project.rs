//! Project file discovery.
//!
//! When compiling a single file the compiler looks for adjacent project
//! metadata files (`project.ez`, `config.ez`, `settings.ez`) so authors can
//! store title, description, theme color etc. once and reuse it.

use crate::config::SiteConfig;
use std::path::Path;

/// Different on-disk file names that can hold site config.
pub const PROJECT_FILE_NAMES: &[&str] = &[
    "project.ez",
    "config.ez",
    "settings.ez",
    "site.ez.json",
    "site.ez.toml",
];

/// A loaded project file.
#[derive(Debug, Clone)]
pub struct ProjectFile {
    /// Absolute path on disk.
    pub path: std::path::PathBuf,
    /// Parsed configuration.
    pub config: SiteConfig,
}

/// Discover project files relative to the given directory.
pub fn discover(dir: &Path) -> Option<ProjectFile> {
    for name in PROJECT_FILE_NAMES {
        let candidate = dir.join(name);
        if candidate.exists() {
            if let Ok(p) = load(&candidate) {
                return Some(p);
            }
        }
    }
    None
}

/// Load a project file from disk and parse it.
pub fn load(path: &Path) -> Result<ProjectFile, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config = parse(&raw, path)?;
    Ok(ProjectFile {
        path: path.to_path_buf(),
        config,
    })
}

/// Parse raw text. Supports JSON, TOML and the native `.ez` key/value format.
pub fn parse(raw: &str, path: &Path) -> Result<SiteConfig, String> {
    let trimmed = raw.trim();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let looks_like_json = trimmed.starts_with('{') || ext == "json";
    let looks_like_toml = trimmed.starts_with('[')
        || ext == "toml"
        || (trimmed.contains("=") && trimmed.contains('\n'));
    if looks_like_json {
        return serde_json::from_str::<SiteConfig>(trimmed).map_err(|e| e.to_string());
    }
    if looks_like_toml {
        return toml::from_str::<SiteConfig>(trimmed).map_err(|e| e.to_string());
    }
    parse_ez(raw)
}

/// Native `.ez` key/value format: `title "Hello"`, `keyword "a", "b"`, etc.
fn parse_ez(raw: &str) -> Result<SiteConfig, String> {
    let mut config = SiteConfig::default();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let key = parts.next().unwrap_or("").trim();
        let value = parts.next().unwrap_or("").trim();
        match key {
            "title" => config.metadata.title = unquote(value),
            "description" => config.metadata.description = Some(unquote(value)),
            "author" => config.metadata.author = Some(unquote(value)),
            "theme_color" | "themeColor" | "theme-color" => {
                config.metadata.theme_color = Some(unquote(value))
            }
            "language" | "lang" => config.page_settings.language = unquote(value),
            "favicon" => config.metadata.favicon = Some(unquote(value)),
            "manifest" => config.metadata.manifest = Some(unquote(value)),
            "image" | "og_image" | "ogImage" => {
                config.metadata.og_image = Some(unquote(value))
            }
            "url" => config.metadata.url = Some(unquote(value)),
            "keyword" | "keywords" => {
                for w in split_words(value) {
                    config.metadata.keywords.push(w);
                }
            }
            _ => {}
        }
    }
    Ok(config)
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"'))
        || (s.starts_with('\'') && s.ends_with('\''))
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn split_words(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_quote: Option<char> = None;
    for c in s.chars() {
        match c {
            '"' | '\'' if in_quote.is_none() => in_quote = Some(c),
            q if Some(q) == in_quote => in_quote = None,
            ',' if in_quote.is_none => {
                let v = buf.trim().to_string();
                if !v.is_empty() {
                    out.push(unquote(&v));
                }
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    let v = buf.trim().to_string();
    if !v.is_empty() {
        out.push(unquote(&v));
    }
    out
}

/// Load options for the supplied directory, falling back to defaults.
pub fn load_options_from_dir(dir: &Path) -> crate::config::CompileOptions {
    if let Some(pf) = discover(dir) {
        pf.config.to_options(dir.to_string_lossy().to_string())
    } else {
        crate::config::CompileOptions {
            source_name: dir.to_string_lossy().to_string(),
            ..Default::default()
        }
    }
}
