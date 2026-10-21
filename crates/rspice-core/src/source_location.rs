//! Where a line of deck text was written.
//!
//! A diagnostic or a refusal names the file and line that authored what it is
//! about. That location is data, not parsing, so it sits below everything
//! that carries one: the parser that records it, the configuration and
//! capability refusals that cite it, and the engine's error taxonomy.
//! `netlist` re-exports it under the name callers have always used.

/// Exact physical location in one netlist source.
///
/// In-memory parses have no path. File-backed parses retain the top-level or
/// included source path so diagnostics do not collapse onto expanded-text line
/// numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistSourceLocation {
    pub path: Option<std::path::PathBuf>,
    pub line: usize,
}

impl NetlistSourceLocation {
    pub fn in_memory(line: usize) -> Self {
        Self { path: None, line }
    }

    pub fn in_file(path: impl Into<std::path::PathBuf>, line: usize) -> Self {
        Self {
            path: Some(path.into()),
            line,
        }
    }
}

impl std::fmt::Display for NetlistSourceLocation {
    /// The `path:line` prefix a diagnostic shows. The path is kept canonical
    /// for comparison, so it is shown through [`display_source_path`].
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(formatter, "{}:{}", display_source_path(path), self.line),
            None => write!(formatter, "line {}", self.line),
        }
    }
}

/// A path as a diagnostic shows it. A canonical Windows path carries the
/// verbatim `\\?\` prefix, which is not what anyone wrote; a drive or UNC
/// path reads the same without it. Device paths reached through `\\?\` have
/// no plain spelling and keep it. Comparisons keep using the canonical path.
#[doc(hidden)]
pub fn display_source_path(path: &std::path::Path) -> String {
    let text = path.display().to_string();
    // A resolved include path can also arrive with its separators turned to
    // `/` (`//?/C:/...`); it reads the same way.
    for (verbatim, unc, separator) in [(r"\\?\", r"\\?\UNC\", '\\'), ("//?/", "//?/UNC/", '/')] {
        if let Some(share) = text.strip_prefix(unc) {
            return format!("{separator}{separator}{share}");
        }
        if let Some(drive) = text.strip_prefix(verbatim)
            && drive
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
            && drive.get(1..2) == Some(":")
            && drive[2..].starts_with(separator)
        {
            return drive.to_owned();
        }
    }
    text
}

#[cfg(test)]
mod display_source_path_tests {
    use super::{NetlistSourceLocation, display_source_path};
    use std::path::Path;

    /// The `path:line:` prefix of a diagnostic inside an included file showed
    /// the canonical verbatim form (`\\?\C:\...\bad.inc:2:`); it shows the
    /// path as it would be written, and the stored path stays canonical.
    #[test]
    fn diagnostics_show_a_canonical_path_without_the_verbatim_prefix() {
        for (path, shown) in [
            (r"\\?\C:\decks\bad.inc", r"C:\decks\bad.inc"),
            (r"\\?\UNC\server\share\bad.inc", r"\\server\share\bad.inc"),
            ("//?/C:/decks/bad.inc", "C:/decks/bad.inc"),
            ("//?/UNC/server/share/bad.inc", "//server/share/bad.inc"),
            (r"\\?\Volume{0}\bad.inc", r"\\?\Volume{0}\bad.inc"),
            ("decks/bad.inc", "decks/bad.inc"),
        ] {
            assert_eq!(display_source_path(Path::new(path)), shown, "{path}");
            let location = NetlistSourceLocation::in_file(path, 2);
            assert_eq!(location.to_string(), format!("{shown}:2"), "{path}");
            assert_eq!(location.path.as_deref(), Some(Path::new(path)));
        }
    }
}
