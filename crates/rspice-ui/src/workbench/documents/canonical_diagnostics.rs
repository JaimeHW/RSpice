//! Canonical diagnostic identity shared by every source-language workflow.
//!
//! The editor, Problems grid, inspector, status summary, and validation report
//! are projections of these records. They must never invent their own count or
//! strip revision/currentness data while copying a message between views.

use std::collections::HashMap;
use std::sync::Arc;

pub(crate) const MAX_DIAGNOSTIC_IDENTITY_BYTES: usize = 16 * 1024;
pub(crate) const MAX_DIAGNOSTIC_MESSAGE_BYTES: usize = 64 * 1024;
pub(crate) const MAX_DIAGNOSTIC_DETAIL_BYTES: usize = 256 * 1024;
pub(crate) const MAX_DIAGNOSTIC_REPLACEMENT_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const MAX_DIAGNOSTIC_COLLECTION_TEXT_BYTES: usize = 64 * 1024 * 1024;
const MAX_DIAGNOSTIC_RELATED_LOCATIONS: usize = 256;
const MAX_DIAGNOSTIC_QUICK_FIXES: usize = 64;
const MAX_DIAGNOSTIC_EDITS_PER_FIX: usize = 256;

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

    /// Validate the recursively owned portion of a diagnostic and return the
    /// number of UTF-8 text bytes that cannot be shared by the collection's
    /// top-level string interner.
    pub(crate) fn retained_nested_text_bytes(&self) -> Result<usize, String> {
        for (label, value) in [
            ("producer", self.source.as_ref()),
            ("code", self.code.as_ref()),
            ("document identity", self.document_id.as_ref()),
            ("validation identity", self.validation_id.as_ref()),
        ] {
            validate_diagnostic_text(label, value, MAX_DIAGNOSTIC_IDENTITY_BYTES)?;
        }
        validate_diagnostic_count(
            "related-location",
            self.related_locations.len(),
            MAX_DIAGNOSTIC_RELATED_LOCATIONS,
        )?;
        validate_diagnostic_count(
            "quick-fix",
            self.quick_fixes.len(),
            MAX_DIAGNOSTIC_QUICK_FIXES,
        )?;

        let mut retained_bytes = 0usize;
        for location in &self.related_locations {
            add_retained_text(
                &mut retained_bytes,
                "related-location document identity",
                &location.document_id,
                MAX_DIAGNOSTIC_IDENTITY_BYTES,
            )?;
            add_retained_text(
                &mut retained_bytes,
                "related-location message",
                &location.message,
                MAX_DIAGNOSTIC_MESSAGE_BYTES,
            )?;
        }
        for quick_fix in &self.quick_fixes {
            validate_diagnostic_count(
                "quick-fix edit",
                quick_fix.edits.len(),
                MAX_DIAGNOSTIC_EDITS_PER_FIX,
            )?;
            add_retained_text(
                &mut retained_bytes,
                "quick-fix identity",
                &quick_fix.fix_id,
                MAX_DIAGNOSTIC_IDENTITY_BYTES,
            )?;
            add_retained_text(
                &mut retained_bytes,
                "quick-fix label",
                &quick_fix.label,
                MAX_DIAGNOSTIC_MESSAGE_BYTES,
            )?;
            for edit in &quick_fix.edits {
                add_retained_text(
                    &mut retained_bytes,
                    "quick-fix document identity",
                    &edit.document_id,
                    MAX_DIAGNOSTIC_IDENTITY_BYTES,
                )?;
                add_retained_text(
                    &mut retained_bytes,
                    "quick-fix replacement",
                    &edit.replacement,
                    MAX_DIAGNOSTIC_REPLACEMENT_BYTES,
                )?;
            }
        }
        if let Some(suppression) = &self.suppression {
            add_retained_text(
                &mut retained_bytes,
                "suppression rule",
                &suppression.rule,
                MAX_DIAGNOSTIC_IDENTITY_BYTES,
            )?;
            add_retained_text(
                &mut retained_bytes,
                "suppression scope",
                &suppression.scope,
                MAX_DIAGNOSTIC_IDENTITY_BYTES,
            )?;
        }
        Ok(retained_bytes)
    }
}

pub(crate) fn validate_diagnostic_text(
    label: &str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), String> {
    validate_diagnostic_byte_length(label, value.len(), maximum_bytes)
}

pub(crate) fn validate_diagnostic_byte_length(
    label: &str,
    byte_length: usize,
    maximum_bytes: usize,
) -> Result<(), String> {
    if byte_length > maximum_bytes {
        return Err(format!(
            "Diagnostic {label} contains {} encoded bytes; the supported maximum is {maximum_bytes} bytes.",
            byte_length
        ));
    }
    Ok(())
}

pub(crate) fn additional_interned_text_bytes(
    pool: &HashMap<Arc<str>, Arc<str>>,
    values: &[&str],
) -> Result<usize, String> {
    let mut additional_bytes = 0usize;
    for (index, value) in values.iter().copied().enumerate() {
        if pool.contains_key(value) || values[..index].contains(&value) {
            continue;
        }
        additional_bytes = additional_bytes.checked_add(value.len()).ok_or_else(|| {
            "Diagnostic collection text accounting overflowed the platform limit.".to_owned()
        })?;
    }
    Ok(additional_bytes)
}

pub(crate) fn checked_diagnostic_text_total(
    retained_bytes: usize,
    additional_bytes: usize,
    maximum_bytes: usize,
) -> Result<usize, String> {
    let total = retained_bytes
        .checked_add(additional_bytes)
        .ok_or_else(|| {
            "Diagnostic collection text accounting overflowed the platform limit.".to_owned()
        })?;
    if total > maximum_bytes {
        return Err(format!(
            "Diagnostic collection retains {total} UTF-8 text bytes; the supported maximum is {maximum_bytes} bytes."
        ));
    }
    Ok(total)
}

fn validate_diagnostic_count(label: &str, count: usize, maximum: usize) -> Result<(), String> {
    if count > maximum {
        return Err(format!(
            "Diagnostic contains {count} {label} records; the supported maximum is {maximum}."
        ));
    }
    Ok(())
}

fn add_retained_text(
    retained_bytes: &mut usize,
    label: &str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), String> {
    validate_diagnostic_text(label, value, maximum_bytes)?;
    *retained_bytes = retained_bytes.checked_add(value.len()).ok_or_else(|| {
        "Diagnostic collection text accounting overflowed the platform limit.".to_owned()
    })?;
    Ok(())
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

    #[test]
    fn canonical_payload_rejects_unbounded_nested_records() {
        let mut metadata = CanonicalDiagnosticMetadata::current(
            "rspice.runtime",
            "PY-43",
            "workflow.py",
            None,
            1,
            "validation-1",
            "failure",
        );
        metadata.related_locations = vec![
            DiagnosticRelatedLocation {
                document_id: "workflow.py".to_owned(),
                range: None,
                message: "related".to_owned(),
            };
            MAX_DIAGNOSTIC_RELATED_LOCATIONS + 1
        ];
        let error = metadata
            .retained_nested_text_bytes()
            .expect_err("nested diagnostic records must be bounded");
        assert!(error.contains("related-location"));
        assert!(error.contains("supported maximum"));
    }
}
