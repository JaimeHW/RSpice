//! Locations in an include-expanded deck, read back to the files that wrote
//! them.
//!
//! A prepared run solves its deck's includes expanded in place, so a parse or
//! an elaboration of that text reports line 274 of a text no user can open.
//! The run keeps the expansion's line map ([`ExpandedLineMap`]) and every
//! location a task reports is read through it here, into the `path:line`
//! the CLI reports for the same deck, which parses the files themselves.

use rspice_core::netlist::{ExpandedLineMap, NetlistSourceLocation};
use std::sync::Arc;

use crate::error::SimulationError;

/// The map an expansion kept, when any of its lines came from an included
/// file. A deck that included nothing reads as it always has.
pub(crate) fn included_line_map(lines: ExpandedLineMap) -> Option<Arc<ExpandedLineMap>> {
    let root = lines.locate(1)?.path;
    (1..=lines.len())
        .any(|line| lines.locate(line).is_some_and(|origin| origin.path != root))
        .then(|| Arc::new(lines))
}

/// The part of `lines`, made for the text `mapped`, that still describes
/// `text`, a deck derived from it: all of it when the two differ only within
/// lines (a path rewritten in place), else the lines before the first one
/// that differs (cards spliced before the `.end`). Never the terminal `.end`
/// or anything after it, where a runner splices its own cards.
pub(crate) fn aligned_line_map(
    lines: &Arc<ExpandedLineMap>,
    mapped: &str,
    text: &str,
) -> Option<Arc<ExpandedLineMap>> {
    let line_count = |text: &str| text.bytes().filter(|byte| *byte == b'\n').count();
    let mut kept = if std::ptr::eq(mapped, text) || line_count(mapped) == line_count(text) {
        lines.len()
    } else {
        let common = mapped
            .bytes()
            .zip(text.bytes())
            .take_while(|(a, b)| a == b)
            .count();
        line_count(&text[..common])
    };
    if let Some(end) = crate::netlist_preparation::terminal_end_card_offset(text) {
        kept = kept.min(line_count(&text[..end]));
    }
    match kept {
        0 => None,
        kept if kept >= lines.len() => Some(Arc::clone(lines)),
        kept => Some(Arc::new(lines.truncated(kept))),
    }
}

/// Reads one task's reported locations back to the files that wrote them;
/// a task whose deck included nothing reports what it always has.
#[derive(Debug, Clone, Default)]
pub struct TaskSourceLocator(pub(super) Option<Arc<ExpandedLineMap>>);

impl TaskSourceLocator {
    /// `error` with each location it reports on the expanded deck named by
    /// the file and line that wrote it.
    #[must_use]
    pub fn locate(&self, error: SimulationError) -> SimulationError {
        match &self.0 {
            Some(lines) => error.located_through(lines),
            None => error,
        }
    }
}

impl SimulationError {
    /// This error with every location it reports on the expanded deck read
    /// back to the file and line that wrote it, as the CLI reports it.
    #[must_use]
    pub(crate) fn located_through(self, lines: &ExpandedLineMap) -> Self {
        match self {
            Self::Elaboration {
                instance,
                module,
                kind,
                location: Some(location),
                message,
            } => {
                let Some(origin) = expanded_line(&location).and_then(|line| lines.locate(line))
                else {
                    return Self::Elaboration {
                        instance,
                        module,
                        kind,
                        location: Some(location),
                        message,
                    };
                };
                let origin = origin.to_string();
                let message =
                    match message.strip_prefix(&format!("Elaboration error at {location}: ")) {
                        Some(rest) => format!("Elaboration error at {origin}: {rest}"),
                        None => message,
                    };
                Self::Elaboration {
                    instance,
                    module,
                    kind,
                    location: Some(origin),
                    message,
                }
            }
            Self::ParseError(message) => Self::ParseError(parse_message(message, lines)),
            Self::UnsupportedCapability {
                capability,
                analysis_kind,
                instance,
                module,
                message,
            } => Self::UnsupportedCapability {
                capability,
                analysis_kind,
                instance,
                module,
                message: capability_message(message, lines),
            },
            other => other,
        }
    }
}

/// A parse failure's message, as the manual-deck queue words it (`Netlist
/// parse error: …`), as a parse of the files words it.
pub(crate) fn located_parse_message(message: String, lines: &ExpandedLineMap) -> String {
    const QUEUE: &str = "Netlist parse error: ";
    match message.strip_prefix(QUEUE) {
        Some(rest) => format!("{QUEUE}{}", parse_message(rest.to_owned(), lines)),
        None => parse_message(message, lines),
    }
}

/// `N` of an in-memory location, `line N`.
fn expanded_line(location: &str) -> Option<usize> {
    location.strip_prefix("line ")?.parse().ok()
}

/// The leading `line N` of `text` and what follows its `: `.
fn leading_line(text: &str, prefix: &str) -> Option<(usize, usize)> {
    let rest = text.strip_prefix(prefix)?;
    let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
    let line = rest[..digits].parse().ok()?;
    rest[digits..]
        .starts_with(": ")
        .then_some((line, prefix.len() + digits + 2))
}

fn is_included(origin: &NetlistSourceLocation, lines: &ExpandedLineMap) -> bool {
    origin.path.is_some() && lines.locate(1).is_some_and(|root| root.path != origin.path)
}

/// A parse error's message as a parse of the files words it. A syntax error
/// in an included file names the file and its line there; one in the root
/// keeps the line the parser counted, as the CLI's does. A refused capability
/// names the file and line of its card wherever it is.
fn parse_message(message: String, lines: &ExpandedLineMap) -> String {
    if let Some((line, body)) = leading_line(&message, "Syntax error at line ")
        && let Some(origin) = lines.locate(line)
        && is_included(&origin, lines)
    {
        let prefix = format!("{origin}: ");
        let rest = &message[body..];
        let rest = rest.strip_prefix(&prefix).unwrap_or(rest);
        return format!("Syntax error at line {}: {prefix}{rest}", origin.line);
    }
    capability_message(message, lines)
}

/// A message the parser led with its card's `line N: `: an included card's
/// names its file and line, and so does a refused capability's anywhere.
fn capability_message(message: String, lines: &ExpandedLineMap) -> String {
    let Some((line, body)) = leading_line(&message, "line ") else {
        return message;
    };
    let Some(origin) = lines.locate(line) else {
        return message;
    };
    let rest = &message[body..];
    if is_included(&origin, lines) || rest.starts_with("unsupported capability [") {
        format!("{origin}: {rest}")
    } else {
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    /// `deck.cir` including `sub/a.inc`, which includes `b.inc` beside it,
    /// expanded as a prepared run expands it: 1 deck, 2 a.inc, 3-4 b.inc,
    /// 5 a.inc, 6-7 deck.
    fn expanded() -> (PathBuf, String, Arc<ExpandedLineMap>) {
        let dir = std::env::temp_dir().join(format!(
            "rspice_source_lines_{}_{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(dir.join("sub")).expect("a scratch directory");
        let deck = "nested\n.include sub/a.inc\nR9 a 0 1k\n.end\n";
        std::fs::write(dir.join("sub/a.inc"), "* a\n.include b.inc\nR1 a b 1k\n").unwrap();
        std::fs::write(dir.join("sub/b.inc"), "* b\nR2 b 0 1k\n").unwrap();
        std::fs::write(dir.join("deck.cir"), deck).unwrap();
        // The prepared routes expand from the deck's canonical path.
        let origin = dir.join("deck.cir").canonicalize().unwrap();
        let mut processor = rspice_core::netlist::IncludeProcessor::new(&origin);
        let (text, lines) = processor
            .expand_content_rewriting_with_line_map_and_abort(
                deck,
                &origin,
                &mut |_, _| None,
                &rspice_core::NoAbort,
            )
            .expect("the deck expands");
        let lines = included_line_map(lines).expect("the deck includes files");
        (dir, text, lines)
    }

    fn file(dir: &Path, name: &str, line: usize) -> String {
        NetlistSourceLocation::in_file(dir.join(name).canonicalize().unwrap(), line).to_string()
    }

    #[test]
    fn an_elaboration_refusal_names_the_included_line_that_wrote_the_instance() {
        let (dir, _, lines) = expanded();
        let refusal = |location: &str| SimulationError::Elaboration {
            instance: Some("x1".into()),
            module: Some("m".into()),
            kind: "unknown_module".into(),
            location: Some(location.into()),
            message: format!("Elaboration error at {location}: instance 'x1' (module 'm'): no"),
        };
        let SimulationError::Elaboration {
            location, message, ..
        } = refusal("line 4").located_through(&lines)
        else {
            unreachable!()
        };
        let b2 = file(&dir, "sub/b.inc", 2);
        assert_eq!(location.as_deref(), Some(b2.as_str()));
        assert_eq!(
            message,
            format!("Elaboration error at {b2}: instance 'x1' (module 'm'): no")
        );
        let SimulationError::Elaboration { location, .. } =
            refusal("line 6").located_through(&lines)
        else {
            unreachable!()
        };
        assert_eq!(location, Some(file(&dir, "deck.cir", 3)), "a root line too");
        assert_eq!(
            refusal("line 40").located_through(&lines),
            refusal("line 40"),
            "past the map"
        );
    }

    #[test]
    fn a_syntax_error_in_an_included_file_reads_as_the_clis() {
        let (dir, _, lines) = expanded();
        let parse = |message: &str| {
            let SimulationError::ParseError(message) =
                SimulationError::ParseError(message.into()).located_through(&lines)
            else {
                unreachable!()
            };
            message
        };
        let a3 = file(&dir, "sub/a.inc", 3);
        assert_eq!(
            parse("Syntax error at line 5: Expected value"),
            format!("Syntax error at line 3: {a3}: Expected value")
        );
        assert_eq!(
            parse("Syntax error at line 6: Expected value"),
            "Syntax error at line 6: Expected value",
            "a root line keeps the parser's count, as the CLI's does"
        );
        assert_eq!(
            parse("line 4: Undefined parameter: Z"),
            format!("{}: Undefined parameter: Z", file(&dir, "sub/b.inc", 2))
        );
        assert_eq!(
            parse("line 6: unsupported capability [x]: y"),
            format!(
                "{}: unsupported capability [x]: y",
                file(&dir, "deck.cir", 3)
            )
        );
    }

    #[test]
    fn a_derived_deck_keeps_the_lines_before_what_it_changed_and_never_its_end() {
        let (_, text, lines) = expanded();
        assert_eq!(lines.len(), 7);
        let same = aligned_line_map(&lines, &text, &text).expect("a map");
        assert_eq!(same.len(), 6, "not the .end");
        let rewritten = text.replace("R1 a b 1k", "R1 a b 2k");
        assert_eq!(
            aligned_line_map(&lines, &text, &rewritten).unwrap().len(),
            6
        );
        let spliced =
            crate::netlist_preparation::splice_before_terminal_end_card(&text, ".options temp=30");
        assert_eq!(aligned_line_map(&lines, &text, &spliced).unwrap().len(), 6);
        let inserted = text.replacen("R1 a b 1k\n", "R1 a b 1k\nR8 a 0 1\n", 1);
        assert_eq!(aligned_line_map(&lines, &text, &inserted).unwrap().len(), 5);
    }
}
