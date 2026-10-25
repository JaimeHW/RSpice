//! The netlist editor's canonical findings, as the diagnostic store reads
//! them.
//!
//! The netlist editor produces these records and publishes them as one
//! immutable collection behind an `Arc`; the Problems listing reads that
//! collection in place. The records are the shared model's, so the store
//! below the editor can read them without naming the editor.

use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use rspice_vocabulary::diagnostics::DiagnosticSeverity;

pub const MAX_NETLIST_DIAGNOSTICS: usize = 1_000_000;

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
    pub canonical: rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata,
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
            canonical: rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata::current(
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

    pub fn bind_validation(&mut self, revision: u64, validation_id: &str) {
        self.canonical.revision = revision;
        self.canonical.validation_id = validation_id.into();
        self.refresh_canonical_location();
    }

    pub fn is_current(&self) -> bool {
        self.canonical.is_current()
    }

    pub fn refresh_canonical_location(&mut self) {
        let document_id = self.source_path.as_deref().map_or_else(
            || "netlist-editor".to_owned(),
            |path| path.to_string_lossy().into_owned(),
        );
        let range = rspice_vocabulary::diagnostics::diagnostic_range(
            self.span.as_ref(),
            self.line.or(self.source_line),
            self.column,
        );
        let old = self.canonical.clone();
        self.canonical = rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata::current(
            old.source,
            old.code,
            document_id.clone(),
            range,
            old.revision,
            old.validation_id,
            &self.message,
        );
        self.canonical.related_locations = old.related_locations;
        self.canonical.quick_fixes = self
            .fix
            .as_ref()
            .and_then(|fix| {
                let range = rspice_vocabulary::diagnostics::diagnostic_range(
                    Some(&fix.span),
                    self.line.or(self.source_line),
                    self.column,
                )?;
                Some(vec![rspice_vocabulary::diagnostics::CanonicalQuickFix {
                    fix_id: format!("{}:fix:0", self.canonical.diagnostic_id),
                    label: fix.label.clone(),
                    preferred: true,
                    edits: vec![rspice_vocabulary::diagnostics::DiagnosticTextEdit {
                        document_id,
                        range,
                        replacement: fix.replacement.clone(),
                    }],
                }])
            })
            .unwrap_or(old.quick_fixes);
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
    summary: NetlistDiagnosticSummary,
}

/// A lightweight projection of the canonical collection used by chrome that
/// must not maintain its own diagnostic counters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NetlistDiagnosticSummary {
    pub current_errors: usize,
    pub current_warnings: usize,
    pub current_information: usize,
    pub current_hints: usize,
    pub stale: usize,
}

impl NetlistDiagnosticSummary {
    #[must_use]
    #[cfg(test)]
    pub const fn current_total(self) -> usize {
        self.current_errors + self.current_warnings + self.current_information + self.current_hints
    }

    /// Every record in the collection, current or not.
    ///
    /// Kept for the tests that assert the projection, and deliberately out of
    /// reach of the chrome: counting stale records among the problems is what
    /// made the tab strip claim findings about a design that no longer
    /// existed.
    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub const fn total(self) -> usize {
        self.current_errors
            + self.current_warnings
            + self.current_information
            + self.current_hints
            + self.stale
    }

    #[must_use]
    pub const fn current_advisories(self) -> usize {
        self.current_warnings + self.current_information + self.current_hints
    }
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
                    "Netlist diagnostic collection contains duplicate diagnostic ID {}.",
                    diagnostic.canonical.diagnostic_id
                ));
            }
        }
        let mut interned_strings = HashMap::new();
        let mut severity_by_line = HashMap::new();
        let mut summary = NetlistDiagnosticSummary::default();
        for diagnostic in &mut records {
            intern_netlist_diagnostic_strings(&mut interned_strings, diagnostic);
            if !diagnostic.is_current() {
                summary.stale += 1;
                continue;
            }
            match diagnostic.severity {
                DiagnosticSeverity::Error => summary.current_errors += 1,
                DiagnosticSeverity::Warning => summary.current_warnings += 1,
                DiagnosticSeverity::Info => summary.current_information += 1,
                DiagnosticSeverity::Hint => summary.current_hints += 1,
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
            summary,
        })
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.severity_by_line.clear();
        self.interned_strings.clear();
        self.summary = NetlistDiagnosticSummary::default();
    }

    pub fn mark_all_stale(&mut self) {
        for diagnostic in &mut self.records {
            diagnostic.canonical.mark_currentness(
                rspice_vocabulary::diagnostics::DiagnosticCurrentness::StaleSource,
            );
        }
        self.severity_by_line.clear();
        self.summary = NetlistDiagnosticSummary {
            stale: self.records.len(),
            ..NetlistDiagnosticSummary::default()
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Diagnostic> {
        self.records.iter()
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Diagnostic> {
        self.records.get(index)
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Summarize the same immutable records rendered by the editor,
    /// Problems grid, inspector, status bar, and validation workflow.
    #[must_use]
    pub fn summary(&self) -> NetlistDiagnosticSummary {
        self.summary
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_is_a_projection_of_current_and_stale_canonical_records() {
        let mut records = vec![
            Diagnostic::error("current error"),
            Diagnostic::current(
                "rspice.test",
                "TEST-WARNING",
                DiagnosticSeverity::Warning,
                "current warning",
            ),
            Diagnostic::current(
                "rspice.test",
                "TEST-INFO",
                DiagnosticSeverity::Info,
                "stale information",
            ),
        ];
        records[2]
            .canonical
            .mark_currentness(rspice_vocabulary::diagnostics::DiagnosticCurrentness::StaleSource);
        let diagnostics = NetlistDiagnosticCollection::try_new(records, "").unwrap();

        assert_eq!(
            diagnostics.summary(),
            NetlistDiagnosticSummary {
                current_errors: 1,
                current_warnings: 1,
                current_information: 0,
                current_hints: 0,
                stale: 1,
            }
        );
        assert_eq!(diagnostics.summary().current_total(), 2);
        assert_eq!(diagnostics.summary().total(), 3);
    }

    #[test]
    fn line_column_for_span_counts_zero_based_lines_and_columns() {
        assert_eq!(line_column_for_span("a\nbc\n", 3), (1, 1));
    }

    #[test]
    fn line_column_for_span_clamps_offsets_after_eof() {
        assert_eq!(line_column_for_span("a\nbc", 99), (1, 2));
    }

    #[test]
    fn indexed_collection_uses_current_zero_based_lines() {
        let mut stale = Diagnostic::error("old").with_line(Some(3));
        stale
            .canonical
            .mark_currentness(rspice_vocabulary::diagnostics::DiagnosticCurrentness::StaleSource);
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
        assert!(error.contains("duplicate diagnostic ID"));
    }
}
