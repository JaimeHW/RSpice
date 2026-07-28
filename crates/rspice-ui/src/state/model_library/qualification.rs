//! Versioned model-qualification and model-release contracts.
//!
//! This module is intentionally independent of egui.  It is the domain model
//! behind the Model Editor's existing **Tests** and **Release** surfaces:
//! golden vectors, Desktop/WebAssembly parity, immutable source-bound evidence,
//! release declarations, and fail-closed promotion.  A failed qualification is
//! valid evidence (and remains inspectable); it is never release-eligible.

mod evidence;
mod execution;
mod vector;
mod promotion;

pub use evidence::*;
pub use execution::*;
pub use vector::*;
pub use promotion::*;

use std::{cmp::Ordering, collections::BTreeSet, fmt, path::Path};

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};

use super::definition_metadata::ModelDefinitionMetadata;

/// Current persisted schema for model qualification and release records.
pub const MODEL_QUALIFICATION_SCHEMA_VERSION: u32 = 4;

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


/// A versioned group displayed as one suite row in the Tests surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
        let source = &self.vectors[0].source;
        if self
            .vectors
            .iter()
            .skip(1)
            .any(|vector| vector.source != *source)
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "suite.vectors",
                "every vector in a qualification suite must bind the same exact model source",
            ));
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


/// Exact identity of a vector at one immutable suite revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationVectorBinding {
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub vector_id: String,
    pub input_digest: ContentDigest,
    pub source: ModelSourceEvidenceBinding,
}

impl QualificationVectorBinding {
    fn from_vector(suite: &QualificationSuite, vector: &QualificationVector) -> Self {
        Self {
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            vector_id: vector.id.clone(),
            input_digest: vector.input_digest,
            source: vector.source.clone(),
        }
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.suite_id"), &self.suite_id)?;
        require_text(&format!("{path}.vector_id"), &self.vector_id)?;
        self.source.validate(&format!("{path}.source"))?;
        self.source
            .require_project_bound(&format!("{path}.source"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QualificationVectorDispositionCause {
    Failed,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QualificationVectorRequiredAction {
    Rerun,
    Replace,
    Retire,
}

/// A disposition can only close through solver evidence or an exact
/// replacement/retirement mutation. There is intentionally no waiver state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QualificationVectorDispositionResolution {
    RerunPassed,
    Replaced {
        replacement: QualificationVectorBinding,
    },
    Retired,
}

/// Auditable, non-waiving disposition for a failed or stale vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationVectorDisposition {
    pub id: String,
    pub vector: QualificationVectorBinding,
    pub cause: QualificationVectorDispositionCause,
    pub required_action: QualificationVectorRequiredAction,
    pub reason: String,
    pub resolution: Option<QualificationVectorDispositionResolution>,
}

impl QualificationVectorDisposition {
    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        self.vector.validate(&format!("{path}.vector"))?;
        require_text(&format!("{path}.reason"), &self.reason)?;
        if self.cause == QualificationVectorDispositionCause::Stale
            && self.required_action == QualificationVectorRequiredAction::Rerun
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DispositionInvalid,
                format!("{path}.required_action"),
                "a stale vector must be replaced or retired; rerunning stale source cannot qualify the current source",
            ));
        }
        if let Some(QualificationVectorDispositionResolution::Replaced { replacement }) =
            &self.resolution
        {
            replacement.validate(&format!("{path}.resolution.replacement"))?;
            if replacement == &self.vector {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DispositionInvalid,
                    format!("{path}.resolution.replacement"),
                    "replacement must identify a different vector or suite revision",
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.resolution.is_none()
    }
}

/// Deterministically ordered aggregate used to persist the Tests/Release state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelQualificationState {
    pub schema_version: u32,
    pub suites: Vec<QualificationSuite>,
    #[serde(default)]
    pub platform_runs: Vec<QualificationPlatformRun>,
    pub evidence: Vec<QualificationEvidence>,
    pub candidates: Vec<ModelReleaseCandidate>,
    pub releases: Vec<ModelRelease>,
    pub promotions: Vec<ModelPromotionRecord>,
    #[serde(default)]
    pub vector_dispositions: Vec<QualificationVectorDisposition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedModelQualificationState {
    schema_version: u32,
    suites: Vec<QualificationSuite>,
    #[serde(default)]
    platform_runs: Vec<QualificationPlatformRun>,
    evidence: Vec<QualificationEvidence>,
    candidates: Vec<ModelReleaseCandidate>,
    releases: Vec<ModelRelease>,
    promotions: Vec<ModelPromotionRecord>,
    #[serde(default)]
    vector_dispositions: Vec<QualificationVectorDisposition>,
}

impl<'de> Deserialize<'de> for ModelQualificationState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = PersistedModelQualificationState::deserialize(deserializer)?;
        let is_empty = raw.suites.is_empty()
            && raw.platform_runs.is_empty()
            && raw.evidence.is_empty()
            && raw.candidates.is_empty()
            && raw.releases.is_empty()
            && raw.promotions.is_empty()
            && raw.vector_dispositions.is_empty();
        // Schema 2 lacked exact candidate source snapshots/project UUIDs and
        // schema 3 lacked governed evidence-content digests. Even changing an
        // evidence record's schema changes its content digest, so only an empty
        // aggregate can be upgraded without inventing or mutating lineage.
        let schema_version = if matches!(raw.schema_version, 2 | 3) && is_empty {
            MODEL_QUALIFICATION_SCHEMA_VERSION
        } else {
            raw.schema_version
        };
        Ok(Self {
            schema_version,
            suites: raw.suites,
            platform_runs: raw.platform_runs,
            evidence: raw.evidence,
            candidates: raw.candidates,
            releases: raw.releases,
            promotions: raw.promotions,
            vector_dispositions: raw.vector_dispositions,
        })
    }
}

impl Default for ModelQualificationState {
    fn default() -> Self {
        Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            suites: Vec::new(),
            platform_runs: Vec::new(),
            evidence: Vec::new(),
            candidates: Vec::new(),
            releases: Vec::new(),
            promotions: Vec::new(),
            vector_dispositions: Vec::new(),
        }
    }
}

impl ModelQualificationState {
    pub fn try_new(
        mut suites: Vec<QualificationSuite>,
        mut platform_runs: Vec<QualificationPlatformRun>,
        mut evidence: Vec<QualificationEvidence>,
        mut candidates: Vec<ModelReleaseCandidate>,
        mut releases: Vec<ModelRelease>,
        mut promotions: Vec<ModelPromotionRecord>,
    ) -> QualificationResult<Self> {
        suites.sort_by_key(|value| normalized(&value.id));
        sort_platform_runs(&mut platform_runs);
        evidence.sort_by_key(|value| normalized(&value.id));
        candidates.sort_by_key(|value| normalized(&value.identity.id));
        releases.sort_by_key(|value| normalized(&value.identity.id));
        promotions.sort_by_key(|value| normalized(&value.id));
        let value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            suites,
            platform_runs,
            evidence,
            candidates,
            releases,
            promotions,
            vector_dispositions: Vec::new(),
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
        for run in &self.platform_runs {
            let suite =
                find_ci(&self.suites, &run.suite_id, |value| &value.id).ok_or_else(|| {
                    QualificationValidationError::new(
                        QualificationErrorCode::SuiteBindingMismatch,
                        "qualification_state.platform_runs",
                        format!("suite {:?} does not exist", run.suite_id),
                    )
                })?;
            run.validate_bound(suite, &run.source)?;
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
        for (index, disposition) in self.vector_dispositions.iter().enumerate() {
            let path = format!("qualification_state.vector_dispositions[{index}]");
            disposition.validate(&path)?;
            match &disposition.resolution {
                None => {}
                Some(QualificationVectorDispositionResolution::RerunPassed) => {
                    let (suite, _) = self
                        .resolve_vector_binding(&disposition.vector)
                        .ok_or_else(|| {
                            QualificationValidationError::new(
                                QualificationErrorCode::DispositionInvalid,
                                format!("{path}.resolution"),
                                "rerun resolution requires the exact retained suite and vector",
                            )
                        })?;
                    for platform in QualificationPlatform::REQUIRED {
                        let run = self
                            .platform_runs
                            .iter()
                            .find(|run| {
                                run.platform == platform
                                    && run.source == disposition.vector.source
                                    && normalized(&run.suite_id)
                                        == normalized(&disposition.vector.suite_id)
                                    && run.suite_revision
                                        == disposition.vector.suite_revision
                            })
                            .ok_or_else(|| {
                                QualificationValidationError::new(
                                    QualificationErrorCode::DispositionInvalid,
                                    format!("{path}.resolution"),
                                    "rerun resolution requires passing Desktop and WebAssembly runs",
                                )
                            })?;
                        run.validate_bound(suite, &disposition.vector.source)?;
                        let outcome = find_ci(
                            &run.vector_outcomes,
                            &disposition.vector.vector_id,
                            |outcome| &outcome.vector_id,
                        )
                        .expect("the exact run covers every validated suite vector");
                        if !outcome.outcome.passed {
                            return Err(QualificationValidationError::new(
                                QualificationErrorCode::DispositionInvalid,
                                format!("{path}.resolution"),
                                "rerun resolution requires the vector to pass on every required platform",
                            ));
                        }
                    }
                }
                Some(QualificationVectorDispositionResolution::Replaced { replacement }) => {
                    // A later edit may advance the suite again, so the exact
                    // replacement can become historical. Resolution creation
                    // verifies it while both revisions are in the atomic
                    // mutation; the binding remains an immutable audit key.
                    replacement.validate(&format!("{path}.resolution.replacement"))?;
                }
                Some(QualificationVectorDispositionResolution::Retired) => {
                    if self.resolve_vector_binding(&disposition.vector).is_some() {
                        return Err(QualificationValidationError::new(
                            QualificationErrorCode::DispositionInvalid,
                            format!("{path}.resolution"),
                            "retired disposition still resolves to a retained vector",
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn resolve_vector_binding<'a>(
        &'a self,
        binding: &QualificationVectorBinding,
    ) -> Option<(&'a QualificationSuite, &'a QualificationVector)> {
        let suite = self.suites.iter().find(|suite| {
            normalized(&suite.id) == normalized(&binding.suite_id)
                && suite.revision == binding.suite_revision
        })?;
        let vector = suite.vectors.iter().find(|vector| {
            normalized(&vector.id) == normalized(&binding.vector_id)
                && vector.input_digest == binding.input_digest
                && vector.source == binding.source
        })?;
        Some((suite, vector))
    }

    /// Validate the aggregate and prove that every retained source-bound
    /// record belongs to the exact model key under which the project stores
    /// this state.
    pub fn validate_for_model(&self, model_id: &str) -> QualificationResult<()> {
        self.validate()?;
        require_text("qualification_state.model_id", model_id)?;
        let mut source_model_ids = self
            .suites
            .iter()
            .flat_map(|suite| suite.vectors.iter())
            .map(|vector| vector.source.model_id.as_str())
            .chain(
                self.platform_runs
                    .iter()
                    .map(|record| record.source.model_id.as_str()),
            )
            .chain(
                self.evidence
                    .iter()
                    .map(|record| record.source.model_id.as_str()),
            )
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
            )
            .chain(
                self.vector_dispositions
                    .iter()
                    .map(|record| record.vector.source.model_id.as_str()),
            )
            .chain(self.vector_dispositions.iter().filter_map(|record| {
                match record.resolution.as_ref() {
                    Some(QualificationVectorDispositionResolution::Replaced { replacement }) => {
                        Some(replacement.source.model_id.as_str())
                    }
                    _ => None,
                }
            }));
        if source_model_ids.any(|bound_id| normalized(bound_id) != normalized(model_id)) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "qualification_state.model_id",
                "qualification evidence belongs to a different model identity",
            ));
        }
        Ok(())
    }

    /// Return only suites authored against the exact current project source.
    /// Historical suites retained by promoted release lineage are deliberately
    /// excluded so callers cannot accidentally execute stale qualification.
    pub fn exact_suites_for_source<'a>(
        &'a self,
        source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<Vec<&'a QualificationSuite>> {
        self.validate_for_model(&source.model_id)?;
        source.validate("qualification_state.current_source")?;
        source.require_project_bound("qualification_state.current_source")?;
        Ok(self
            .suites
            .iter()
            .filter(|suite| {
                suite
                    .vectors
                    .first()
                    .is_some_and(|vector| vector.source == *source)
            })
            .collect())
    }

    /// Prove that a section's `Qualified` digest resolves to retained, passing
    /// evidence for this exact project-owned source revision.
    pub fn validate_exact_evidence_digest(
        &self,
        source: &ModelSourceEvidenceBinding,
        evidence_digest: ContentDigest,
    ) -> QualificationResult<()> {
        self.validate_for_model(&source.model_id)?;
        source.require_project_bound("qualification_state.current_source")?;
        let evidence = self
            .evidence
            .iter()
            .find(|evidence| evidence.content_digest().is_ok_and(|digest| digest == evidence_digest))
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::MissingRequiredValue,
                    "qualification_state.evidence_digest",
                    "qualified section evidence digest does not resolve to a retained evidence record",
                )
            })?;
        let suite =
            find_ci(&self.suites, &evidence.suite_id, |value| &value.id).ok_or_else(|| {
                missing(
                    "qualification_state.evidence_digest",
                    "qualified section evidence suite is not retained",
                )
            })?;
        evidence.validate_bound(suite, source)?;
        if !evidence.passed {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::PromotionBlocked,
                "qualification_state.evidence_digest",
                "qualified section evidence must pass every declared vector on every required platform",
            ));
        }
        Ok(())
    }

    /// Prove that an evidence digest covers an explicitly selected named
    /// model section, rather than merely exercising the source-wide base card.
    pub fn validate_exact_section_evidence_digest(
        &self,
        source: &ModelSourceEvidenceBinding,
        section_name: &str,
        evidence_digest: ContentDigest,
    ) -> QualificationResult<()> {
        require_text("qualification_state.section_name", section_name)?;
        self.validate_exact_evidence_digest(source, evidence_digest)?;
        let evidence = self
            .evidence
            .iter()
            .find(|evidence| {
                evidence
                    .content_digest()
                    .is_ok_and(|digest| digest == evidence_digest)
            })
            .expect("the source-wide evidence validator resolved this digest");
        let suite = find_ci(&self.suites, &evidence.suite_id, |value| &value.id)
            .expect("the source-wide evidence validator resolved this suite");
        if !suite.vectors.iter().any(|vector| {
            vector
                .model_section
                .as_deref()
                .is_some_and(|section| section.eq_ignore_ascii_case(section_name))
        }) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "qualification_state.section_name",
                format!(
                    "qualified section {section_name:?} has no exact section-selected vector in the retained evidence suite"
                ),
            ));
        }
        Ok(())
    }

    /// Reconcile an aggregate after a definition revision. Current-source work
    /// is preserved, while stale unpromoted work is removed. Immutable release
    /// lineage and the evidence/suites needed to validate it remain retained.
    pub fn reconcile_after_source_revision(
        &self,
        current_source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<Self> {
        self.validate_for_model(&current_source.model_id)?;
        current_source.require_project_bound("qualification_state.current_source")?;

        let promoted_candidate_ids = self
            .promotions
            .iter()
            .map(|promotion| normalized(&promotion.candidate_identity.id))
            .collect::<BTreeSet<_>>();
        let candidates = self
            .candidates
            .iter()
            .filter(|candidate| {
                candidate.source == *current_source
                    || promoted_candidate_ids.contains(&normalized(&candidate.identity.id))
            })
            .cloned()
            .collect::<Vec<_>>();

        let retained_evidence_ids = self
            .promotions
            .iter()
            .map(|promotion| normalized(&promotion.evidence_id))
            .chain(
                candidates
                    .iter()
                    .map(|candidate| normalized(&candidate.evidence_id)),
            )
            .collect::<BTreeSet<_>>();
        let evidence = self
            .evidence
            .iter()
            .filter(|evidence| {
                evidence.source == *current_source
                    || retained_evidence_ids.contains(&normalized(&evidence.id))
            })
            .cloned()
            .collect::<Vec<_>>();

        let retained_suite_keys = self
            .promotions
            .iter()
            .map(|promotion| (normalized(&promotion.suite_id), promotion.suite_revision))
            .chain(
                candidates
                    .iter()
                    .map(|candidate| (normalized(&candidate.suite_id), candidate.suite_revision)),
            )
            .chain(
                evidence
                    .iter()
                    .map(|evidence| (normalized(&evidence.suite_id), evidence.suite_revision)),
            )
            .collect::<BTreeSet<_>>();
        let suites = self
            .suites
            .iter()
            .filter(|suite| {
                suite
                    .vectors
                    .first()
                    .is_some_and(|vector| vector.source == *current_source)
                    || retained_suite_keys.contains(&(normalized(&suite.id), suite.revision))
            })
            .cloned()
            .collect::<Vec<_>>();

        let platform_runs = self
            .platform_runs
            .iter()
            .filter(|run| {
                run.source == *current_source
                    || evidence.iter().any(|evidence| {
                        evidence.source == run.source
                            && normalized(&evidence.suite_id) == normalized(&run.suite_id)
                            && evidence.suite_revision == run.suite_revision
                    })
            })
            .cloned()
            .collect();

        let mut reconciled = Self::try_new(
            suites,
            platform_runs,
            evidence,
            candidates,
            self.releases.clone(),
            self.promotions.clone(),
        )?;
        reconciled.vector_dispositions = self.vector_dispositions.clone();
        reconciled
            .vector_dispositions
            .sort_by_key(|disposition| normalized(&disposition.id));
        reconciled.validate()?;
        Ok(reconciled)
    }

    /// Inspect one exact retained suite without latest-revision fallback.
    pub fn qualification_suite(&self, suite_id: &str) -> QualificationResult<&QualificationSuite> {
        self.validate()?;
        require_text("qualification_state.suite_id", suite_id)?;
        find_ci(&self.suites, suite_id, |suite| &suite.id).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "qualification_state.suite_id",
                format!("qualification suite {suite_id:?} does not exist"),
            )
        })
    }

    /// Inspect one exact retained vector within a named suite.
    pub fn qualification_vector(
        &self,
        suite_id: &str,
        vector_id: &str,
    ) -> QualificationResult<&QualificationVector> {
        require_text("qualification_state.vector_id", vector_id)?;
        let suite = self.qualification_suite(suite_id)?;
        find_ci(&suite.vectors, vector_id, |vector| &vector.id).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "qualification_state.vector_id",
                format!(
                    "qualification vector {vector_id:?} does not exist in suite {:?}",
                    suite.id
                ),
            )
        })
    }

    /// Return the immutable binding used by dispositions and edit receipts.
    pub fn qualification_vector_binding(
        &self,
        suite_id: &str,
        vector_id: &str,
    ) -> QualificationResult<QualificationVectorBinding> {
        let suite = self.qualification_suite(suite_id)?;
        let vector = find_ci(&suite.vectors, vector_id, |vector| &vector.id).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "qualification_state.vector_id",
                format!(
                    "qualification vector {vector_id:?} does not exist in suite {:?}",
                    suite.id
                ),
            )
        })?;
        Ok(QualificationVectorBinding::from_vector(suite, vector))
    }

    fn ensure_suite_is_editable(&self, suite: &QualificationSuite) -> QualificationResult<()> {
        let evidence_bound = self.evidence.iter().any(|evidence| {
            normalized(&evidence.suite_id) == normalized(&suite.id)
                && evidence.suite_revision == suite.revision
        });
        let candidate_bound = self.candidates.iter().any(|candidate| {
            normalized(&candidate.suite_id) == normalized(&suite.id)
                && candidate.suite_revision == suite.revision
        });
        let promoted = self.promotions.iter().any(|promotion| {
            normalized(&promotion.suite_id) == normalized(&suite.id)
                && promotion.suite_revision == suite.revision
        });
        if evidence_bound || candidate_bound || promoted {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ImmutableRecord,
                "qualification_state.suite",
                "a suite bound by evidence or a release candidate is immutable; create a new suite identity",
            ));
        }
        Ok(())
    }

    /// Replace an editable suite as one revision-advancing transaction. Any
    /// uncommitted platform runs for the prior revision are invalidated.
    pub fn replace_suite_atomically(
        &mut self,
        replacement: QualificationSuite,
    ) -> QualificationResult<()> {
        replacement.validate()?;
        let existing_index = self
            .suites
            .iter()
            .position(|suite| normalized(&suite.id) == normalized(&replacement.id))
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::MissingRequiredValue,
                    "qualification_state.suite_id",
                    format!("qualification suite {:?} does not exist", replacement.id),
                )
            })?;
        let existing = self.suites[existing_index].clone();
        self.ensure_suite_is_editable(&existing)?;
        let next_revision = existing.revision.next().map_err(|error| {
            QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.revision",
                error.to_string(),
            )
        })?;
        if replacement.revision != next_revision {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.revision",
                format!(
                    "replacement suite revision must be the next revision ({})",
                    next_revision.get()
                ),
            ));
        }
        if normalized(&replacement.name) != normalized(&existing.name) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.name",
                "editing a suite must preserve its stable suite name",
            ));
        }

        let mut candidate = self.clone();
        candidate.suites[existing_index] = replacement.clone();
        candidate.platform_runs.retain(|run| {
            normalized(&run.suite_id) != normalized(&existing.id)
                || run.suite_revision != existing.revision
        });
        for disposition in &mut candidate.vector_dispositions {
            if !disposition.is_open()
                || normalized(&disposition.vector.suite_id) != normalized(&existing.id)
                || disposition.vector.suite_revision != existing.revision
            {
                continue;
            }
            disposition.resolution = replacement
                .vectors
                .iter()
                .find(|vector| normalized(&vector.id) == normalized(&disposition.vector.vector_id))
                .map(
                    |vector| QualificationVectorDispositionResolution::Replaced {
                        replacement: QualificationVectorBinding::from_vector(&replacement, vector),
                    },
                )
                .or(Some(QualificationVectorDispositionResolution::Retired));
        }
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Replace or edit one vector, preserving the suite identity and advancing
    /// its revision exactly once.
    pub fn replace_vector_atomically(
        &mut self,
        suite_id: &str,
        vector_id: &str,
        replacement: QualificationVector,
    ) -> QualificationResult<()> {
        let suite = self.qualification_suite(suite_id)?.clone();
        let vector_index = suite
            .vectors
            .iter()
            .position(|vector| normalized(&vector.id) == normalized(vector_id))
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::MissingRequiredValue,
                    "qualification_state.vector_id",
                    format!("qualification vector {vector_id:?} does not exist"),
                )
            })?;
        let mut vectors = suite.vectors.clone();
        vectors[vector_index] = replacement;
        let next_revision = suite.revision.next().map_err(|error| {
            QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.revision",
                error.to_string(),
            )
        })?;
        let replacement_suite =
            QualificationSuite::try_new(suite.id, suite.name, next_revision, vectors)?;
        self.replace_suite_atomically(replacement_suite)
    }

    /// Delete one editable vector. Deleting the final vector is rejected;
    /// callers must explicitly delete the suite instead.
    pub fn delete_vector_atomically(
        &mut self,
        suite_id: &str,
        vector_id: &str,
    ) -> QualificationResult<()> {
        let suite = self.qualification_suite(suite_id)?.clone();
        let vector_index = suite
            .vectors
            .iter()
            .position(|vector| normalized(&vector.id) == normalized(vector_id))
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::MissingRequiredValue,
                    "qualification_state.vector_id",
                    format!("qualification vector {vector_id:?} does not exist"),
                )
            })?;
        if suite.vectors.len() == 1 {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.vectors",
                "the final vector cannot be deleted independently; delete the suite explicitly",
            ));
        }
        let mut vectors = suite.vectors.clone();
        vectors.remove(vector_index);
        let next_revision = suite.revision.next().map_err(|error| {
            QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                "qualification_state.suite.revision",
                error.to_string(),
            )
        })?;
        let replacement_suite =
            QualificationSuite::try_new(suite.id, suite.name, next_revision, vectors)?;
        self.replace_suite_atomically(replacement_suite)
    }

    /// Delete an editable suite and its uncommitted runtime runs. Evidence and
    /// release lineage are never cascaded or rewritten.
    pub fn delete_suite_atomically(&mut self, suite_id: &str) -> QualificationResult<()> {
        let suite = self.qualification_suite(suite_id)?.clone();
        self.ensure_suite_is_editable(&suite)?;
        let mut candidate = self.clone();
        candidate
            .suites
            .retain(|value| normalized(&value.id) != normalized(&suite.id));
        candidate
            .platform_runs
            .retain(|run| normalized(&run.suite_id) != normalized(&suite.id));
        for disposition in &mut candidate.vector_dispositions {
            if disposition.is_open()
                && normalized(&disposition.vector.suite_id) == normalized(&suite.id)
                && disposition.vector.suite_revision == suite.revision
            {
                disposition.resolution = Some(QualificationVectorDispositionResolution::Retired);
            }
        }
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Record an explicit, auditable failed/stale disposition. Failed vectors
    /// require a retained failed platform outcome; stale vectors require an
    /// exact current source that differs from the vector source.
    #[allow(clippy::too_many_arguments)]
    pub fn record_vector_disposition_atomically(
        &mut self,
        disposition_id: impl Into<String>,
        suite_id: &str,
        vector_id: &str,
        current_source: &ModelSourceEvidenceBinding,
        cause: QualificationVectorDispositionCause,
        required_action: QualificationVectorRequiredAction,
        reason: impl Into<String>,
    ) -> QualificationResult<QualificationVectorDisposition> {
        let binding = self.qualification_vector_binding(suite_id, vector_id)?;
        current_source.validate("qualification_state.current_source")?;
        current_source.require_project_bound("qualification_state.current_source")?;
        if normalized(&current_source.model_id) != normalized(&binding.source.model_id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "qualification_state.current_source",
                "current source belongs to a different model identity",
            ));
        }
        match cause {
            QualificationVectorDispositionCause::Failed => {
                if binding.source != *current_source {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::DispositionInvalid,
                        "qualification_state.vector_disposition.cause",
                        "a vector bound to a different source revision must be dispositioned as stale",
                    ));
                }
                let has_failed_outcome = self.platform_runs.iter().any(|run| {
                    normalized(&run.suite_id) == normalized(&binding.suite_id)
                        && run.suite_revision == binding.suite_revision
                        && run.source == binding.source
                        && find_ci(&run.vector_outcomes, &binding.vector_id, |outcome| {
                            &outcome.vector_id
                        })
                        .is_some_and(|outcome| !outcome.outcome.passed)
                });
                if !has_failed_outcome {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::DispositionInvalid,
                        "qualification_state.vector_disposition.cause",
                        "failed disposition requires a retained failing platform outcome",
                    ));
                }
            }
            QualificationVectorDispositionCause::Stale => {
                if binding.source == *current_source {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::DispositionInvalid,
                        "qualification_state.vector_disposition.cause",
                        "stale disposition requires a vector bound to a different source revision",
                    ));
                }
            }
        }
        let disposition = QualificationVectorDisposition {
            id: disposition_id.into(),
            vector: binding,
            cause,
            required_action,
            reason: reason.into(),
            resolution: None,
        };
        disposition.validate("qualification_state.vector_disposition")?;
        let mut candidate = self.clone();
        candidate.vector_dispositions.push(disposition.clone());
        candidate
            .vector_dispositions
            .sort_by_key(|value| normalized(&value.id));
        candidate.validate()?;
        *self = candidate;
        Ok(disposition)
    }

    /// Close a rerun disposition only after exact passing Desktop and
    /// WebAssembly runs are retained for the same suite/vector/source.
    pub fn resolve_vector_disposition_by_rerun_atomically(
        &mut self,
        disposition_id: &str,
    ) -> QualificationResult<()> {
        let mut candidate = self.clone();
        let disposition_index = candidate
            .vector_dispositions
            .iter()
            .position(|value| value.id.eq_ignore_ascii_case(disposition_id))
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::MissingRequiredValue,
                    "qualification_state.vector_disposition.id",
                    format!("vector disposition {disposition_id:?} does not exist"),
                )
            })?;
        let disposition = &mut candidate.vector_dispositions[disposition_index];
        if !disposition.is_open()
            || disposition.required_action != QualificationVectorRequiredAction::Rerun
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DispositionInvalid,
                "qualification_state.vector_disposition.resolution",
                "only an open rerun disposition can be resolved by rerun evidence",
            ));
        }
        disposition.resolution = Some(QualificationVectorDispositionResolution::RerunPassed);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Execute and atomically retain the current platform's exact suite run.
    /// Failed vectors are retained as real failing outcomes; cancellation or
    /// invalid contracts leave state unchanged.
    pub fn rerun_suite_current_platform_atomically(
        &mut self,
        suite_id: &str,
        source: &ModelSourceEvidenceBinding,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationPlatformRun, QualificationExecutionError> {
        let suite = self.qualification_suite(suite_id)?.clone();
        validate_execution_contract(&suite, source)?;
        let run = QualificationExecutionService::execute_current_platform(&suite, source, abort)?;
        self.upsert_platform_run_atomically(run.clone())?;
        Ok(run)
    }

    /// Retain the current runtime's complete run, replacing only the exact
    /// same suite/source/platform key. Validation occurs on a cloned aggregate
    /// so stale or tampered records cannot disturb previously retained runs.
    pub fn upsert_platform_run_atomically(
        &mut self,
        run: QualificationPlatformRun,
    ) -> QualificationResult<()> {
        let mut candidate = self.clone();
        let suite =
            find_ci(&candidate.suites, &run.suite_id, |value| &value.id).ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::SuiteBindingMismatch,
                    "qualification_state.platform_runs",
                    format!("suite {:?} does not exist", run.suite_id),
                )
            })?;
        run.validate_bound(suite, &run.source)?;
        if let Some(existing) = candidate
            .platform_runs
            .iter_mut()
            .find(|value| same_platform_run_key(value, &run))
        {
            *existing = run;
        } else {
            candidate.platform_runs.push(run);
        }
        sort_platform_runs(&mut candidate.platform_runs);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Retrieve the exact Desktop and WebAssembly run pair for one suite and
    /// source revision. No latest/stale fallback is permitted.
    pub fn exact_platform_run_pair<'a>(
        &'a self,
        suite_id: &str,
        source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<[&'a QualificationPlatformRun; 2]> {
        self.validate()?;
        let suite = find_ci(&self.suites, suite_id, |value| &value.id).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::SuiteBindingMismatch,
                "qualification_state.platform_runs",
                format!("suite {suite_id:?} does not exist"),
            )
        })?;
        validate_execution_contract(suite, source)?;
        let exact = |platform| {
            self.platform_runs.iter().find(|run| {
                run.platform == platform
                    && normalized(&run.suite_id) == normalized(&suite.id)
                    && run.suite_revision == suite.revision
                    && &run.source == source
            })
        };
        let desktop = exact(QualificationPlatform::Desktop).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "qualification_state.platform_runs.desktop",
                "the exact Desktop qualification run is not retained",
            )
        })?;
        let webassembly = exact(QualificationPlatform::WebAssembly).ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "qualification_state.platform_runs.web-assembly",
                "the exact WebAssembly qualification run is not retained",
            )
        })?;
        Ok([desktop, webassembly])
    }

    /// Assemble parity evidence from the exact retained pair and commit it as
    /// one aggregate transaction. Missing or stale runtime coverage leaves the
    /// state unchanged.
    pub fn assemble_and_upsert_evidence_atomically(
        &mut self,
        evidence_id: impl Into<String>,
        suite_id: &str,
        source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<QualificationEvidence> {
        let (suite, runs) = {
            let suite = find_ci(&self.suites, suite_id, |value| &value.id)
                .ok_or_else(|| {
                    QualificationValidationError::new(
                        QualificationErrorCode::SuiteBindingMismatch,
                        "qualification_state.platform_runs",
                        format!("suite {suite_id:?} does not exist"),
                    )
                })?
                .clone();
            let [desktop, webassembly] = self.exact_platform_run_pair(suite_id, source)?;
            (suite, vec![desktop.clone(), webassembly.clone()])
        };
        let evidence =
            QualificationExecutionService::assemble_evidence(evidence_id, &suite, source, runs)?;
        self.upsert_evidence_atomically(evidence.clone())?;
        Ok(evidence)
    }

    /// Append one immutable evidence record without ever exposing a
    /// partially-invalid aggregate. Reusing an ID is idempotent only when the
    /// complete evidence payload is byte-for-byte equivalent; replacement is
    /// forbidden even before a candidate references the record.
    pub fn upsert_evidence_atomically(
        &mut self,
        evidence: QualificationEvidence,
    ) -> QualificationResult<()> {
        if let Some(existing) = self
            .evidence
            .iter()
            .find(|value| normalized(&value.id) == normalized(&evidence.id))
        {
            if existing == &evidence {
                return self.validate();
            }
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ImmutableRecord,
                "qualification_state.evidence",
                "qualification evidence IDs are append-only; retain the original record and use a new evidence identity",
            ));
        }
        let mut candidate = self.clone();
        candidate.evidence.push(evidence);
        candidate
            .evidence
            .sort_by_key(|value| normalized(&value.id));
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    /// Promote a retained candidate and append its release and audit record as
    /// one aggregate transaction. Duplicate identities or any newly-invalid
    /// binding leave the original state byte-for-byte unchanged.
    pub fn promote_candidate_atomically(
        &mut self,
        promotion_id: impl Into<String>,
        release_identity: ModelReleaseIdentity,
        candidate_id: &str,
    ) -> QualificationResult<()> {
        require_text("qualification_state.candidate_id", candidate_id)?;
        if self.promotions.iter().any(|promotion| {
            normalized(&promotion.candidate_identity.id) == normalized(candidate_id)
        }) || self
            .releases
            .iter()
            .any(|release| normalized(&release.candidate_id) == normalized(candidate_id))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                "qualification_state.candidate_id",
                "a release candidate may be promoted at most once",
            ));
        }
        let candidate = find_ci(&self.candidates, candidate_id, |value| &value.identity.id)
            .ok_or_else(|| {
                missing(
                    "qualification_state.candidate_id",
                    format!("candidate {candidate_id:?} does not exist"),
                )
            })?;
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
        if self.vector_dispositions.iter().any(|disposition| {
            disposition.is_open()
                && normalized(&disposition.vector.suite_id) == normalized(&candidate.suite_id)
                && disposition.vector.suite_revision == candidate.suite_revision
                && disposition.vector.source == candidate.source
        }) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::PromotionBlocked,
                "qualification_state.vector_dispositions",
                "open failed or stale vector dispositions must be resolved by a passing rerun, exact replacement, or retirement before promotion",
            ));
        }
        let (promotion, release) = ModelPromotionRecord::promote(
            promotion_id,
            release_identity,
            candidate,
            suite,
            evidence,
        )?;

        let mut next = self.clone();
        next.promotions.push(promotion);
        next.releases.push(release);
        next.promotions.sort_by_key(|value| normalized(&value.id));
        next.releases
            .sort_by_key(|value| normalized(&value.identity.id));
        next.validate()?;
        *self = next;
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
    let mut platform_run_keys = BTreeSet::new();
    for (index, run) in state.platform_runs.iter().enumerate() {
        let key = platform_run_key(run);
        if !platform_run_keys.insert(key) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                format!("qualification_state.platform_runs[{index}]"),
                "suite/source/platform run key is duplicated",
            ));
        }
    }
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
    )?;
    ensure_unique(
        "qualification_state.promotions",
        state
            .promotions
            .iter()
            .map(|value| value.candidate_identity.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "promoted candidate ID",
    )?;
    ensure_unique(
        "qualification_state.releases",
        state
            .releases
            .iter()
            .map(|value| value.candidate_id.as_str()),
        QualificationErrorCode::DuplicateId,
        "released candidate ID",
    )?;
    ensure_unique(
        "qualification_state.vector_dispositions",
        state
            .vector_dispositions
            .iter()
            .map(|value| value.id.as_str()),
        QualificationErrorCode::DuplicateId,
        "vector disposition ID",
    )?;
    let mut open_bindings = BTreeSet::new();
    for (index, disposition) in state.vector_dispositions.iter().enumerate() {
        if !disposition.is_open() {
            continue;
        }
        let binding = (
            normalized(&disposition.vector.suite_id),
            disposition.vector.suite_revision,
            normalized(&disposition.vector.vector_id),
            disposition.vector.input_digest.to_string(),
        );
        if !open_bindings.insert(binding) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                format!("qualification_state.vector_dispositions[{index}]"),
                "a vector may have at most one open disposition",
            ));
        }
    }
    Ok(())
}

fn platform_run_key(
    run: &QualificationPlatformRun,
) -> (
    String,
    u64,
    String,
    Option<String>,
    ContentDigest,
    u64,
    QualificationPlatform,
) {
    (
        normalized(&run.suite_id),
        run.suite_revision.get(),
        normalized(&run.source.model_id),
        run.source.source_id.map(|source_id| source_id.to_string()),
        run.source.source_digest,
        run.source.source_revision.get(),
        run.platform,
    )
}

fn same_platform_run_key(
    left: &QualificationPlatformRun,
    right: &QualificationPlatformRun,
) -> bool {
    platform_run_key(left) == platform_run_key(right)
}

fn sort_platform_runs(runs: &mut [QualificationPlatformRun]) {
    runs.sort_by_key(platform_run_key);
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

fn digest_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn calculate_reference_errors(expected: f64, observed: f64) -> Option<(f64, f64)> {
    let absolute = (observed - expected).abs();
    if !absolute.is_finite() {
        return None;
    }
    let relative = if expected == 0.0 {
        if absolute == 0.0 { 0.0 } else { f64::MAX }
    } else {
        absolute / expected.abs()
    };
    if !relative.is_finite() {
        return None;
    }
    Some((
        canonical_serialized_f64(absolute)?,
        canonical_serialized_f64(relative)?,
    ))
}

fn canonical_serialized_f64(value: f64) -> Option<f64> {
    let encoded = serde_json::to_string(&value).ok()?;
    serde_json::from_str(&encoded).ok()
}

fn same_within_one_ulp(left: f64, right: f64) -> bool {
    left == right || left.to_bits().abs_diff(right.to_bits()) <= 1
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
mod tests;
