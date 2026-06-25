use std::ops::Range;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticFix {
    pub label: String,
    pub span: Range<usize>,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub span: Option<Range<usize>>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
    pub fix: Option<DiagnosticFix>,
}

pub(super) fn line_column_for_span(buffer: &str, offset: usize) -> (usize, usize) {
    let mut line = 0usize;
    let mut line_start = 0usize;
    for (idx, ch) in buffer.char_indices() {
        if idx >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            line_start = idx + ch.len_utf8();
        }
    }
    (line, offset.saturating_sub(line_start))
}

pub(super) fn unknown_reference_diagnostics(buffer: &str) -> Vec<Diagnostic> {
    if contains_external_include(buffer) {
        return Vec::new();
    }

    let Ok(netlist) = rspice_core::Netlist::parse(buffer) else {
        return Vec::new();
    };
    let map = netlist.source_map();
    let mut model_names = definition_names_from_source(buffer, ".model");
    model_names.extend(netlist.models.iter().map(|model| model.name.clone()));
    let mut subckt_names = definition_names_from_source(buffer, ".subckt");
    subckt_names.extend(netlist.subcircuits.iter().map(|subckt| subckt.name.clone()));
    for include in &netlist.veriloga_includes {
        if let Some(name) = &include.model_name {
            model_names.push(name.clone());
            subckt_names.push(name.clone());
        }
    }

    map.lint_unknown_references(&netlist)
        .into_iter()
        .filter_map(|diagnostic| {
            let start =
                byte_offset_for_line_column(buffer, diagnostic.range.line, diagnostic.range.start)?;
            let end =
                byte_offset_for_line_column(buffer, diagnostic.range.line, diagnostic.range.end)
                    .unwrap_or(start + diagnostic.name.len());
            let span = start..end;
            let candidates = match diagnostic.kind {
                rspice_core::netlist::UnknownReferenceKind::Model => &model_names,
                rspice_core::netlist::UnknownReferenceKind::Subcircuit => &subckt_names,
            };
            let fix = nearest_name(&diagnostic.name, candidates).map(|replacement| DiagnosticFix {
                label: format!("Replace with {replacement}"),
                span: span.clone(),
                replacement,
            });
            let (line, column) = line_column_for_span(buffer, start);
            Some(Diagnostic {
                severity: DiagnosticSeverity::Error,
                span: Some(span),
                line: Some(line),
                column: Some(column),
                message: diagnostic.message,
                fix,
            })
        })
        .collect()
}

pub(super) fn parser_diagnostics(netlist: &rspice_core::Netlist) -> Vec<Diagnostic> {
    netlist
        .diagnostics
        .iter()
        .map(|diagnostic| Diagnostic {
            severity: match diagnostic.severity {
                rspice_core::netlist::DiagnosticSeverity::Warning => DiagnosticSeverity::Warning,
            },
            span: None,
            line: diagnostic.line.checked_sub(1),
            column: None,
            message: diagnostic.message.clone(),
            fix: None,
        })
        .collect()
}

fn contains_external_include(buffer: &str) -> bool {
    buffer.lines().any(|line| {
        let trimmed = line.trim_start();
        dot_command_matches(trimmed, ".include") || dot_command_matches(trimmed, ".lib")
    })
}

fn dot_command_matches(line: &str, command: &str) -> bool {
    if line.len() < command.len() || !line[..command.len()].eq_ignore_ascii_case(command) {
        return false;
    }
    line[command.len()..]
        .chars()
        .next()
        .is_none_or(char::is_whitespace)
}

fn definition_names_from_source(buffer: &str, command: &str) -> Vec<String> {
    buffer
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if !dot_command_matches(trimmed, command) {
                return None;
            }
            trimmed.split_whitespace().nth(1).map(str::to_string)
        })
        .collect()
}

fn byte_offset_for_line_column(buffer: &str, line: usize, column: usize) -> Option<usize> {
    let mut offset = 0usize;
    for (idx, text) in buffer.split_inclusive('\n').enumerate() {
        let line_text = text.strip_suffix('\n').unwrap_or(text);
        if idx + 1 == line {
            return Some(offset + column.min(line_text.len()));
        }
        offset += text.len();
    }
    None
}

fn nearest_name(name: &str, candidates: &[String]) -> Option<String> {
    candidates
        .iter()
        .map(|candidate| (levenshtein_ci(name, candidate), candidate))
        .filter(|(distance, _)| *distance <= 2)
        .min_by_key(|(distance, candidate)| (*distance, candidate.len()))
        .map(|(_, candidate)| candidate.clone())
}

fn levenshtein_ci(a: &str, b: &str) -> usize {
    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];

    for (i, ca) in a.bytes().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.bytes().enumerate() {
            let substitution = prev[j] + usize::from(ca != cb);
            let insertion = cur[j] + 1;
            let deletion = prev[j + 1] + 1;
            cur[j + 1] = substitution.min(insertion).min(deletion);
        }
        std::mem::swap(&mut prev, &mut cur);
    }

    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_column_maps_byte_offsets() {
        assert_eq!(line_column_for_span("a\nbc\n", 3), (1, 1));
    }

    #[test]
    fn unknown_model_lint_suggests_nearest_known_model() {
        let src = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
        let diagnostics = unknown_reference_diagnostics(src);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("NCHH"));
        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(diagnostics[0].fix.as_ref().unwrap().replacement, "nch");
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
