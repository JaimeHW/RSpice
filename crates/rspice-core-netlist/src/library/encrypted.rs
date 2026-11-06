//! Vendor-encrypted model sources, recognised by their published markers.
//!
//! An encrypted block is refused by name and never decrypted. The block's
//! lines are blanked before parsing, so the definitions around it still
//! import and every reported line number stays the source's own.

use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// A vendor encryption scheme, named by the marker that opens it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncryptionScheme {
    /// LTspice/ADI text encryption: `* LTspice Encrypted File`, a hex payload
    /// between `* Begin:` and `* End`.
    LtspiceText,
    /// LTspice/ADI binary encryption: a `<Binary File>` header.
    LtspiceBinary,
    /// PSpice/Cadence `$CDNENCSTART` … `$CDNENCFINISH`.
    Pspice,
    /// HSPICE or QSPICE `.prot` … `.unprot` carrying ciphertext. Plain SPICE
    /// between the markers is HSPICE listing suppression and parses as usual.
    ProtectedBlock,
    /// Spectre `protect` … `unprotect`.
    Spectre,
    /// IEEE 1735 `pragma protect begin_protected` envelope.
    Ieee1735,
}

impl EncryptionScheme {
    fn label(self) -> &'static str {
        match self {
            Self::LtspiceText => "an LTspice encrypted file",
            Self::LtspiceBinary => "an LTspice binary-encrypted file",
            Self::Pspice => "a PSpice encrypted block ($CDNENCSTART)",
            Self::ProtectedBlock => "an HSPICE/QSPICE protected block (.prot)",
            Self::Spectre => "a Spectre protected block (protect)",
            Self::Ieee1735 => "an IEEE 1735 protected envelope",
        }
    }

    /// Whether the scheme always encrypts the whole file.
    fn is_whole_file(self) -> bool {
        matches!(self, Self::LtspiceText | Self::LtspiceBinary)
    }
}

/// One encrypted definition or block the parser refused to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedDefinition {
    /// The `.subckt` whose body is encrypted, when its interface line is
    /// plain text. A block that hides its own interface has no name.
    pub name: Option<String>,
    pub scheme: EncryptionScheme,
    pub file: Option<PathBuf>,
    /// One-based line of the `.subckt` header, or of the opening marker.
    pub line: usize,
}

impl EncryptedDefinition {
    /// One sentence naming the definition and its format, and what to ask
    /// the vendor for instead.
    pub fn message(&self) -> String {
        let subject = match (&self.name, self.scheme.is_whole_file(), &self.file) {
            (Some(name), _, _) => format!("Subcircuit {name} is"),
            (None, true, Some(file)) => format!(
                "{} is",
                file.file_name().map_or_else(
                    || file.display().to_string(),
                    |name| name.to_string_lossy().into_owned()
                )
            ),
            (None, true, None) => "This file is".to_owned(),
            (None, false, _) => format!("Line {} opens", self.line),
        };
        format!(
            "{subject} {}, which RSpice cannot read; ask the vendor for an unencrypted SPICE or \
             Verilog-A model.",
            self.scheme.label()
        )
    }
}

/// The marker that opens an encrypted region on this (trimmed) line.
fn opening_marker(line: &str, first_line: bool) -> Option<EncryptionScheme> {
    let lower = line.to_ascii_lowercase();
    if lower.starts_with("* ltspice encrypted file") {
        return Some(EncryptionScheme::LtspiceText);
    }
    if first_line && line.starts_with("<Binary File>") {
        return Some(EncryptionScheme::LtspiceBinary);
    }
    if line.starts_with("$CDNENCSTART") {
        return Some(EncryptionScheme::Pspice);
    }
    if lower == ".prot" || lower == ".protect" {
        return Some(EncryptionScheme::ProtectedBlock);
    }
    if lower == "protect" {
        return Some(EncryptionScheme::Spectre);
    }
    if lower.contains("pragma protect begin_protected") {
        return Some(EncryptionScheme::Ieee1735);
    }
    None
}

/// Whether this (trimmed) line closes a region the scheme opened.
fn closing_marker(scheme: EncryptionScheme, line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    match scheme {
        EncryptionScheme::LtspiceText => lower.starts_with("* end"),
        EncryptionScheme::LtspiceBinary => false,
        EncryptionScheme::Pspice => line.starts_with("$CDNENCFINISH"),
        EncryptionScheme::ProtectedBlock => lower == ".unprot" || lower == ".unprotect",
        EncryptionScheme::Spectre => lower == "unprotect",
        EncryptionScheme::Ieee1735 => lower.contains("pragma protect end_protected"),
    }
}

/// Ciphertext is one whitespace-free token per line. No SPICE card is: an
/// element names at least two nodes, and directives, comments and
/// continuations open with their own characters.
fn is_ciphertext(lines: &[&str]) -> bool {
    lines.iter().map(|line| line.trim()).any(|line| {
        !line.is_empty()
            && !line.starts_with(['*', '+', '.', '$'])
            && line.split_whitespace().nth(1).is_none()
    })
}

/// The `.subckt` (SPICE) or `subckt` (Spectre) name a header line opens.
fn subcircuit_header(line: &str) -> Option<&str> {
    let mut tokens = line.split_whitespace();
    let mut first = tokens.next()?;
    if first.eq_ignore_ascii_case("inline") {
        first = tokens.next()?;
    }
    (first.eq_ignore_ascii_case(".subckt") || first.eq_ignore_ascii_case("subckt"))
        .then(|| tokens.next())
        .flatten()
        .map(|name| name.split('(').next().unwrap_or(name))
        .filter(|name| !name.is_empty())
}

fn closes_subcircuit(line: &str) -> bool {
    line.split_whitespace().next().is_some_and(|first| {
        first.eq_ignore_ascii_case(".ends") || first.eq_ignore_ascii_case("ends")
    })
}

/// Blank every encrypted region in `content` and report each one.
///
/// A region inside a `.subckt` takes the whole definition with it, header
/// to `.ends`, so a body the parser cannot see never imports as an empty
/// subcircuit. Line count and line numbers are preserved.
pub(crate) fn excise_encrypted<'a>(
    content: &'a str,
    file: Option<&Path>,
) -> (Cow<'a, str>, Vec<EncryptedDefinition>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut blank = vec![false; lines.len()];
    let mut found = Vec::new();
    // The open subcircuit: name, header index, and whether a region opened in it.
    let mut open_subckt: Option<(&str, usize, Option<EncryptionScheme>)> = None;
    let mut seen_text = false;
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].trim();
        let first_line = !seen_text;
        seen_text |= !line.is_empty();
        if let Some(scheme) = opening_marker(line, first_line) {
            let mut end = index + 1;
            while end < lines.len() && !closing_marker(scheme, lines[end].trim()) {
                end += 1;
            }
            let last = end.min(lines.len() - 1);
            if scheme == EncryptionScheme::ProtectedBlock && !is_ciphertext(&lines[index + 1..end])
            {
                index = last + 1;
                continue;
            }
            match open_subckt.as_mut() {
                Some((_, _, region)) => *region = region.or(Some(scheme)),
                None => found.push(EncryptedDefinition {
                    name: None,
                    scheme,
                    file: file.map(Path::to_path_buf),
                    line: index + 1,
                }),
            }
            blank[index..=last].iter_mut().for_each(|line| *line = true);
            index = last + 1;
            continue;
        }
        if let Some(name) = subcircuit_header(line) {
            open_subckt = Some((name, index, None));
        } else if closes_subcircuit(line)
            && let Some((name, start, region)) = open_subckt.take()
            && let Some(scheme) = region
        {
            found.push(EncryptedDefinition {
                name: Some(name.to_owned()),
                scheme,
                file: file.map(Path::to_path_buf),
                line: start + 1,
            });
            blank[start..=index]
                .iter_mut()
                .for_each(|line| *line = true);
        }
        index += 1;
    }
    if let Some((name, start, Some(scheme))) = open_subckt {
        found.push(EncryptedDefinition {
            name: Some(name.to_owned()),
            scheme,
            file: file.map(Path::to_path_buf),
            line: start + 1,
        });
        blank[start..].iter_mut().for_each(|line| *line = true);
    }
    if found.is_empty() {
        return (Cow::Borrowed(content), found);
    }
    let mut kept = String::with_capacity(content.len());
    for (line, blanked) in lines.iter().zip(blank) {
        if !blanked {
            kept.push_str(line);
        }
        kept.push('\n');
    }
    (Cow::Owned(kept), found)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn excise(content: &str) -> (String, Vec<EncryptedDefinition>) {
        let (kept, found) = excise_encrypted(content, Some(Path::new("/lib/vendor.lib")));
        (kept.into_owned(), found)
    }

    #[test]
    fn a_protected_subcircuit_body_takes_its_whole_definition() {
        let source = "* vendor\n.subckt SEALED A B C\n.prot\nQx7Zr0PdLm2Nb8Vt4Kc1Hs6Yw3Fg9\n\
                      Jq5Ue0Xa7Ri2Ob4Mn8Wk1Sd6Tl3Cv\n.unprot\n.ends SEALED\n\
                      .subckt OPEN A B\nR1 A B 1k\n.ends OPEN\n";
        let (kept, found) = excise(source);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name.as_deref(), Some("SEALED"));
        assert_eq!(found[0].scheme, EncryptionScheme::ProtectedBlock);
        assert_eq!(found[0].line, 2);
        assert_eq!(kept.lines().count(), source.lines().count());
        assert!(!kept.contains("SEALED") && !kept.contains("Qx7Z"));
        assert!(kept.contains(".subckt OPEN A B\nR1 A B 1k\n.ends OPEN"));
        assert_eq!(
            found[0].message(),
            "Subcircuit SEALED is an HSPICE/QSPICE protected block (.prot), which RSpice cannot \
             read; ask the vendor for an unencrypted SPICE or Verilog-A model."
        );
    }

    #[test]
    fn plain_spice_between_prot_markers_is_listing_suppression_not_encryption() {
        let source = ".subckt LISTED A B\n.prot\nR1 A B 1k\n.unprot\n.ends LISTED\n";
        let (kept, found) = excise(source);
        assert!(found.is_empty());
        assert_eq!(kept, source);
    }

    #[test]
    fn each_whole_file_scheme_is_named_by_its_file() {
        for (source, scheme) in [
            (
                "* LTspice Encrypted File\n*\n* Begin:\n 7C 60 2F 2B\n D7 31 F9 4C\n* End 1 2\n",
                EncryptionScheme::LtspiceText,
            ),
            (
                "\r\n<Binary File>\r\n\r\n\u{1a}&\u{7f}",
                EncryptionScheme::LtspiceBinary,
            ),
        ] {
            let (kept, found) = excise(source);
            assert_eq!(found.len(), 1, "{source:?}");
            assert_eq!(found[0].scheme, scheme);
            assert_eq!(found[0].name, None);
            assert!(kept.trim().is_empty(), "{kept:?}");
            assert!(found[0].message().starts_with("vendor.lib is an LTspice "));
        }
    }

    #[test]
    fn a_top_level_block_is_named_by_its_line_and_its_neighbours_survive() {
        for (marker, close, scheme) in [
            ("$CDNENCSTART", "$CDNENCFINISH", EncryptionScheme::Pspice),
            ("protect", "unprotect", EncryptionScheme::Spectre),
            (
                "`pragma protect begin_protected",
                "`pragma protect end_protected",
                EncryptionScheme::Ieee1735,
            ),
        ] {
            let source = format!(
                ".model D1 D (IS=1e-14)\n{marker}\nb6c1e0f4a9d27735e81c\n{close}\n\
                 .model D2 D (IS=2e-14)\n"
            );
            let (kept, found) = excise(&source);
            assert_eq!(found.len(), 1, "{marker}");
            assert_eq!(found[0].scheme, scheme);
            assert_eq!(found[0].line, 2);
            assert!(kept.contains(".model D1") && kept.contains(".model D2"));
            assert!(!kept.contains("b6c1e0"));
            assert!(found[0].message().starts_with("Line 2 opens "));
        }
    }

    #[test]
    fn an_unclosed_region_runs_to_the_end_of_the_file() {
        let (kept, found) = excise(".model D1 D\n$CDNENCSTART\n0011aabb\n");
        assert_eq!(found.len(), 1);
        assert_eq!(kept, ".model D1 D\n\n\n");
    }
}
