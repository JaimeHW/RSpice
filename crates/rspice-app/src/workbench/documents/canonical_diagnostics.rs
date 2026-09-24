//! Canonical diagnostic identity shared by every source-language workflow.
//!
//! The editor, Problems grid, inspector, status summary, and validation report
//! are projections of these records. They must never invent their own count or
//! strip revision/currentness data while copying a message between views.

use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DiagnosticCurrentness {
    Current,
    StaleSource,
    StaleEnvironment,
    StalePermissions,
    SupersededValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DiagnosticConsumer {
    EditorDecoration,
    ProblemsGrid,
    InspectorSummary,
    StatusSummary,
    ValidationReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticPosition {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticRange {
    pub start: DiagnosticPosition,
    pub end: DiagnosticPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticRelatedLocation {
    pub document_id: String,
    pub range: Option<DiagnosticRange>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticTextEdit {
    pub document_id: String,
    pub range: DiagnosticRange,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanonicalQuickFix {
    pub fix_id: String,
    pub label: String,
    pub preferred: bool,
    pub edits: Vec<DiagnosticTextEdit>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticSuppression {
    pub rule: String,
    pub scope: String,
    pub justification_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CanonicalDiagnosticMetadata {
    pub diagnostic_id: uuid::Uuid,
    pub code: Arc<str>,
    /// Producer identity, for example `rspice.veriloga.compiler`.
    pub source: Arc<str>,
    pub document_id: Arc<str>,
    pub range: Option<DiagnosticRange>,
    pub related_locations: Vec<DiagnosticRelatedLocation>,
    pub quick_fixes: Vec<CanonicalQuickFix>,
    pub suppression: Option<DiagnosticSuppression>,
    pub revision: u64,
    pub validation_id: Arc<str>,
    pub currentness: DiagnosticCurrentness,
    pub affected_consumers: [DiagnosticConsumer; 5],
}

impl CanonicalDiagnosticMetadata {
    pub(crate) fn current(
        source: impl Into<Arc<str>>,
        code: impl Into<Arc<str>>,
        document_id: impl Into<Arc<str>>,
        range: Option<DiagnosticRange>,
        revision: u64,
        validation_id: impl Into<Arc<str>>,
        message: &str,
    ) -> Self {
        let source = source.into();
        let code = code.into();
        let document_id = document_id.into();
        let validation_id = validation_id.into();
        let diagnostic_id = stable_diagnostic_id(
            source.as_ref(),
            code.as_ref(),
            document_id.as_ref(),
            range,
            revision,
            validation_id.as_ref(),
            message,
        );
        Self {
            diagnostic_id,
            code,
            source,
            document_id,
            range,
            related_locations: Vec::new(),
            quick_fixes: Vec::new(),
            suppression: None,
            revision,
            validation_id,
            currentness: DiagnosticCurrentness::Current,
            affected_consumers: [
                DiagnosticConsumer::EditorDecoration,
                DiagnosticConsumer::ProblemsGrid,
                DiagnosticConsumer::InspectorSummary,
                DiagnosticConsumer::StatusSummary,
                DiagnosticConsumer::ValidationReport,
            ],
        }
    }

    pub(crate) fn mark_currentness(&mut self, currentness: DiagnosticCurrentness) {
        self.currentness = currentness;
    }

    pub(crate) const fn is_current(&self) -> bool {
        matches!(self.currentness, DiagnosticCurrentness::Current)
    }
}

pub(crate) fn range_from_legacy(
    byte_range: Option<&std::ops::Range<usize>>,
    line: Option<usize>,
    column: Option<usize>,
) -> Option<DiagnosticRange> {
    if byte_range.is_none() && line.is_none() && column.is_none() {
        return None;
    }
    let byte_range = byte_range.cloned().unwrap_or(0..0);
    let start = DiagnosticPosition {
        line: line.unwrap_or(0),
        column: column.unwrap_or(0),
        byte_offset: byte_range.start,
    };
    Some(DiagnosticRange {
        start,
        end: DiagnosticPosition {
            byte_offset: byte_range.end,
            ..start
        },
    })
}

fn stable_diagnostic_id(
    source: &str,
    code: &str,
    document_id: &str,
    range: Option<DiagnosticRange>,
    revision: u64,
    validation_id: &str,
    message: &str,
) -> uuid::Uuid {
    const NAMESPACE: uuid::Uuid = uuid::Uuid::from_u128(0x9cc9d141_951a_5553_b787_b2c80c98c43f);
    let mut identity = String::with_capacity(
        source.len() + code.len() + document_id.len() + validation_id.len() + message.len() + 96,
    );
    for value in [source, code, document_id, validation_id, message] {
        identity.push_str(&value.len().to_string());
        identity.push(':');
        identity.push_str(value);
        identity.push('|');
    }
    identity.push_str(&revision.to_string());
    if let Some(range) = range {
        use std::fmt::Write as _;
        let _ = write!(
            identity,
            "|{}:{}:{}-{}:{}:{}",
            range.start.line,
            range.start.column,
            range.start.byte_offset,
            range.end.line,
            range.end.column,
            range.end.byte_offset
        );
    }
    uuid::Uuid::new_v5(&NAMESPACE, identity.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_identity_is_stable_and_revision_bound() {
        let first = CanonicalDiagnosticMetadata::current(
            "rspice.parser",
            "SPICE-001",
            "top.sp",
            range_from_legacy(Some(&(4..7)), Some(2), Some(1)),
            8,
            "validation-4",
            "unknown model",
        );
        let repeat = first.clone();
        let next_revision = CanonicalDiagnosticMetadata::current(
            "rspice.parser",
            "SPICE-001",
            "top.sp",
            first.range,
            9,
            "validation-5",
            "unknown model",
        );
        assert_eq!(first.diagnostic_id, repeat.diagnostic_id);
        assert_ne!(first.diagnostic_id, next_revision.diagnostic_id);
        assert_eq!(first.affected_consumers.len(), 5);
    }

    #[test]
    fn canonical_metadata_serializes_contract_field_names() {
        let metadata = CanonicalDiagnosticMetadata::current(
            "rspice.runtime",
            "PY-42",
            "workflow.py",
            None,
            1,
            "validation-1",
            "failure",
        );
        let value = serde_json::to_value(metadata).unwrap();
        for field in [
            "diagnosticId",
            "code",
            "source",
            "documentId",
            "range",
            "relatedLocations",
            "quickFixes",
            "suppression",
            "revision",
            "validationId",
            "currentness",
            "affectedConsumers",
        ] {
            assert!(value.get(field).is_some(), "missing {field}");
        }
    }
}
