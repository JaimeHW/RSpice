//! Structured diagnostics for the netlist editor.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use rspice_core::netlist::{NetlistDefinition, NetlistReference, ReferenceKind};

pub const MAX_NETLIST_DIAGNOSTICS: usize = 1_000_000;

/// Severity levels shown by the gutter, strip, and syntax highlighter.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum DiagnosticSeverity {
    #[serde(rename = "hint")]
    Hint,
    #[serde(rename = "information")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

/// A single quick fix candidate for a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticFix {
    pub label: String,
    pub span: Range<usize>,
    pub replacement: String,
}

/// One structured problem reported for the current editor buffer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    #[serde(flatten)]
    pub(crate) canonical:
        crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata,
    pub severity: DiagnosticSeverity,
    /// Physical source that owns this diagnostic. `None` identifies the
    /// in-memory editor buffer. Included-source diagnostics retain their path
    /// and deliberately do not masquerade as a line in the root buffer.
    #[serde(rename = "logicalPath")]
    pub source_path: Option<PathBuf>,
    /// Zero-based physical line in `source_path`, including for an included
    /// source that is not the active editor buffer.
    #[serde(rename = "sourceLine")]
    pub source_line: Option<usize>,
    /// Byte span in the editor buffer, when known.
    #[serde(rename = "byteRange")]
    pub span: Option<Range<usize>>,
    /// 0-based buffer line, when localized.
    pub line: Option<usize>,
    /// 0-based UTF-8 byte column within `line`, when localized.
    pub column: Option<usize>,
    pub message: String,
    pub details: String,
    pub fix: Option<DiagnosticFix>,
}

impl Diagnostic {
    pub fn current(
        producer: impl Into<String>,
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        Self {
            canonical: crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata::current(
                std::sync::Arc::<str>::from(producer.into()),
                std::sync::Arc::<str>::from(code.into()),
                "netlist-editor",
                None,
                0,
                "unbound-validation",
                &message,
            ),
            severity,
            source_path: None,
            source_line: None,
            span: None,
            line: None,
            column: None,
            message,
            details: String::new(),
            fix: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::current(
            "rspice.netlist.parser",
            "SPICE-PARSE-ERROR",
            DiagnosticSeverity::Error,
            message,
        )
    }

    pub fn with_line(mut self, line: Option<usize>) -> Self {
        self.line = line;
        self.source_line = line;
        self.refresh_canonical_location();
        self
    }

    pub fn with_source_location(
        mut self,
        origin: &rspice_core::netlist::NetlistSourceLocation,
        editor_source_path: Option<&std::path::Path>,
    ) -> Self {
        self.source_path = origin.path.clone();
        self.source_line = origin.line.checked_sub(1);
        self.line = if origin.path.as_deref().is_none()
            || same_source(origin.path.as_deref(), editor_source_path)
        {
            origin.line.checked_sub(1)
        } else {
            None
        };
        self.refresh_canonical_location();
        self
    }

    pub fn with_source_path(mut self, source_path: Option<PathBuf>) -> Self {
        self.source_path = source_path;
        self.refresh_canonical_location();
        self
    }

    pub(super) fn bind_validation(&mut self, revision: u64, validation_id: &str) {
        self.canonical.revision = revision;
        self.canonical.validation_id = validation_id.into();
        self.refresh_canonical_location();
    }

    pub fn is_current(&self) -> bool {
        self.canonical.is_current()
    }

    pub(super) fn refresh_canonical_location(&mut self) {
        let document_id = self.source_path.as_deref().map_or_else(
            || "netlist-editor".to_owned(),
            |path| path.to_string_lossy().into_owned(),
        );
        let range = crate::workbench::documents::canonical_diagnostics::range_from_legacy(
            self.span.as_ref(),
            self.line.or(self.source_line),
            self.column,
        );
        let old = self.canonical.clone();
        self.canonical = crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata::current(
            old.source,
            old.code,
            document_id.clone(),
            range,
            old.revision,
            old.validation_id,
            &self.message,
        );
        self.canonical.related_locations = old.related_locations;
        self.canonical.quick_fixes = self.fix.as_ref().and_then(|fix| {
            let range = crate::workbench::documents::canonical_diagnostics::range_from_legacy(
                Some(&fix.span),
                self.line.or(self.source_line),
                self.column,
            )?;
            Some(vec![
                crate::workbench::documents::canonical_diagnostics::CanonicalQuickFix {
                    fix_id: format!("{}:fix:0", self.canonical.diagnostic_id),
                    label: fix.label.clone(),
                    preferred: true,
                    edits: vec![crate::workbench::documents::canonical_diagnostics::DiagnosticTextEdit {
                        document_id,
                        range,
                        replacement: fix.replacement.clone(),
                    }],
                },
            ])
        }).unwrap_or(old.quick_fixes);
        self.canonical.suppression = old.suppression;
        self.canonical.currentness = old.currentness;
        self.canonical.affected_consumers = old.affected_consumers;
    }
}

/// Immutable publication unit for the canonical Netlist diagnostics. The
/// editor keeps it behind an `Arc`, so rendering never clones or rescans a
/// release-scale result set.
#[derive(Debug, Clone, Default)]
pub struct NetlistDiagnosticCollection {
    records: Vec<Diagnostic>,
    severity_by_line: HashMap<usize, DiagnosticSeverity>,
    interned_strings: HashMap<Arc<str>, Arc<str>>,
}

impl NetlistDiagnosticCollection {
    pub fn try_new(mut records: Vec<Diagnostic>, buffer: &str) -> Result<Self, String> {
        if records.len() > MAX_NETLIST_DIAGNOSTICS {
            return Err(format!(
                "Netlist diagnostic collection contains {} records; the supported maximum is {MAX_NETLIST_DIAGNOSTICS}.",
                records.len()
            ));
        }
        let mut diagnostic_ids = HashSet::with_capacity(records.len());
        for diagnostic in &records {
            if !diagnostic_ids.insert(diagnostic.canonical.diagnostic_id) {
                return Err(format!(
                    "Netlist diagnostic collection contains duplicate canonical ID {}.",
                    diagnostic.canonical.diagnostic_id
                ));
            }
        }
        let mut interned_strings = HashMap::new();
        let mut severity_by_line = HashMap::new();
        for diagnostic in &mut records {
            intern_netlist_diagnostic_strings(&mut interned_strings, diagnostic);
            if !diagnostic.is_current() {
                continue;
            }
            let line = diagnostic.line.or_else(|| {
                diagnostic
                    .span
                    .as_ref()
                    .map(|span| line_column_for_span(buffer, span.start).0)
            });
            if let Some(line) = line {
                severity_by_line
                    .entry(line)
                    .and_modify(|severity: &mut DiagnosticSeverity| {
                        *severity = (*severity).max(diagnostic.severity)
                    })
                    .or_insert(diagnostic.severity);
            }
        }
        Ok(Self {
            records,
            severity_by_line,
            interned_strings,
        })
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.severity_by_line.clear();
        self.interned_strings.clear();
    }

    pub fn mark_all_stale(&mut self) {
        for diagnostic in &mut self.records {
            diagnostic.canonical.mark_currentness(
                crate::workbench::documents::canonical_diagnostics::DiagnosticCurrentness::StaleSource,
            );
        }
        self.severity_by_line.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Diagnostic> {
        self.records.iter()
    }

    pub fn severity_for_line(&self, zero_based_line: usize) -> Option<DiagnosticSeverity> {
        self.severity_by_line.get(&zero_based_line).copied()
    }
}

fn intern_netlist_diagnostic_strings(
    pool: &mut HashMap<Arc<str>, Arc<str>>,
    diagnostic: &mut Diagnostic,
) {
    intern_string(pool, &mut diagnostic.canonical.source);
    intern_string(pool, &mut diagnostic.canonical.code);
    intern_string(pool, &mut diagnostic.canonical.document_id);
    intern_string(pool, &mut diagnostic.canonical.validation_id);
}

fn intern_string(pool: &mut HashMap<Arc<str>, Arc<str>>, value: &mut Arc<str>) {
    if let Some(existing) = pool.get(value.as_ref()) {
        *value = Arc::clone(existing);
    } else {
        pool.insert(Arc::clone(value), Arc::clone(value));
    }
}

fn same_source(left: Option<&std::path::Path>, right: Option<&std::path::Path>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            left == right
                || (left.is_absolute() != right.is_absolute()
                    && std::env::current_dir().is_ok_and(|current| {
                        let absolute_left = if left.is_absolute() {
                            left.to_path_buf()
                        } else {
                            current.join(left)
                        };
                        let absolute_right = if right.is_absolute() {
                            right.to_path_buf()
                        } else {
                            current.join(right)
                        };
                        absolute_left == absolute_right
                    }))
        }
        (None, None) => true,
        _ => false,
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

pub(super) fn parser_diagnostics(
    netlist: &rspice_core::Netlist,
    editor_source_path: Option<&std::path::Path>,
) -> Vec<Diagnostic> {
    netlist
        .diagnostics
        .iter()
        .map(|diagnostic| {
            let fallback_origin = match &netlist.source_path {
                Some(path) => {
                    rspice_core::netlist::NetlistSourceLocation::in_file(path, diagnostic.line)
                }
                None => rspice_core::netlist::NetlistSourceLocation::in_memory(diagnostic.line),
            };
            let origin = diagnostic.origin.as_ref().unwrap_or(&fallback_origin);
            Diagnostic::current(
                "rspice.netlist.parser",
                "SPICE-PARSER-WARNING",
                match diagnostic.severity {
                    rspice_core::netlist::DiagnosticSeverity::Warning => {
                        DiagnosticSeverity::Warning
                    }
                },
                diagnostic.message.clone(),
            )
            .with_source_location(origin, editor_source_path)
        })
        .collect()
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

    #[test]
    fn indexed_collection_uses_current_zero_based_lines() {
        let mut stale = Diagnostic::error("old").with_line(Some(3));
        stale.canonical.mark_currentness(
            crate::workbench::documents::canonical_diagnostics::DiagnosticCurrentness::StaleSource,
        );
        let diagnostics = NetlistDiagnosticCollection::try_new(
            vec![
                Diagnostic::current(
                    "rspice.test",
                    "TEST-WARNING",
                    DiagnosticSeverity::Warning,
                    "warning",
                )
                .with_line(Some(3)),
                Diagnostic::error("error").with_line(Some(3)),
                stale,
            ],
            "a\nb\nc\nd\n",
        )
        .unwrap();
        assert_eq!(diagnostics.iter().count(), 3);
        assert_eq!(
            diagnostics.severity_for_line(3),
            Some(DiagnosticSeverity::Error)
        );
    }

    #[test]
    fn indexed_collection_shares_repeated_canonical_identity_strings() {
        let diagnostics = NetlistDiagnosticCollection::try_new(
            vec![
                Diagnostic::current(
                    "rspice.test",
                    "TEST-SHARED",
                    DiagnosticSeverity::Warning,
                    "first",
                ),
                Diagnostic::current(
                    "rspice.test",
                    "TEST-SHARED",
                    DiagnosticSeverity::Warning,
                    "second",
                ),
            ],
            "",
        )
        .unwrap();
        let mut records = diagnostics.iter();
        let first = records.next().unwrap();
        let second = records.next().unwrap();

        assert!(Arc::ptr_eq(
            &first.canonical.source,
            &second.canonical.source
        ));
        assert!(Arc::ptr_eq(&first.canonical.code, &second.canonical.code));
        assert!(Arc::ptr_eq(
            &first.canonical.document_id,
            &second.canonical.document_id
        ));
        assert!(Arc::ptr_eq(
            &first.canonical.validation_id,
            &second.canonical.validation_id
        ));
    }

    #[test]
    fn indexed_collection_rejects_duplicate_canonical_identity() {
        let diagnostic = Diagnostic::current(
            "rspice.test",
            "TEST-DUPLICATE",
            DiagnosticSeverity::Error,
            "duplicate",
        );
        let error = NetlistDiagnosticCollection::try_new(vec![diagnostic.clone(), diagnostic], "")
            .expect_err("duplicate canonical IDs must fail closed");
        assert!(error.contains("duplicate canonical ID"));
    }
}
