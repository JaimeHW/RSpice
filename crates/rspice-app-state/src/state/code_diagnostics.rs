//! The code editors' canonical findings, as the diagnostic store reads them.
//!
//! The Verilog-A and Automation editors publish their compiler and
//! validation findings as one bounded collection behind an `Arc`; the
//! Problems listing reads that collection in place. The collection is the
//! shared model's, so the store below the editors can read it without naming
//! them.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rspice_vocabulary::diagnostics::{
    DiagnosticSeverity as CodeEditorSeverity, MAX_DIAGNOSTICS,
    SourceDiagnostic as CodeEditorDiagnostic, insert_diagnostic_id, validate_diagnostic_count,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeDiagnosticSummary {
    pub hints: usize,
    pub information: usize,
    pub warnings: usize,
    pub errors: usize,
}

impl CodeDiagnosticSummary {
    pub const fn total(self) -> usize {
        self.hints + self.information + self.warnings + self.errors
    }

    fn record(&mut self, severity: CodeEditorSeverity) {
        match severity {
            CodeEditorSeverity::Hint => self.hints += 1,
            CodeEditorSeverity::Info => self.information += 1,
            CodeEditorSeverity::Warning => self.warnings += 1,
            CodeEditorSeverity::Error => self.errors += 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct DocumentDiagnosticIndex {
    severity_by_line: HashMap<usize, CodeEditorSeverity>,
}

/// Bounded canonical collection with publication-time summaries and line
/// indexes. Product surfaces clone an `Arc` to this collection, so a million
/// diagnostics are never copied or rescanned during a frame.
#[derive(Debug, Clone, Default)]
pub struct CodeDiagnosticCollection {
    records: Vec<CodeEditorDiagnostic>,
    summary: CodeDiagnosticSummary,
    global: DocumentDiagnosticIndex,
    documents: HashMap<String, DocumentDiagnosticIndex>,
    interned_strings: HashMap<Arc<str>, Arc<str>>,
    diagnostic_ids: HashSet<uuid::Uuid>,
}

impl CodeDiagnosticCollection {
    pub fn try_new(records: Vec<CodeEditorDiagnostic>) -> Result<Self, String> {
        validate_diagnostic_count(records.len())?;
        let mut collection = Self::default();
        collection.records.reserve(records.len());
        for record in records {
            collection.push_indexed(record)?;
        }
        Ok(collection)
    }

    pub fn try_push(&mut self, record: CodeEditorDiagnostic) -> Result<(), String> {
        if self.records.len() >= MAX_DIAGNOSTICS {
            return Err(format!(
                "Diagnostic collection reached the supported maximum of {MAX_DIAGNOSTICS} records."
            ));
        }
        self.push_indexed(record)
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.summary = CodeDiagnosticSummary::default();
        self.global = DocumentDiagnosticIndex::default();
        self.documents.clear();
        self.interned_strings.clear();
        self.diagnostic_ids.clear();
    }

    pub fn summary(&self) -> CodeDiagnosticSummary {
        self.summary
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, CodeEditorDiagnostic> {
        self.records.iter()
    }

    pub fn as_slice(&self) -> &[CodeEditorDiagnostic] {
        &self.records
    }

    pub fn view<'a>(&'a self, logical_path: Option<&str>) -> CodeDiagnosticView<'a> {
        CodeDiagnosticView {
            collection: self,
            document_key: logical_path.map(normalize_diagnostic_path),
        }
    }

    fn push_indexed(&mut self, mut record: CodeEditorDiagnostic) -> Result<(), String> {
        insert_diagnostic_id(&mut self.diagnostic_ids, record.canonical.diagnostic_id)?;
        self.summary.record(record.severity);
        let line = diagnostic_line(&record);
        if record.line.is_none() {
            record.line = line;
        }
        // Source text belongs to the versioned project bundle, not to every
        // retained diagnostic. Line information is materialized above before
        // releasing this transient compiler payload.
        record.source = None;
        intern_diagnostic_strings(&mut self.interned_strings, &mut record);
        let target = match record.source_path.as_deref() {
            Some(path) => self
                .documents
                .entry(normalize_diagnostic_path(path))
                .or_default(),
            None => &mut self.global,
        };
        if let Some(line) = line {
            target
                .severity_by_line
                .entry(line)
                .and_modify(|severity| *severity = (*severity).max(record.severity))
                .or_insert(record.severity);
        }
        self.records.push(record);
        Ok(())
    }
}

fn intern_diagnostic_strings(
    pool: &mut HashMap<Arc<str>, Arc<str>>,
    diagnostic: &mut CodeEditorDiagnostic,
) {
    intern_string(pool, &mut diagnostic.canonical.source);
    intern_string(pool, &mut diagnostic.canonical.code);
    intern_string(pool, &mut diagnostic.canonical.document_id);
    intern_string(pool, &mut diagnostic.canonical.validation_id);
    if let Some(path) = &mut diagnostic.source_path {
        intern_string(pool, path);
    }
}

pub(super) fn intern_string(pool: &mut HashMap<Arc<str>, Arc<str>>, value: &mut Arc<str>) {
    if let Some(existing) = pool.get(value.as_ref()) {
        *value = Arc::clone(existing);
    } else {
        pool.insert(Arc::clone(value), Arc::clone(value));
    }
}

impl AsRef<[CodeEditorDiagnostic]> for CodeDiagnosticCollection {
    fn as_ref(&self) -> &[CodeEditorDiagnostic] {
        self.as_slice()
    }
}

#[derive(Debug, Clone)]
pub struct CodeDiagnosticView<'a> {
    collection: &'a CodeDiagnosticCollection,
    document_key: Option<String>,
}

impl CodeDiagnosticView<'_> {
    pub fn severity_for_line(&self, one_based_line: usize) -> Option<CodeEditorSeverity> {
        let global = self
            .collection
            .global
            .severity_by_line
            .get(&one_based_line)
            .copied();
        let document = self
            .document_index()
            .and_then(|index| index.severity_by_line.get(&one_based_line).copied());
        global.max(document)
    }

    fn document_index(&self) -> Option<&DocumentDiagnosticIndex> {
        self.document_key
            .as_deref()
            .and_then(|key| self.collection.documents.get(key))
    }
}

fn normalize_diagnostic_path(path: &str) -> String {
    path.replace('\\', "/").to_ascii_lowercase()
}

fn diagnostic_line(diagnostic: &CodeEditorDiagnostic) -> Option<usize> {
    diagnostic.line.or_else(|| {
        diagnostic
            .source
            .as_deref()
            .zip(diagnostic.byte_range.as_ref())
            .map(|(source, range)| byte_offset_line(source, range.start))
    })
}

fn byte_offset_line(source: &str, offset: usize) -> usize {
    source
        .as_bytes()
        .iter()
        .take(offset.min(source.len()))
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_line_lookup_is_byte_safe() {
        let source = "module µ;\nanalog V(o) <+ 1;\nendmodule\n";
        assert_eq!(byte_offset_line(source, source.find("analog").unwrap()), 2);
    }

    #[test]
    fn line_diagnostics_use_one_based_lines() {
        let diagnostic = CodeEditorDiagnostic::current(
            "rspice.test",
            "TEST-001",
            CodeEditorSeverity::Error,
            "broken",
            "",
            Some("model.va".to_owned()),
            None,
            None,
            Some(2),
            Some(1),
        );
        let diagnostics = CodeDiagnosticCollection::try_new(vec![diagnostic]).unwrap();
        assert_eq!(
            diagnostics.view(Some("MODEL.VA")).severity_for_line(2),
            Some(CodeEditorSeverity::Error)
        );
        assert_eq!(
            diagnostics.view(Some("other.va")).severity_for_line(2),
            None
        );
    }

    #[test]
    fn diagnostic_collection_summarizes_without_frame_time_rescans() {
        let diagnostics = CodeDiagnosticCollection::try_new(vec![
            CodeEditorDiagnostic::current(
                "rspice.test",
                "TEST-ERROR",
                CodeEditorSeverity::Error,
                "error",
                "",
                None,
                None,
                None,
                None,
                None,
            ),
            CodeEditorDiagnostic::current(
                "rspice.test",
                "TEST-WARNING",
                CodeEditorSeverity::Warning,
                "warning",
                "",
                None,
                None,
                None,
                None,
                None,
            ),
        ])
        .unwrap();
        assert_eq!(diagnostics.summary().errors, 1);
        assert_eq!(diagnostics.summary().warnings, 1);
        assert_eq!(diagnostics.summary().total(), 2);
    }

    #[test]
    fn diagnostic_collection_compacts_identity_and_releases_embedded_source() {
        let records = (0..2)
            .map(|index| {
                CodeEditorDiagnostic::current(
                    "rspice.test",
                    "TEST-COMPACT",
                    CodeEditorSeverity::Warning,
                    format!("message {index}"),
                    "detail",
                    Some("models/compact.va".to_owned()),
                    Some("first\nsecond\n".to_owned()),
                    Some(6..12),
                    None,
                    Some(1),
                )
                .bind_validation("models/compact.va", 7, "validation-7")
            })
            .collect::<Vec<_>>();
        let diagnostics = CodeDiagnosticCollection::try_new(records).unwrap();
        let first = &diagnostics.as_slice()[0];
        let second = &diagnostics.as_slice()[1];

        assert_eq!(first.line, Some(2));
        assert!(first.source.is_none());
        assert!(second.source.is_none());
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
        assert!(Arc::ptr_eq(
            first.source_path.as_ref().unwrap(),
            second.source_path.as_ref().unwrap()
        ));
    }

    #[test]
    fn diagnostic_collection_rejects_duplicate_canonical_identity() {
        let diagnostic = CodeEditorDiagnostic::current(
            "rspice.test",
            "TEST-DUPLICATE",
            CodeEditorSeverity::Error,
            "duplicate",
            "detail",
            None,
            None,
            None,
            None,
            None,
        );
        let error = CodeDiagnosticCollection::try_new(vec![diagnostic.clone(), diagnostic])
            .expect_err("duplicate canonical IDs must fail closed");
        assert!(error.contains("duplicate canonical ID"));
    }
}
