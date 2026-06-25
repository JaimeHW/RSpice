//! Structured diagnostics for the netlist editor.

use std::ops::Range;

use rspice_core::netlist::{NetlistDefinition, NetlistReference, ReferenceKind};

/// Severity levels shown by the gutter, strip, and syntax highlighter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A single quick fix candidate for a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticFix {
    pub label: String,
    pub span: Range<usize>,
    pub replacement: String,
}

/// One structured problem reported for the current editor buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    /// Byte span in the editor buffer, when known.
    pub span: Option<Range<usize>>,
    /// 0-based buffer line, when localized.
    pub line: Option<usize>,
    /// 0-based UTF-8 byte column within `line`, when localized.
    pub column: Option<usize>,
    pub message: String,
    pub fix: Option<DiagnosticFix>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            span: None,
            line: None,
            column: None,
            message: message.into(),
            fix: None,
        }
    }

    pub fn with_line(mut self, line: Option<usize>) -> Self {
        self.line = line;
        self
    }
}

/// Convert a byte offset into a 0-based `(line, column)` pair.
pub fn line_column_for_span(buffer: &str, offset: usize) -> (usize, usize) {
    let target = offset.min(buffer.len());
    let mut line = 0usize;
    let mut line_start = 0usize;

    for (idx, byte) in buffer.as_bytes().iter().enumerate() {
        if idx >= target {
            break;
        }
        if *byte == b'\n' {
            line += 1;
            line_start = idx + 1;
        }
    }

    (line, target.saturating_sub(line_start))
}

pub(super) fn unknown_reference_diagnostics(buffer: &str) -> Vec<Diagnostic> {
    if has_external_model_sources(buffer) {
        return Vec::new();
    }

    let map = rspice_core::netlist::source_map_for_editor(buffer);
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

        diagnostics.push(Diagnostic {
            severity: DiagnosticSeverity::Error,
            span: Some(reference.span),
            line: Some(line),
            column: Some(column),
            message: format!("Unknown {kind} `{}` in this deck.", reference.name),
            fix,
        });
    }

    diagnostics
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_column_for_span_counts_zero_based_lines_and_columns() {
        assert_eq!(line_column_for_span("a\nbc\n", 3), (1, 1));
    }

    #[test]
    fn line_column_for_span_clamps_offsets_after_eof() {
        assert_eq!(line_column_for_span("a\nbc", 99), (1, 2));
    }

    #[test]
    fn unknown_model_lint_suggests_nearest_known_model() {
        let src = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src);

        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(&src[diagnostic.span.clone().unwrap()], "nchh");
        assert_eq!(diagnostic.line, Some(1));
        assert!(diagnostic.message.contains("nchh"));
        let fix = diagnostic.fix.as_ref().unwrap();
        assert_eq!(fix.replacement, "nch");
        assert_eq!(&src[fix.span.clone()], "nchh");
    }

    #[test]
    fn unknown_subckt_lint_suggests_nearest_known_subckt() {
        let src = "deck\nX1 in out invv\n.subckt inv in out\n.ends\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src);

        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(&src[diagnostic.span.clone().unwrap()], "invv");
        assert!(diagnostic.message.contains("invv"));
        assert_eq!(diagnostic.fix.as_ref().unwrap().replacement, "inv");
    }

    #[test]
    fn unknown_reference_lint_skips_decks_with_external_includes() {
        let src = "deck\n.include models.scs\nM1 d g s b nchh W=1u L=1u\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn unknown_model_lint_does_not_resolve_sibling_local_model() {
        let src = "deck\n.subckt amp in out\n.model nch nmos\n.ends\n.subckt buf in out\nM1 out in 0 0 nch\n.ends\n.end\n";

        let diagnostics = unknown_reference_diagnostics(src);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(&src[diagnostics[0].span.clone().unwrap()], "nch");
        assert!(diagnostics[0].fix.is_none());
    }

    #[test]
    fn parser_warnings_convert_to_editor_diagnostics() {
        let src = "deck\nV1 in 0 1\nR1 in 0 1k\n.options vendorcompat=1\n.end\n";
        let netlist = rspice_core::Netlist::parse(src).expect("deck parses with warning");
        let diagnostics = parser_diagnostics(&netlist);

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
