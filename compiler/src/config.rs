//! Configuration types.
//!
//! [`CompileOptions`] controls how a document is rendered – title, language,
//! theme color, OpenGraph image, etc. Most of this can be supplied by a
//! `project.ez` / `config.ez` / `settings.ez` file in the same directory
//! (see [`crate::project`]).

use serde::{Deserialize, Serialize};

/// Top-level options passed to [`crate::compile`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOptions {
    /// `<head>` metadata.
    pub metadata: Metadata,
    /// Page-local settings such as language or direction.
    pub page_settings: PageSettings,
    /// Whether the compiler should produce a build-summary comment.
    pub emit_build_comment: bool,
    /// Source filename for diagnostics & embedded comments.
    pub source_name: String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            page_settings: PageSettings::default(),
            emit_build_comment: true,
            source_name: "<source>".to_string(),
        }
    }
}

/// `<head>` metadata used by the emitter.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// `<title>` and `og:title`.
    pub title: String,
    /// `<meta name="description">` and `og:description`.
    pub description: Option<String>,
    /// `<meta name="author">`.
    pub author: Option<String>,
    /// `<meta name="keywords">`.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// `<meta name="theme-color">`.
    pub theme_color: Option<String>,
    /// `<link rel="icon">`.
    pub favicon: Option<String>,
    /// `<link rel="manifest">`.
    pub manifest: Option<String>,
    /// `<meta property="og:image">` / `twitter:image`.
    pub og_image: Option<String>,
    /// `<meta property="og:url">`.
    pub url: Option<String>,
}

/// Page-settings (currently just language + text direction).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettings {
    /// `<html lang>`.
    pub language: String,
    /// `<html dir>` – `"ltr"` or `"rtl"`.
    pub text_direction: TextDirection,
}

impl Default for PageSettings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            text_direction: TextDirection::LeftToRight,
        }
    }
}

/// Writing direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
    /// Left-to-right (default).
    Ltr,
    /// Right-to-left.
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// On-disk `site.ez` / `config.ez` / `settings.ez` schema.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SiteConfig {
    /// Site-wide metadata.
    #[serde(default)]
    pub metadata: Metadata,
    /// Site-wide page settings.
    #[serde(default)]
    pub page_settings: PageSettings,
}

impl SiteConfig {
    /// Compile a [`SiteConfig`] into [`CompileOptions`].
    pub fn to_options(&self, source_name: impl Into<String>) -> CompileOptions {
        CompileOptions {
            metadata: self.metadata.clone(),
            page_settings: self.page_settings.clone(),
            emit_build_comment: true,
            source_name: source_name.into(),
        }
    }
}
