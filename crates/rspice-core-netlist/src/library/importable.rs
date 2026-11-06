//! A library source as its import reads it.
//!
//! An import keeps every good definition of a vendor file and skips the rest:
//! an encrypted block, a card that fails to parse, a second definition of a
//! name in its scope. A run reads the same file again, so it reads it through
//! [`importable_source_text`], and the cards it binds are the ones the import
//! listed: a skipped definition can neither fail the run nor stand in for the
//! definition the import kept.

use std::borrow::Cow;
use std::collections::HashSet;
use std::path::Path;

use super::encrypted::excise_encrypted;
use super::lib_parser::LibParser;

/// `content` with every definition its import skips blanked, line for line,
/// so a diagnostic about the rest still names the source's own line. A native
/// Spectre source is lowered before it is parsed, so only its encrypted
/// blocks are blanked.
pub fn importable_source_text<'a>(path: &Path, content: &'a str) -> Cow<'a, str> {
    let (text, _) = excise_encrypted(content, Some(path));
    let skipped = skipped_lines(path, &text);
    if !skipped.contains(&true) {
        return text;
    }
    let mut kept = String::with_capacity(text.len());
    for (line, skip) in text.lines().zip(skipped) {
        if !skip {
            kept.push_str(line);
        }
        kept.push('\n');
    }
    Cow::Owned(kept)
}

/// One flag per line of `text`: whether it belongs to a definition the
/// import skips.
fn skipped_lines(path: &Path, text: &str) -> Vec<bool> {
    let lines = text.lines().collect::<Vec<_>>();
    let mut skipped = vec![false; lines.len()];
    if !matches!(
        super::adapt_spectre_model_library(path, text),
        Ok(Cow::Borrowed(_))
    ) {
        return skipped;
    }
    let result = LibParser::parse_single_source(path, text);
    let broken = result
        .errors
        .iter()
        .filter(|error| error.definition.is_some())
        .filter_map(|error| error.line);
    // The first definition of a name in a scope is the one the import keeps.
    let repeated = std::iter::once(&result.top_level_subcircuits)
        .chain(result.sections.iter().map(|section| &section.subcircuits))
        .flat_map(|subcircuits| {
            let mut seen = HashSet::new();
            subcircuits
                .iter()
                .filter(move |subcircuit| !seen.insert(subcircuit.name.to_ascii_lowercase()))
                .filter_map(|subcircuit| subcircuit.source_line)
        });
    for line in broken.chain(repeated) {
        if let Some(start) = line.checked_sub(1) {
            mark_definition(&lines, start, &mut skipped);
        }
    }
    skipped
}

/// Mark the definition whose first line is `start`. A `.subckt` runs through
/// its `.ends`, or up to the section boundary or the end of the source when
/// it has none; any other card runs through its last `+` continuation, past
/// the comments between them.
fn mark_definition(lines: &[&str], start: usize, skipped: &mut [bool]) {
    let Some(first) = lines.get(start) else {
        return;
    };
    let directive = |line: &str| {
        line.split_whitespace()
            .next()
            .map(str::to_ascii_lowercase)
            .unwrap_or_default()
    };
    let mut end = start;
    if directive(first) == ".subckt" {
        for (index, line) in lines.iter().enumerate().skip(start + 1) {
            let directive = directive(line);
            if directive.starts_with(".ends") {
                end = index;
                break;
            }
            if directive == ".lib" || directive.starts_with(".endl") {
                break;
            }
            end = index;
        }
    } else {
        for (index, line) in lines.iter().enumerate().skip(start + 1) {
            let line = line.trim();
            if line.starts_with('+') {
                end = index;
            } else if !(line.is_empty() || line.starts_with('*')) {
                break;
            }
        }
    }
    skipped[start..=end].fill(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn importable(source: &str) -> String {
        importable_source_text(Path::new("/lib/vendor.lib"), source).into_owned()
    }

    #[test]
    fn a_source_with_nothing_to_skip_is_read_as_written() {
        let source = "* vendor\n.model D1 D (IS=1e-14)\n.subckt HALF IN OUT\nR1 IN OUT 1k\n\
                      R2 OUT 0 1k\n.ends HALF\n";
        assert!(matches!(
            importable_source_text(Path::new("/lib/vendor.lib"), source),
            Cow::Borrowed(text) if text == source
        ));
    }

    #[test]
    fn a_broken_card_goes_with_its_continuations_and_its_neighbours_stay() {
        let source = ".model GOOD D (IS=1e-14)\n.model BROKEN\n* the vendor's note\n\
                      + IS=1e-14\n.subckt HALF IN OUT\nR1 IN OUT 1k\n.ends HALF\n";
        let kept = importable(source);
        assert_eq!(kept.lines().count(), source.lines().count());
        assert_eq!(
            kept,
            ".model GOOD D (IS=1e-14)\n\n\n\n.subckt HALF IN OUT\nR1 IN OUT 1k\n.ends HALF\n"
        );
    }

    #[test]
    fn a_subcircuit_with_a_broken_header_goes_through_its_ends() {
        let source = ".subckt BROKEN\nR1 A B 1k\n.ends BROKEN\n.subckt OPEN A B\nR1 A B 1k\n\
                      .ends OPEN\n";
        assert_eq!(
            importable(source),
            "\n\n\n.subckt OPEN A B\nR1 A B 1k\n.ends OPEN\n"
        );
    }

    #[test]
    fn a_repeated_subcircuit_keeps_its_first_definition_in_each_section() {
        let source = ".lib TT\n.subckt AMP A B\nR1 A B 1k\n.ends AMP\n.subckt amp A B\n\
                      R1 A B 2k\n.ends amp\n.endl TT\n.lib FF\n.subckt AMP A B\nR1 A B 3k\n\
                      .ends AMP\n.endl FF\n";
        let kept = importable(source);
        assert!(
            kept.contains("R1 A B 1k") && kept.contains("R1 A B 3k"),
            "{kept}"
        );
        assert!(!kept.contains("R1 A B 2k"), "{kept}");
        assert!(kept.contains(".endl TT\n.lib FF\n"), "{kept}");
    }

    #[test]
    fn an_encrypted_block_is_blanked_with_the_rest() {
        let source = ".model D1 D (IS=1e-14)\n$CDNENCSTART\nb6c1e0f4a9d27735e81c\n\
                      $CDNENCFINISH\n.model BROKEN\n.model D2 D (IS=2e-14)\n";
        assert_eq!(
            importable(source),
            ".model D1 D (IS=1e-14)\n\n\n\n\n.model D2 D (IS=2e-14)\n"
        );
    }
}
