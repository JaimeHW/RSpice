//! Structured diagnostics for the netlist editor: the lints and parser
//! adapters that produce the canonical records in
//! [`crate::state::netlist_diagnostics`].

use std::ops::Range;

use rspice_core::netlist::{NetlistDefinition, NetlistReference, ReferenceKind};

use crate::state::netlist_diagnostics::{
    Diagnostic, DiagnosticFix, NetlistDiagnosticCollection, line_column_for_span,
};

/// Severity levels shown by the gutter, strip, and syntax highlighter: the
/// shared source-diagnostic severity, so an editor finding and a compiler
/// service finding rank and serialize alike.
pub use rspice_vocabulary::diagnostics::DiagnosticSeverity;

/// Carry diagnostics of a composed narrow-override deck back into the
/// override's own text.
///
/// A narrow override executes as the generated base with the override's
/// lines spliced in before the base's `.end`, so that is the deck the editor
/// parses and every line, column and byte range the parse reports is a
/// position in it. The override is what the editor shows and what a fix
/// edits, so a position taken over unchanged rewrote the wrong bytes of the
/// reader's file and decorated the wrong lines.
///
/// The splice re-terminates every line in the base's own line ending, so the
/// override's bytes are not one span of the composed deck — but each of its
/// lines is one whole line of it, the block that ends at the composed deck's
/// terminal `.end`. Positions are carried line by line. A diagnostic about
/// the generated base has no place in this text: it keeps its message, says
/// where it is, and loses its range and its fix.
pub(super) fn in_authored_text(
    diagnostics: Vec<Diagnostic>,
    composed: &str,
    authored: &str,
) -> Vec<Diagnostic> {
    let composed_starts: Vec<usize> = std::iter::once(0)
        .chain(composed.match_indices('\n').map(|(index, _)| index + 1))
        .collect();
    // Each authored line as the splice sees it: its start, and the length of
    // its text without the line ending `str::lines` drops.
    let mut authored_lines = Vec::new();
    let mut start = 0;
    for line in authored.lines() {
        authored_lines.push((start, line.len()));
        start = authored[start..]
            .find('\n')
            .map_or(authored.len(), |newline| start + newline + 1);
    }
    let end_line = rspice_simulation::netlist_preparation::terminal_end_card_offset(composed)
        .map(|end| composed[..end].matches('\n').count());
    let first = end_line.and_then(|end| end.checked_sub(authored_lines.len()));
    let authored_line = |composed_line: usize| {
        first
            .zip(end_line)
            .filter(|(first, end)| (*first..*end).contains(&composed_line))
            .map(|(first, _)| composed_line - first)
    };
    let line_of = |offset: usize| composed_starts.partition_point(|start| *start <= offset) - 1;
    let offset_in_authored = |offset: usize| {
        let line = line_of(offset);
        let (start, length) = authored_lines[authored_line(line)?];
        Some(start + (offset - composed_starts[line]).min(length))
    };
    let range_in_authored = |range: &Range<usize>| {
        let start = offset_in_authored(range.start)?;
        let end = if range.end > range.start {
            offset_in_authored(range.end - 1)? + 1
        } else {
            start
        };
        Some(start..end.max(start))
    };

    diagnostics
        .into_iter()
        .map(|mut diagnostic| {
            let Some(line) = diagnostic
                .line
                .or_else(|| diagnostic.span.as_ref().map(|span| line_of(span.start)))
            else {
                // Not located in the root deck at all: an included source's
                // own line, or no place.
                return diagnostic;
            };
            match authored_line(line) {
                Some(own_line) => {
                    if diagnostic.source_line == diagnostic.line {
                        diagnostic.source_line = diagnostic.line.map(|_| own_line);
                    }
                    diagnostic.line = diagnostic.line.map(|_| own_line);
                    diagnostic.span = diagnostic.span.as_ref().and_then(range_in_authored);
                    diagnostic.fix = diagnostic.fix.take().and_then(|fix| {
                        range_in_authored(&fix.span).map(|span| DiagnosticFix { span, ..fix })
                    });
                }
                None => {
                    diagnostic.message = format!("{} (in the generated base)", diagnostic.message);
                    diagnostic.line = None;
                    diagnostic.source_line = None;
                    diagnostic.column = None;
                    diagnostic.span = None;
                    diagnostic.fix = None;
                }
            }
            if diagnostic.fix.is_none() {
                // The canonical record keeps the fixes it already had when
                // there is no new one to derive, and the ones it has are
                // ranges of the composed deck.
                diagnostic.canonical.quick_fixes.clear();
            }
            diagnostic.refresh_canonical_location();
            diagnostic
        })
        .collect()
}

/// One finding per model or subcircuit reference in `buffer` that no
/// definition visible to it names.
///
/// `unread` is what the parse of `buffer` did not read: a definition there
/// defines nothing, and a reference there is not one.
pub(super) fn unknown_reference_diagnostics(
    buffer: &str,
    unread: &[rspice_core::netlist::UnreadSourceRegion],
) -> Vec<Diagnostic> {
    if has_external_model_sources(buffer) {
        return Vec::new();
    }

    let map = rspice_core::netlist::source_map_for_editor_as_read(buffer, unread);
    let mut diagnostics = Vec::new();

    for reference in map.references {
        let definitions = match reference.kind {
            ReferenceKind::Model => &map.model_defs,
            ReferenceKind::Subcircuit => &map.subckt_defs,
        };
        let visible = visible_definitions(&reference, definitions);
        if visible
            .iter()
            .any(|definition| definition.name.eq_ignore_ascii_case(&reference.name))
        {
            continue;
        }

        let replacement = nearest_definition(&reference.name, &visible);
        let (line, column) = line_column_for_span(buffer, reference.span.start);
        let kind = match reference.kind {
            ReferenceKind::Model => "model",
            ReferenceKind::Subcircuit => "subcircuit",
        };
        let fix = replacement.as_ref().map(|replacement| DiagnosticFix {
            label: format!("Replace with {replacement}"),
            span: reference.span.clone(),
            replacement: replacement.clone(),
        });

        let mut diagnostic = Diagnostic::current(
            "rspice.netlist.semantic",
            match reference.kind {
                ReferenceKind::Model => "SPICE-UNKNOWN-MODEL",
                ReferenceKind::Subcircuit => "SPICE-UNKNOWN-SUBCIRCUIT",
            },
            DiagnosticSeverity::Error,
            format!("Unknown {kind} `{}` in this deck.", reference.name),
        );
        diagnostic.source_line = Some(line);
        diagnostic.span = Some(reference.span);
        diagnostic.line = Some(line);
        diagnostic.column = Some(column);
        diagnostic.fix = fix;
        diagnostic.refresh_canonical_location();
        diagnostics.push(diagnostic);
    }

    diagnostics
}

/// Canonical code for a diagnostic the engine's parser produced.
///
/// A dropped `.control` command keeps its own code so the deck surface can
/// count it without re-reading the buffer or knowing the engine's spelling.
pub(super) const CONTROL_DROPPED_CODE: &str = "SPICE-CONTROL-DROPPED";

/// Canonical code for a `.control` command the engine promoted into a
/// declarative directive.
pub(super) const CONTROL_PROMOTED_CODE: &str = "SPICE-CONTROL-PROMOTED";

pub(super) fn parser_diagnostics(
    netlist: &rspice_core::Netlist,
    editor_source_path: Option<&std::path::Path>,
) -> Vec<Diagnostic> {
    let origin_of = |line: usize,
                     origin: Option<&rspice_core::netlist::NetlistSourceLocation>|
     -> rspice_core::netlist::NetlistSourceLocation {
        origin
            .cloned()
            .unwrap_or_else(|| match &netlist.source_path {
                Some(path) => rspice_core::netlist::NetlistSourceLocation::in_file(path, line),
                None => rspice_core::netlist::NetlistSourceLocation::in_memory(line),
            })
    };
    let mut diagnostics = netlist
        .diagnostics
        .iter()
        .map(|diagnostic| {
            let origin = origin_of(diagnostic.line, diagnostic.origin.as_ref());
            Diagnostic::current(
                "rspice.netlist.parser",
                if diagnostic.code == "control-command-dropped" {
                    CONTROL_DROPPED_CODE
                } else {
                    "SPICE-PARSER-WARNING"
                },
                match diagnostic.severity {
                    rspice_core::netlist::DiagnosticSeverity::Warning => {
                        DiagnosticSeverity::Warning
                    }
                },
                diagnostic.message.clone(),
            )
            .with_source_location(&origin, editor_source_path)
        })
        .collect::<Vec<_>>();
    // A promotion is not a warning, and the engine's diagnostic stream has no
    // informational level to carry it. The editor reads the structured record
    // instead, so the line that authored a promoted command still says what
    // became of it.
    diagnostics.extend(
        netlist
            .control_dispositions
            .iter()
            .filter_map(|record| match &record.disposition {
                rspice_core::netlist::ControlCommandDisposition::Promoted { directives } => {
                    Some((record, directives))
                }
                _ => None,
            })
            .map(|(record, directives)| {
                let origin = origin_of(record.line, record.origin.as_ref());
                Diagnostic::current(
                    "rspice.netlist.parser",
                    CONTROL_PROMOTED_CODE,
                    DiagnosticSeverity::Info,
                    format!(
                        "Control command `{}` was promoted into `{}`.",
                        record.command,
                        directives.join("` `")
                    ),
                )
                .with_source_location(&origin, editor_source_path)
            }),
    );
    diagnostics
}

/// How many `.control` commands the active parse promoted and how many it
/// ignored, with the first few dispositions for the band's hover.
///
/// Derived from the published diagnostics rather than re-read from the buffer,
/// so the band, the gutter pip and the Problems row all cite one parse.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ControlDispositionSummary {
    /// Commands lifted out as declarative directives.
    pub promoted: usize,
    /// Commands the engine read and ignored.
    pub dropped: usize,
    /// Leading disposition messages, in buffer order.
    pub examples: Vec<String>,
}

/// Longest hover list the band paints before it stops enumerating.
const CONTROL_SUMMARY_EXAMPLES: usize = 4;

/// Summarize the control-region dispositions in a published collection.
///
/// Returns `None` when the parse recorded no control region at all, which is
/// the state in which the band must not be painted.
#[must_use]
pub fn control_disposition_summary(
    diagnostics: &NetlistDiagnosticCollection,
) -> Option<ControlDispositionSummary> {
    let mut summary = ControlDispositionSummary::default();
    let mut examples = Vec::new();
    for diagnostic in diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_current())
    {
        match diagnostic.canonical.code.as_ref() {
            CONTROL_PROMOTED_CODE => summary.promoted += 1,
            CONTROL_DROPPED_CODE => summary.dropped += 1,
            _ => continue,
        }
        // Promotions and drops arrive as two runs; the hover reads as a walk
        // down the block, so order by the line that authored each command.
        examples.push((
            diagnostic.line.or(diagnostic.source_line),
            match diagnostic.line.or(diagnostic.source_line) {
                Some(line) => format!("line {} · {}", line + 1, diagnostic.message),
                None => diagnostic.message.clone(),
            },
        ));
    }
    if summary.promoted + summary.dropped == 0 {
        return None;
    }
    examples.sort_by_key(|(line, _)| *line);
    summary.examples = examples
        .into_iter()
        .take(CONTROL_SUMMARY_EXAMPLES)
        .map(|(_, message)| message)
        .collect();
    Some(summary)
}

fn visible_definitions<'a>(
    reference: &NetlistReference,
    definitions: &'a [NetlistDefinition],
) -> Vec<&'a NetlistDefinition> {
    definitions
        .iter()
        .filter(|definition| definition_visible(reference, definition))
        .collect()
}

fn definition_visible(reference: &NetlistReference, definition: &NetlistDefinition) -> bool {
    match reference.kind {
        ReferenceKind::Model => definition
            .scope
            .as_deref()
            .is_none_or(|scope| scope_visible_from(scope, reference.scope.as_deref())),
        ReferenceKind::Subcircuit => {
            definition.scope.is_none() || definition.scope == reference.scope
        }
    }
}

fn scope_visible_from(definition_scope: &str, reference_scope: Option<&str>) -> bool {
    let Some(reference_scope) = reference_scope else {
        return false;
    };
    reference_scope == definition_scope
        || reference_scope
            .strip_prefix(definition_scope)
            .is_some_and(|rest| rest.starts_with('.'))
}

fn nearest_definition(reference: &str, definitions: &[&NetlistDefinition]) -> Option<String> {
    definitions
        .iter()
        .filter_map(|candidate| {
            let distance = levenshtein(
                &reference.to_ascii_lowercase(),
                &candidate.name.to_ascii_lowercase(),
            );
            (distance <= 2).then_some((distance, candidate.name.clone()))
        })
        .min_by(|(a_distance, a_name), (b_distance, b_name)| {
            a_distance.cmp(b_distance).then_with(|| a_name.cmp(b_name))
        })
        .map(|(_, candidate)| candidate)
}

fn levenshtein(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.chars().count();
    }
    if b.is_empty() {
        return a.chars().count();
    }

    let b_chars = b.chars().collect::<Vec<_>>();
    let mut previous = (0..=b_chars.len()).collect::<Vec<_>>();
    let mut current = vec![0usize; b_chars.len() + 1];

    for (i, a_ch) in a.chars().enumerate() {
        current[0] = i + 1;
        for (j, b_ch) in b_chars.iter().enumerate() {
            let substitution = usize::from(a_ch != *b_ch);
            current[j + 1] = (previous[j + 1] + 1)
                .min(current[j] + 1)
                .min(previous[j] + substitution);
        }
        std::mem::swap(&mut previous, &mut current);
    }

    previous[b_chars.len()]
}

/// Whether the deck defines masters in a file this lint never reads.
///
/// The reference scan sees only the buffer, so a card naming something the
/// buffer does not spell is an unknown reference — unless the deck imports
/// definitions from elsewhere, in which case reporting one is a false positive
/// on every name the imported file supplies. `.include`/`.inc`/`.lib` bring in
/// `.subckt` and `.model` cards; a Verilog-A source directive brings in the
/// modules its file declares, which is the same claim on the master namespace.
///
/// The Verilog-A spellings come from the parser that owns them, so a spelling
/// added there is honoured here rather than listed twice. A malformed
/// directive parses as no directive at all, which costs nothing: this lint
/// only runs once the deck has parsed.
fn has_external_model_sources(buffer: &str) -> bool {
    buffer.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            return false;
        }
        let head = trimmed.split_whitespace().next().unwrap_or_default();
        head.eq_ignore_ascii_case(".include")
            || head.eq_ignore_ascii_case(".inc")
            || head.eq_ignore_ascii_case(".lib")
            || rspice_core::netlist::parse_veriloga_source_directive(trimmed).is_some()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Positions in a composed override deck are carried back into the
    /// override's own text, line by line — here an override written with
    /// CRLF endings spliced into a base that uses LF, so the override's bytes
    /// are not one span of the composed deck. A finding in the base keeps no
    /// range and no fix in this text, and says where it is.
    #[test]
    fn composed_positions_are_carried_back_into_the_override() {
        let base = "deck\nM2 d g s b pchh W=1u L=1u\n.model nch nmos\n.model pch pmos\n.end\n";
        let authored = "* override\r\nM1 d g s b nchh W=1u L=1u\r\n";
        let composed =
            rspice_simulation::netlist_preparation::splice_before_terminal_end_card(base, authored);

        let diagnostics = in_authored_text(
            unknown_reference_diagnostics(&composed, &[]),
            &composed,
            authored,
        );
        assert_eq!(diagnostics.len(), 2, "{diagnostics:?}");

        let own = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.message.contains("nchh"))
            .expect("the override's own finding");
        assert_eq!(own.line, Some(1), "the override's second line");
        assert_eq!(&authored[own.span.clone().unwrap()], "nchh");
        let fix = own.fix.as_ref().expect("its fix is kept");
        assert_eq!(&authored[fix.span.clone()], "nchh");
        let mut fixed = authored.to_owned();
        fixed.replace_range(fix.span.clone(), &fix.replacement);
        assert_eq!(fixed, "* override\r\nM1 d g s b nch W=1u L=1u\r\n");

        let base_finding = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.message.contains("pchh"))
            .expect("the base's finding");
        assert!(
            base_finding.message.ends_with("(in the generated base)"),
            "{}",
            base_finding.message,
        );
        assert_eq!(base_finding.line, None);
        assert_eq!(base_finding.source_line, None);
        assert!(base_finding.span.is_none() && base_finding.fix.is_none());
        assert!(
            base_finding.canonical.quick_fixes.is_empty(),
            "no fix of the composed deck survives into this text",
        );
    }

    #[test]
    fn unknown_model_lint_suggests_nearest_known_model() {
        let src = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src, &[]);

        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(&src[diagnostic.span.clone().unwrap()], "nchh");
        assert_eq!(diagnostic.line, Some(1));
        assert!(diagnostic.message.contains("nchh"));
        let fix = diagnostic.fix.as_ref().unwrap();
        assert_eq!(fix.replacement, "nch");
        assert_eq!(&src[fix.span.clone()], "nchh");
        assert_eq!(diagnostic.canonical.code.as_ref(), "SPICE-UNKNOWN-MODEL");
        assert_eq!(diagnostic.canonical.quick_fixes.len(), 1);
        assert_eq!(
            diagnostic.canonical.quick_fixes[0].edits[0].replacement,
            "nch"
        );
        let serialized = serde_json::to_value(diagnostic).unwrap();
        for field in [
            "diagnosticId",
            "code",
            "severity",
            "source",
            "documentId",
            "range",
            "message",
            "details",
            "relatedLocations",
            "quickFixes",
            "suppression",
            "revision",
            "validationId",
            "currentness",
            "affectedConsumers",
        ] {
            assert!(serialized.get(field).is_some(), "missing {field}");
        }
    }

    #[test]
    fn unknown_subckt_lint_suggests_nearest_known_subckt() {
        let src = "deck\nX1 in out invv\n.subckt inv in out\n.ends\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src, &[]);

        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(&src[diagnostic.span.clone().unwrap()], "invv");
        assert!(diagnostic.message.contains("invv"));
        assert_eq!(diagnostic.fix.as_ref().unwrap().replacement, "inv");
    }

    #[test]
    fn unknown_reference_lint_skips_decks_with_external_includes() {
        let src = "deck\n.include models.scs\nM1 d g s b nchh W=1u L=1u\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src, &[]);

        assert!(diagnostics.is_empty());
    }

    /// A Verilog-A source directive declares masters the buffer never spells,
    /// exactly as `.include` declares subcircuits it never spells. Every
    /// spelling of it has to silence the unknown-master lint, or a deck that
    /// compiles its own device reads as a deck full of typos.
    #[test]
    fn unknown_reference_lint_skips_decks_with_a_veriloga_source() {
        let bare = "deck\nX1 a k my_diode\n.end\n";
        assert_eq!(
            unknown_reference_diagnostics(bare, &[]).len(),
            1,
            "a master nothing declares is still reported"
        );

        for directive in [".va", ".veriloga", ".hdl", ".vams", ".verilog"] {
            for cased in [
                directive.to_ascii_lowercase(),
                directive.to_ascii_uppercase(),
            ] {
                let src = format!("deck\n{cased} \"models/dev.va\"\nX1 a k my_diode\n.end\n");
                assert!(
                    unknown_reference_diagnostics(&src, &[]).is_empty(),
                    "{cased} declares the master"
                );
            }
        }

        // A card that only looks like one of them is not one.
        let near_miss = "deck\n.vams2 \"models/dev.va\"\nX1 a k my_diode\n.end\n";
        assert_eq!(unknown_reference_diagnostics(near_miss, &[]).len(), 1);
    }

    #[test]
    fn unknown_model_lint_does_not_resolve_sibling_local_model() {
        let src = "deck\n.subckt amp in out\n.model nch nmos\n.ends\n.subckt buf in out\nM1 out in 0 0 nch\n.ends\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src, &[]);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(&src[diagnostics[0].span.clone().unwrap()], "nch");
        assert!(diagnostics[0].fix.is_none());
    }

    #[test]
    fn parser_warnings_convert_to_editor_diagnostics() {
        let src = "deck\nV1 in 0 1\nR1 in 0 1k\n.options vendorcompat=1\n.end\n";
        let netlist = rspice_core::Netlist::parse(src).expect("deck parses with warning");
        let diagnostics = parser_diagnostics(&netlist, None);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diagnostics[0].line, Some(3));
        assert!(
            diagnostics[0]
                .message
                .to_ascii_lowercase()
                .contains("vendorcompat")
        );
    }
}
