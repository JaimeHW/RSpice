//! Validated numeric values and exact model-source identities used by model evidence.

use std::{cmp::Ordering, fmt};

use rspice_app_types::product::{ContentDigest, ModelSourceId, ObjectRevision};
use serde::{Deserialize, Deserializer, Serialize};

pub type QualificationResult<T> = Result<T, QualificationValidationError>;

/// Stable categories suitable for UI diagnostics and automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QualificationErrorCode {
    UnsupportedSchema,
    MissingRequiredValue,
    DuplicateId,
    DuplicateName,
    InvalidNumber,
    InconsistentResult,
    InputDigestMismatch,
    InvalidExecutionDefinition,
    SourceBindingMismatch,
    SuiteBindingMismatch,
    EvidenceCoverageMismatch,
    DocumentationIncomplete,
    LicenseIncomplete,
    ConsumerImpactIncomplete,
    CompatibilityIncomplete,
    ApprovalIncomplete,
    ChecklistMismatch,
    ImmutableRecord,
    DispositionInvalid,
    PromotionBlocked,
}

/// One actionable, path-addressed validation failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationValidationError {
    pub code: QualificationErrorCode,
    pub path: String,
    pub message: String,
}

impl QualificationValidationError {
    pub fn new(
        code: QualificationErrorCode,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for QualificationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for QualificationValidationError {}

/// An IEEE-754 value whose invariant excludes NaN, infinity, and negatives.
///
/// `-0.0` is canonicalized to `0.0`, making equality reflexive and stable for
/// serialized qualification evidence.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct NonNegativeFinite(f64);

impl NonNegativeFinite {
    pub fn new(value: f64) -> QualificationResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidNumber,
                "number",
                "value must be finite and non-negative",
            ));
        }
        Ok(Self(if value == 0.0 { 0.0 } else { value }))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for NonNegativeFinite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl PartialEq for NonNegativeFinite {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for NonNegativeFinite {}

impl PartialOrd for NonNegativeFinite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonNegativeFinite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// A finite IEEE-754 value retained bit-for-bit in an executable contract.
///
/// Qualification references and sweep bounds must never contain NaN or
/// infinity. `-0.0` is canonicalized so serialized contracts have one stable
/// representation for zero.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct FiniteValue(f64);

impl FiniteValue {
    pub fn new(value: f64) -> QualificationResult<Self> {
        if !value.is_finite() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidNumber,
                "number",
                "value must be finite",
            ));
        }
        Ok(Self(if value == 0.0 { 0.0 } else { value }))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for FiniteValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl PartialEq for FiniteValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for FiniteValue {}

impl PartialOrd for FiniteValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiniteValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Exact model source identity against which results were produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSourceEvidenceBinding {
    pub model_id: String,
    /// Legacy records did not retain the manager-owned source UUID. They may
    /// still be inspected, but cannot produce new evidence or be promoted.
    #[serde(default)]
    pub source_id: Option<ModelSourceId>,
    pub source_digest: ContentDigest,
    pub source_revision: ObjectRevision,
}

impl ModelSourceEvidenceBinding {
    pub fn try_new(
        model_id: impl Into<String>,
        source_digest: ContentDigest,
        source_revision: ObjectRevision,
    ) -> QualificationResult<Self> {
        let value = Self {
            model_id: model_id.into(),
            source_id: None,
            source_digest,
            source_revision,
        };
        value.validate("source_binding")?;
        Ok(value)
    }

    pub fn try_new_project_bound(
        model_id: impl Into<String>,
        source_id: ModelSourceId,
        source_digest: ContentDigest,
        source_revision: ObjectRevision,
    ) -> QualificationResult<Self> {
        let value = Self {
            model_id: model_id.into(),
            source_id: Some(source_id),
            source_digest,
            source_revision,
        };
        value.validate("source_binding")?;
        value.require_project_bound("source_binding")?;
        Ok(value)
    }

    pub fn validate(&self, path: &str) -> QualificationResult<()> {
        if self.model_id.trim().is_empty() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                format!("{path}.model_id"),
                "value must not be blank",
            ));
        }
        Ok(())
    }

    pub fn require_project_bound(&self, path: &str) -> QualificationResult<ModelSourceId> {
        self.source_id.ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                format!("{path}.source_id"),
                "new qualification evidence requires an exact project-owned source identity",
            )
        })
    }
}
