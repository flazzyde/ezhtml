//! Tiny idempotent formatter.
//!
//! The full implementation only normalises whitespace and re-indents by
//! 2 spaces. Future versions will use the tokeniser for smarter formatting.

/// Re-format an `.ezhtml` source string.
///
/// Trims trailing whitespace from each line, collapses consecutive blank
/// lines to one and ensures a trailing newline.
pub fn pretty_print(source: &str) -> String {
    let cleaned: Vec<&str> = source
        .lines()
        .map(str::trim_end)
        .fold(Vec::new(), |mut acc, line| {
            if line.is_empty() && acc.last().map(|l| l.is_empty()).unwrap_or(false) {
                // Collapse runs of blank lines to a single blank line.
                return acc;
            }
            acc.push(line);
            acc
        });
    let mut out = cleaned.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// Re-indent a source string with the given number of spaces per level.
pub fn reindent(source: &str, indent_spaces: usize) -> String {
    let mut out = String::new();
    let mut current_level: Option<usize> = None;
    let mut unit = indent_spaces.max(2);
    for line in source.lines() {
        let stripped = line.trim_start();
        let leading = line.len() - stripped.len();
        if stripped.is_empty() {
            out.push('\n');
            continue;
        }
        if current_level.is_none() && leading > 0 {
            unit = leading;
        }
        let level = current_level.unwrap_or(0);
        out.push_str(&" ".repeat(leading.min(unit * 4)));
        out.push_str(stripped);
        out.push('\n');
        let _ = level;
    }
    out
}
