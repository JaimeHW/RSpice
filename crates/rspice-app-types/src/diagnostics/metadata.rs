//! Canonical diagnostic identity shared by every source-language workflow.
//!
//! The editor, Problems grid, inspector, status summary, and validation report
//! are projections of these records. They must never invent their own count or
//! strip revision/currentness data while copying a message between views.

use super::{DiagnosticRange, stable_diagnostic_id};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticCurrentness {
    Current,
    StaleSource,
    StaleEnvironment,
    StalePermissions,
    SupersededValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticConsumer {
    EditorDecoration,
    ProblemsGrid,
    InspectorSummary,
    StatusSummary,
    ValidationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRelatedLocation {
    pub document_id: String,
    pub range: Option<DiagnosticRange>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticTextEdit {
    pub document_id: String,
    pub range: DiagnosticRange,
    pub replacement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanonicalQuickFix {
    pub fix_id: String,
    pub label: String,
    pub preferred: bool,
    pub edits: Vec<DiagnosticTextEdit>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticSuppression {
    pub rule: String,
    pub scope: String,
    pub justification_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanonicalDiagnosticMetadata {
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
    pub fn current(
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

    /// A finding whose identity is what it is about, not when it was found.
    ///
    /// [`Self::current`] binds the identity to a revision, a validation pass
    /// and the message, which is right for a parser reading one buffer and
    /// wrong for a rule finding about a design object: every re-check would
    /// re-mint every identity, and a fix that removed one finding would hand
    /// its identity to the next finding of the same rule. Here the identity is
    /// the producer, the rule, the document the object is in, a stable key of
    /// the object itself, and an occurrence index that separates findings
    /// whose keys are otherwise identical. The revision and the validation
    /// pass are still recorded; they just do not name the finding.
    pub fn structural(
        source: impl Into<Arc<str>>,
        code: impl Into<Arc<str>>,
        document_id: impl Into<Arc<str>>,
        object: &str,
        occurrence: usize,
        revision: u64,
        validation_id: impl Into<Arc<str>>,
    ) -> Self {
        let source = source.into();
        let code = code.into();
        let document_id = document_id.into();
        let diagnostic_id = structural_diagnostic_id(
            source.as_ref(),
            code.as_ref(),
            document_id.as_ref(),
            object,
            occurrence,
        );
        Self {
            diagnostic_id,
            ..Self::current(
                source,
                code,
                document_id,
                None,
                revision,
                validation_id,
                "",
            )
        }
    }

    pub fn mark_currentness(&mut self, currentness: DiagnosticCurrentness) {
        self.currentness = currentness;
    }

    pub const fn is_current(&self) -> bool {
        matches!(self.currentness, DiagnosticCurrentness::Current)
    }
}

/// The identity [`CanonicalDiagnosticMetadata::structural`] mints.
///
/// Its own namespace, so no structural identity can coincide with one minted
/// from a revision and a message. Every field is length-prefixed, so no two
/// different tuples spell the same string.
fn structural_diagnostic_id(
    source: &str,
    code: &str,
    document_id: &str,
    object: &str,
    occurrence: usize,
) -> uuid::Uuid {
    const NAMESPACE: uuid::Uuid = uuid::Uuid::from_u128(0x5b0e_4f1c_2d7a_5e93_8c61_a4f2_0b9d_3e57);
    let mut identity =
        String::with_capacity(source.len() + code.len() + document_id.len() + object.len() + 48);
    for value in [source, code, document_id, object] {
        identity.push_str(&value.len().to_string());
        identity.push(':');
        identity.push_str(value);
        identity.push('|');
    }
    identity.push_str(&occurrence.to_string());
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
            crate::diagnostics::diagnostic_range(Some(&(4..7)), Some(2), Some(1)),
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

    /// A structural identity names the object and nothing about the pass:
    /// the revision, the validation pass and the wording can all move without
    /// moving it, and only the object or the occurrence index can.
    #[test]
    fn a_structural_identity_ignores_the_revision_the_pass_and_the_wording() {
        let first = CanonicalDiagnosticMetadata::structural(
            "rspice.design.rules",
            "DRC-UNCONNECTED-PIN",
            "lib/top/schematic",
            "terminal:4:A",
            0,
            8,
            "design-check:8",
        );
        let rechecked = CanonicalDiagnosticMetadata::structural(
            "rspice.design.rules",
            "DRC-UNCONNECTED-PIN",
            "lib/top/schematic",
            "terminal:4:A",
            0,
            9,
            "design-check:9",
        );
        assert_eq!(first.diagnostic_id, rechecked.diagnostic_id);
        assert_eq!(rechecked.revision, 9, "the revision is still recorded");
        assert_eq!(rechecked.validation_id.as_ref(), "design-check:9");

        for (object, occurrence) in [("terminal:5:A", 0), ("terminal:4:A", 1)] {
            let other = CanonicalDiagnosticMetadata::structural(
                "rspice.design.rules",
                "DRC-UNCONNECTED-PIN",
                "lib/top/schematic",
                object,
                occurrence,
                8,
                "design-check:8",
            );
            assert_ne!(first.diagnostic_id, other.diagnostic_id, "{object} #{occurrence}");
        }
        // Length-prefixed, so moving a separator between fields is a
        // different identity rather than the same string.
        let shifted = CanonicalDiagnosticMetadata::structural(
            "rspice.design.rules",
            "DRC-UNCONNECTED-PIN",
            "lib/top/schematic|terminal:4",
            ":A",
            0,
            8,
            "design-check:8",
        );
        assert_ne!(first.diagnostic_id, shifted.diagnostic_id);
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
