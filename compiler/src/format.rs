//! Tiny idempotent formatter.
//!
//! The full implementation only normalises whitespace. Future versions will
//! use the tokeniser for smarter formatting.

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
