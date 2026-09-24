//! The evidence a qualification run produced, bound to what produced it.
//!
//! Every outcome carries the exact model source, suite revision, and platform
//! it was measured against, so evidence can never be re-attributed to a
//! different input after the fact. A failure is recorded with its stage rather
//! than collapsed into "did not pass", because the distinction between a
//! vector that ran and missed tolerance and one that never ran is what makes
//! the evidence actionable.

use super::*;

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

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.model_id"), &self.model_id)
    }

    pub(super) fn require_project_bound(&self, path: &str) -> QualificationResult<ModelSourceId> {
        self.source_id.ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                format!("{path}.source_id"),
                "new qualification evidence requires an exact project-owned source identity",
            )
        })
    }
}

/// Observed error and the exact tolerance used to make its disposition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceErrorEvidence {
    pub quantity: String,
    pub expected_value: FiniteValue,
    pub observed_value: FiniteValue,
    pub absolute_error: NonNegativeFinite,
    pub relative_error: NonNegativeFinite,
    pub absolute_tolerance: NonNegativeFinite,
    pub relative_tolerance: NonNegativeFinite,
    pub passed: bool,
}

impl ReferenceErrorEvidence {
    pub(super) fn try_new(
        quantity: impl Into<String>,
        expected_value: f64,
        observed_value: f64,
        absolute_tolerance: f64,
        relative_tolerance: f64,
    ) -> QualificationResult<Self> {
        let expected_value = FiniteValue::new(expected_value)
            .map_err(|error| at_path(error, "reference_error.expected_value"))?;
        let observed_value = FiniteValue::new(observed_value)
            .map_err(|error| at_path(error, "reference_error.observed_value"))?;
        let (absolute_error_value, relative_error_value) =
            calculate_reference_errors(expected_value.get(), observed_value.get()).ok_or_else(
                || {
                    QualificationValidationError::new(
                        QualificationErrorCode::InvalidNumber,
                        "reference_error",
                        "reference error overflowed the finite evidence representation",
                    )
                },
            )?;
        let absolute_error = NonNegativeFinite::new(absolute_error_value)
            .map_err(|error| at_path(error, "reference_error.absolute_error"))?;
        let relative_error = NonNegativeFinite::new(relative_error_value)
            .map_err(|error| at_path(error, "reference_error.relative_error"))?;
        let absolute_tolerance = NonNegativeFinite::new(absolute_tolerance)
            .map_err(|error| at_path(error, "reference_error.absolute_tolerance"))?;
        let relative_tolerance = NonNegativeFinite::new(relative_tolerance)
            .map_err(|error| at_path(error, "reference_error.relative_tolerance"))?;
        let passed = absolute_error <= absolute_tolerance || relative_error <= relative_tolerance;
        let value = Self {
            quantity: quantity.into(),
            expected_value,
            observed_value,
            absolute_error,
            relative_error,
            absolute_tolerance,
            relative_tolerance,
            passed,
        };
        value.validate("reference_error")?;
        Ok(value)
    }

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.quantity"), &self.quantity)?;
        let Some((absolute_error, relative_error)) =
            calculate_reference_errors(self.expected_value.get(), self.observed_value.get())
        else {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidNumber,
                path,
                "reference error overflowed the finite evidence representation",
            ));
        };
        if !same_within_one_ulp(self.absolute_error.get(), absolute_error)
            || !same_within_one_ulp(self.relative_error.get(), relative_error)
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                path,
                "recorded errors do not match the retained expected and observed values",
            ));
        }
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
        expected: &QualificationReference,
        path: &str,
    ) -> QualificationResult<()> {
        self.validate(path)?;
        if normalized(&self.quantity) != normalized(&expected.quantity)
            || self.expected_value != expected.expected
            || self.absolute_tolerance != expected.absolute_tolerance
            || self.relative_tolerance != expected.relative_tolerance
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

/// Stage at which a vector failed before reference evidence could be emitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QualificationFailureStage {
    Input,
    Parse,
    Simulation,
    Measurement,
}

/// Stable failure retained as a failing platform outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationExecutionFailure {
    pub stage: QualificationFailureStage,
    pub code: String,
    pub message: String,
}

impl QualificationExecutionFailure {
    pub(super) fn try_new(
        stage: QualificationFailureStage,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> QualificationResult<Self> {
        let value = Self {
            stage,
            code: code.into(),
            message: message.into(),
        };
        value.validate("execution_failure")?;
        Ok(value)
    }

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.code"), &self.code)?;
        require_text(&format!("{path}.message"), &self.message)
    }
}

/// Result for one vector on one runtime platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformQualificationOutcome {
    pub platform: QualificationPlatform,
    pub references: Vec<ReferenceErrorEvidence>,
    pub failure: Option<QualificationExecutionFailure>,
    pub passed: bool,
}

impl PlatformQualificationOutcome {
    pub(super) fn try_new(
        platform: QualificationPlatform,
        mut references: Vec<ReferenceErrorEvidence>,
    ) -> QualificationResult<Self> {
        references
            .sort_by(|left, right| normalized(&left.quantity).cmp(&normalized(&right.quantity)));
        let passed = !references.is_empty() && references.iter().all(|value| value.passed);
        let value = Self {
            platform,
            references,
            failure: None,
            passed,
        };
        value.validate("platform_outcome")?;
        Ok(value)
    }

    pub(super) fn try_failed(
        platform: QualificationPlatform,
        failure: QualificationExecutionFailure,
    ) -> QualificationResult<Self> {
        let value = Self {
            platform,
            references: Vec::new(),
            failure: Some(failure),
            passed: false,
        };
        value.validate("platform_outcome")?;
        Ok(value)
    }

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
        if let Some(failure) = &self.failure {
            failure.validate(&format!("{path}.failure"))?;
            if !self.references.is_empty() || self.passed {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::InconsistentResult,
                    path,
                    "a failed platform outcome cannot contain references or pass",
                ));
            }
            return Ok(());
        }
        if self.references.is_empty() {
            return Err(missing(
                format!("{path}.references"),
                "a completed platform outcome requires reference evidence",
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
        if self.failure.is_some() {
            return Ok(());
        }
        if self.references.len() != vector.references.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                format!("{path}.references"),
                "platform evidence must cover every declared reference quantity exactly once",
            ));
        }
        for expected in &vector.references {
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
#[serde(deny_unknown_fields)]
pub struct QualificationVectorOutcome {
    pub vector_id: String,
    pub input_digest: ContentDigest,
    pub platforms: Vec<PlatformQualificationOutcome>,
    pub passed: bool,
}

impl QualificationVectorOutcome {
    pub(super) fn try_new(
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

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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
#[serde(deny_unknown_fields)]
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
    pub(super) fn try_new(
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
        value.source.require_project_bound("evidence.source")?;
        Ok(value)
    }

    pub(super) fn validate_internal(&self) -> QualificationResult<()> {
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
        for (index, vector) in suite.vectors.iter().enumerate() {
            vector.validate_source_binding(
                expected_source,
                &format!("suite.vectors[{index}].source"),
            )?;
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

    /// Cryptographic identity of the complete retained evidence record. This
    /// digest is the only value accepted by a `Qualified` model section.
    pub fn content_digest(&self) -> QualificationResult<ContentDigest> {
        self.validate_internal()?;
        let bytes = serde_json::to_vec(self).map_err(|error| {
            QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                "evidence",
                format!("qualification evidence cannot be canonically encoded: {error}"),
            )
        })?;
        Ok(digest_bytes(&bytes))
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

/// Current-runtime result for one vector. Platform runs are intentionally
/// separate from cross-platform evidence: a desktop process cannot claim a
/// WebAssembly result (and vice versa).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationPlatformVectorOutcome {
    pub vector_id: String,
    pub input_digest: ContentDigest,
    pub outcome: PlatformQualificationOutcome,
}

impl QualificationPlatformVectorOutcome {
    fn validate_against(
        &self,
        vector: &QualificationVector,
        platform: QualificationPlatform,
        path: &str,
    ) -> QualificationResult<()> {
        require_text(&format!("{path}.vector_id"), &self.vector_id)?;
        if normalized(&self.vector_id) != normalized(&vector.id)
            || self.input_digest != vector.input_digest
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                path,
                "platform result is not bound to the declared vector and executable input",
            ));
        }
        if self.outcome.platform != platform {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                format!("{path}.outcome.platform"),
                "vector outcome platform does not match its platform run",
            ));
        }
        self.outcome
            .validate_against(vector, &format!("{path}.outcome"))
    }
}

/// One atomic execution of an entire suite on a single real runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationPlatformRun {
    pub schema_version: u32,
    pub platform: QualificationPlatform,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub vector_outcomes: Vec<QualificationPlatformVectorOutcome>,
    pub passed: bool,
}

impl QualificationPlatformRun {
    pub(super) fn try_new(
        platform: QualificationPlatform,
        source: ModelSourceEvidenceBinding,
        suite: &QualificationSuite,
        mut vector_outcomes: Vec<QualificationPlatformVectorOutcome>,
    ) -> QualificationResult<Self> {
        vector_outcomes
            .sort_by(|left, right| normalized(&left.vector_id).cmp(&normalized(&right.vector_id)));
        let passed =
            !vector_outcomes.is_empty() && vector_outcomes.iter().all(|value| value.outcome.passed);
        let value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            platform,
            source,
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            vector_outcomes,
            passed,
        };
        value.validate_bound(suite, &value.source)?;
        Ok(value)
    }

    pub fn validate_bound(
        &self,
        suite: &QualificationSuite,
        expected_source: &ModelSourceEvidenceBinding,
    ) -> QualificationResult<()> {
        require_schema("platform_run.schema_version", self.schema_version)?;
        suite.validate()?;
        expected_source.validate("platform_run.expected_source")?;
        if &self.source != expected_source {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "platform_run.source",
                "platform run does not match the exact expected model source revision",
            ));
        }
        if normalized(&self.suite_id) != normalized(&suite.id)
            || self.suite_revision != suite.revision
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SuiteBindingMismatch,
                "platform_run.suite",
                "platform run does not match the qualification suite revision",
            ));
        }
        if self.vector_outcomes.len() != suite.vectors.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "platform_run.vector_outcomes",
                "platform run must cover every suite vector exactly once",
            ));
        }
        ensure_unique(
            "platform_run.vector_outcomes",
            self.vector_outcomes
                .iter()
                .map(|value| value.vector_id.as_str()),
            QualificationErrorCode::DuplicateId,
            "vector outcome ID",
        )?;
        for (index, vector) in suite.vectors.iter().enumerate() {
            vector.validate_source_binding(
                expected_source,
                &format!("suite.vectors[{index}].source"),
            )?;
            let outcome = find_ci(&self.vector_outcomes, &vector.id, |value| &value.vector_id)
                .ok_or_else(|| {
                    QualificationValidationError::new(
                        QualificationErrorCode::EvidenceCoverageMismatch,
                        "platform_run.vector_outcomes",
                        format!("missing outcome for vector {:?}", vector.id),
                    )
                })?;
            outcome.validate_against(vector, self.platform, "platform_run.vector_outcomes")?;
        }
        let expected_passed = self
            .vector_outcomes
            .iter()
            .all(|value| value.outcome.passed);
        if self.passed != expected_passed {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InconsistentResult,
                "platform_run.passed",
                "platform-run disposition does not match its vector outcomes",
            ));
        }
        Ok(())
    }
}
