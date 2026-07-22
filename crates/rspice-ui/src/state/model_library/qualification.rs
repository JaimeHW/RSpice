//! Versioned model-qualification and model-release contracts.
//!
//! This module is intentionally independent of egui.  It is the domain model
//! behind the Model Editor's existing **Tests** and **Release** surfaces:
//! golden vectors, Desktop/WebAssembly parity, immutable source-bound evidence,
//! release declarations, and fail-closed promotion.  A failed qualification is
//! valid evidence (and remains inspectable); it is never release-eligible.

use std::{cmp::Ordering, collections::BTreeSet, fmt};

use serde::{Deserialize, Deserializer, Serialize};

use crate::product::{ContentDigest, ObjectRevision};

/// Current persisted schema for model qualification and release records.
pub const MODEL_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

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
    SourceBindingMismatch,
    SuiteBindingMismatch,
    EvidenceCoverageMismatch,
    DocumentationIncomplete,
    LicenseIncomplete,
    ConsumerImpactIncomplete,
    CompatibilityIncomplete,
    ApprovalIncomplete,
    ChecklistMismatch,
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
    fn new(
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

/// The two runtime targets represented by the current Model Editor parity UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QualificationPlatform {
    Desktop,
    WebAssembly,
}

impl QualificationPlatform {
    pub const REQUIRED: [Self; 2] = [Self::Desktop, Self::WebAssembly];
}

/// Expected tolerance for one named reference quantity in a golden vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceTolerance {
    pub quantity: String,
    pub absolute: NonNegativeFinite,
    pub relative: NonNegativeFinite,
}

impl ReferenceTolerance {
    pub fn try_new(
        quantity: impl Into<String>,
        absolute: f64,
        relative: f64,
    ) -> QualificationResult<Self> {
        let value = Self {
            quantity: quantity.into(),
            absolute: NonNegativeFinite::new(absolute)
                .map_err(|error| at_path(error, "reference_tolerance.absolute"))?,
            relative: NonNegativeFinite::new(relative)
                .map_err(|error| at_path(error, "reference_tolerance.relative"))?,
        };
        require_text("reference_tolerance.quantity", &value.quantity)?;
        Ok(value)
    }
}

/// One immutable test input and its declared reference quantities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationVector {
    pub id: String,
    pub name: String,
    pub input_digest: ContentDigest,
    pub tolerances: Vec<ReferenceTolerance>,
}

impl QualificationVector {
    pub fn try_new(
        id: impl Into<String>,
        name: impl Into<String>,
        input_digest: ContentDigest,
        mut tolerances: Vec<ReferenceTolerance>,
    ) -> QualificationResult<Self> {
        tolerances
            .sort_by(|left, right| normalized(&left.quantity).cmp(&normalized(&right.quantity)));
        let value = Self {
            id: id.into(),
            name: name.into(),
            input_digest,
            tolerances,
        };
        value.validate("vector")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.name"), &self.name)?;
        if self.tolerances.is_empty() {
            return Err(missing(
                format!("{path}.tolerances"),
                "a qualification vector requires at least one reference tolerance",
            ));
        }
        for (index, tolerance) in self.tolerances.iter().enumerate() {
            require_text(
                &format!("{path}.tolerances[{index}].quantity"),
                &tolerance.quantity,
            )?;
        }
        ensure_unique(
            &format!("{path}.tolerances"),
            self.tolerances.iter().map(|value| value.quantity.as_str()),
            QualificationErrorCode::DuplicateName,
            "reference quantity",
        )
    }
}

/// A versioned group displayed as one suite row in the Tests surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationSuite {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub revision: ObjectRevision,
    pub vectors: Vec<QualificationVector>,
}

impl QualificationSuite {
    pub fn try_new(
        id: impl Into<String>,
        name: impl Into<String>,
        revision: ObjectRevision,
        mut vectors: Vec<QualificationVector>,
    ) -> QualificationResult<Self> {
        vectors.sort_by(|left, right| normalized(&left.id).cmp(&normalized(&right.id)));
        let value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: id.into(),
            name: name.into(),
            revision,
            vectors,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> QualificationResult<()> {
        require_schema("suite.schema_version", self.schema_version)?;
        require_text("suite.id", &self.id)?;
        require_text("suite.name", &self.name)?;
        if self.vectors.is_empty() {
            return Err(missing(
                "suite.vectors",
                "a qualification suite requires at least one vector",
            ));
        }
        for (index, vector) in self.vectors.iter().enumerate() {
            vector.validate(&format!("suite.vectors[{index}]"))?;
        }
        ensure_unique(
            "suite.vectors",
            self.vectors.iter().map(|value| value.id.as_str()),
            QualificationErrorCode::DuplicateId,
            "vector ID",
        )?;
        ensure_unique(
            "suite.vectors",
            self.vectors.iter().map(|value| value.name.as_str()),
            QualificationErrorCode::DuplicateName,
            "vector name",
        )
    }
}

/// Exact model source identity against which results were produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelSourceEvidenceBinding {
    pub model_id: String,
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
            source_digest,
            source_revision,
        };
        value.validate("source_binding")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.model_id"), &self.model_id)
    }
}

/// Observed error and the exact tolerance used to make its disposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceErrorEvidence {
    pub quantity: String,
    pub absolute_error: NonNegativeFinite,
    pub relative_error: NonNegativeFinite,
    pub absolute_tolerance: NonNegativeFinite,
    pub relative_tolerance: NonNegativeFinite,
    pub passed: bool,
}

impl ReferenceErrorEvidence {
    pub fn try_new(
        quantity: impl Into<String>,
        absolute_error: f64,
        relative_error: f64,
        absolute_tolerance: f64,
        relative_tolerance: f64,
    ) -> QualificationResult<Self> {
        let absolute_error = NonNegativeFinite::new(absolute_error)
            .map_err(|error| at_path(error, "reference_error.absolute_error"))?;
        let relative_error = NonNegativeFinite::new(relative_error)
            .map_err(|error| at_path(error, "reference_error.relative_error"))?;
        let absolute_tolerance = NonNegativeFinite::new(absolute_tolerance)
            .map_err(|error| at_path(error, "reference_error.absolute_tolerance"))?;
        let relative_tolerance = NonNegativeFinite::new(relative_tolerance)
            .map_err(|error| at_path(error, "reference_error.relative_tolerance"))?;
        let passed = absolute_error <= absolute_tolerance || relative_error <= relative_tolerance;
        let value = Self {
            quantity: quantity.into(),
            absolute_error,
            relative_error,
            absolute_tolerance,
            relative_tolerance,
            passed,
        };
        value.validate("reference_error")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.quantity"), &self.quantity)?;
        let expected = self.absolute_error <= self.absolute_tolerance
            || self.relative_error <= self.relative_tolerance;
        if self.passed != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                format!("{path}.passed"),
                "reference disposition does not match the recorded error and tolerance",
            ));
        }
        Ok(())
    }

    fn validate_against(
        &self,
        expected: &ReferenceTolerance,
        path: &str,
    ) -> QualificationResult<()> {
        self.validate(path)?;
        if normalized(&self.quantity) != normalized(&expected.quantity)
            || self.absolute_tolerance != expected.absolute
            || self.relative_tolerance != expected.relative
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                path,
                "reference evidence does not use the vector's declared quantity and tolerances",
            ));
        }
        Ok(())
    }
}

/// Result for one vector on one runtime platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformQualificationOutcome {
    pub platform: QualificationPlatform,
    pub references: Vec<ReferenceErrorEvidence>,
    pub passed: bool,
}

impl PlatformQualificationOutcome {
    pub fn try_new(
        platform: QualificationPlatform,
        mut references: Vec<ReferenceErrorEvidence>,
    ) -> QualificationResult<Self> {
        references
            .sort_by(|left, right| normalized(&left.quantity).cmp(&normalized(&right.quantity)));
        let passed = !references.is_empty() && references.iter().all(|value| value.passed);
        let value = Self {
            platform,
            references,
            passed,
        };
        value.validate("platform_outcome")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        if self.references.is_empty() {
            return Err(missing(
                format!("{path}.references"),
                "a platform outcome requires reference evidence",
            ));
        }
        for (index, reference) in self.references.iter().enumerate() {
            reference.validate(&format!("{path}.references[{index}]"))?;
        }
        ensure_unique(
            &format!("{path}.references"),
            self.references.iter().map(|value| value.quantity.as_str()),
            QualificationErrorCode::DuplicateName,
            "reference quantity",
        )?;
        let expected = self.references.iter().all(|value| value.passed);
        if self.passed != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                format!("{path}.passed"),
                "platform result does not match its reference results",
            ));
        }
        Ok(())
    }

    fn validate_against(
        &self,
        vector: &QualificationVector,
        path: &str,
    ) -> QualificationResult<()> {
        self.validate(path)?;
        if self.references.len() != vector.tolerances.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                format!("{path}.references"),
                "platform evidence must cover every declared reference quantity exactly once",
            ));
        }
        for expected in &vector.tolerances {
            let observed = find_ci(&self.references, &expected.quantity, |value| {
                &value.quantity
            })
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::EvidenceCoverageMismatch,
                    format!("{path}.references"),
                    format!("missing reference evidence for {:?}", expected.quantity),
                )
            })?;
            observed.validate_against(expected, path)?;
        }
        Ok(())
    }
}

/// Desktop and WebAssembly results for one golden vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationVectorOutcome {
    pub vector_id: String,
    pub input_digest: ContentDigest,
    pub platforms: Vec<PlatformQualificationOutcome>,
    pub passed: bool,
}

impl QualificationVectorOutcome {
    pub fn try_new(
        vector_id: impl Into<String>,
        input_digest: ContentDigest,
        mut platforms: Vec<PlatformQualificationOutcome>,
    ) -> QualificationResult<Self> {
        platforms.sort_by_key(|value| value.platform);
        let passed = platforms.len() == QualificationPlatform::REQUIRED.len()
            && platforms.iter().all(|value| value.passed);
        let value = Self {
            vector_id: vector_id.into(),
            input_digest,
            platforms,
            passed,
        };
        value.validate("vector_outcome")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.vector_id"), &self.vector_id)?;
        if self.platforms.len() != QualificationPlatform::REQUIRED.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                format!("{path}.platforms"),
                "each vector requires exactly one Desktop and one WebAssembly outcome",
            ));
        }
        let mut seen = BTreeSet::new();
        for (index, outcome) in self.platforms.iter().enumerate() {
            if !seen.insert(outcome.platform) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("{path}.platforms[{index}]"),
                    "qualification platform is duplicated",
                ));
            }
            outcome.validate(&format!("{path}.platforms[{index}]"))?;
        }
        if !QualificationPlatform::REQUIRED
            .iter()
            .all(|platform| seen.contains(platform))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                format!("{path}.platforms"),
                "Desktop and WebAssembly outcomes are both required",
            ));
        }
        let expected = self.platforms.iter().all(|value| value.passed);
        if self.passed != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                format!("{path}.passed"),
                "vector result does not match its platform results",
            ));
        }
        Ok(())
    }

    fn validate_against(
        &self,
        vector: &QualificationVector,
        path: &str,
    ) -> QualificationResult<()> {
        self.validate(path)?;
        if normalized(&self.vector_id) != normalized(&vector.id)
            || self.input_digest != vector.input_digest
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                path,
                "vector evidence is not bound to the declared vector identity and digest",
            ));
        }
        for outcome in &self.platforms {
            outcome.validate_against(vector, path)?;
        }
        Ok(())
    }
}

/// Immutable qualification evidence for one suite and exact model source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationEvidence {
    pub schema_version: u32,
    pub id: String,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub vector_outcomes: Vec<QualificationVectorOutcome>,
    pub passed: bool,
}

impl QualificationEvidence {
    pub fn try_new(
        id: impl Into<String>,
        source: ModelSourceEvidenceBinding,
        suite_id: impl Into<String>,
        suite_revision: ObjectRevision,
        mut vector_outcomes: Vec<QualificationVectorOutcome>,
    ) -> QualificationResult<Self> {
        vector_outcomes
            .sort_by(|left, right| normalized(&left.vector_id).cmp(&normalized(&right.vector_id)));
        let passed =
            !vector_outcomes.is_empty() && vector_outcomes.iter().all(|value| value.passed);
        let value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: id.into(),
            source,
            suite_id: suite_id.into(),
            suite_revision,
            vector_outcomes,
            passed,
        };
        value.validate_internal()?;
        Ok(value)
    }

    fn validate_internal(&self) -> QualificationResult<()> {
        require_schema("evidence.schema_version", self.schema_version)?;
        require_text("evidence.id", &self.id)?;
        require_text("evidence.suite_id", &self.suite_id)?;
        self.source.validate("evidence.source")?;
        if self.vector_outcomes.is_empty() {
            return Err(missing(
                "evidence.vector_outcomes",
                "qualification evidence requires vector outcomes",
            ));
        }
        for (index, outcome) in self.vector_outcomes.iter().enumerate() {
            outcome.validate(&format!("evidence.vector_outcomes[{index}]"))?;
        }
        ensure_unique(
            "evidence.vector_outcomes",
            self.vector_outcomes
                .iter()
                .map(|value| value.vector_id.as_str()),
            QualificationErrorCode::DuplicateId,
            "vector outcome ID",
        )?;
        let expected = self.vector_outcomes.iter().all(|value| value.passed);
        if self.passed != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                "evidence.passed",
                "suite result does not match its vector results",
            ));
        }
        Ok(())
    }

    /// Validates coverage, tolerances, vector digests, and the exact source.
    /// A consistently recorded failing result is valid evidence and returns
    /// `Ok(())`; promotion eligibility is checked separately.
    pub fn validate_bound(
        &self,
        suite: &QualificationSuite,
        expected_source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<()> {
        self.validate_internal()?;
        suite.validate()?;
        if &self.source != expected_source {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "evidence.source",
                "evidence model ID, source digest, or source revision does not match",
            ));
        }
        if normalized(&self.suite_id) != normalized(&suite.id)
            || self.suite_revision != suite.revision
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SuiteBindingMismatch,
                "evidence.suite",
                "evidence suite ID or revision does not match",
            ));
        }
        if self.vector_outcomes.len() != suite.vectors.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "evidence.vector_outcomes",
                "evidence must cover every suite vector exactly once",
            ));
        }
        for vector in &suite.vectors {
            let outcome = find_ci(&self.vector_outcomes, &vector.id, |value| &value.vector_id)
                .ok_or_else(|| {
                    QualificationValidationError::new(
                        QualificationErrorCode::EvidenceCoverageMismatch,
                        "evidence.vector_outcomes",
                        format!("missing outcome for vector {:?}", vector.id),
                    )
                })?;
            outcome.validate_against(vector, "evidence.vector_outcomes")?;
        }
        Ok(())
    }

    #[must_use]
    pub fn platform_passed(&self, platform: QualificationPlatform) -> bool {
        !self.vector_outcomes.is_empty()
            && self.vector_outcomes.iter().all(|vector| {
                vector
                    .platforms
                    .iter()
                    .find(|outcome| outcome.platform == platform)
                    .is_some_and(|outcome| outcome.passed)
            })
    }
}

/// Content-addressed document used by release declarations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentReference {
    pub id: String,
    pub digest: ContentDigest,
}

impl DocumentReference {
    pub fn try_new(id: impl Into<String>, digest: ContentDigest) -> QualificationResult<Self> {
        let value = Self {
            id: id.into(),
            digest,
        };
        value.validate("document")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.id"), &self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequiredDocumentation {
    ModelDescription,
    ParameterReference,
    QualificationReport,
}

impl RequiredDocumentation {
    pub const REQUIRED: [Self; 3] = [
        Self::ModelDescription,
        Self::ParameterReference,
        Self::QualificationReport,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentationDeclaration {
    pub kind: RequiredDocumentation,
    pub document: DocumentReference,
}

/// Documents declared by a release candidate. Incomplete sets remain useful
/// draft state, but `validate_complete` fails closed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DocumentationSet {
    pub declarations: Vec<DocumentationDeclaration>,
}

impl DocumentationSet {
    pub fn try_new(mut declarations: Vec<DocumentationDeclaration>) -> QualificationResult<Self> {
        declarations.sort_by_key(|value| value.kind);
        let value = Self { declarations };
        value.validate_structure()?;
        Ok(value)
    }

    fn validate_structure(&self) -> QualificationResult<()> {
        let mut kinds = BTreeSet::new();
        let mut ids = BTreeSet::new();
        for (index, declaration) in self.declarations.iter().enumerate() {
            declaration
                .document
                .validate(&format!("documentation.declarations[{index}].document"))?;
            if !kinds.insert(declaration.kind) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("documentation.declarations[{index}].kind"),
                    "required document kind is duplicated",
                ));
            }
            if !ids.insert(normalized(&declaration.document.id)) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("documentation.declarations[{index}].document.id"),
                    "document ID is duplicated case-insensitively",
                ));
            }
        }
        Ok(())
    }

    pub fn validate_complete(&self) -> QualificationResult<()> {
        self.validate_structure()?;
        let present: BTreeSet<_> = self.declarations.iter().map(|value| value.kind).collect();
        if !RequiredDocumentation::REQUIRED
            .iter()
            .all(|kind| present.contains(kind))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DocumentationIncomplete,
                "documentation.declarations",
                "model description, parameter reference, and qualification report are required",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LicenseScope {
    FoundryProject,
    OrganizationInternal,
    Redistributable,
}

/// Reviewed license declaration for the promoted model package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseDeclaration {
    pub license_id: String,
    pub expression: String,
    pub scope: LicenseScope,
    pub commercial_use_allowed: bool,
    pub redistribution_allowed: bool,
    pub reviewed: bool,
    pub notice: DocumentReference,
}

impl LicenseDeclaration {
    pub fn validate_for_release(&self) -> QualificationResult<()> {
        require_text("license.license_id", &self.license_id)
            .map_err(|_| license_incomplete("license.license_id", "license ID is required"))?;
        require_text("license.expression", &self.expression).map_err(|_| {
            license_incomplete("license.expression", "license expression is required")
        })?;
        self.notice
            .validate("license.notice")
            .map_err(|_| license_incomplete("license.notice", "license notice is required"))?;
        if !self.reviewed || !self.commercial_use_allowed {
            return Err(license_incomplete(
                "license",
                "a reviewed license permitting commercial use is required",
            ));
        }
        if self.scope == LicenseScope::Redistributable && !self.redistribution_allowed {
            return Err(license_incomplete(
                "license.redistribution_allowed",
                "a redistributable package must explicitly permit redistribution",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsumerChange {
    NoImpact,
    Compatible,
    Breaking,
}

/// Explicit impact assessment for cells and plans that consume the model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerImpactAssessment {
    pub change: ConsumerChange,
    pub summary: String,
    pub affected_consumer_ids: Vec<String>,
    pub migration_plan: Option<DocumentReference>,
    pub reviewed: bool,
}

impl ConsumerImpactAssessment {
    pub fn try_new(
        change: ConsumerChange,
        summary: impl Into<String>,
        mut affected_consumer_ids: Vec<String>,
        migration_plan: Option<DocumentReference>,
        reviewed: bool,
    ) -> QualificationResult<Self> {
        affected_consumer_ids.sort_by_key(|value| normalized(value));
        let value = Self {
            change,
            summary: summary.into(),
            affected_consumer_ids,
            migration_plan,
            reviewed,
        };
        value.validate_for_release()?;
        Ok(value)
    }

    pub fn validate_for_release(&self) -> QualificationResult<()> {
        require_text("consumer_impact.summary", &self.summary).map_err(|_| {
            consumer_incomplete("consumer_impact.summary", "impact summary is required")
        })?;
        for (index, id) in self.affected_consumer_ids.iter().enumerate() {
            require_text(
                &format!("consumer_impact.affected_consumer_ids[{index}]"),
                id,
            )
            .map_err(|_| {
                consumer_incomplete(
                    format!("consumer_impact.affected_consumer_ids[{index}]"),
                    "consumer ID is required",
                )
            })?;
        }
        ensure_unique(
            "consumer_impact.affected_consumer_ids",
            self.affected_consumer_ids.iter().map(String::as_str),
            QualificationErrorCode::DuplicateId,
            "consumer ID",
        )?;
        if let Some(plan) = &self.migration_plan {
            plan.validate("consumer_impact.migration_plan")?;
        }
        if !self.reviewed {
            return Err(consumer_incomplete(
                "consumer_impact.reviewed",
                "consumer impact must be reviewed",
            ));
        }
        if self.change == ConsumerChange::Breaking && self.migration_plan.is_none() {
            return Err(consumer_incomplete(
                "consumer_impact.migration_plan",
                "a breaking change requires a content-addressed migration plan",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatibilityDisposition {
    Compatible,
    MigrationRequired,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformCompatibilityEvidence {
    pub platform: QualificationPlatform,
    pub disposition: CompatibilityDisposition,
    pub evidence: DocumentReference,
}

/// Runtime and pinned-consumer compatibility declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityAssessment {
    pub platforms: Vec<PlatformCompatibilityEvidence>,
    pub existing_projects: CompatibilityDisposition,
    pub reviewed: bool,
}

impl CompatibilityAssessment {
    pub fn try_new(
        mut platforms: Vec<PlatformCompatibilityEvidence>,
        existing_projects: CompatibilityDisposition,
        reviewed: bool,
    ) -> QualificationResult<Self> {
        platforms.sort_by_key(|value| value.platform);
        let value = Self {
            platforms,
            existing_projects,
            reviewed,
        };
        value.validate_structure()?;
        Ok(value)
    }

    fn validate_structure(&self) -> QualificationResult<()> {
        let mut seen = BTreeSet::new();
        for (index, platform) in self.platforms.iter().enumerate() {
            if !seen.insert(platform.platform) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("compatibility.platforms[{index}].platform"),
                    "compatibility platform is duplicated",
                ));
            }
            platform
                .evidence
                .validate(&format!("compatibility.platforms[{index}].evidence"))?;
        }
        Ok(())
    }

    pub fn validate_for_release(
        &self,
        impact: &ConsumerImpactAssessment,
    ) -> QualificationResult<()> {
        self.validate_structure()?;
        if !self.reviewed {
            return Err(compatibility_incomplete(
                "compatibility.reviewed",
                "compatibility assessment must be reviewed",
            ));
        }
        for required in QualificationPlatform::REQUIRED {
            let result = self
                .platforms
                .iter()
                .find(|value| value.platform == required);
            let Some(result) = result else {
                return Err(compatibility_incomplete(
                    "compatibility.platforms",
                    "Desktop and WebAssembly compatibility evidence are required",
                ));
            };
            if result.disposition == CompatibilityDisposition::Incompatible {
                return Err(compatibility_incomplete(
                    "compatibility.platforms",
                    "an incompatible runtime platform blocks release",
                ));
            }
        }
        if self.platforms.len() != QualificationPlatform::REQUIRED.len() {
            return Err(compatibility_incomplete(
                "compatibility.platforms",
                "exactly one Desktop and one WebAssembly compatibility result are required",
            ));
        }
        let migration_required = self.existing_projects
            == CompatibilityDisposition::MigrationRequired
            || self
                .platforms
                .iter()
                .any(|value| value.disposition == CompatibilityDisposition::MigrationRequired);
        if self.existing_projects == CompatibilityDisposition::Incompatible {
            return Err(compatibility_incomplete(
                "compatibility.existing_projects",
                "incompatible existing projects block release",
            ));
        }
        if migration_required
            && (impact.change != ConsumerChange::Breaking || impact.migration_plan.is_none())
        {
            return Err(compatibility_incomplete(
                "compatibility",
                "migration-required compatibility needs a reviewed breaking-impact migration plan",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PromotionApprovalRole {
    ModelOwner,
    QualificationApprover,
}

impl PromotionApprovalRole {
    pub const REQUIRED: [Self; 2] = [Self::ModelOwner, Self::QualificationApprover];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApprovalDecision {
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionApproval {
    pub role: PromotionApprovalRole,
    pub approver_id: String,
    pub decision: ApprovalDecision,
    pub decision_revision: ObjectRevision,
}

/// Stable identity of a draft or frozen model release candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseCandidateIdentity {
    pub id: String,
    pub model_id: String,
    pub version: String,
}

impl ReleaseCandidateIdentity {
    fn validate(&self) -> QualificationResult<()> {
        require_text("candidate.identity.id", &self.id)?;
        require_text("candidate.identity.model_id", &self.model_id)?;
        require_text("candidate.identity.version", &self.version)
    }
}

/// Stable identity assigned only after a candidate passes promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelReleaseIdentity {
    pub id: String,
    pub model_id: String,
    pub version: String,
}

impl ModelReleaseIdentity {
    fn validate(&self) -> QualificationResult<()> {
        require_text("release.identity.id", &self.id)?;
        require_text("release.identity.model_id", &self.model_id)?;
        require_text("release.identity.version", &self.version)
    }
}

/// Materialized release gates shown by the Release surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionChecklist {
    pub source_evidence_bound: bool,
    pub qualification_passed: bool,
    pub desktop_passed: bool,
    pub webassembly_passed: bool,
    pub documentation_complete: bool,
    pub license_complete: bool,
    pub consumer_impact_complete: bool,
    pub compatibility_complete: bool,
    pub independent_approval_complete: bool,
}

impl PromotionChecklist {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.source_evidence_bound
            && self.qualification_passed
            && self.desktop_passed
            && self.webassembly_passed
            && self.documentation_complete
            && self.license_complete
            && self.consumer_impact_complete
            && self.compatibility_complete
            && self.independent_approval_complete
    }
}

/// Versioned candidate state consumed by the existing Release surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelReleaseCandidate {
    pub schema_version: u32,
    pub identity: ReleaseCandidateIdentity,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub evidence_id: String,
    pub documentation: DocumentationSet,
    pub license: Option<LicenseDeclaration>,
    pub consumer_impact: Option<ConsumerImpactAssessment>,
    pub compatibility: Option<CompatibilityAssessment>,
    pub approvals: Vec<PromotionApproval>,
    pub checklist: PromotionChecklist,
}

impl ModelReleaseCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        identity: ReleaseCandidateIdentity,
        source: ModelSourceEvidenceBinding,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
        documentation: DocumentationSet,
        license: Option<LicenseDeclaration>,
        consumer_impact: Option<ConsumerImpactAssessment>,
        compatibility: Option<CompatibilityAssessment>,
        mut approvals: Vec<PromotionApproval>,
    ) -> QualificationResult<Self> {
        approvals.sort_by_key(|value| value.role);
        let mut value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            identity,
            source,
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            evidence_id: evidence.id.clone(),
            documentation,
            license,
            consumer_impact,
            compatibility,
            approvals,
            checklist: PromotionChecklist {
                source_evidence_bound: false,
                qualification_passed: false,
                desktop_passed: false,
                webassembly_passed: false,
                documentation_complete: false,
                license_complete: false,
                consumer_impact_complete: false,
                compatibility_complete: false,
                independent_approval_complete: false,
            },
        };
        value.checklist = value.derived_checklist(suite, evidence);
        value.validate_bound(suite, evidence)?;
        Ok(value)
    }

    fn approvals_complete(&self) -> bool {
        let mut roles = BTreeSet::new();
        let mut people = BTreeSet::new();
        self.approvals.iter().all(|approval| {
            approval.decision == ApprovalDecision::Approved
                && !approval.approver_id.trim().is_empty()
                && roles.insert(approval.role)
                && people.insert(normalized(&approval.approver_id))
        }) && PromotionApprovalRole::REQUIRED
            .iter()
            .all(|role| roles.contains(role))
            && self.approvals.len() == PromotionApprovalRole::REQUIRED.len()
    }

    fn derived_checklist(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> PromotionChecklist {
        let source_evidence_bound = evidence.validate_bound(suite, &self.source).is_ok()
            && normalized(&self.suite_id) == normalized(&suite.id)
            && self.suite_revision == suite.revision
            && normalized(&self.evidence_id) == normalized(&evidence.id);
        let consumer_impact_complete = self
            .consumer_impact
            .as_ref()
            .is_some_and(|value| value.validate_for_release().is_ok());
        let compatibility_complete = self
            .compatibility
            .as_ref()
            .zip(self.consumer_impact.as_ref())
            .is_some_and(|(compatibility, impact)| {
                compatibility.validate_for_release(impact).is_ok()
            });
        PromotionChecklist {
            source_evidence_bound,
            qualification_passed: source_evidence_bound && evidence.passed,
            desktop_passed: source_evidence_bound
                && evidence.platform_passed(QualificationPlatform::Desktop),
            webassembly_passed: source_evidence_bound
                && evidence.platform_passed(QualificationPlatform::WebAssembly),
            documentation_complete: self.documentation.validate_complete().is_ok(),
            license_complete: self
                .license
                .as_ref()
                .is_some_and(|value| value.validate_for_release().is_ok()),
            consumer_impact_complete,
            compatibility_complete,
            independent_approval_complete: self.approvals_complete(),
        }
    }

    /// Validates references and stored checklist state without requiring the
    /// candidate to be promotable. This preserves honest blocked candidates.
    pub fn validate_bound(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        require_schema("candidate.schema_version", self.schema_version)?;
        self.identity.validate()?;
        self.source.validate("candidate.source")?;
        if normalized(&self.identity.model_id) != normalized(&self.source.model_id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "candidate.identity.model_id",
                "candidate model identity does not match its source binding",
            ));
        }
        require_text("candidate.suite_id", &self.suite_id)?;
        require_text("candidate.evidence_id", &self.evidence_id)?;
        self.documentation.validate_structure()?;
        if let Some(license) = &self.license {
            license.notice.validate("candidate.license.notice")?;
        }
        if let Some(impact) = &self.consumer_impact {
            for (index, consumer) in impact.affected_consumer_ids.iter().enumerate() {
                require_text(
                    &format!("candidate.consumer_impact.affected_consumer_ids[{index}]"),
                    consumer,
                )?;
            }
            ensure_unique(
                "candidate.consumer_impact.affected_consumer_ids",
                impact.affected_consumer_ids.iter().map(String::as_str),
                QualificationErrorCode::DuplicateId,
                "consumer ID",
            )?;
        }
        if let Some(compatibility) = &self.compatibility {
            compatibility.validate_structure()?;
        }
        validate_approvals(&self.approvals)?;
        evidence.validate_bound(suite, &self.source)?;
        if normalized(&self.suite_id) != normalized(&suite.id)
            || self.suite_revision != suite.revision
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SuiteBindingMismatch,
                "candidate.suite",
                "candidate suite binding does not match",
            ));
        }
        if normalized(&self.evidence_id) != normalized(&evidence.id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "candidate.evidence_id",
                "candidate evidence identity does not match",
            ));
        }
        let expected = self.derived_checklist(suite, evidence);
        if self.checklist != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ChecklistMismatch,
                "candidate.checklist",
                "stored promotion checklist does not match the bound declarations and evidence",
            ));
        }
        Ok(())
    }

    /// Fail-closed validation used by the guarded promotion operation.
    pub fn validate_for_promotion(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        self.validate_bound(suite, evidence)?;
        self.documentation.validate_complete()?;
        self.license
            .as_ref()
            .ok_or_else(|| {
                license_incomplete("candidate.license", "license declaration is required")
            })?
            .validate_for_release()?;
        let impact = self.consumer_impact.as_ref().ok_or_else(|| {
            consumer_incomplete(
                "candidate.consumer_impact",
                "consumer-impact assessment is required",
            )
        })?;
        impact.validate_for_release()?;
        self.compatibility
            .as_ref()
            .ok_or_else(|| {
                compatibility_incomplete(
                    "candidate.compatibility",
                    "compatibility assessment is required",
                )
            })?
            .validate_for_release(impact)?;
        if !self.approvals_complete() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ApprovalIncomplete,
                "candidate.approvals",
                "distinct model-owner and qualification-approver approvals are required",
            ));
        }
        if !evidence.passed
            || !evidence.platform_passed(QualificationPlatform::Desktop)
            || !evidence.platform_passed(QualificationPlatform::WebAssembly)
            || !self.checklist.is_complete()
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::PromotionBlocked,
                "candidate.checklist",
                "promotion requires fully passing, source-bound Desktop and WebAssembly evidence",
            ));
        }
        Ok(())
    }
}

/// Immutable audit record emitted only by the guarded promotion constructor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPromotionRecord {
    pub schema_version: u32,
    pub id: String,
    pub candidate_identity: ReleaseCandidateIdentity,
    pub release_identity: ModelReleaseIdentity,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub evidence_id: String,
    pub checklist: PromotionChecklist,
}

/// Immutable model release created together with its promotion record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRelease {
    pub schema_version: u32,
    pub identity: ModelReleaseIdentity,
    pub candidate_id: String,
    pub source: ModelSourceEvidenceBinding,
    pub evidence_id: String,
    pub promotion_record_id: String,
}

impl ModelPromotionRecord {
    /// Creates a release and its audit record atomically in memory. No record
    /// is returned unless every gate validates against the exact bound source.
    pub fn promote(
        id: impl Into<String>,
        release_identity: ModelReleaseIdentity,
        candidate: &ModelReleaseCandidate,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<(Self, ModelRelease)> {
        candidate.validate_for_promotion(suite, evidence)?;
        let id = id.into();
        require_text("promotion.id", &id)?;
        release_identity.validate()?;
        if normalized(&release_identity.model_id) != normalized(&candidate.identity.model_id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "release.identity.model_id",
                "release and candidate model identities do not match",
            ));
        }
        if normalized(&release_identity.id) == normalized(&candidate.identity.id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                "release.identity.id",
                "release identity must be distinct from the candidate identity",
            ));
        }
        let record = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: id.clone(),
            candidate_identity: candidate.identity.clone(),
            release_identity: release_identity.clone(),
            source: candidate.source.clone(),
            suite_id: candidate.suite_id.clone(),
            suite_revision: candidate.suite_revision,
            evidence_id: candidate.evidence_id.clone(),
            checklist: candidate.checklist.clone(),
        };
        let release = ModelRelease {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            identity: release_identity,
            candidate_id: candidate.identity.id.clone(),
            source: candidate.source.clone(),
            evidence_id: candidate.evidence_id.clone(),
            promotion_record_id: id,
        };
        record.validate_bound(&release, candidate, suite, evidence)?;
        Ok((record, release))
    }

    pub fn validate_bound(
        &self,
        release: &ModelRelease,
        candidate: &ModelReleaseCandidate,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        require_schema("promotion.schema_version", self.schema_version)?;
        require_schema("release.schema_version", release.schema_version)?;
        require_text("promotion.id", &self.id)?;
        self.release_identity.validate()?;
        release.identity.validate()?;
        candidate.validate_for_promotion(suite, evidence)?;
        let exact = self.candidate_identity == candidate.identity
            && self.release_identity == release.identity
            && self.source == candidate.source
            && self.source == release.source
            && normalized(&self.suite_id) == normalized(&suite.id)
            && self.suite_revision == suite.revision
            && normalized(&self.evidence_id) == normalized(&evidence.id)
            && normalized(&release.evidence_id) == normalized(&evidence.id)
            && normalized(&release.candidate_id) == normalized(&candidate.identity.id)
            && normalized(&release.promotion_record_id) == normalized(&self.id)
            && self.checklist == candidate.checklist;
        if !exact {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "promotion",
                "promotion record, release, candidate, suite, and evidence are not exactly bound",
            ));
        }
        Ok(())
    }
}

/// Deterministically ordered aggregate used to persist the Tests/Release state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelQualificationState {
    pub schema_version: u32,
    pub suites: Vec<QualificationSuite>,
    pub evidence: Vec<QualificationEvidence>,
    pub candidates: Vec<ModelReleaseCandidate>,
    pub releases: Vec<ModelRelease>,
    pub promotions: Vec<ModelPromotionRecord>,
}

impl Default for ModelQualificationState {
    fn default() -> Self {
        Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            suites: Vec::new(),
            evidence: Vec::new(),
            candidates: Vec::new(),
            releases: Vec::new(),
            promotions: Vec::new(),
        }
    }
}

impl ModelQualificationState {
    pub fn try_new(
        mut suites: Vec<QualificationSuite>,
        mut evidence: Vec<QualificationEvidence>,
        mut candidates: Vec<ModelReleaseCandidate>,
        mut releases: Vec<ModelRelease>,
        mut promotions: Vec<ModelPromotionRecord>,
    ) -> QualificationResult<Self> {
        suites.sort_by_key(|value| normalized(&value.id));
        evidence.sort_by_key(|value| normalized(&value.id));
        candidates.sort_by_key(|value| normalized(&value.identity.id));
        releases.sort_by_key(|value| normalized(&value.identity.id));
        promotions.sort_by_key(|value| normalized(&value.id));
        let value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            suites,
            evidence,
            candidates,
            releases,
            promotions,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> QualificationResult<()> {
        require_schema("qualification_state.schema_version", self.schema_version)?;
        validate_state_uniqueness(self)?;
        for suite in &self.suites {
            suite.validate()?;
        }
        for evidence in &self.evidence {
            let suite =
                find_ci(&self.suites, &evidence.suite_id, |value| &value.id).ok_or_else(|| {
                    QualificationValidationError::new(
                        QualificationErrorCode::SuiteBindingMismatch,
                        "qualification_state.evidence",
                        format!("suite {:?} does not exist", evidence.suite_id),
                    )
                })?;
            evidence.validate_bound(suite, &evidence.source)?;
        }
        for candidate in &self.candidates {
            let suite =
                find_ci(&self.suites, &candidate.suite_id, |value| &value.id).ok_or_else(|| {
                    missing(
                        "qualification_state.candidates",
                        "candidate suite is missing",
                    )
                })?;
            let evidence = find_ci(&self.evidence, &candidate.evidence_id, |value| &value.id)
                .ok_or_else(|| {
                    missing(
                        "qualification_state.candidates",
                        "candidate evidence is missing",
                    )
                })?;
            candidate.validate_bound(suite, evidence)?;
        }
        for promotion in &self.promotions {
            let candidate = find_ci(
                &self.candidates,
                &promotion.candidate_identity.id,
                |value| &value.identity.id,
            )
            .ok_or_else(|| {
                missing(
                    "qualification_state.promotions",
                    "promotion candidate is missing",
                )
            })?;
            let suite =
                find_ci(&self.suites, &promotion.suite_id, |value| &value.id).ok_or_else(|| {
                    missing(
                        "qualification_state.promotions",
                        "promotion suite is missing",
                    )
                })?;
            let evidence = find_ci(&self.evidence, &promotion.evidence_id, |value| &value.id)
                .ok_or_else(|| {
                    missing(
                        "qualification_state.promotions",
                        "promotion evidence is missing",
                    )
                })?;
            let release = find_ci(&self.releases, &promotion.id, |value| {
                &value.promotion_record_id
            })
            .ok_or_else(|| {
                missing(
                    "qualification_state.promotions",
                    "promoted release is missing",
                )
            })?;
            promotion.validate_bound(release, candidate, suite, evidence)?;
        }
        for release in &self.releases {
            if find_ci(&self.promotions, &release.promotion_record_id, |value| {
                &value.id
            })
            .is_none()
            {
                return Err(missing(
                    "qualification_state.releases",
                    "release promotion record is missing",
                ));
            }
        }
        Ok(())
    }

    /// Validate the aggregate and prove that every retained source-bound
    /// record belongs to the exact model key under which the project stores
    /// this state.
    pub fn validate_for_model(&self, model_id: &str) -> QualificationResult<()> {
        self.validate()?;
        require_text("qualification_state.model_id", model_id)?;
        let mut source_model_ids = self
            .evidence
            .iter()
            .map(|record| record.source.model_id.as_str())
            .chain(
                self.candidates
                    .iter()
                    .map(|record| record.source.model_id.as_str()),
            )
            .chain(
                self.releases
                    .iter()
                    .map(|record| record.source.model_id.as_str()),
            )
            .chain(
                self.promotions
                    .iter()
                    .map(|record| record.source.model_id.as_str()),
            );
        if source_model_ids.any(|bound_id| normalized(bound_id) != normalized(model_id)) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "qualification_state.model_id",
                "qualification evidence belongs to a different model identity",
            ));
        }
        Ok(())
    }
}

fn validate_state_uniqueness(state: &ModelQualificationState) -> QualificationResult<()> {
    ensure_unique(
        "qualification_state.suites",
        state.suites.iter().map(|value| value.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "suite ID",
    )?;
    ensure_unique(
        "qualification_state.suites",
        state.suites.iter().map(|value| value.name.as_str()),
        QualificationErrorCode::DuplicateName,
        "suite name",
    )?;
    ensure_unique(
        "qualification_state.evidence",
        state.evidence.iter().map(|value| value.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "evidence ID",
    )?;
    ensure_unique(
        "qualification_state.candidates",
        state
            .candidates
            .iter()
            .map(|value| value.identity.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "candidate ID",
    )?;
    ensure_unique_scoped(
        "qualification_state.candidates",
        state.candidates.iter().map(|value| {
            (
                value.identity.model_id.as_str(),
                value.identity.version.as_str(),
            )
        }),
        "candidate version",
    )?;
    ensure_unique(
        "qualification_state.releases",
        state
            .releases
            .iter()
            .map(|value| value.identity.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "release ID",
    )?;
    ensure_unique_scoped(
        "qualification_state.releases",
        state.releases.iter().map(|value| {
            (
                value.identity.model_id.as_str(),
                value.identity.version.as_str(),
            )
        }),
        "release version",
    )?;
    ensure_unique(
        "qualification_state.promotions",
        state.promotions.iter().map(|value| value.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "promotion ID",
    )
}

fn validate_approvals(approvals: &[PromotionApproval]) -> QualificationResult<()> {
    let mut roles = BTreeSet::new();
    let mut people = BTreeSet::new();
    for (index, approval) in approvals.iter().enumerate() {
        require_text(
            &format!("candidate.approvals[{index}].approver_id"),
            &approval.approver_id,
        )?;
        if !roles.insert(approval.role) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                format!("candidate.approvals[{index}].role"),
                "approval role is duplicated",
            ));
        }
        if !people.insert(normalized(&approval.approver_id)) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                format!("candidate.approvals[{index}].approver_id"),
                "independent approvals must use distinct approvers",
            ));
        }
    }
    Ok(())
}

fn require_schema(path: &str, version: u32) -> QualificationResult<()> {
    if version != MODEL_QUALIFICATION_SCHEMA_VERSION {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::UnsupportedSchema,
            path,
            format!(
                "expected schema version {MODEL_QUALIFICATION_SCHEMA_VERSION}, received {version}"
            ),
        ));
    }
    Ok(())
}

fn require_text(path: &str, value: &str) -> QualificationResult<()> {
    if value.trim().is_empty() {
        return Err(missing(path, "value must not be blank"));
    }
    Ok(())
}

fn ensure_unique<'a>(
    path: &str,
    values: impl IntoIterator<Item = &'a str>,
    code: QualificationErrorCode,
    label: &str,
) -> QualificationResult<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(normalized(value)) {
            return Err(QualificationValidationError::new(
                code,
                path,
                format!("duplicate {label} {value:?} (case-insensitive)"),
            ));
        }
    }
    Ok(())
}

fn ensure_unique_scoped<'a>(
    path: &str,
    values: impl IntoIterator<Item = (&'a str, &'a str)>,
    label: &str,
) -> QualificationResult<()> {
    let mut seen = BTreeSet::new();
    for (scope, value) in values {
        if !seen.insert((normalized(scope), normalized(value))) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateName,
                path,
                format!("duplicate {label} {value:?} for {scope:?} (case-insensitive)"),
            ));
        }
    }
    Ok(())
}

fn find_ci<'a, T>(values: &'a [T], needle: &str, key: impl Fn(&T) -> &str) -> Option<&'a T> {
    let needle = normalized(needle);
    values.iter().find(|value| normalized(key(value)) == needle)
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}

fn missing(path: impl Into<String>, message: impl Into<String>) -> QualificationValidationError {
    QualificationValidationError::new(QualificationErrorCode::MissingRequiredValue, path, message)
}

fn license_incomplete(
    path: impl Into<String>,
    message: impl Into<String>,
) -> QualificationValidationError {
    QualificationValidationError::new(QualificationErrorCode::LicenseIncomplete, path, message)
}

fn consumer_incomplete(
    path: impl Into<String>,
    message: impl Into<String>,
) -> QualificationValidationError {
    QualificationValidationError::new(
        QualificationErrorCode::ConsumerImpactIncomplete,
        path,
        message,
    )
}

fn compatibility_incomplete(
    path: impl Into<String>,
    message: impl Into<String>,
) -> QualificationValidationError {
    QualificationValidationError::new(
        QualificationErrorCode::CompatibilityIncomplete,
        path,
        message,
    )
}

fn at_path(mut error: QualificationValidationError, path: &str) -> QualificationValidationError {
    error.path = path.to_owned();
    error
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn source(byte: u8) -> ModelSourceEvidenceBinding {
        ModelSourceEvidenceBinding::try_new(
            "demo180_nch",
            digest(byte),
            ObjectRevision::new(7).unwrap(),
        )
        .unwrap()
    }

    fn vector(id: &str, name: &str, byte: u8) -> QualificationVector {
        QualificationVector::try_new(
            id,
            name,
            digest(byte),
            vec![ReferenceTolerance::try_new("drain_current", 0.01, 0.005).unwrap()],
        )
        .unwrap()
    }

    fn suite() -> QualificationSuite {
        QualificationSuite::try_new(
            "dc-iv",
            "DC IV",
            ObjectRevision::new(3).unwrap(),
            vec![
                vector("dc-002", "Output sweep", 2),
                vector("dc-001", "Transfer sweep", 1),
            ],
        )
        .unwrap()
    }

    fn platform_outcome(
        platform: QualificationPlatform,
        passes: bool,
    ) -> PlatformQualificationOutcome {
        let error = if passes { 0.001 } else { 0.02 };
        PlatformQualificationOutcome::try_new(
            platform,
            vec![
                ReferenceErrorEvidence::try_new("drain_current", error, error, 0.01, 0.005)
                    .unwrap(),
            ],
        )
        .unwrap()
    }

    fn evidence(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        passes: bool,
    ) -> QualificationEvidence {
        let outcomes = suite
            .vectors
            .iter()
            .map(|vector| {
                QualificationVectorOutcome::try_new(
                    vector.id.clone(),
                    vector.input_digest,
                    vec![
                        platform_outcome(QualificationPlatform::WebAssembly, passes),
                        platform_outcome(QualificationPlatform::Desktop, true),
                    ],
                )
                .unwrap()
            })
            .collect();
        QualificationEvidence::try_new(
            "evidence-7",
            source.clone(),
            suite.id.clone(),
            suite.revision,
            outcomes,
        )
        .unwrap()
    }

    fn document(id: &str, byte: u8) -> DocumentReference {
        DocumentReference::try_new(id, digest(byte)).unwrap()
    }

    fn documents() -> DocumentationSet {
        DocumentationSet::try_new(vec![
            DocumentationDeclaration {
                kind: RequiredDocumentation::QualificationReport,
                document: document("qualification-report", 13),
            },
            DocumentationDeclaration {
                kind: RequiredDocumentation::ModelDescription,
                document: document("model-description", 11),
            },
            DocumentationDeclaration {
                kind: RequiredDocumentation::ParameterReference,
                document: document("parameter-reference", 12),
            },
        ])
        .unwrap()
    }

    fn license() -> LicenseDeclaration {
        LicenseDeclaration {
            license_id: "foundry-project-42".into(),
            expression: "LicenseRef-Demo180-Project".into(),
            scope: LicenseScope::FoundryProject,
            commercial_use_allowed: true,
            redistribution_allowed: false,
            reviewed: true,
            notice: document("license-notice", 20),
        }
    }

    fn impact() -> ConsumerImpactAssessment {
        ConsumerImpactAssessment::try_new(
            ConsumerChange::Compatible,
            "Parameter-only correction within the declared model domain.",
            vec!["plan-4".into(), "cell-18".into()],
            None,
            true,
        )
        .unwrap()
    }

    fn compatibility() -> CompatibilityAssessment {
        CompatibilityAssessment::try_new(
            vec![
                PlatformCompatibilityEvidence {
                    platform: QualificationPlatform::WebAssembly,
                    disposition: CompatibilityDisposition::Compatible,
                    evidence: document("wasm-compat", 22),
                },
                PlatformCompatibilityEvidence {
                    platform: QualificationPlatform::Desktop,
                    disposition: CompatibilityDisposition::Compatible,
                    evidence: document("desktop-compat", 21),
                },
            ],
            CompatibilityDisposition::Compatible,
            true,
        )
        .unwrap()
    }

    fn approvals() -> Vec<PromotionApproval> {
        vec![
            PromotionApproval {
                role: PromotionApprovalRole::QualificationApprover,
                approver_id: "m.chen".into(),
                decision: ApprovalDecision::Approved,
                decision_revision: ObjectRevision::new(9).unwrap(),
            },
            PromotionApproval {
                role: PromotionApprovalRole::ModelOwner,
                approver_id: "a.singh".into(),
                decision: ApprovalDecision::Approved,
                decision_revision: ObjectRevision::new(8).unwrap(),
            },
        ]
    }

    fn candidate(
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
        source: &ModelSourceEvidenceBinding,
    ) -> ModelReleaseCandidate {
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "demo180-nch-candidate-18".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.1".into(),
            },
            source.clone(),
            suite,
            evidence,
            documents(),
            Some(license()),
            Some(impact()),
            Some(compatibility()),
            approvals(),
        )
        .unwrap()
    }

    #[test]
    fn non_negative_finite_rejects_negative_nan_and_infinity() {
        for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = NonNegativeFinite::new(invalid).unwrap_err();
            assert_eq!(error.code, QualificationErrorCode::InvalidNumber);
        }
        assert_eq!(
            NonNegativeFinite::new(-0.0).unwrap(),
            NonNegativeFinite::new(0.0).unwrap()
        );
        assert!(ReferenceTolerance::try_new("gain", -0.1, 0.1).is_err());
        assert!(ReferenceErrorEvidence::try_new("gain", f64::NAN, 0.0, 0.1, 0.1).is_err());
        assert!(serde_json::from_str::<NonNegativeFinite>("-0.1").is_err());
    }

    #[test]
    fn suites_are_deterministic_and_reject_case_insensitive_duplicates() {
        let suite = suite();
        assert_eq!(suite.vectors[0].id, "dc-001");
        let duplicate = QualificationSuite::try_new(
            "dc",
            "DC",
            ObjectRevision::INITIAL,
            vec![vector("Case-A", "First", 1), vector("case-a", "Second", 2)],
        )
        .unwrap_err();
        assert_eq!(duplicate.code, QualificationErrorCode::DuplicateId);

        let duplicate_name = QualificationSuite::try_new(
            "dc",
            "DC",
            ObjectRevision::INITIAL,
            vec![vector("a", "Transfer", 1), vector("b", "TRANSFER", 2)],
        )
        .unwrap_err();
        assert_eq!(duplicate_name.code, QualificationErrorCode::DuplicateName);
    }

    #[test]
    fn inconsistent_reference_platform_vector_and_suite_states_fail_closed() {
        let suite = suite();
        let source = source(7);
        let mut first_evidence = evidence(&suite, &source, true);
        first_evidence.vector_outcomes[0].platforms[0].references[0].passed = false;
        let error = first_evidence.validate_bound(&suite, &source).unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::InconsistentResult);

        let mut second_evidence = evidence(&suite, &source, true);
        second_evidence.vector_outcomes[0].platforms[0].passed = false;
        assert_eq!(
            second_evidence
                .validate_bound(&suite, &source)
                .unwrap_err()
                .code,
            QualificationErrorCode::InconsistentResult
        );

        let mut third_evidence = evidence(&suite, &source, true);
        third_evidence.passed = false;
        assert_eq!(
            third_evidence
                .validate_bound(&suite, &source)
                .unwrap_err()
                .code,
            QualificationErrorCode::InconsistentResult
        );
    }

    #[test]
    fn evidence_requires_exact_source_suite_vector_platform_and_tolerance_binding() {
        let suite = suite();
        let source_binding = source(7);
        let mut first_evidence = evidence(&suite, &source_binding, true);
        assert!(
            first_evidence
                .validate_bound(&suite, &source_binding)
                .is_ok()
        );

        let other_source = source(8);
        assert_eq!(
            first_evidence
                .validate_bound(&suite, &other_source)
                .unwrap_err()
                .code,
            QualificationErrorCode::SourceBindingMismatch
        );

        first_evidence.vector_outcomes.pop();
        first_evidence.passed = first_evidence
            .vector_outcomes
            .iter()
            .all(|value| value.passed);
        assert_eq!(
            first_evidence
                .validate_bound(&suite, &source_binding)
                .unwrap_err()
                .code,
            QualificationErrorCode::EvidenceCoverageMismatch
        );

        let mut second_evidence = evidence(&suite, &source_binding, true);
        second_evidence.vector_outcomes[0].platforms[0].references[0].absolute_tolerance =
            NonNegativeFinite::new(99.0).unwrap();
        assert_eq!(
            second_evidence
                .validate_bound(&suite, &source_binding)
                .unwrap_err()
                .code,
            QualificationErrorCode::EvidenceCoverageMismatch
        );
    }

    #[test]
    fn failed_wasm_evidence_is_valid_but_not_promotion_eligible() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, false);
        assert!(evidence.validate_bound(&suite, &source).is_ok());
        assert!(!evidence.passed);
        assert!(evidence.platform_passed(QualificationPlatform::Desktop));
        assert!(!evidence.platform_passed(QualificationPlatform::WebAssembly));

        let candidate = candidate(&suite, &evidence, &source);
        let error = candidate
            .validate_for_promotion(&suite, &evidence)
            .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::PromotionBlocked);
        assert!(!candidate.checklist.webassembly_passed);
    }

    #[test]
    fn missing_required_document_license_impact_or_compatibility_blocks_promotion() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);

        let incomplete_docs = DocumentationSet::try_new(vec![DocumentationDeclaration {
            kind: RequiredDocumentation::ModelDescription,
            document: document("description", 1),
        }])
        .unwrap();
        let candidate = ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-docs".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.docs".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            incomplete_docs,
            Some(license()),
            Some(impact()),
            Some(compatibility()),
            approvals(),
        )
        .unwrap();
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::DocumentationIncomplete
        );

        let candidate = ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-missing".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.missing".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            None,
            None,
            None,
            approvals(),
        )
        .unwrap();
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::LicenseIncomplete
        );

        let candidate = ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-impact".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.impact".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            Some(license()),
            None,
            Some(compatibility()),
            approvals(),
        )
        .unwrap();
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::ConsumerImpactIncomplete
        );

        let candidate = ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-compatibility".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.compatibility".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            Some(license()),
            Some(impact()),
            None,
            approvals(),
        )
        .unwrap();
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::CompatibilityIncomplete
        );
    }

    #[test]
    fn license_and_breaking_compatibility_rules_fail_closed() {
        let mut invalid_license = license();
        invalid_license.commercial_use_allowed = false;
        assert_eq!(
            invalid_license.validate_for_release().unwrap_err().code,
            QualificationErrorCode::LicenseIncomplete
        );

        let breaking_impact = ConsumerImpactAssessment {
            change: ConsumerChange::Breaking,
            summary: "Pin meaning changed".into(),
            affected_consumer_ids: vec!["cell-1".into()],
            migration_plan: None,
            reviewed: true,
        };
        assert_eq!(
            breaking_impact.validate_for_release().unwrap_err().code,
            QualificationErrorCode::ConsumerImpactIncomplete
        );

        let mut compatibility = compatibility();
        compatibility.platforms[0].disposition = CompatibilityDisposition::Incompatible;
        assert_eq!(
            compatibility
                .validate_for_release(&impact())
                .unwrap_err()
                .code,
            QualificationErrorCode::CompatibilityIncomplete
        );
    }

    #[test]
    fn approvals_are_independent_and_checklist_cannot_be_forged() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let mut same_person = approvals();
        same_person[1].approver_id = "M.CHEN".into();
        assert_eq!(
            ModelReleaseCandidate::try_new(
                ReleaseCandidateIdentity {
                    id: "candidate-approval".into(),
                    model_id: "demo180_nch".into(),
                    version: "4.8.3-rc.approval".into(),
                },
                source.clone(),
                &suite,
                &evidence,
                documents(),
                Some(license()),
                Some(impact()),
                Some(compatibility()),
                same_person,
            )
            .unwrap_err()
            .code,
            QualificationErrorCode::DuplicateId
        );

        let mut candidate = candidate(&suite, &evidence, &source);
        candidate.checklist.webassembly_passed = false;
        assert_eq!(
            candidate
                .validate_bound(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::ChecklistMismatch
        );
    }

    #[test]
    fn guarded_promotion_creates_exactly_bound_release_and_record() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &evidence, &source);
        let (record, release) = ModelPromotionRecord::promote(
            "promotion-19",
            ModelReleaseIdentity {
                id: "demo180-nch-release-4.8.3".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3".into(),
            },
            &candidate,
            &suite,
            &evidence,
        )
        .unwrap();
        assert!(record.checklist.is_complete());
        assert_eq!(release.promotion_record_id, record.id);
        assert!(
            record
                .validate_bound(&release, &candidate, &suite, &evidence)
                .is_ok()
        );

        let json = serde_json::to_string(&record).unwrap();
        let restored: ModelPromotionRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, record);
    }

    #[test]
    fn promotion_rejects_mismatched_source_and_failed_evidence() {
        let suite = suite();
        let source = source(7);
        let passing = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &passing, &source);
        let failed = evidence(&suite, &source, false);
        let error = ModelPromotionRecord::promote(
            "promotion-blocked",
            ModelReleaseIdentity {
                id: "release-blocked".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3".into(),
            },
            &candidate,
            &suite,
            &failed,
        )
        .unwrap_err();
        assert!(matches!(
            error.code,
            QualificationErrorCode::SourceBindingMismatch
                | QualificationErrorCode::ChecklistMismatch
                | QualificationErrorCode::PromotionBlocked
        ));
    }

    #[test]
    fn state_orders_records_and_rejects_case_insensitive_suite_names() {
        let first = suite();
        let second = QualificationSuite::try_new(
            "ac",
            "AC",
            ObjectRevision::INITIAL,
            vec![vector("ac-1", "Gain", 4)],
        )
        .unwrap();
        let state = ModelQualificationState::try_new(
            vec![first.clone(), second],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        assert_eq!(state.suites[0].id, "ac");

        let mut duplicate = first;
        duplicate.id = "other".into();
        duplicate.name = "dc iv".into();
        assert_eq!(
            ModelQualificationState::try_new(
                vec![suite(), duplicate],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
            .unwrap_err()
            .code,
            QualificationErrorCode::DuplicateName
        );
    }

    #[test]
    fn complete_state_round_trips_with_deterministic_ordering() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &evidence, &source);
        let (promotion, release) = ModelPromotionRecord::promote(
            "promotion-19",
            ModelReleaseIdentity {
                id: "release-4.8.3".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3".into(),
            },
            &candidate,
            &suite,
            &evidence,
        )
        .unwrap();
        let state = ModelQualificationState::try_new(
            vec![suite],
            vec![evidence],
            vec![candidate],
            vec![release],
            vec![promotion],
        )
        .unwrap();
        let first = serde_json::to_string(&state).unwrap();
        let restored: ModelQualificationState = serde_json::from_str(&first).unwrap();
        restored.validate().unwrap();
        restored.validate_for_model("demo180_nch").unwrap();
        assert_eq!(
            restored
                .validate_for_model("different_model")
                .expect_err("cross-model evidence must fail")
                .code,
            QualificationErrorCode::SourceBindingMismatch
        );
        assert_eq!(serde_json::to_string(&restored).unwrap(), first);
        assert_eq!(restored, state);
    }
}
