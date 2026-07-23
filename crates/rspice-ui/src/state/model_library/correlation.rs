//! Versioned, source-bound measurement-correlation contracts.
//!
//! This module owns imported reference/simulation datasets, explicit
//! alignment and metric definitions, governed outlier decisions, deterministic
//! evaluation, and immutable reviewer evidence. It contains no egui code and
//! makes no instrument, network, or release-authority claims.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use csv::{ReaderBuilder, StringRecord, Trim};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ObjectRevision};

use super::qualification::{FiniteValue, ModelSourceEvidenceBinding, NonNegativeFinite};

pub const MODEL_CORRELATION_SCHEMA_VERSION: u32 = 2;
pub const MAX_CORRELATION_SOURCE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_CORRELATION_ROWS: usize = 200_000;
pub const MAX_CORRELATION_COLUMNS: usize = 128;
pub const MAX_CORRELATION_TEXT_BYTES: usize = 4_096;
pub const MAX_CORRELATION_DATASET_REVISIONS: usize = 256;
pub const MAX_CORRELATION_METRICS: usize = 512;
pub const MAX_CORRELATION_DISPOSITIONS: usize = 20_000;
pub const MAX_CORRELATION_SUITE_REVISIONS: usize = 2_048;
pub const MAX_CORRELATION_EVIDENCE_RECORDS: usize = 2_048;
pub const MAX_CORRELATION_SUITE_SOURCE_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_CORRELATION_SUITE_OBSERVATIONS: usize = 1_000_000;
pub const MAX_CORRELATION_EVIDENCE_RESIDUALS: usize = 1_000_000;
pub const MAX_CORRELATION_STATE_SOURCE_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_CORRELATION_STATE_OBSERVATIONS: usize = 2_000_000;
pub const MAX_CORRELATION_STATE_RESIDUALS: usize = 2_000_000;

pub type CorrelationResult<T> = Result<T, CorrelationError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationErrorCode {
    UnsupportedSchema,
    MissingValue,
    DuplicateIdentity,
    InvalidNumber,
    InvalidCsv,
    ResourceLimit,
    SourceDigestMismatch,
    SourceBindingMismatch,
    UnitMismatch,
    AlignmentInvalid,
    ExtrapolationForbidden,
    MetricInvalid,
    DispositionInvalid,
    ReviewInvalid,
    EvidenceStale,
    ImmutableRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationError {
    pub code: CorrelationErrorCode,
    pub path: String,
    pub message: String,
}

impl CorrelationError {
    fn new(
        code: CorrelationErrorCode,
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

impl fmt::Display for CorrelationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for CorrelationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationDatasetClass {
    BenchMeasurement,
    SiliconCharacterization,
    VendorReference,
    IndependentOracle,
    ModelSimulation,
}

impl CorrelationDatasetClass {
    #[must_use]
    pub const fn is_simulation(self) -> bool {
        matches!(self, Self::ModelSimulation)
    }
}

/// Exact retained simulation lineage for candidate-model datasets. Imported
/// numerical rows alone are insufficient to reproduce engineering evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationSimulationProvenance {
    pub run_id: String,
    pub run_dataset_id: String,
    pub analysis_id: u64,
    pub analysis_result_digest: ContentDigest,
    pub plan_id: String,
    pub project_revision: ObjectRevision,
    pub prepared_snapshot_digest: ContentDigest,
    pub source_content_digest: ContentDigest,
    pub task_config_digest: ContentDigest,
    pub execution_target: String,
    pub export_digest: ContentDigest,
    pub model_source: ModelSourceEvidenceBinding,
    pub executed_at_unix_ms: u64,
}

impl CorrelationSimulationProvenance {
    pub fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.run_id"), &self.run_id)?;
        require_text(&format!("{path}.run_dataset_id"), &self.run_dataset_id)?;
        require_text(&format!("{path}.plan_id"), &self.plan_id)?;
        require_text(&format!("{path}.execution_target"), &self.execution_target)?;
        require_text(
            &format!("{path}.model_source.model_id"),
            &self.model_source.model_id,
        )?;
        if self.model_source.source_id.is_none() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                format!("{path}.model_source.source_id"),
                "simulation provenance requires an exact project-owned model source",
            ));
        }
        if self.analysis_id == 0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                format!("{path}.analysis_id"),
                "simulation provenance requires a non-zero retained analysis identity",
            ));
        }
        if self.executed_at_unix_ms == 0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                format!("{path}.executed_at_unix_ms"),
                "simulation provenance requires a non-zero execution timestamp",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationCoordinate {
    pub dimension: String,
    pub value: FiniteValue,
    pub unit: String,
}

impl CorrelationCoordinate {
    pub fn try_new(
        dimension: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
    ) -> CorrelationResult<Self> {
        let coordinate = Self {
            dimension: dimension.into(),
            value: finite("coordinate.value", value)?,
            unit: unit.into(),
        };
        coordinate.validate("coordinate")?;
        Ok(coordinate)
    }

    fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.dimension"), &self.dimension)?;
        require_text(&format!("{path}.unit"), &self.unit)?;
        bounded_text(&format!("{path}.dimension"), &self.dimension)?;
        bounded_text(&format!("{path}.unit"), &self.unit)?;
        unit_spec(&self.unit).map(|_| ())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationObservation {
    pub id: String,
    pub quantity: String,
    pub value: FiniteValue,
    pub unit: String,
    pub uncertainty: NonNegativeFinite,
    pub weight: NonNegativeFinite,
    pub coordinates: Vec<CorrelationCoordinate>,
}

impl CorrelationObservation {
    fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.quantity"), &self.quantity)?;
        require_text(&format!("{path}.unit"), &self.unit)?;
        bounded_text(&format!("{path}.id"), &self.id)?;
        bounded_text(&format!("{path}.quantity"), &self.quantity)?;
        bounded_text(&format!("{path}.unit"), &self.unit)?;
        unit_spec(&self.unit)?;
        if self.weight.get() <= 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::InvalidNumber,
                format!("{path}.weight"),
                "observation weight must be finite and greater than zero",
            ));
        }
        let mut dimensions = BTreeSet::new();
        for (index, coordinate) in self.coordinates.iter().enumerate() {
            coordinate.validate(&format!("{path}.coordinates[{index}]"))?;
            if !dimensions.insert(normalized(&coordinate.dimension)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("{path}.coordinates[{index}].dimension"),
                    "an observation cannot repeat a condition dimension",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationDatasetRevision {
    pub schema_version: u32,
    pub id: String,
    pub revision: ObjectRevision,
    pub name: String,
    pub class: CorrelationDatasetClass,
    pub authority: String,
    pub device_or_lot: String,
    pub fixture: String,
    pub calibration: String,
    pub source_name: String,
    pub raw_source: Vec<u8>,
    pub raw_digest: ContentDigest,
    pub model_source: Option<ModelSourceEvidenceBinding>,
    pub simulation_provenance: Option<CorrelationSimulationProvenance>,
    pub observations: Vec<CorrelationObservation>,
}

impl CorrelationDatasetRevision {
    #[allow(clippy::too_many_arguments)]
    pub fn try_from_csv(
        id: impl Into<String>,
        revision: ObjectRevision,
        name: impl Into<String>,
        class: CorrelationDatasetClass,
        authority: impl Into<String>,
        device_or_lot: impl Into<String>,
        fixture: impl Into<String>,
        calibration: impl Into<String>,
        source_name: impl Into<String>,
        raw_source: Vec<u8>,
        model_source: Option<ModelSourceEvidenceBinding>,
    ) -> CorrelationResult<Self> {
        Self::try_from_csv_with_provenance(
            id,
            revision,
            name,
            class,
            authority,
            device_or_lot,
            fixture,
            calibration,
            source_name,
            raw_source,
            model_source,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_from_csv_with_provenance(
        id: impl Into<String>,
        revision: ObjectRevision,
        name: impl Into<String>,
        class: CorrelationDatasetClass,
        authority: impl Into<String>,
        device_or_lot: impl Into<String>,
        fixture: impl Into<String>,
        calibration: impl Into<String>,
        source_name: impl Into<String>,
        raw_source: Vec<u8>,
        model_source: Option<ModelSourceEvidenceBinding>,
        simulation_provenance: Option<CorrelationSimulationProvenance>,
    ) -> CorrelationResult<Self> {
        let observations = parse_correlation_csv(&raw_source)?;
        let raw_digest = digest(&raw_source);
        let value = Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            id: id.into(),
            revision,
            name: name.into(),
            class,
            authority: authority.into(),
            device_or_lot: device_or_lot.into(),
            fixture: fixture.into(),
            calibration: calibration.into(),
            source_name: source_name.into(),
            raw_source,
            raw_digest,
            model_source,
            simulation_provenance,
            observations,
        };
        value.validate("dataset")?;
        Ok(value)
    }

    pub fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_schema(&format!("{path}.schema_version"), self.schema_version)?;
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.name"), &self.name)?;
        require_text(&format!("{path}.authority"), &self.authority)?;
        require_text(&format!("{path}.device_or_lot"), &self.device_or_lot)?;
        require_text(&format!("{path}.fixture"), &self.fixture)?;
        require_text(&format!("{path}.calibration"), &self.calibration)?;
        require_text(&format!("{path}.source_name"), &self.source_name)?;
        bounded_text(&format!("{path}.id"), &self.id)?;
        bounded_text(&format!("{path}.name"), &self.name)?;
        bounded_text(&format!("{path}.authority"), &self.authority)?;
        bounded_text(&format!("{path}.device_or_lot"), &self.device_or_lot)?;
        bounded_text(&format!("{path}.fixture"), &self.fixture)?;
        bounded_text(&format!("{path}.calibration"), &self.calibration)?;
        bounded_text(&format!("{path}.source_name"), &self.source_name)?;
        if self.raw_source.is_empty() || self.raw_source.len() > MAX_CORRELATION_SOURCE_BYTES {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                format!("{path}.raw_source"),
                format!("dataset source must contain 1..={MAX_CORRELATION_SOURCE_BYTES} bytes"),
            ));
        }
        if digest(&self.raw_source) != self.raw_digest {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceDigestMismatch,
                format!("{path}.raw_digest"),
                "retained raw dataset bytes do not match their SHA-256 digest",
            ));
        }
        match (self.class.is_simulation(), &self.model_source) {
            (true, None) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::SourceBindingMismatch,
                    format!("{path}.model_source"),
                    "a model-simulation dataset requires an exact model source binding",
                ));
            }
            (false, Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::SourceBindingMismatch,
                    format!("{path}.model_source"),
                    "independent reference datasets cannot claim a model source binding",
                ));
            }
            _ => {}
        }
        match (self.class.is_simulation(), &self.simulation_provenance) {
            (true, Some(provenance)) => {
                provenance.validate(&format!("{path}.simulation_provenance"))?;
                if provenance.export_digest != self.raw_digest {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::SourceDigestMismatch,
                        format!("{path}.simulation_provenance.export_digest"),
                        "simulation export digest must identify the exact retained CSV bytes",
                    ));
                }
                if self.model_source.as_ref() != Some(&provenance.model_source) {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::SourceBindingMismatch,
                        format!("{path}.simulation_provenance.model_source"),
                        "simulation provenance model source must match the dataset source binding",
                    ));
                }
            }
            (true, None) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::SourceBindingMismatch,
                    format!("{path}.simulation_provenance"),
                    "a model-simulation dataset requires exact retained run, analysis, plan, model, platform, and result provenance",
                ));
            }
            (false, Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::SourceBindingMismatch,
                    format!("{path}.simulation_provenance"),
                    "independent reference datasets cannot claim candidate simulation provenance",
                ));
            }
            (false, None) => {}
        }
        if self
            .model_source
            .as_ref()
            .is_some_and(|binding| binding.source_id.is_none())
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                format!("{path}.model_source.source_id"),
                "new model-simulation datasets require an exact project-owned source identity",
            ));
        }
        if self.observations.is_empty() || self.observations.len() > MAX_CORRELATION_ROWS {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                format!("{path}.observations"),
                format!("dataset must contain 1..={MAX_CORRELATION_ROWS} observations"),
            ));
        }
        let reparsed = parse_correlation_csv(&self.raw_source)?;
        if reparsed != self.observations {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceDigestMismatch,
                format!("{path}.observations"),
                "normalized observations do not match the retained raw CSV source",
            ));
        }
        let mut ids = BTreeSet::new();
        for (index, observation) in self.observations.iter().enumerate() {
            observation.validate(&format!("{path}.observations[{index}]"))?;
            if !ids.insert(normalized(&observation.id)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("{path}.observations[{index}].id"),
                    "observation IDs must be unique within a dataset revision",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationExtrapolationPolicy {
    Forbid,
    Limited {
        max_axis_span_fraction: NonNegativeFinite,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CorrelationAlignmentPolicy {
    ExactOnly,
    MonotoneInterpolation {
        axis: String,
        extrapolation: CorrelationExtrapolationPolicy,
    },
}

impl CorrelationAlignmentPolicy {
    fn validate(&self, path: &str) -> CorrelationResult<()> {
        match self {
            Self::ExactOnly => Ok(()),
            Self::MonotoneInterpolation {
                axis,
                extrapolation,
            } => {
                require_text(&format!("{path}.axis"), axis)?;
                bounded_text(&format!("{path}.axis"), axis)?;
                if let CorrelationExtrapolationPolicy::Limited {
                    max_axis_span_fraction,
                } = extrapolation
                    && max_axis_span_fraction.get() > 1.0
                {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::AlignmentInvalid,
                        format!("{path}.extrapolation.max_axis_span_fraction"),
                        "limited extrapolation cannot exceed one retained axis span",
                    ));
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationMetricDomain {
    pub axis: String,
    pub unit: String,
    pub minimum: FiniteValue,
    pub maximum: FiniteValue,
}

impl CorrelationMetricDomain {
    fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.axis"), &self.axis)?;
        require_text(&format!("{path}.unit"), &self.unit)?;
        bounded_text(&format!("{path}.axis"), &self.axis)?;
        bounded_text(&format!("{path}.unit"), &self.unit)?;
        unit_spec(&self.unit)?;
        if self.minimum.get() >= self.maximum.get() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                path,
                "metric domain minimum must be less than its maximum",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationCalculation {
    AbsoluteLinear,
    AbsoluteDecibels,
    Relative,
    WeightedRelative,
    PhaseWrappedDegrees,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationAggregation {
    EveryPoint,
    WorstCondition,
    Percentile95,
    RootMeanSquare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationReleaseRole {
    Review,
    Advisory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationMetricDefinition {
    pub id: String,
    pub name: String,
    pub reference_dataset_id: String,
    pub simulation_dataset_id: String,
    pub quantity: String,
    pub calculation: CorrelationCalculation,
    pub domain: Option<CorrelationMetricDomain>,
    pub limit: NonNegativeFinite,
    pub uncertainty_multiplier: NonNegativeFinite,
    /// Minimum fraction of eligible reference observations that must remain
    /// evaluated after governed exclusions.
    pub minimum_coverage: NonNegativeFinite,
    pub aggregation: CorrelationAggregation,
    pub alignment: CorrelationAlignmentPolicy,
    pub release_role: CorrelationReleaseRole,
}

impl CorrelationMetricDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        id: impl Into<String>,
        name: impl Into<String>,
        reference_dataset_id: impl Into<String>,
        simulation_dataset_id: impl Into<String>,
        quantity: impl Into<String>,
        calculation: CorrelationCalculation,
        domain: Option<CorrelationMetricDomain>,
        limit: f64,
        uncertainty_multiplier: f64,
        minimum_coverage: f64,
        aggregation: CorrelationAggregation,
        alignment: CorrelationAlignmentPolicy,
        release_role: CorrelationReleaseRole,
    ) -> CorrelationResult<Self> {
        let metric = Self {
            id: id.into(),
            name: name.into(),
            reference_dataset_id: reference_dataset_id.into(),
            simulation_dataset_id: simulation_dataset_id.into(),
            quantity: quantity.into(),
            calculation,
            domain,
            limit: non_negative("metric.limit", limit)?,
            uncertainty_multiplier: non_negative(
                "metric.uncertainty_multiplier",
                uncertainty_multiplier,
            )?,
            minimum_coverage: non_negative("metric.minimum_coverage", minimum_coverage)?,
            aggregation,
            alignment,
            release_role,
        };
        metric.validate("metric")?;
        Ok(metric)
    }

    fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.name"), &self.name)?;
        require_text(
            &format!("{path}.reference_dataset_id"),
            &self.reference_dataset_id,
        )?;
        require_text(
            &format!("{path}.simulation_dataset_id"),
            &self.simulation_dataset_id,
        )?;
        if normalized(&self.reference_dataset_id) == normalized(&self.simulation_dataset_id) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("{path}.simulation_dataset_id"),
                "reference and simulation datasets must be distinct",
            ));
        }
        require_text(&format!("{path}.quantity"), &self.quantity)?;
        bounded_text(&format!("{path}.id"), &self.id)?;
        bounded_text(&format!("{path}.name"), &self.name)?;
        bounded_text(
            &format!("{path}.reference_dataset_id"),
            &self.reference_dataset_id,
        )?;
        bounded_text(
            &format!("{path}.simulation_dataset_id"),
            &self.simulation_dataset_id,
        )?;
        bounded_text(&format!("{path}.quantity"), &self.quantity)?;
        if self.limit.get() <= 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("{path}.limit"),
                "metric limit must be finite and greater than zero",
            ));
        }
        if self.minimum_coverage.get() > 1.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("{path}.minimum_coverage"),
                "minimum metric coverage must be within zero to one",
            ));
        }
        if let Some(domain) = &self.domain {
            domain.validate(&format!("{path}.domain"))?;
        }
        if self.calculation == CorrelationCalculation::WeightedRelative
            && self.aggregation != CorrelationAggregation::RootMeanSquare
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("{path}.aggregation"),
                "weighted-relative metrics require root-mean-square aggregation so weights are normalized by their retained sum",
            ));
        }
        self.alignment.validate(&format!("{path}.alignment"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationOutlierDecision {
    Retain,
    ExcludeFixtureFault,
    LimitOnlyEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationOutlierDisposition {
    pub id: String,
    pub metric_id: String,
    pub reference_observation_id: String,
    pub decision: CorrelationOutlierDecision,
    pub reason: String,
    pub owner_id: String,
    pub reviewer_id: String,
    pub decided_at_unix_ms: u64,
    pub supersedes: Option<String>,
}

impl CorrelationOutlierDisposition {
    fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.metric_id"), &self.metric_id)?;
        require_text(
            &format!("{path}.reference_observation_id"),
            &self.reference_observation_id,
        )?;
        require_text(&format!("{path}.reason"), &self.reason)?;
        require_text(&format!("{path}.owner_id"), &self.owner_id)?;
        require_text(&format!("{path}.reviewer_id"), &self.reviewer_id)?;
        bounded_text(&format!("{path}.id"), &self.id)?;
        bounded_text(&format!("{path}.metric_id"), &self.metric_id)?;
        bounded_text(
            &format!("{path}.reference_observation_id"),
            &self.reference_observation_id,
        )?;
        bounded_text(&format!("{path}.reason"), &self.reason)?;
        bounded_text(&format!("{path}.owner_id"), &self.owner_id)?;
        bounded_text(&format!("{path}.reviewer_id"), &self.reviewer_id)?;
        if let Some(supersedes) = &self.supersedes {
            require_text(&format!("{path}.supersedes"), supersedes)?;
            bounded_text(&format!("{path}.supersedes"), supersedes)?;
        }
        if normalized(&self.owner_id) == normalized(&self.reviewer_id) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.reviewer_id"),
                "outlier disposition requires a reviewer independent from its owner",
            ));
        }
        if self.decided_at_unix_ms == 0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.decided_at_unix_ms"),
                "outlier disposition requires a non-zero decision timestamp",
            ));
        }
        if self
            .supersedes
            .as_ref()
            .is_some_and(|id| normalized(id) == normalized(&self.id))
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.supersedes"),
                "a disposition cannot supersede itself",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationSuite {
    pub schema_version: u32,
    pub id: String,
    pub revision: ObjectRevision,
    pub name: String,
    /// Accountable owner of the model-correlation suite. Reviewer decisions
    /// must be retained under a distinct identity.
    pub owner_id: String,
    pub source: ModelSourceEvidenceBinding,
    pub datasets: Vec<CorrelationDatasetRevision>,
    pub metrics: Vec<CorrelationMetricDefinition>,
    pub dispositions: Vec<CorrelationOutlierDisposition>,
}

impl CorrelationSuite {
    pub fn try_new(
        id: impl Into<String>,
        revision: ObjectRevision,
        name: impl Into<String>,
        owner_id: impl Into<String>,
        source: ModelSourceEvidenceBinding,
        mut datasets: Vec<CorrelationDatasetRevision>,
        mut metrics: Vec<CorrelationMetricDefinition>,
        mut dispositions: Vec<CorrelationOutlierDisposition>,
    ) -> CorrelationResult<Self> {
        datasets.sort_by_key(|dataset| (normalized(&dataset.id), dataset.revision));
        metrics.sort_by_key(|metric| normalized(&metric.id));
        dispositions.sort_by_key(|disposition| {
            (disposition.decided_at_unix_ms, normalized(&disposition.id))
        });
        let value = Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            id: id.into(),
            revision,
            name: name.into(),
            owner_id: owner_id.into(),
            source,
            datasets,
            metrics,
            dispositions,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> CorrelationResult<()> {
        require_schema("suite.schema_version", self.schema_version)?;
        require_text("suite.id", &self.id)?;
        require_text("suite.name", &self.name)?;
        require_text("suite.owner_id", &self.owner_id)?;
        require_text("suite.source.model_id", &self.source.model_id)?;
        bounded_text("suite.id", &self.id)?;
        bounded_text("suite.name", &self.name)?;
        bounded_text("suite.owner_id", &self.owner_id)?;
        bounded_text("suite.source.model_id", &self.source.model_id)?;
        if self.source.source_id.is_none() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                "suite.source.source_id",
                "new correlation suites require an exact project-owned source identity",
            ));
        }
        if self.datasets.is_empty() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                "suite.datasets",
                "a correlation suite requires at least one retained dataset",
            ));
        }
        require_count(
            "suite.datasets",
            self.datasets.len(),
            MAX_CORRELATION_DATASET_REVISIONS,
            "dataset revisions",
        )?;
        require_count(
            "suite.metrics",
            self.metrics.len(),
            MAX_CORRELATION_METRICS,
            "metric definitions",
        )?;
        require_count(
            "suite.dispositions",
            self.dispositions.len(),
            MAX_CORRELATION_DISPOSITIONS,
            "outlier disposition events",
        )?;
        let mut total_source_bytes = 0_usize;
        let mut total_observations = 0_usize;
        let mut dataset_keys = BTreeSet::new();
        for (index, dataset) in self.datasets.iter().enumerate() {
            dataset.validate(&format!("suite.datasets[{index}]"))?;
            total_source_bytes = checked_count_add(
                "suite.datasets",
                total_source_bytes,
                dataset.raw_source.len(),
                "retained dataset source bytes",
            )?;
            total_observations = checked_count_add(
                "suite.datasets",
                total_observations,
                dataset.observations.len(),
                "retained dataset observations",
            )?;
            if !dataset_keys.insert((normalized(&dataset.id), dataset.revision)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("suite.datasets[{index}]"),
                    "dataset ID/revision pairs must be unique",
                ));
            }
            if let Some(binding) = &dataset.model_source
                && binding != &self.source
            {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::SourceBindingMismatch,
                    format!("suite.datasets[{index}].model_source"),
                    "simulated datasets must bind the exact suite model source revision",
                ));
            }
        }
        require_count(
            "suite.datasets",
            total_source_bytes,
            MAX_CORRELATION_SUITE_SOURCE_BYTES,
            "retained dataset source bytes",
        )?;
        require_count(
            "suite.datasets",
            total_observations,
            MAX_CORRELATION_SUITE_OBSERVATIONS,
            "retained dataset observations",
        )?;
        let latest_datasets = self.latest_datasets()?;
        let mut metric_ids = BTreeSet::new();
        for (index, metric) in self.metrics.iter().enumerate() {
            metric.validate(&format!("suite.metrics[{index}]"))?;
            if !metric_ids.insert(normalized(&metric.id)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("suite.metrics[{index}].id"),
                    "metric IDs must be unique",
                ));
            }
            let reference =
                find_ci(&latest_datasets, &metric.reference_dataset_id).ok_or_else(|| {
                    CorrelationError::new(
                        CorrelationErrorCode::MetricInvalid,
                        format!("suite.metrics[{index}].reference_dataset_id"),
                        "metric reference dataset does not exist",
                    )
                })?;
            if reference.class.is_simulation() {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::MetricInvalid,
                    format!("suite.metrics[{index}].reference_dataset_id"),
                    "metric reference must be independent of the candidate model simulation",
                ));
            }
            let simulation =
                find_ci(&latest_datasets, &metric.simulation_dataset_id).ok_or_else(|| {
                    CorrelationError::new(
                        CorrelationErrorCode::MetricInvalid,
                        format!("suite.metrics[{index}].simulation_dataset_id"),
                        "metric simulation dataset does not exist",
                    )
                })?;
            if !simulation.class.is_simulation() {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::MetricInvalid,
                    format!("suite.metrics[{index}].simulation_dataset_id"),
                    "metric simulation input must be an exact source-bound model dataset",
                ));
            }
        }
        validate_disposition_ledger(self, &metric_ids, &latest_datasets)?;
        Ok(())
    }

    pub fn content_digest(&self) -> CorrelationResult<ContentDigest> {
        self.validate()?;
        serde_json::to_vec(self)
            .map(|bytes| digest(&bytes))
            .map_err(|error| {
                CorrelationError::new(
                    CorrelationErrorCode::SourceDigestMismatch,
                    "suite",
                    format!("suite could not be canonically serialized: {error}"),
                )
            })
    }

    pub fn latest_datasets(&self) -> CorrelationResult<Vec<&CorrelationDatasetRevision>> {
        let mut latest = BTreeMap::<String, &CorrelationDatasetRevision>::new();
        for dataset in &self.datasets {
            let key = normalized(&dataset.id);
            if let Some(existing) = latest.get(&key)
                && existing.revision == dataset.revision
            {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    "suite.datasets",
                    "dataset ID/revision pairs must be unique",
                ));
            }
            if latest
                .get(&key)
                .is_none_or(|existing| existing.revision < dataset.revision)
            {
                latest.insert(key, dataset);
            }
        }
        Ok(latest.into_values().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationResidualPoint {
    pub id: String,
    pub metric_id: String,
    pub reference_observation_id: String,
    pub reference_value: FiniteValue,
    pub simulated_value: FiniteValue,
    pub simulation_observation_ids: Vec<String>,
    pub alignment_evidence: CorrelationAlignmentEvidence,
    pub metric_error: NonNegativeFinite,
    pub effective_limit: NonNegativeFinite,
    pub normalized_error: NonNegativeFinite,
    pub weight: NonNegativeFinite,
    /// Canonical non-sweep condition identity used by worst-condition
    /// aggregation. The alignment axis is deliberately omitted.
    pub condition_group: String,
    pub excluded: bool,
    pub exclusion_disposition_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationAlignmentEvidence {
    Exact,
    Interpolated,
    Extrapolated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationMetricOutcome {
    pub metric_id: String,
    pub release_role: CorrelationReleaseRole,
    pub evaluated_points: usize,
    pub excluded_points: usize,
    pub coverage: NonNegativeFinite,
    pub minimum_coverage: NonNegativeFinite,
    pub aggregate_error: NonNegativeFinite,
    pub aggregate_normalized_error: NonNegativeFinite,
    pub passed: bool,
    pub residuals: Vec<CorrelationResidualPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationEvaluation {
    pub schema_version: u32,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub suite_digest: ContentDigest,
    pub source: ModelSourceEvidenceBinding,
    pub metric_outcomes: Vec<CorrelationMetricOutcome>,
    pub passed: bool,
}

impl CorrelationEvaluation {
    pub fn evaluate(suite: &CorrelationSuite) -> CorrelationResult<Self> {
        suite.validate()?;
        if suite.metrics.is_empty() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                "suite.metrics",
                "validation requires at least one retained metric definition",
            ));
        }
        let suite_digest = suite.content_digest()?;
        let latest = suite.latest_datasets()?;
        let dispositions = current_dispositions(suite)?;
        let mut outcomes = Vec::with_capacity(suite.metrics.len());
        for metric in &suite.metrics {
            let reference = find_ci(&latest, &metric.reference_dataset_id).ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::MetricInvalid,
                    "metric.reference_dataset_id",
                    "reference dataset is missing",
                )
            })?;
            let simulation = find_ci(&latest, &metric.simulation_dataset_id).ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::MetricInvalid,
                    "metric.simulation_dataset_id",
                    "simulation dataset is missing",
                )
            })?;
            outcomes.push(evaluate_metric(
                metric,
                reference,
                simulation,
                &dispositions,
            )?);
        }
        outcomes.sort_by_key(|outcome| normalized(&outcome.metric_id));
        validate_metric_outcomes(&outcomes, "evaluation")?;
        let review_outcomes = outcomes
            .iter()
            .filter(|outcome| outcome.release_role == CorrelationReleaseRole::Review)
            .collect::<Vec<_>>();
        let passed =
            !review_outcomes.is_empty() && review_outcomes.iter().all(|outcome| outcome.passed);
        Ok(Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            suite_digest,
            source: suite.source.clone(),
            metric_outcomes: outcomes,
            passed,
        })
    }
}

/// Independent reviewer disposition. Numerical gate results remain immutable
/// facts; this decision records whether the reviewer accepts those facts as
/// sufficient evidence for the next governed workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationReviewDecision {
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationEvidence {
    pub schema_version: u32,
    pub id: String,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub suite_digest: ContentDigest,
    pub source: ModelSourceEvidenceBinding,
    pub metric_outcomes: Vec<CorrelationMetricOutcome>,
    /// Numerical result of the review-role metric and coverage gates.
    pub passed: bool,
    pub reviewer_id: String,
    pub decision: CorrelationReviewDecision,
    pub conclusion: String,
    pub reviewed_at_unix_ms: u64,
}

impl CorrelationEvidence {
    pub fn try_new(
        id: impl Into<String>,
        evaluation: &CorrelationEvaluation,
        reviewer_id: impl Into<String>,
        decision: CorrelationReviewDecision,
        conclusion: impl Into<String>,
        reviewed_at_unix_ms: u64,
    ) -> CorrelationResult<Self> {
        let value = Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            id: id.into(),
            suite_id: evaluation.suite_id.clone(),
            suite_revision: evaluation.suite_revision,
            suite_digest: evaluation.suite_digest,
            source: evaluation.source.clone(),
            metric_outcomes: evaluation.metric_outcomes.clone(),
            passed: evaluation.passed,
            reviewer_id: reviewer_id.into(),
            decision,
            conclusion: conclusion.into(),
            reviewed_at_unix_ms,
        };
        value.validate("evidence")?;
        Ok(value)
    }

    pub fn validate(&self, path: &str) -> CorrelationResult<()> {
        require_schema(&format!("{path}.schema_version"), self.schema_version)?;
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.suite_id"), &self.suite_id)?;
        require_text(&format!("{path}.source.model_id"), &self.source.model_id)?;
        if self.source.source_id.is_none() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                format!("{path}.source.source_id"),
                "correlation evidence requires an exact project-owned source identity",
            ));
        }
        require_text(&format!("{path}.reviewer_id"), &self.reviewer_id)?;
        require_text(&format!("{path}.conclusion"), &self.conclusion)?;
        bounded_text(&format!("{path}.conclusion"), &self.conclusion)?;
        bounded_text(&format!("{path}.id"), &self.id)?;
        bounded_text(&format!("{path}.suite_id"), &self.suite_id)?;
        bounded_text(&format!("{path}.source.model_id"), &self.source.model_id)?;
        bounded_text(&format!("{path}.reviewer_id"), &self.reviewer_id)?;
        if self.reviewed_at_unix_ms == 0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                format!("{path}.reviewed_at_unix_ms"),
                "evidence requires a non-zero review timestamp",
            ));
        }
        if self.metric_outcomes.is_empty() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MissingValue,
                format!("{path}.metric_outcomes"),
                "evidence requires at least one evaluated metric",
            ));
        }
        validate_metric_outcomes(&self.metric_outcomes, path)?;
        let review_outcomes = self
            .metric_outcomes
            .iter()
            .filter(|outcome| outcome.release_role == CorrelationReleaseRole::Review)
            .collect::<Vec<_>>();
        let expected_passed =
            !review_outcomes.is_empty() && review_outcomes.iter().all(|outcome| outcome.passed);
        if self.passed != expected_passed {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{path}.passed"),
                "evidence pass state does not match its review-role metric outcomes",
            ));
        }
        if self.decision == CorrelationReviewDecision::Accept && !self.passed {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ReviewInvalid,
                format!("{path}.decision"),
                "a reviewer cannot accept correlation evidence that fails its numerical or coverage gates",
            ));
        }
        Ok(())
    }

    /// True only when deterministic numerical gates pass and an independent
    /// reviewer explicitly accepts the evidence.
    #[must_use]
    pub const fn approved(&self) -> bool {
        self.passed && matches!(self.decision, CorrelationReviewDecision::Accept)
    }

    pub fn validate_current(&self, suite: &CorrelationSuite) -> CorrelationResult<()> {
        self.validate("evidence")?;
        if normalized(&self.reviewer_id) == normalized(&suite.owner_id) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ReviewInvalid,
                "evidence.reviewer_id",
                "correlation review requires a reviewer identity independent from the suite owner",
            ));
        }
        if !self.suite_id.eq_ignore_ascii_case(&suite.id)
            || self.suite_revision != suite.revision
            || self.suite_digest != suite.content_digest()?
            || self.source != suite.source
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                "evidence",
                "correlation evidence does not bind the exact current suite and model source",
            ));
        }
        let evaluation = CorrelationEvaluation::evaluate(suite)?;
        if self.metric_outcomes != evaluation.metric_outcomes || self.passed != evaluation.passed {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                "evidence.metric_outcomes",
                "correlation evidence does not reproduce the deterministic current-suite evaluation",
            ));
        }
        Ok(())
    }

    pub fn content_digest(&self) -> CorrelationResult<ContentDigest> {
        self.validate("evidence")?;
        serde_json::to_vec(self)
            .map(|bytes| digest(&bytes))
            .map_err(|error| {
                CorrelationError::new(
                    CorrelationErrorCode::SourceDigestMismatch,
                    "evidence",
                    format!("evidence could not be canonically serialized: {error}"),
                )
            })
    }
}

fn validate_metric_outcomes(
    outcomes: &[CorrelationMetricOutcome],
    path: &str,
) -> CorrelationResult<()> {
    require_count(
        &format!("{path}.metric_outcomes"),
        outcomes.len(),
        MAX_CORRELATION_METRICS,
        "metric outcomes",
    )?;
    let mut metric_ids = BTreeSet::new();
    let mut residual_ids = BTreeSet::new();
    let mut total_residuals = 0_usize;
    for (outcome_index, outcome) in outcomes.iter().enumerate() {
        let outcome_path = format!("{path}.metric_outcomes[{outcome_index}]");
        require_text(&format!("{outcome_path}.metric_id"), &outcome.metric_id)?;
        bounded_text(&format!("{outcome_path}.metric_id"), &outcome.metric_id)?;
        if !metric_ids.insert(normalized(&outcome.metric_id)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("{outcome_path}.metric_id"),
                "evidence metric outcome IDs must be unique",
            ));
        }
        let total_points = outcome
            .evaluated_points
            .checked_add(outcome.excluded_points)
            .ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::ResourceLimit,
                    format!("{outcome_path}.excluded_points"),
                    "metric point counters overflow the supported range",
                )
            })?;
        if total_points != outcome.residuals.len() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.evaluated_points"),
                "evaluated plus excluded point counts must match retained residuals",
            ));
        }
        require_count(
            &format!("{outcome_path}.residuals"),
            total_points,
            MAX_CORRELATION_ROWS,
            "metric residual points",
        )?;
        total_residuals = checked_count_add(
            &format!("{path}.metric_outcomes"),
            total_residuals,
            total_points,
            "retained evidence residual points",
        )?;
        if total_points == 0 || !(0.0..=1.0).contains(&outcome.coverage.get()) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.coverage"),
                "metric coverage must be finite and within zero to one",
            ));
        }
        let expected_coverage = outcome.evaluated_points as f64 / total_points as f64;
        if (expected_coverage - outcome.coverage.get()).abs()
            > f64::EPSILON * expected_coverage.abs().max(1.0) * 8.0
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.coverage"),
                "metric coverage does not match retained evaluated and excluded point counts",
            ));
        }
        if outcome.minimum_coverage.get() > 1.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.minimum_coverage"),
                "retained minimum coverage must be within zero to one",
            ));
        }
        if outcome
            .residuals
            .iter()
            .filter(|residual| residual.excluded)
            .count()
            != outcome.excluded_points
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.excluded_points"),
                "excluded point count does not match retained residual dispositions",
            ));
        }
        let expected_passed = outcome.aggregate_normalized_error.get() <= 1.0
            && outcome.coverage.get() >= outcome.minimum_coverage.get();
        if outcome.passed != expected_passed {
            return Err(CorrelationError::new(
                CorrelationErrorCode::EvidenceStale,
                format!("{outcome_path}.passed"),
                "metric pass state does not match its normalized aggregate error and declared minimum coverage",
            ));
        }
        for (residual_index, residual) in outcome.residuals.iter().enumerate() {
            let residual_path = format!("{outcome_path}.residuals[{residual_index}]");
            require_text(&format!("{residual_path}.id"), &residual.id)?;
            bounded_text(&format!("{residual_path}.id"), &residual.id)?;
            bounded_text(&format!("{residual_path}.metric_id"), &residual.metric_id)?;
            require_text(
                &format!("{residual_path}.reference_observation_id"),
                &residual.reference_observation_id,
            )?;
            bounded_text(
                &format!("{residual_path}.reference_observation_id"),
                &residual.reference_observation_id,
            )?;
            let expected_sources = match residual.alignment_evidence {
                CorrelationAlignmentEvidence::Exact => 1,
                CorrelationAlignmentEvidence::Interpolated
                | CorrelationAlignmentEvidence::Extrapolated => 2,
            };
            if residual.simulation_observation_ids.len() != expected_sources {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.simulation_observation_ids"),
                    "alignment evidence does not retain the required exact simulation point identities",
                ));
            }
            let mut simulation_ids = BTreeSet::new();
            for (source_index, source_id) in residual.simulation_observation_ids.iter().enumerate()
            {
                require_text(
                    &format!("{residual_path}.simulation_observation_ids[{source_index}]"),
                    source_id,
                )?;
                bounded_text(
                    &format!("{residual_path}.simulation_observation_ids[{source_index}]"),
                    source_id,
                )?;
                if !simulation_ids.insert(normalized(source_id)) {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::DuplicateIdentity,
                        format!("{residual_path}.simulation_observation_ids[{source_index}]"),
                        "alignment evidence cannot repeat a simulation observation identity",
                    ));
                }
            }
            if !residual.metric_id.eq_ignore_ascii_case(&outcome.metric_id) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.metric_id"),
                    "residual metric identity does not match its owning outcome",
                ));
            }
            if !residual_ids.insert(normalized(&residual.id)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("{residual_path}.id"),
                    "evidence residual IDs must be unique",
                ));
            }
            if residual.excluded != residual.exclusion_disposition_id.is_some() {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.excluded"),
                    "excluded residuals must retain exactly one disposition identity",
                ));
            }
            if residual.weight.get() <= 0.0 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.weight"),
                    "retained residual weights must be greater than zero",
                ));
            }
            require_text(
                &format!("{residual_path}.condition_group"),
                &residual.condition_group,
            )?;
            bounded_text(
                &format!("{residual_path}.condition_group"),
                &residual.condition_group,
            )?;
            if residual.effective_limit.get() <= 0.0 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.effective_limit"),
                    "retained residual effective limits must be greater than zero",
                ));
            }
            let expected = residual.metric_error.get() / residual.effective_limit.get();
            let tolerance = f64::EPSILON * expected.abs().max(1.0) * 8.0;
            if (expected - residual.normalized_error.get()).abs() > tolerance {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    format!("{residual_path}.normalized_error"),
                    "retained normalized residual does not match error divided by effective limit",
                ));
            }
        }
    }
    require_count(
        &format!("{path}.metric_outcomes"),
        total_residuals,
        MAX_CORRELATION_EVIDENCE_RESIDUALS,
        "retained evidence residual points",
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelCorrelationState {
    pub schema_version: u32,
    #[serde(default)]
    pub suites: Vec<CorrelationSuite>,
    #[serde(default)]
    pub evidence: Vec<CorrelationEvidence>,
}

impl Default for ModelCorrelationState {
    fn default() -> Self {
        Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            suites: Vec::new(),
            evidence: Vec::new(),
        }
    }
}

impl ModelCorrelationState {
    pub fn try_new(
        mut suites: Vec<CorrelationSuite>,
        mut evidence: Vec<CorrelationEvidence>,
    ) -> CorrelationResult<Self> {
        suites.sort_by_key(|suite| (normalized(&suite.id), suite.revision));
        evidence.sort_by_key(|record| normalized(&record.id));
        let value = Self {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            suites,
            evidence,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> CorrelationResult<()> {
        require_schema("correlation_state.schema_version", self.schema_version)?;
        require_count(
            "correlation_state.suites",
            self.suites.len(),
            MAX_CORRELATION_SUITE_REVISIONS,
            "correlation suite revisions",
        )?;
        require_count(
            "correlation_state.evidence",
            self.evidence.len(),
            MAX_CORRELATION_EVIDENCE_RECORDS,
            "correlation evidence records",
        )?;
        let mut suites = BTreeSet::new();
        let mut total_source_bytes = 0_usize;
        let mut total_observations = 0_usize;
        for (index, suite) in self.suites.iter().enumerate() {
            suite.validate()?;
            for dataset in &suite.datasets {
                total_source_bytes = checked_count_add(
                    "correlation_state.suites",
                    total_source_bytes,
                    dataset.raw_source.len(),
                    "retained historical dataset source bytes",
                )?;
                total_observations = checked_count_add(
                    "correlation_state.suites",
                    total_observations,
                    dataset.observations.len(),
                    "retained historical dataset observations",
                )?;
            }
            if !suites.insert((normalized(&suite.id), suite.revision)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("correlation_state.suites[{index}]"),
                    "suite ID/revision pairs must be unique",
                ));
            }
        }
        require_count(
            "correlation_state.suites",
            total_source_bytes,
            MAX_CORRELATION_STATE_SOURCE_BYTES,
            "retained historical dataset source bytes",
        )?;
        require_count(
            "correlation_state.suites",
            total_observations,
            MAX_CORRELATION_STATE_OBSERVATIONS,
            "retained historical dataset observations",
        )?;
        let mut evidence_ids = BTreeSet::new();
        let mut total_residuals = 0_usize;
        for (index, evidence) in self.evidence.iter().enumerate() {
            evidence.validate(&format!("correlation_state.evidence[{index}]"))?;
            for outcome in &evidence.metric_outcomes {
                total_residuals = checked_count_add(
                    "correlation_state.evidence",
                    total_residuals,
                    outcome.residuals.len(),
                    "retained historical evidence residuals",
                )?;
            }
            if !evidence_ids.insert(normalized(&evidence.id)) {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DuplicateIdentity,
                    format!("correlation_state.evidence[{index}].id"),
                    "evidence IDs are immutable and must be unique",
                ));
            }
            let suite = self
                .suites
                .iter()
                .find(|suite| {
                    suite.id.eq_ignore_ascii_case(&evidence.suite_id)
                        && suite.revision == evidence.suite_revision
                })
                .ok_or_else(|| {
                    CorrelationError::new(
                        CorrelationErrorCode::EvidenceStale,
                        format!("correlation_state.evidence[{index}].suite_id"),
                        "evidence must reference an exact retained suite revision",
                    )
                })?;
            evidence.validate_current(suite)?;
        }
        require_count(
            "correlation_state.evidence",
            total_residuals,
            MAX_CORRELATION_STATE_RESIDUALS,
            "retained historical evidence residuals",
        )?;
        Ok(())
    }

    pub fn validate_for_model(&self, model_name: &str) -> CorrelationResult<()> {
        require_text("model_name", model_name)?;
        bounded_text("model_name", model_name)?;
        self.validate()?;
        if self
            .suites
            .iter()
            .any(|suite| !suite.source.model_id.eq_ignore_ascii_case(model_name))
            || self
                .evidence
                .iter()
                .any(|evidence| !evidence.source.model_id.eq_ignore_ascii_case(model_name))
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                "correlation_state",
                "every correlation record must bind the exact owning model identity",
            ));
        }
        Ok(())
    }

    pub fn latest_suite(&self, suite_id: &str) -> Option<&CorrelationSuite> {
        self.suites
            .iter()
            .filter(|suite| suite.id.eq_ignore_ascii_case(suite_id))
            .max_by_key(|suite| suite.revision)
    }

    pub fn suite_lineages(&self) -> Vec<&CorrelationSuite> {
        let mut latest = BTreeMap::<String, &CorrelationSuite>::new();
        for suite in &self.suites {
            let key = normalized(&suite.id);
            if latest
                .get(&key)
                .is_none_or(|current| current.revision < suite.revision)
            {
                latest.insert(key, suite);
            }
        }
        latest.into_values().collect()
    }

    /// Enforce the model-release correlation gate against the exact current
    /// project source. An entirely unconfigured correlation state is
    /// optional, but once any suite exists the latest revision of every suite
    /// lineage bound to the current source must have a current, numerically
    /// passing reviewer acceptance. Historical source revisions remain
    /// retained but cannot authorize the current model.
    pub fn require_release_approval(
        &self,
        model_name: &str,
        source: &ModelSourceEvidenceBinding,
    ) -> CorrelationResult<()> {
        self.validate_for_model(model_name)?;
        if self.suites.is_empty() {
            return Ok(());
        }
        let current_suites = self
            .suite_lineages()
            .into_iter()
            .filter(|suite| suite.source == *source)
            .collect::<Vec<_>>();
        if current_suites.is_empty() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::SourceBindingMismatch,
                "correlation_state.suites",
                "configured correlation evidence is stale for the current model source revision",
            ));
        }
        for suite in current_suites {
            let latest = self
                .evidence
                .iter()
                .filter(|evidence| {
                    evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                        && evidence.suite_revision == suite.revision
                        && evidence.source == *source
                })
                .max_by_key(|evidence| (evidence.reviewed_at_unix_ms, evidence.id.as_str()));
            let Some(evidence) = latest else {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::ReviewInvalid,
                    format!("correlation_state.suite.{}", suite.id),
                    "the current correlation suite has no retained reviewer decision",
                ));
            };
            evidence.validate_current(suite)?;
            if !evidence.approved() {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::ReviewInvalid,
                    format!("correlation_state.suite.{}", suite.id),
                    "the latest current correlation decision is not an accepted passing review",
                ));
            }
        }
        Ok(())
    }

    pub fn add_suite_revision(&mut self, suite: CorrelationSuite) -> CorrelationResult<()> {
        suite.validate()?;
        if self.suites.iter().any(|existing| {
            existing.id.eq_ignore_ascii_case(&suite.id) && existing.revision == suite.revision
        }) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ImmutableRecord,
                "correlation_state.suites",
                "a suite revision with this identity already exists",
            ));
        }
        if self
            .suites
            .iter()
            .filter(|existing| existing.id.eq_ignore_ascii_case(&suite.id))
            .any(|existing| existing.revision >= suite.revision)
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ImmutableRecord,
                "correlation_state.suites",
                "new suite revisions must advance monotonically",
            ));
        }
        let mut candidate = self.clone();
        candidate.suites.push(suite);
        candidate
            .suites
            .sort_by_key(|suite| (normalized(&suite.id), suite.revision));
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn add_evidence(&mut self, evidence: CorrelationEvidence) -> CorrelationResult<()> {
        evidence.validate("evidence")?;
        if self
            .evidence
            .iter()
            .any(|existing| existing.id.eq_ignore_ascii_case(&evidence.id))
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ImmutableRecord,
                "correlation_state.evidence",
                "correlation evidence identities are immutable and cannot be overwritten",
            ));
        }
        let suite = self
            .suites
            .iter()
            .find(|suite| {
                suite.id.eq_ignore_ascii_case(&evidence.suite_id)
                    && suite.revision == evidence.suite_revision
            })
            .ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::EvidenceStale,
                    "correlation_state.evidence",
                    "evidence suite revision is not retained",
                )
            })?;
        evidence.validate_current(suite)?;
        let mut candidate = self.clone();
        candidate.evidence.push(evidence);
        candidate
            .evidence
            .sort_by_key(|record| normalized(&record.id));
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }
}

fn parse_correlation_csv(raw: &[u8]) -> CorrelationResult<Vec<CorrelationObservation>> {
    if raw.is_empty() || raw.len() > MAX_CORRELATION_SOURCE_BYTES {
        return Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            "csv",
            format!("CSV source must contain 1..={MAX_CORRELATION_SOURCE_BYTES} bytes"),
        ));
    }
    std::str::from_utf8(raw).map_err(|error| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv",
            format!("correlation CSV must be strict UTF-8: {error}"),
        )
    })?;
    let csv_source = raw.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(raw);
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(false)
        .from_reader(csv_source);
    let headers = reader.headers().map_err(csv_error)?.clone();
    if headers.is_empty() || headers.len() > MAX_CORRELATION_COLUMNS {
        return Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            "csv.headers",
            format!("CSV must contain 1..={MAX_CORRELATION_COLUMNS} columns"),
        ));
    }
    let mut header_names = BTreeSet::new();
    for (index, header) in headers.iter().enumerate() {
        require_text(&format!("csv.headers[{index}]"), header)?;
        bounded_text(&format!("csv.headers[{index}]"), header)?;
        if !header_names.insert(normalized(header)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("csv.headers[{index}]"),
                "CSV header names must be unique ignoring case and surrounding whitespace",
            ));
        }
    }
    let required = [
        ("id", required_column(&headers, "id")?),
        ("quantity", required_column(&headers, "quantity")?),
        ("value", required_column(&headers, "value")?),
        ("unit", required_column(&headers, "unit")?),
    ];
    let uncertainty = optional_column(&headers, "uncertainty");
    let weight = optional_column(&headers, "weight");
    let known = required
        .iter()
        .map(|(_, index)| *index)
        .chain(uncertainty)
        .chain(weight)
        .collect::<BTreeSet<_>>();
    let mut conditions = Vec::new();
    for (index, header) in headers.iter().enumerate() {
        if known.contains(&index) {
            continue;
        }
        conditions.push((index, parse_condition_header(header)?));
    }

    let mut observations = Vec::new();
    let mut ids = BTreeSet::new();
    for (row_index, row) in reader.records().enumerate() {
        if row_index >= MAX_CORRELATION_ROWS {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                "csv.rows",
                format!("CSV exceeds the {MAX_CORRELATION_ROWS}-row import limit"),
            ));
        }
        let row = row.map_err(csv_error)?;
        validate_cells(&row, row_index)?;
        let value = |column: usize| row.get(column).unwrap_or_default();
        let id = value(required[0].1).trim().to_owned();
        let quantity = value(required[1].1).trim().to_owned();
        let observed = parse_finite_cell(value(required[2].1), row_index, "value")?;
        let unit = value(required[3].1).trim().to_owned();
        require_text(&format!("csv.rows[{row_index}].id"), &id)?;
        require_text(&format!("csv.rows[{row_index}].quantity"), &quantity)?;
        require_text(&format!("csv.rows[{row_index}].unit"), &unit)?;
        unit_spec(&unit)?;
        if !ids.insert(normalized(&id)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("csv.rows[{row_index}].id"),
                "observation IDs must be unique within the imported dataset",
            ));
        }
        let uncertainty = uncertainty.map_or(Ok(0.0), |column| {
            parse_non_negative_cell(value(column), row_index, "uncertainty")
        })?;
        let weight = weight.map_or(Ok(1.0), |column| {
            parse_positive_cell(value(column), row_index, "weight")
        })?;
        let mut coordinates = Vec::with_capacity(conditions.len());
        for (column, condition) in &conditions {
            let coordinate_value =
                parse_finite_cell(value(*column), row_index, &condition.dimension)?;
            coordinates.push(CorrelationCoordinate {
                dimension: condition.dimension.clone(),
                value: finite("csv.condition", coordinate_value)?,
                unit: condition.unit.clone(),
            });
        }
        coordinates.sort_by_key(|coordinate| normalized(&coordinate.dimension));
        let observation = CorrelationObservation {
            id,
            quantity,
            value: finite("csv.value", observed)?,
            unit,
            uncertainty: non_negative("csv.uncertainty", uncertainty)?,
            weight: non_negative("csv.weight", weight)?,
            coordinates,
        };
        observation.validate(&format!("csv.rows[{row_index}]"))?;
        observations.push(observation);
    }
    if observations.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MissingValue,
            "csv.rows",
            "correlation CSV contains no data rows",
        ));
    }
    Ok(observations)
}

fn required_column(headers: &StringRecord, name: &str) -> CorrelationResult<usize> {
    optional_column(headers, name).ok_or_else(|| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("required CSV column '{name}' is missing"),
        )
    })
}

fn optional_column(headers: &StringRecord, name: &str) -> Option<usize> {
    headers
        .iter()
        .position(|header| header.trim().eq_ignore_ascii_case(name))
}

fn parse_condition_header(header: &str) -> CorrelationResult<CorrelationHeaderCondition> {
    let Some((prefix, rest)) = header.trim().split_once(':') else {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("unknown column '{header}'; condition columns use condition:<name>[<unit>]"),
        ));
    };
    if !prefix.eq_ignore_ascii_case("condition") {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("unknown column '{header}'; condition columns use condition:<name>[<unit>]"),
        ));
    }
    let (dimension, unit) = if let Some(open) = rest.rfind('[') {
        if !rest.ends_with(']') {
            return Err(CorrelationError::new(
                CorrelationErrorCode::InvalidCsv,
                "csv.headers",
                format!("condition column '{header}' has an unterminated unit"),
            ));
        }
        (&rest[..open], &rest[open + 1..rest.len() - 1])
    } else {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidCsv,
            "csv.headers",
            format!("condition column '{header}' must declare a unit"),
        ));
    };
    require_text("csv.condition.dimension", dimension)?;
    require_text("csv.condition.unit", unit)?;
    unit_spec(unit)?;
    Ok(CorrelationHeaderCondition {
        dimension: dimension.trim().to_owned(),
        unit: unit.trim().to_owned(),
    })
}

#[derive(Debug)]
struct CorrelationHeaderCondition {
    dimension: String,
    unit: String,
}

fn validate_cells(row: &StringRecord, row_index: usize) -> CorrelationResult<()> {
    for (column, cell) in row.iter().enumerate() {
        if cell.len() > MAX_CORRELATION_TEXT_BYTES {
            return Err(CorrelationError::new(
                CorrelationErrorCode::ResourceLimit,
                format!("csv.rows[{row_index}][{column}]"),
                format!("CSV cells are limited to {MAX_CORRELATION_TEXT_BYTES} UTF-8 bytes"),
            ));
        }
    }
    Ok(())
}

fn parse_finite_cell(value: &str, row: usize, field: &str) -> CorrelationResult<f64> {
    let value = value.trim().parse::<f64>().map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a finite decimal number",
        )
    })?;
    if !value.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a finite decimal number",
        ));
    }
    Ok(if value == 0.0 { 0.0 } else { value })
}

fn parse_non_negative_cell(value: &str, row: usize, field: &str) -> CorrelationResult<f64> {
    let value = parse_finite_cell(value, row, field)?;
    if value < 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a non-negative value",
        ));
    }
    Ok(value)
}

fn parse_positive_cell(value: &str, row: usize, field: &str) -> CorrelationResult<f64> {
    let value = parse_finite_cell(value, row, field)?;
    if value <= 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("csv.rows[{row}].{field}"),
            "cell must contain a value greater than zero",
        ));
    }
    Ok(value)
}

fn csv_error(error: csv::Error) -> CorrelationError {
    CorrelationError::new(
        CorrelationErrorCode::InvalidCsv,
        "csv",
        format!("CSV parsing failed: {error}"),
    )
}

fn validate_disposition_ledger(
    suite: &CorrelationSuite,
    metric_ids: &BTreeSet<String>,
    datasets: &[&CorrelationDatasetRevision],
) -> CorrelationResult<()> {
    let mut by_id = BTreeMap::<String, &CorrelationOutlierDisposition>::new();
    let mut current = BTreeMap::<(String, String), &CorrelationOutlierDisposition>::new();
    for (index, disposition) in suite.dispositions.iter().enumerate() {
        let path = format!("suite.dispositions[{index}]");
        disposition.validate(&path)?;
        if !metric_ids.contains(&normalized(&disposition.metric_id)) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.metric_id"),
                "outlier disposition metric does not exist",
            ));
        }
        let disposition_metric_id = normalized(&disposition.metric_id);
        let metric = suite
            .metrics
            .iter()
            .find(|metric| normalized(&metric.id) == disposition_metric_id)
            .ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.metric_id"),
                    "outlier disposition metric does not exist",
                )
            })?;
        let reference = find_ci(datasets, &metric.reference_dataset_id).ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.metric_id"),
                "outlier disposition reference dataset does not exist",
            )
        })?;
        if !reference.observations.iter().any(|observation| {
            observation
                .id
                .eq_ignore_ascii_case(&disposition.reference_observation_id)
        }) {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DispositionInvalid,
                format!("{path}.reference_observation_id"),
                "outlier disposition observation does not exist in the metric reference dataset",
            ));
        }
        let id_key = normalized(&disposition.id);
        if by_id.insert(id_key.clone(), disposition).is_some() {
            return Err(CorrelationError::new(
                CorrelationErrorCode::DuplicateIdentity,
                format!("{path}.id"),
                "disposition event IDs must be unique",
            ));
        }
        let subject = (
            normalized(&disposition.metric_id),
            normalized(&disposition.reference_observation_id),
        );
        match (current.get(&subject), disposition.supersedes.as_deref()) {
            (None, None) => {}
            (Some(previous), Some(supersedes))
                if previous.id.eq_ignore_ascii_case(supersedes)
                    && disposition.decided_at_unix_ms > previous.decided_at_unix_ms => {}
            (Some(previous), Some(supersedes)) if previous.id.eq_ignore_ascii_case(supersedes) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.decided_at_unix_ms"),
                    "a superseding disposition must have a strictly later decision timestamp",
                ));
            }
            (None, Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "the superseded disposition is not an earlier event for this subject",
                ));
            }
            (Some(_), None) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "a later decision must explicitly supersede the current disposition",
                ));
            }
            (Some(_), Some(_)) => {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::DispositionInvalid,
                    format!("{path}.supersedes"),
                    "a later decision must supersede the immediately prior disposition",
                ));
            }
        }
        current.insert(subject, disposition);
    }
    Ok(())
}

fn current_dispositions(
    suite: &CorrelationSuite,
) -> CorrelationResult<BTreeMap<(String, String), &CorrelationOutlierDisposition>> {
    let latest = suite.latest_datasets()?;
    let metric_ids = suite
        .metrics
        .iter()
        .map(|metric| normalized(&metric.id))
        .collect();
    validate_disposition_ledger(suite, &metric_ids, &latest)?;
    let mut current = BTreeMap::new();
    for disposition in &suite.dispositions {
        current.insert(
            (
                normalized(&disposition.metric_id),
                normalized(&disposition.reference_observation_id),
            ),
            disposition,
        );
    }
    Ok(current)
}

fn evaluate_metric(
    metric: &CorrelationMetricDefinition,
    reference: &CorrelationDatasetRevision,
    simulation: &CorrelationDatasetRevision,
    dispositions: &BTreeMap<(String, String), &CorrelationOutlierDisposition>,
) -> CorrelationResult<CorrelationMetricOutcome> {
    let mut eligible = Vec::new();
    for observation in reference
        .observations
        .iter()
        .filter(|observation| observation.quantity.eq_ignore_ascii_case(&metric.quantity))
    {
        if observation_inside_domain(observation, metric.domain.as_ref())? {
            eligible.push(observation);
        }
    }
    eligible.sort_by_key(|observation| normalized(&observation.id));
    if eligible.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.quantity", metric.id),
            "metric selects no reference observations inside its declared domain",
        ));
    }
    let candidates = simulation
        .observations
        .iter()
        .filter(|observation| observation.quantity.eq_ignore_ascii_case(&metric.quantity))
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.simulation_dataset_id", metric.id),
            "simulation dataset has no observations for the metric quantity",
        ));
    }
    let alignment_index = AlignmentIndex::try_new(&candidates, &metric.alignment)?;

    let mut residuals = Vec::new();
    let mut excluded_points = 0;
    for observation in &eligible {
        let disposition = dispositions.get(&(normalized(&metric.id), normalized(&observation.id)));
        let excluded = disposition
            .is_some_and(|disposition| disposition.decision != CorrelationOutlierDecision::Retain);
        let aligned = align_observation(observation, &alignment_index)?;
        let (metric_error, metric_uncertainty) =
            metric_error(metric.calculation, observation, &aligned)?;
        let effective_limit =
            metric.limit.get() + metric.uncertainty_multiplier.get() * metric_uncertainty;
        if !effective_limit.is_finite() || effective_limit <= 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("metric.{}.limit", metric.id),
                "effective metric limit must remain finite and greater than zero",
            ));
        }
        let normalized_error = metric_error / effective_limit;
        if !normalized_error.is_finite() || normalized_error < 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::InvalidNumber,
                format!("metric.{}.normalized_error", metric.id),
                "metric produced a non-finite normalized residual",
            ));
        }
        residuals.push(CorrelationResidualPoint {
            id: format!("{}:{}", metric.id, observation.id),
            metric_id: metric.id.clone(),
            reference_observation_id: observation.id.clone(),
            reference_value: observation.value,
            simulated_value: finite("residual.simulated_value", aligned.value)?,
            simulation_observation_ids: aligned.observation_ids.clone(),
            alignment_evidence: aligned.evidence,
            metric_error: non_negative("residual.metric_error", metric_error)?,
            effective_limit: non_negative("residual.effective_limit", effective_limit)?,
            normalized_error: non_negative("residual.normalized_error", normalized_error)?,
            weight: non_negative("residual.weight", observation.weight.get() * aligned.weight)?,
            condition_group: condition_group_key(
                observation,
                match &metric.alignment {
                    CorrelationAlignmentPolicy::ExactOnly => None,
                    CorrelationAlignmentPolicy::MonotoneInterpolation { axis, .. } => {
                        Some(axis.as_str())
                    }
                },
            )?,
            excluded,
            exclusion_disposition_id: disposition
                .filter(|disposition| disposition.decision != CorrelationOutlierDecision::Retain)
                .map(|disposition| disposition.id.clone()),
        });
        excluded_points += usize::from(excluded);
    }
    if residuals.iter().all(|residual| residual.excluded) {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.residuals", metric.id),
            "every eligible observation is excluded; no numerical correlation result exists",
        ));
    }
    residuals.sort_by_key(|residual| normalized(&residual.reference_observation_id));
    let (aggregate_error, aggregate_normalized_error) =
        aggregate_residuals(&residuals, metric.calculation, metric.aggregation);
    let covered_points = residuals.len().saturating_sub(excluded_points);
    let coverage = covered_points as f64 / eligible.len() as f64;
    let passed = aggregate_normalized_error <= 1.0 && coverage >= metric.minimum_coverage.get();
    Ok(CorrelationMetricOutcome {
        metric_id: metric.id.clone(),
        release_role: metric.release_role,
        evaluated_points: covered_points,
        excluded_points,
        coverage: non_negative("metric_outcome.coverage", coverage)?,
        minimum_coverage: metric.minimum_coverage,
        aggregate_error: non_negative("metric_outcome.aggregate_error", aggregate_error)?,
        aggregate_normalized_error: non_negative(
            "metric_outcome.aggregate_normalized_error",
            aggregate_normalized_error,
        )?,
        passed,
        residuals,
    })
}

fn observation_inside_domain(
    observation: &CorrelationObservation,
    domain: Option<&CorrelationMetricDomain>,
) -> CorrelationResult<bool> {
    let Some(domain) = domain else {
        return Ok(true);
    };
    let coordinate = observation
        .coordinates
        .iter()
        .find(|coordinate| coordinate.dimension.eq_ignore_ascii_case(&domain.axis))
        .ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "metric domain axis '{}' is missing from a selected reference observation",
                    domain.axis
                ),
            )
        })?;
    let value = convert_value(coordinate.value.get(), &coordinate.unit, &domain.unit)?;
    Ok(value >= domain.minimum.get() && value <= domain.maximum.get())
}

#[derive(Debug, Clone)]
struct AlignedObservation {
    value: f64,
    uncertainty: f64,
    weight: f64,
    observation_ids: Vec<String>,
    evidence: CorrelationAlignmentEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CoordinateKeyPart {
    dimension: String,
    physical_dimension: UnitDimension,
    canonical_value_bits: u64,
}

type CoordinateKey = Vec<CoordinateKeyPart>;

#[derive(Debug, Clone, Copy)]
struct IndexedAxisPoint<'a> {
    axis_value: f64,
    observation: &'a CorrelationObservation,
}

#[derive(Debug)]
enum AlignmentIndex<'a> {
    Exact {
        dimensions: BTreeMap<String, UnitDimension>,
        groups: BTreeMap<CoordinateKey, Vec<&'a CorrelationObservation>>,
    },
    Monotone {
        axis: String,
        axis_dimension: UnitDimension,
        dimensions: BTreeMap<String, UnitDimension>,
        groups: BTreeMap<CoordinateKey, Vec<IndexedAxisPoint<'a>>>,
        extrapolation: CorrelationExtrapolationPolicy,
    },
}

impl<'a> AlignmentIndex<'a> {
    fn try_new(
        candidates: &[&'a CorrelationObservation],
        policy: &CorrelationAlignmentPolicy,
    ) -> CorrelationResult<Self> {
        let first = candidates.first().copied().ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                "alignment.candidates",
                "alignment requires at least one simulation observation",
            )
        })?;
        match policy {
            CorrelationAlignmentPolicy::ExactOnly => {
                let dimensions = coordinate_dimensions(first, None)?;
                let mut groups = BTreeMap::<CoordinateKey, Vec<&'a CorrelationObservation>>::new();
                for candidate in candidates {
                    let key = coordinate_key(candidate, None, &dimensions)?;
                    groups.entry(key).or_default().push(candidate);
                }
                Ok(Self::Exact { dimensions, groups })
            }
            CorrelationAlignmentPolicy::MonotoneInterpolation {
                axis,
                extrapolation,
            } => {
                let dimensions = coordinate_dimensions(first, Some(axis))?;
                let first_axis = coordinate(first, axis)?;
                let (axis_dimension, _) = canonical_coordinate(first, first_axis)?;
                let mut groups = BTreeMap::<CoordinateKey, Vec<IndexedAxisPoint<'a>>>::new();
                for candidate in candidates {
                    let key = coordinate_key(candidate, Some(axis), &dimensions)?;
                    let candidate_axis = coordinate(candidate, axis)?;
                    let (candidate_dimension, axis_value) =
                        canonical_coordinate(candidate, candidate_axis)?;
                    if candidate_dimension != axis_dimension {
                        return Err(CorrelationError::new(
                            CorrelationErrorCode::UnitMismatch,
                            format!("observation.{}.coordinates", candidate.id),
                            format!(
                                "alignment axis '{axis}' changes physical dimension across simulation observations"
                            ),
                        ));
                    }
                    groups.entry(key).or_default().push(IndexedAxisPoint {
                        axis_value,
                        observation: candidate,
                    });
                }
                for points in groups.values_mut() {
                    points.sort_by(|left, right| left.axis_value.total_cmp(&right.axis_value));
                    for pair in points.windows(2) {
                        if pair[0].axis_value >= pair[1].axis_value {
                            return Err(CorrelationError::new(
                                CorrelationErrorCode::AlignmentInvalid,
                                "alignment.candidates",
                                "candidate interpolation axes must be strictly increasing and unique within each condition group",
                            ));
                        }
                    }
                }
                Ok(Self::Monotone {
                    axis: axis.clone(),
                    axis_dimension,
                    dimensions,
                    groups,
                    extrapolation: *extrapolation,
                })
            }
        }
    }
}

fn align_observation(
    reference: &CorrelationObservation,
    index: &AlignmentIndex<'_>,
) -> CorrelationResult<AlignedObservation> {
    match index {
        AlignmentIndex::Exact { dimensions, groups } => {
            let key = coordinate_key(reference, None, dimensions)?;
            let matches = groups.get(&key).map_or(&[][..], Vec::as_slice);
            if matches.len() != 1 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    format!(
                        "exact alignment requires one candidate point; found {}",
                        matches.len()
                    ),
                ));
            }
            converted_candidate(reference, matches[0])
        }
        AlignmentIndex::Monotone {
            axis,
            axis_dimension,
            dimensions,
            groups,
            extrapolation,
        } => {
            let key = coordinate_key(reference, Some(axis), dimensions)?;
            let points = groups.get(&key).ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    "no simulation observations share the reference condition key",
                )
            })?;
            if points.len() < 2 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    "monotone interpolation requires at least two compatible candidate points",
                ));
            }
            let reference_axis = coordinate(reference, axis)?;
            let (reference_dimension, axis_value) =
                canonical_coordinate(reference, reference_axis)?;
            if reference_dimension != *axis_dimension {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::UnitMismatch,
                    format!("observation.{}.coordinates", reference.id),
                    format!(
                        "reference alignment axis '{axis}' has a different physical dimension from the simulation axis"
                    ),
                ));
            }
            match points.binary_search_by(|point| point.axis_value.total_cmp(&axis_value)) {
                Ok(index) => converted_candidate(reference, points[index].observation),
                Err(index) if index > 0 && index < points.len() => interpolate(
                    (
                        points[index - 1].axis_value,
                        converted_candidate(reference, points[index - 1].observation)?,
                    ),
                    (
                        points[index].axis_value,
                        converted_candidate(reference, points[index].observation)?,
                    ),
                    axis_value,
                    CorrelationAlignmentEvidence::Interpolated,
                ),
                Err(index) => {
                    let first = points.first().expect("two indexed points");
                    let last = points.last().expect("two indexed points");
                    let span = last.axis_value - first.axis_value;
                    let distance = if index == 0 {
                        first.axis_value - axis_value
                    } else {
                        axis_value - last.axis_value
                    };
                    match extrapolation {
                        CorrelationExtrapolationPolicy::Forbid => Err(CorrelationError::new(
                            CorrelationErrorCode::ExtrapolationForbidden,
                            format!("observation.{}", reference.id),
                            "reference coordinate lies outside the candidate domain and extrapolation is forbidden",
                        )),
                        CorrelationExtrapolationPolicy::Limited {
                            max_axis_span_fraction,
                        } if span > 0.0
                            && distance <= span * max_axis_span_fraction.get()
                            && max_axis_span_fraction.get() > 0.0 =>
                        {
                            let (left, right) = if index == 0 {
                                (&points[0], &points[1])
                            } else {
                                let length = points.len();
                                (&points[length - 2], &points[length - 1])
                            };
                            interpolate(
                                (
                                    left.axis_value,
                                    converted_candidate(reference, left.observation)?,
                                ),
                                (
                                    right.axis_value,
                                    converted_candidate(reference, right.observation)?,
                                ),
                                axis_value,
                                CorrelationAlignmentEvidence::Extrapolated,
                            )
                        }
                        CorrelationExtrapolationPolicy::Limited { .. } => {
                            Err(CorrelationError::new(
                                CorrelationErrorCode::ExtrapolationForbidden,
                                format!("observation.{}", reference.id),
                                "reference coordinate exceeds the declared limited-extrapolation envelope",
                            ))
                        }
                    }
                }
            }
        }
    }
}

fn converted_candidate(
    reference: &CorrelationObservation,
    candidate: &CorrelationObservation,
) -> CorrelationResult<AlignedObservation> {
    Ok(AlignedObservation {
        value: convert_value(candidate.value.get(), &candidate.unit, &reference.unit)?,
        uncertainty: convert_delta(
            candidate.uncertainty.get(),
            &candidate.unit,
            &reference.unit,
        )?,
        weight: candidate.weight.get(),
        observation_ids: vec![candidate.id.clone()],
        evidence: CorrelationAlignmentEvidence::Exact,
    })
}

fn interpolate(
    left: (f64, AlignedObservation),
    right: (f64, AlignedObservation),
    x: f64,
    evidence: CorrelationAlignmentEvidence,
) -> CorrelationResult<AlignedObservation> {
    let fraction = (x - left.0) / (right.0 - left.0);
    let lerp = |a: f64, b: f64| a + fraction * (b - a);
    let value = lerp(left.1.value, right.1.value);
    let (uncertainty, weight) = if evidence == CorrelationAlignmentEvidence::Extrapolated {
        // Extrapolation uses the absolute linear coefficients as a
        // worst-direction uncertainty bound. It can never cancel or shrink
        // below both endpoint uncertainties as an ordinary linear blend can.
        let left_coefficient = 1.0 - fraction;
        let right_coefficient = fraction;
        (
            left_coefficient.abs() * left.1.uncertainty
                + right_coefficient.abs() * right.1.uncertainty,
            left.1.weight.min(right.1.weight),
        )
    } else {
        (
            lerp(left.1.uncertainty, right.1.uncertainty).abs(),
            lerp(left.1.weight, right.1.weight),
        )
    };
    if !value.is_finite() || !uncertainty.is_finite() || !weight.is_finite() || weight <= 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "alignment",
            "interpolation produced a non-finite value, uncertainty, or weight",
        ));
    }
    Ok(AlignedObservation {
        value,
        uncertainty,
        weight,
        observation_ids: vec![
            left.1.observation_ids[0].clone(),
            right.1.observation_ids[0].clone(),
        ],
        evidence,
    })
}

fn coordinate<'a>(
    observation: &'a CorrelationObservation,
    dimension: &str,
) -> CorrelationResult<&'a CorrelationCoordinate> {
    observation
        .coordinates
        .iter()
        .find(|coordinate| coordinate.dimension.eq_ignore_ascii_case(dimension))
        .ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!("required alignment axis '{dimension}' is missing"),
            )
        })
}

fn coordinate_dimensions(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
) -> CorrelationResult<BTreeMap<String, UnitDimension>> {
    let mut dimensions = BTreeMap::new();
    for coordinate in observation.coordinates.iter().filter(|coordinate| {
        ignored_axis.is_none_or(|axis| !coordinate.dimension.eq_ignore_ascii_case(axis))
    }) {
        let name = normalized(&coordinate.dimension);
        let physical_dimension = unit_spec(&coordinate.unit)?.dimension;
        if dimensions
            .insert(name.clone(), physical_dimension)
            .is_some()
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!("condition dimension '{}' is repeated", coordinate.dimension),
            ));
        }
    }
    Ok(dimensions)
}

fn coordinate_key(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
    expected_dimensions: &BTreeMap<String, UnitDimension>,
) -> CorrelationResult<CoordinateKey> {
    let mut parts = Vec::with_capacity(expected_dimensions.len());
    for coordinate in observation.coordinates.iter().filter(|coordinate| {
        ignored_axis.is_none_or(|axis| !coordinate.dimension.eq_ignore_ascii_case(axis))
    }) {
        let dimension = normalized(&coordinate.dimension);
        let expected = expected_dimensions.get(&dimension).ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "condition dimension '{}' is not part of the indexed alignment key",
                    coordinate.dimension
                ),
            )
        })?;
        let (physical_dimension, canonical_value) = canonical_coordinate(observation, coordinate)?;
        if physical_dimension != *expected {
            return Err(CorrelationError::new(
                CorrelationErrorCode::UnitMismatch,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "condition dimension '{}' changes physical unit dimension",
                    coordinate.dimension
                ),
            ));
        }
        parts.push(CoordinateKeyPart {
            dimension,
            physical_dimension,
            canonical_value_bits: canonical_value.to_bits(),
        });
    }
    parts.sort();
    if parts.len() != expected_dimensions.len() {
        let present = parts
            .iter()
            .map(|part| part.dimension.as_str())
            .collect::<BTreeSet<_>>();
        let missing = expected_dimensions
            .keys()
            .filter(|dimension| !present.contains(dimension.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        return Err(CorrelationError::new(
            CorrelationErrorCode::AlignmentInvalid,
            format!("observation.{}.coordinates", observation.id),
            format!(
                "alignment key is missing required condition dimension(s): {}",
                missing.join(", ")
            ),
        ));
    }
    Ok(parts)
}

fn canonical_coordinate(
    observation: &CorrelationObservation,
    coordinate: &CorrelationCoordinate,
) -> CorrelationResult<(UnitDimension, f64)> {
    let unit = unit_spec(&coordinate.unit)?;
    let value = coordinate.value.get() * unit.scale + unit.bias;
    if !value.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("observation.{}.coordinates", observation.id),
            format!(
                "condition dimension '{}' cannot be converted to a finite canonical value",
                coordinate.dimension
            ),
        ));
    }
    Ok((unit.dimension, if value == 0.0 { 0.0 } else { value }))
}

fn condition_group_key(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
) -> CorrelationResult<String> {
    let dimensions = coordinate_dimensions(observation, ignored_axis)?;
    let key = coordinate_key(observation, ignored_axis, &dimensions)?;
    if key.is_empty() {
        Ok("all-conditions".to_owned())
    } else {
        let mut hasher = Sha256::new();
        for part in key {
            let length = u64::try_from(part.dimension.len()).map_err(|_| {
                CorrelationError::new(
                    CorrelationErrorCode::ResourceLimit,
                    "condition_group",
                    "condition dimension identity exceeds the supported range",
                )
            })?;
            hasher.update(length.to_le_bytes());
            hasher.update(part.dimension.as_bytes());
            hasher.update([part.physical_dimension.stable_tag()]);
            hasher.update(part.canonical_value_bits.to_le_bytes());
        }
        Ok(format!(
            "conditions:{}",
            ContentDigest::from_bytes(hasher.finalize().into())
        ))
    }
}

fn metric_error(
    calculation: CorrelationCalculation,
    reference: &CorrelationObservation,
    simulated: &AlignedObservation,
) -> CorrelationResult<(f64, f64)> {
    let reference_value = reference.value.get();
    let reference_uncertainty = reference.uncertainty.get();
    let rss = |left: f64, right: f64| left.hypot(right);
    match calculation {
        CorrelationCalculation::AbsoluteLinear => Ok((
            (simulated.value - reference_value).abs(),
            rss(reference_uncertainty, simulated.uncertainty),
        )),
        CorrelationCalculation::AbsoluteDecibels => {
            let unit = unit_spec(&reference.unit)?;
            if unit.dimension == UnitDimension::Decibel {
                Ok((
                    (simulated.value - reference_value).abs(),
                    rss(reference_uncertainty, simulated.uncertainty),
                ))
            } else {
                if reference_value == 0.0 || simulated.value == 0.0 {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::MetricInvalid,
                        format!("observation.{}", reference.id),
                        "decibel error from linear values requires non-zero magnitudes",
                    ));
                }
                let factor = if unit.dimension == UnitDimension::Power {
                    10.0
                } else {
                    20.0
                };
                let error =
                    (factor * (simulated.value.abs() / reference_value.abs()).log10()).abs();
                let uncertainty = factor / std::f64::consts::LN_10
                    * rss(
                        reference_uncertainty / reference_value.abs(),
                        simulated.uncertainty / simulated.value.abs(),
                    );
                Ok((error, uncertainty))
            }
        }
        CorrelationCalculation::Relative => {
            let scale = reference_value.abs().max(f64::MIN_POSITIVE);
            Ok((
                (simulated.value - reference_value).abs() / scale,
                rss(reference_uncertainty, simulated.uncertainty) / scale,
            ))
        }
        CorrelationCalculation::WeightedRelative => {
            let denominator = reference_value
                .abs()
                .max(simulated.value.abs())
                .max(f64::MIN_POSITIVE);
            Ok((
                (simulated.value - reference_value).abs() / denominator,
                rss(reference_uncertainty, simulated.uncertainty) / denominator,
            ))
        }
        CorrelationCalculation::PhaseWrappedDegrees => {
            let reference_degrees = convert_value(reference_value, &reference.unit, "deg")?;
            let simulated_degrees = convert_value(simulated.value, &reference.unit, "deg")?;
            let mut delta = (simulated_degrees - reference_degrees) % 360.0;
            if delta > 180.0 {
                delta -= 360.0;
            } else if delta < -180.0 {
                delta += 360.0;
            }
            let reference_uncertainty =
                convert_delta(reference_uncertainty, &reference.unit, "deg")?;
            let simulated_uncertainty =
                convert_delta(simulated.uncertainty, &reference.unit, "deg")?;
            Ok((
                delta.abs(),
                rss(reference_uncertainty, simulated_uncertainty),
            ))
        }
    }
}

fn aggregate_residuals(
    residuals: &[CorrelationResidualPoint],
    calculation: CorrelationCalculation,
    aggregation: CorrelationAggregation,
) -> (f64, f64) {
    let included = residuals
        .iter()
        .filter(|residual| !residual.excluded)
        .collect::<Vec<_>>();
    match aggregation {
        CorrelationAggregation::EveryPoint => included
            .iter()
            .map(|residual| (residual.metric_error.get(), residual.normalized_error.get()))
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .expect("non-empty residuals"),
        CorrelationAggregation::WorstCondition => {
            let mut groups = BTreeMap::<&str, Vec<&CorrelationResidualPoint>>::new();
            for residual in &included {
                groups
                    .entry(residual.condition_group.as_str())
                    .or_default()
                    .push(residual);
            }
            groups
                .values()
                .map(|group| rms_residuals(group, false))
                .max_by(|left, right| left.1.total_cmp(&right.1))
                .expect("non-empty residual groups")
        }
        CorrelationAggregation::Percentile95 => {
            let mut ranked = included
                .iter()
                .map(|residual| (residual.metric_error.get(), residual.normalized_error.get()))
                .collect::<Vec<_>>();
            ranked.sort_by(|left, right| left.1.total_cmp(&right.1));
            let rank = ((ranked.len() as f64 * 0.95).ceil() as usize).clamp(1, ranked.len());
            ranked[rank - 1]
        }
        CorrelationAggregation::RootMeanSquare => rms_residuals(
            &included,
            calculation == CorrelationCalculation::WeightedRelative,
        ),
    }
}

fn rms_residuals(residuals: &[&CorrelationResidualPoint], weighted: bool) -> (f64, f64) {
    let denominator = if weighted {
        residuals
            .iter()
            .map(|residual| residual.weight.get())
            .sum::<f64>()
    } else {
        residuals.len() as f64
    };
    let error_energy = residuals
        .iter()
        .map(|residual| {
            residual.metric_error.get().powi(2) * if weighted { residual.weight.get() } else { 1.0 }
        })
        .sum::<f64>();
    let normalized_energy = residuals
        .iter()
        .map(|residual| {
            residual.normalized_error.get().powi(2)
                * if weighted { residual.weight.get() } else { 1.0 }
        })
        .sum::<f64>();
    (
        (error_energy / denominator).sqrt(),
        (normalized_energy / denominator).sqrt(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum UnitDimension {
    Dimensionless,
    Voltage,
    Current,
    Resistance,
    Power,
    Frequency,
    Time,
    Temperature,
    Decibel,
    Angle,
    VoltageNoiseDensity,
    CurrentNoiseDensity,
}

impl UnitDimension {
    const fn stable_tag(self) -> u8 {
        match self {
            Self::Dimensionless => 0,
            Self::Voltage => 1,
            Self::Current => 2,
            Self::Resistance => 3,
            Self::Power => 4,
            Self::Frequency => 5,
            Self::Time => 6,
            Self::Temperature => 7,
            Self::Decibel => 8,
            Self::Angle => 9,
            Self::VoltageNoiseDensity => 10,
            Self::CurrentNoiseDensity => 11,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct UnitSpec {
    dimension: UnitDimension,
    scale: f64,
    bias: f64,
}

fn unit_spec(unit: &str) -> CorrelationResult<UnitSpec> {
    let normalized = normalized_unit(unit);
    let spec = match normalized.as_str() {
        "1" | "dimensionless" => UnitSpec::linear(UnitDimension::Dimensionless, 1.0),
        "v" | "volt" | "volts" => UnitSpec::linear(UnitDimension::Voltage, 1.0),
        "mv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-3),
        "uv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-6),
        "nv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-9),
        "v/sqrthz" | "v/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0),
        "mv/sqrthz" | "mv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-3),
        "uv/sqrthz" | "uv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-6),
        "nv/sqrthz" | "nv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-9),
        "a" | "amp" | "amps" | "ampere" | "amperes" => {
            UnitSpec::linear(UnitDimension::Current, 1.0)
        }
        "ma" => UnitSpec::linear(UnitDimension::Current, 1.0e-3),
        "ua" => UnitSpec::linear(UnitDimension::Current, 1.0e-6),
        "na" => UnitSpec::linear(UnitDimension::Current, 1.0e-9),
        "pa" => UnitSpec::linear(UnitDimension::Current, 1.0e-12),
        "a/sqrthz" | "a/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0),
        "ma/sqrthz" | "ma/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-3),
        "ua/sqrthz" | "ua/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-6),
        "na/sqrthz" | "na/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-9),
        "pa/sqrthz" | "pa/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-12),
        "ohm" | "ohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0),
        "kohm" | "kohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0e3),
        "mohm" | "mohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0e6),
        "w" | "watt" | "watts" => UnitSpec::linear(UnitDimension::Power, 1.0),
        "mw" => UnitSpec::linear(UnitDimension::Power, 1.0e-3),
        "hz" => UnitSpec::linear(UnitDimension::Frequency, 1.0),
        "khz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e3),
        "mhz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e6),
        "ghz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e9),
        "s" | "sec" | "second" | "seconds" => UnitSpec::linear(UnitDimension::Time, 1.0),
        "ms" => UnitSpec::linear(UnitDimension::Time, 1.0e-3),
        "us" => UnitSpec::linear(UnitDimension::Time, 1.0e-6),
        "ns" => UnitSpec::linear(UnitDimension::Time, 1.0e-9),
        "ps" => UnitSpec::linear(UnitDimension::Time, 1.0e-12),
        "k" | "kelvin" => UnitSpec::linear(UnitDimension::Temperature, 1.0),
        "degc" | "celsius" => UnitSpec {
            dimension: UnitDimension::Temperature,
            scale: 1.0,
            bias: 273.15,
        },
        "db" => UnitSpec::linear(UnitDimension::Decibel, 1.0),
        "deg" | "degree" | "degrees" => UnitSpec::linear(UnitDimension::Angle, 1.0),
        "rad" | "radian" | "radians" => {
            UnitSpec::linear(UnitDimension::Angle, 180.0 / std::f64::consts::PI)
        }
        _ => {
            return Err(CorrelationError::new(
                CorrelationErrorCode::UnitMismatch,
                "unit",
                format!("unsupported correlation unit '{unit}'"),
            ));
        }
    };
    Ok(spec)
}

impl UnitSpec {
    const fn linear(dimension: UnitDimension, scale: f64) -> Self {
        Self {
            dimension,
            scale,
            bias: 0.0,
        }
    }
}

fn convert_value(value: f64, from: &str, to: &str) -> CorrelationResult<f64> {
    let from = unit_spec(from)?;
    let to = unit_spec(to)?;
    if from.dimension != to.dimension {
        return Err(CorrelationError::new(
            CorrelationErrorCode::UnitMismatch,
            "unit",
            "correlation units describe different physical dimensions",
        ));
    }
    let converted = (value * from.scale + from.bias - to.bias) / to.scale;
    if !converted.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "unit",
            "unit conversion produced a non-finite value",
        ));
    }
    Ok(if converted == 0.0 { 0.0 } else { converted })
}

fn convert_delta(value: f64, from: &str, to: &str) -> CorrelationResult<f64> {
    let from = unit_spec(from)?;
    let to = unit_spec(to)?;
    if from.dimension != to.dimension {
        return Err(CorrelationError::new(
            CorrelationErrorCode::UnitMismatch,
            "unit",
            "correlation units describe different physical dimensions",
        ));
    }
    let converted = value * from.scale / to.scale;
    if !converted.is_finite() || converted < 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "unit",
            "uncertainty conversion produced an invalid value",
        ));
    }
    Ok(converted)
}

fn normalized_unit(unit: &str) -> String {
    unit.trim()
        .to_lowercase()
        .replace([' ', '_'], "")
        .replace('√', "sqrt")
        .replace(['µ', 'μ'], "u")
        .replace(['ω', 'Ω'], "ohm")
        .replace('°', "deg")
}

fn find_ci<'a>(
    datasets: &[&'a CorrelationDatasetRevision],
    id: &str,
) -> Option<&'a CorrelationDatasetRevision> {
    datasets
        .iter()
        .copied()
        .find(|dataset| dataset.id.eq_ignore_ascii_case(id))
}

fn require_schema(path: &str, value: u32) -> CorrelationResult<()> {
    if value == MODEL_CORRELATION_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::UnsupportedSchema,
            path,
            format!(
                "expected correlation schema {MODEL_CORRELATION_SCHEMA_VERSION}, received {value}"
            ),
        ))
    }
}

fn require_text(path: &str, value: &str) -> CorrelationResult<()> {
    if value.trim().is_empty() {
        Err(CorrelationError::new(
            CorrelationErrorCode::MissingValue,
            path,
            "value is required",
        ))
    } else {
        Ok(())
    }
}

fn bounded_text(path: &str, value: &str) -> CorrelationResult<()> {
    if value.len() <= MAX_CORRELATION_TEXT_BYTES {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("text is limited to {MAX_CORRELATION_TEXT_BYTES} UTF-8 bytes"),
        ))
    }
}

fn require_count(
    path: &str,
    count: usize,
    maximum: usize,
    description: &str,
) -> CorrelationResult<()> {
    if count <= maximum {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("{description} are limited to {maximum}; received {count}"),
        ))
    }
}

fn checked_count_add(
    path: &str,
    left: usize,
    right: usize,
    description: &str,
) -> CorrelationResult<usize> {
    left.checked_add(right).ok_or_else(|| {
        CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("{description} exceed the supported count range"),
        )
    })
}

fn finite(path: &str, value: f64) -> CorrelationResult<FiniteValue> {
    FiniteValue::new(value).map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            path,
            "value must be finite",
        )
    })
}

fn non_negative(path: &str, value: f64) -> CorrelationResult<NonNegativeFinite> {
    NonNegativeFinite::new(value).map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            path,
            "value must be finite and non-negative",
        )
    })
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ModelSourceId;

    fn source(byte: u8) -> ModelSourceEvidenceBinding {
        ModelSourceEvidenceBinding::try_new_project_bound(
            "dut",
            ModelSourceId::new(),
            ContentDigest::from_bytes([byte; 32]),
            ObjectRevision::INITIAL,
        )
        .expect("source fixture")
    }

    fn reference_csv() -> Vec<u8> {
        b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
r1,gain,0,dB,0.05,1,10,27\n\
r2,gain,-1,dB,0.05,1,100,27\n\
r3,gain,-2,dB,0.05,1,1000,27\n"
            .to_vec()
    }

    fn simulation_csv(offset: f64) -> Vec<u8> {
        format!(
            "id,quantity,value,unit,uncertainty,weight,condition:frequency[kHz],condition:temperature[K]\n\
s1,gain,{offset},dB,0.04,1,0.01,300.15\n\
s2,gain,{},dB,0.04,1,0.1,300.15\n\
s3,gain,{},dB,0.04,1,1,300.15\n",
            -1.0 + offset,
            -2.0 + offset,
        )
        .into_bytes()
    }

    fn dataset(
        id: &str,
        class: CorrelationDatasetClass,
        bytes: Vec<u8>,
        source: Option<ModelSourceEvidenceBinding>,
    ) -> CorrelationDatasetRevision {
        let provenance = class
            .is_simulation()
            .then(|| CorrelationSimulationProvenance {
                run_id: format!("{id}-run"),
                run_dataset_id: format!("{id}-run-dataset"),
                analysis_id: 1,
                analysis_result_digest: ContentDigest::from_bytes([0x43; 32]),
                plan_id: "correlation-plan".to_owned(),
                project_revision: ObjectRevision::INITIAL,
                prepared_snapshot_digest: ContentDigest::from_bytes([0x44; 32]),
                source_content_digest: ContentDigest::from_bytes([0x45; 32]),
                task_config_digest: ContentDigest::from_bytes([0x55; 32]),
                execution_target: "test-platform".to_owned(),
                export_digest: digest(&bytes),
                model_source: source.clone().expect("simulation source fixture"),
                executed_at_unix_ms: 1,
            });
        CorrelationDatasetRevision::try_from_csv_with_provenance(
            id,
            ObjectRevision::INITIAL,
            id,
            class,
            "qualified test authority",
            "lot-1",
            "fixture-1",
            "calibration-1",
            format!("{id}.csv"),
            bytes,
            source,
            provenance,
        )
        .expect("dataset fixture")
    }

    fn metric(
        aggregation: CorrelationAggregation,
        alignment: CorrelationAlignmentPolicy,
    ) -> CorrelationMetricDefinition {
        CorrelationMetricDefinition::try_new(
            "gain-error",
            "Gain error",
            "reference",
            "simulation",
            "gain",
            CorrelationCalculation::AbsoluteDecibels,
            None,
            0.25,
            1.0,
            0.5,
            aggregation,
            alignment,
            CorrelationReleaseRole::Review,
        )
        .expect("metric fixture")
    }

    fn suite_with(
        source: ModelSourceEvidenceBinding,
        simulation: Vec<u8>,
        metric: CorrelationMetricDefinition,
        dispositions: Vec<CorrelationOutlierDisposition>,
    ) -> CorrelationSuite {
        CorrelationSuite::try_new(
            "correlation",
            ObjectRevision::INITIAL,
            "DUT correlation",
            "model-owner",
            source.clone(),
            vec![
                dataset(
                    "reference",
                    CorrelationDatasetClass::BenchMeasurement,
                    reference_csv(),
                    None,
                ),
                dataset(
                    "simulation",
                    CorrelationDatasetClass::ModelSimulation,
                    simulation,
                    Some(source),
                ),
            ],
            vec![metric],
            dispositions,
        )
        .expect("suite fixture")
    }

    #[test]
    fn csv_import_is_bounded_utf8_bom_aware_and_content_addressed() {
        let mut csv = b"\xEF\xBB\xBF".to_vec();
        csv.extend(reference_csv());
        let dataset = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            csv.clone(),
            None,
        );
        assert_eq!(dataset.observations.len(), 3);
        assert_eq!(dataset.raw_digest, digest(&csv));
        assert_eq!(
            dataset.observations[0].coordinates[0].dimension,
            "frequency"
        );
        assert_eq!(
            convert_value(
                dataset.observations[0].coordinates[1].value.get(),
                "degC",
                "K",
            )
            .unwrap(),
            300.15
        );

        let invalid = CorrelationDatasetRevision::try_from_csv(
            "bad",
            ObjectRevision::INITIAL,
            "bad",
            CorrelationDatasetClass::BenchMeasurement,
            "authority",
            "lot",
            "fixture",
            "calibration",
            "bad.csv",
            b"id,quantity,value,unit\nx,q,NaN,V\n".to_vec(),
            None,
        )
        .unwrap_err();
        assert_eq!(invalid.code, CorrelationErrorCode::InvalidNumber);
    }

    #[test]
    fn dataset_validation_detects_raw_or_normalized_tampering() {
        let mut tampered_raw = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            reference_csv(),
            None,
        );
        tampered_raw.raw_source.push(b' ');
        assert_eq!(
            tampered_raw.validate("dataset").unwrap_err().code,
            CorrelationErrorCode::SourceDigestMismatch
        );

        let mut tampered_normalized = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            reference_csv(),
            None,
        );
        tampered_normalized.observations[0].value = FiniteValue::new(9.0).unwrap();
        assert_eq!(
            tampered_normalized.validate("dataset").unwrap_err().code,
            CorrelationErrorCode::SourceDigestMismatch
        );

        let mut tampered_provenance = dataset(
            "simulation",
            CorrelationDatasetClass::ModelSimulation,
            simulation_csv(0.0),
            Some(source(9)),
        );
        tampered_provenance
            .simulation_provenance
            .as_mut()
            .unwrap()
            .export_digest = ContentDigest::from_bytes([0x99; 32]);
        assert_eq!(
            tampered_provenance.validate("dataset").unwrap_err().code,
            CorrelationErrorCode::SourceDigestMismatch
        );
    }

    #[test]
    fn exact_alignment_converts_condition_units_and_evaluates_uncertainty() {
        let source = source(1);
        let suite = suite_with(
            source,
            simulation_csv(0.1),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).expect("evaluation");
        assert!(evaluation.passed);
        let outcome = &evaluation.metric_outcomes[0];
        assert_eq!(outcome.evaluated_points, 3);
        assert_eq!(outcome.excluded_points, 0);
        assert!(outcome.aggregate_error.get() > 0.09);
        assert!(outcome.aggregate_normalized_error.get() < 1.0);
    }

    #[test]
    fn absolute_linear_metrics_support_compound_noise_density_units() {
        let source = source(21);
        let reference = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            "id,quantity,value,unit,uncertainty,condition:frequency[Hz]\n\
r1,noise_density,5,nV/√Hz,0.1,1000\n"
                .as_bytes()
                .to_vec(),
            None,
        );
        let simulation = dataset(
            "simulation",
            CorrelationDatasetClass::ModelSimulation,
            "id,quantity,value,unit,uncertainty,condition:frequency[kHz]\n\
s1,noise_density,0.0051,uV/√Hz,0.00005,1\n"
                .as_bytes()
                .to_vec(),
            Some(source.clone()),
        );
        let metric = CorrelationMetricDefinition::try_new(
            "noise-density-error",
            "Noise density error",
            "reference",
            "simulation",
            "noise_density",
            CorrelationCalculation::AbsoluteLinear,
            None,
            0.5,
            1.0,
            1.0,
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
            CorrelationReleaseRole::Review,
        )
        .unwrap();
        let suite = CorrelationSuite::try_new(
            "noise-correlation",
            ObjectRevision::INITIAL,
            "Noise density correlation",
            "model-owner",
            source,
            vec![reference, simulation],
            vec![metric],
            Vec::new(),
        )
        .unwrap();
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        let outcome = &evaluation.metric_outcomes[0];
        assert!(evaluation.passed);
        assert!((outcome.aggregate_error.get() - 0.1).abs() < 1.0e-12);
        assert!(outcome.aggregate_normalized_error.get() < 1.0);
    }

    #[test]
    fn interpolation_is_deterministic_and_extrapolation_fails_closed() {
        let source_binding = source(2);
        let simulation = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
s1,gain,0,dB,0,1,10,27\n\
s2,gain,-2,dB,0,1,190,27\n\
s3,gain,-2,dB,0,1,1000,27\n"
            .to_vec();
        let suite = suite_with(
            source_binding,
            simulation,
            metric(
                CorrelationAggregation::WorstCondition,
                CorrelationAlignmentPolicy::MonotoneInterpolation {
                    axis: "frequency".to_owned(),
                    extrapolation: CorrelationExtrapolationPolicy::Forbid,
                },
            ),
            Vec::new(),
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).expect("interpolation");
        assert!(evaluation.passed);
        let middle = evaluation.metric_outcomes[0]
            .residuals
            .iter()
            .find(|point| point.reference_observation_id == "r2")
            .unwrap();
        assert!(middle.metric_error.get() < 0.1);

        let mut outside = suite.clone();
        outside.datasets[0] = dataset(
            "reference",
            CorrelationDatasetClass::BenchMeasurement,
            b"id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz],condition:temperature[degC]\n\
r0,gain,0,dB,0,1,1,27\n"
                .to_vec(),
            None,
        );
        assert_eq!(
            CorrelationEvaluation::evaluate(&outside).unwrap_err().code,
            CorrelationErrorCode::ExtrapolationForbidden
        );

        let mut missing_axis_metric = metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        );
        missing_axis_metric.domain = Some(CorrelationMetricDomain {
            axis: "supply".to_owned(),
            unit: "V".to_owned(),
            minimum: FiniteValue::new(0.0).unwrap(),
            maximum: FiniteValue::new(2.0).unwrap(),
        });
        let missing_axis = suite_with(
            source(22),
            simulation_csv(0.0),
            missing_axis_metric,
            Vec::new(),
        );
        assert_eq!(
            CorrelationEvaluation::evaluate(&missing_axis)
                .unwrap_err()
                .code,
            CorrelationErrorCode::MetricInvalid
        );
    }

    #[test]
    fn indexed_alignment_propagates_condition_unit_mismatches() {
        let simulation = b"id,quantity,value,unit,uncertainty,weight,condition:frequency[kHz],condition:temperature[V]\n\
s1,gain,0,dB,0.04,1,0.01,300.15\n\
s2,gain,-1,dB,0.04,1,0.1,300.15\n\
s3,gain,-2,dB,0.04,1,1,300.15\n"
            .to_vec();
        let suite = suite_with(
            source(23),
            simulation,
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        assert_eq!(
            CorrelationEvaluation::evaluate(&suite).unwrap_err().code,
            CorrelationErrorCode::UnitMismatch
        );
    }

    #[test]
    fn extrapolation_uses_a_conservative_uncertainty_bound() {
        let aligned = |id: &str, uncertainty: f64| AlignedObservation {
            value: 0.0,
            uncertainty,
            weight: 1.0,
            observation_ids: vec![id.to_owned()],
            evidence: CorrelationAlignmentEvidence::Exact,
        };
        let extrapolated = interpolate(
            (0.0, aligned("left", 1.0)),
            (1.0, aligned("right", 2.0)),
            -1.0,
            CorrelationAlignmentEvidence::Extrapolated,
        )
        .unwrap();
        assert_eq!(extrapolated.uncertainty, 4.0);
        assert_eq!(extrapolated.weight, 1.0);

        let interpolated = interpolate(
            (0.0, aligned("left", 1.0)),
            (1.0, aligned("right", 2.0)),
            0.5,
            CorrelationAlignmentEvidence::Interpolated,
        )
        .unwrap();
        assert_eq!(interpolated.uncertainty, 1.5);
    }

    #[test]
    fn phase_wrapping_and_nearest_rank_percentile_are_exact() {
        let reference = CorrelationObservation {
            id: "phase".to_owned(),
            quantity: "phase".to_owned(),
            value: FiniteValue::new(179.0).unwrap(),
            unit: "deg".to_owned(),
            uncertainty: NonNegativeFinite::new(0.0).unwrap(),
            weight: NonNegativeFinite::new(1.0).unwrap(),
            coordinates: Vec::new(),
        };
        let simulated = AlignedObservation {
            value: -179.0,
            uncertainty: 0.0,
            weight: 1.0,
            observation_ids: vec!["phase-simulation".to_owned()],
            evidence: CorrelationAlignmentEvidence::Exact,
        };
        let (error, _) = metric_error(
            CorrelationCalculation::PhaseWrappedDegrees,
            &reference,
            &simulated,
        )
        .unwrap();
        assert_eq!(error, 2.0);

        let residual = |id: usize, normalized: f64| CorrelationResidualPoint {
            id: id.to_string(),
            metric_id: "metric".to_owned(),
            reference_observation_id: id.to_string(),
            reference_value: FiniteValue::new(0.0).unwrap(),
            simulated_value: FiniteValue::new(0.0).unwrap(),
            simulation_observation_ids: vec![format!("sim-{id}")],
            alignment_evidence: CorrelationAlignmentEvidence::Exact,
            metric_error: NonNegativeFinite::new(normalized).unwrap(),
            effective_limit: NonNegativeFinite::new(1.0).unwrap(),
            normalized_error: NonNegativeFinite::new(normalized).unwrap(),
            weight: NonNegativeFinite::new(1.0).unwrap(),
            condition_group: "all-conditions".to_owned(),
            excluded: false,
            exclusion_disposition_id: None,
        };
        let residuals = (1..=20)
            .map(|value| residual(value, value as f64))
            .collect::<Vec<_>>();
        assert_eq!(
            aggregate_residuals(
                &residuals,
                CorrelationCalculation::Relative,
                CorrelationAggregation::Percentile95,
            ),
            (19.0, 19.0)
        );

        let mut unequal = vec![residual(21, 0.0), residual(22, 2.0)];
        let rms = aggregate_residuals(
            &unequal,
            CorrelationCalculation::Relative,
            CorrelationAggregation::RootMeanSquare,
        );
        assert!((rms.0 - 2.0_f64.sqrt()).abs() < 1.0e-12);
        assert_eq!(
            aggregate_residuals(
                &unequal,
                CorrelationCalculation::Relative,
                CorrelationAggregation::EveryPoint,
            ),
            (2.0, 2.0)
        );
        let worst_condition = aggregate_residuals(
            &unequal,
            CorrelationCalculation::Relative,
            CorrelationAggregation::WorstCondition,
        );
        assert!((worst_condition.0 - 2.0_f64.sqrt()).abs() < 1.0e-12);

        unequal[0].weight = NonNegativeFinite::new(1.0).unwrap();
        unequal[1].weight = NonNegativeFinite::new(3.0).unwrap();
        let weighted = aggregate_residuals(
            &unequal,
            CorrelationCalculation::WeightedRelative,
            CorrelationAggregation::RootMeanSquare,
        );
        assert!((weighted.0 - 3.0_f64.sqrt()).abs() < 1.0e-12);
    }

    #[test]
    fn outlier_decisions_are_append_only_reviewed_and_scope_visible() {
        let source = source(3);
        let first = CorrelationOutlierDisposition {
            id: "disp-1".to_owned(),
            metric_id: "gain-error".to_owned(),
            reference_observation_id: "r3".to_owned(),
            decision: CorrelationOutlierDecision::ExcludeFixtureFault,
            reason: "Independent fixture inspection found leakage.".to_owned(),
            owner_id: "model-owner".to_owned(),
            reviewer_id: "characterization-reviewer".to_owned(),
            decided_at_unix_ms: 1,
            supersedes: None,
        };
        let suite = suite_with(
            source,
            simulation_csv(0.1),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            vec![first.clone()],
        );
        let outcome = &CorrelationEvaluation::evaluate(&suite)
            .expect("reviewed exclusion")
            .metric_outcomes[0];
        assert_eq!(outcome.evaluated_points, 2);
        assert_eq!(outcome.excluded_points, 1);
        assert_eq!(outcome.residuals.len(), 3);
        assert!((outcome.coverage.get() - 2.0 / 3.0).abs() < 1.0e-12);
        let excluded = outcome
            .residuals
            .iter()
            .find(|residual| residual.reference_observation_id == "r3")
            .unwrap();
        assert!(excluded.excluded);
        assert_eq!(excluded.exclusion_disposition_id.as_deref(), Some("disp-1"));

        let mut invalid = suite.clone();
        invalid.dispositions.push(CorrelationOutlierDisposition {
            id: "disp-2".to_owned(),
            decision: CorrelationOutlierDecision::Retain,
            decided_at_unix_ms: 2,
            supersedes: None,
            ..first
        });
        assert_eq!(
            invalid.validate().unwrap_err().code,
            CorrelationErrorCode::DispositionInvalid
        );
    }

    #[test]
    fn declared_minimum_coverage_is_a_numerical_gate_and_is_revalidated() {
        let disposition = CorrelationOutlierDisposition {
            id: "coverage-exclusion".to_owned(),
            metric_id: "gain-error".to_owned(),
            reference_observation_id: "r3".to_owned(),
            decision: CorrelationOutlierDecision::ExcludeFixtureFault,
            reason: "Verified fixture fault.".to_owned(),
            owner_id: "model-owner".to_owned(),
            reviewer_id: "independent-reviewer".to_owned(),
            decided_at_unix_ms: 1,
            supersedes: None,
        };
        let mut gated_metric = metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        );
        gated_metric.minimum_coverage = NonNegativeFinite::new(0.75).unwrap();
        let suite = suite_with(
            source(33),
            simulation_csv(0.1),
            gated_metric,
            vec![disposition],
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        let outcome = &evaluation.metric_outcomes[0];
        assert!(outcome.aggregate_normalized_error.get() <= 1.0);
        assert!((outcome.coverage.get() - 2.0 / 3.0).abs() < 1.0e-12);
        assert_eq!(outcome.minimum_coverage.get(), 0.75);
        assert!(!outcome.passed);
        assert!(!evaluation.passed);

        let mut tampered = outcome.clone();
        tampered.passed = true;
        assert_eq!(
            validate_metric_outcomes(&[tampered], "tampered")
                .unwrap_err()
                .code,
            CorrelationErrorCode::EvidenceStale
        );
    }

    #[test]
    fn immutable_evidence_rejects_stale_suite_or_source() {
        let source = source(4);
        let suite = suite_with(
            source,
            simulation_csv(0.1),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        let evidence = CorrelationEvidence::try_new(
            "evidence-1",
            &evaluation,
            "independent-reviewer",
            CorrelationReviewDecision::Accept,
            "The retained comparison is accepted for model review.",
            10,
        )
        .unwrap();
        evidence.validate_current(&suite).unwrap();
        assert!(evidence.approved());
        assert_eq!(
            evidence.content_digest().unwrap(),
            evidence.content_digest().unwrap()
        );
        let self_review = CorrelationEvidence::try_new(
            "self-review",
            &evaluation,
            "model-owner",
            CorrelationReviewDecision::Accept,
            "The suite owner attempted to approve their own evidence.",
            9,
        )
        .unwrap();
        assert_eq!(
            self_review.validate_current(&suite).unwrap_err().code,
            CorrelationErrorCode::ReviewInvalid
        );

        let rejected = CorrelationEvidence::try_new(
            "evidence-rejected",
            &evaluation,
            "independent-reviewer",
            CorrelationReviewDecision::Reject,
            "Numerical gates pass, but the retained evidence is rejected pending fixture review.",
            11,
        )
        .unwrap();
        assert!(rejected.passed);
        assert!(!rejected.approved());

        let mut revised = suite.clone();
        revised.metrics[0].limit = NonNegativeFinite::new(0.2).unwrap();
        assert_eq!(
            evidence.validate_current(&revised).unwrap_err().code,
            CorrelationErrorCode::EvidenceStale
        );

        let mut tampered = evidence.clone();
        tampered.metric_outcomes[0].residuals[0].metric_error =
            NonNegativeFinite::new(0.0).unwrap();
        assert_eq!(
            tampered.validate_current(&suite).unwrap_err().code,
            CorrelationErrorCode::EvidenceStale
        );

        let orphan = ModelCorrelationState {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            suites: Vec::new(),
            evidence: vec![evidence],
        };
        assert_eq!(
            orphan.validate().unwrap_err().code,
            CorrelationErrorCode::EvidenceStale
        );
    }

    #[test]
    fn release_approval_gate_is_optional_until_configured_and_then_fails_closed() {
        let current_source = source(44);
        ModelCorrelationState::default()
            .require_release_approval("dut", &current_source)
            .unwrap();

        let suite = suite_with(
            current_source.clone(),
            simulation_csv(0.1),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        let mut state = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
        assert_eq!(
            state
                .require_release_approval("dut", &current_source)
                .unwrap_err()
                .code,
            CorrelationErrorCode::ReviewInvalid
        );

        let accepted = CorrelationEvidence::try_new(
            "accepted-review",
            &evaluation,
            "independent-reviewer",
            CorrelationReviewDecision::Accept,
            "Current retained evidence is accepted.",
            10,
        )
        .unwrap();
        state.add_evidence(accepted).unwrap();
        state
            .require_release_approval("dut", &current_source)
            .unwrap();

        let rejected = CorrelationEvidence::try_new(
            "later-rejection",
            &evaluation,
            "independent-reviewer",
            CorrelationReviewDecision::Reject,
            "A later review rejects the current retained evidence.",
            11,
        )
        .unwrap();
        state.add_evidence(rejected).unwrap();
        assert_eq!(
            state
                .require_release_approval("dut", &current_source)
                .unwrap_err()
                .code,
            CorrelationErrorCode::ReviewInvalid
        );
        assert_eq!(
            state
                .require_release_approval("dut", &source(45))
                .unwrap_err()
                .code,
            CorrelationErrorCode::SourceBindingMismatch
        );
    }

    #[test]
    fn reviewer_acceptance_cannot_override_failed_numerical_gates() {
        let mut failing_metric = metric(
            CorrelationAggregation::EveryPoint,
            CorrelationAlignmentPolicy::ExactOnly,
        );
        failing_metric.limit = NonNegativeFinite::new(0.01).unwrap();
        let suite = suite_with(source(34), simulation_csv(0.1), failing_metric, Vec::new());
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        assert!(!evaluation.passed);
        assert_eq!(
            CorrelationEvidence::try_new(
                "invalid-acceptance",
                &evaluation,
                "reviewer",
                CorrelationReviewDecision::Accept,
                "Attempted acceptance.",
                1,
            )
            .unwrap_err()
            .code,
            CorrelationErrorCode::ReviewInvalid
        );
        let rejected = CorrelationEvidence::try_new(
            "valid-rejection",
            &evaluation,
            "reviewer",
            CorrelationReviewDecision::Reject,
            "Numerical gates failed.",
            1,
        )
        .unwrap();
        assert!(!rejected.passed);
        assert!(!rejected.approved());
    }

    #[test]
    fn advisory_failures_remain_visible_without_failing_the_review_gate() {
        let source = source(31);
        let mut suite = suite_with(
            source,
            simulation_csv(0.1),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let mut advisory = suite.metrics[0].clone();
        advisory.id = "advisory-gain".to_owned();
        advisory.name = "Advisory gain".to_owned();
        advisory.limit = NonNegativeFinite::new(0.01).unwrap();
        advisory.release_role = CorrelationReleaseRole::Advisory;
        suite.metrics.push(advisory);
        suite.validate().unwrap();

        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        assert!(evaluation.passed);
        assert!(
            evaluation
                .metric_outcomes
                .iter()
                .any(
                    |outcome| outcome.release_role == CorrelationReleaseRole::Advisory
                        && !outcome.passed
                )
        );

        suite.metrics[0].limit = NonNegativeFinite::new(0.01).unwrap();
        assert!(!CorrelationEvaluation::evaluate(&suite).unwrap().passed);
    }

    #[test]
    fn a_dataset_only_suite_is_a_valid_draft_but_cannot_produce_evidence() {
        let source = source(32);
        let suite = CorrelationSuite::try_new(
            "draft",
            ObjectRevision::INITIAL,
            "Dataset import draft",
            "model-owner",
            source,
            vec![dataset(
                "reference",
                CorrelationDatasetClass::BenchMeasurement,
                reference_csv(),
                None,
            )],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        assert_eq!(
            CorrelationEvaluation::evaluate(&suite).unwrap_err().code,
            CorrelationErrorCode::MissingValue
        );
    }

    #[test]
    fn aggregate_suite_and_state_collections_are_bounded() {
        let suite = suite_with(
            source(35),
            simulation_csv(0.0),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let mut oversized_suite = suite.clone();
        oversized_suite.metrics = (0..=MAX_CORRELATION_METRICS)
            .map(|index| {
                let mut metric = oversized_suite.metrics[0].clone();
                metric.id = format!("metric-{index}");
                metric.name = format!("Metric {index}");
                metric
            })
            .collect();
        assert_eq!(
            oversized_suite.validate().unwrap_err().code,
            CorrelationErrorCode::ResourceLimit
        );

        let oversized_state = ModelCorrelationState {
            schema_version: MODEL_CORRELATION_SCHEMA_VERSION,
            suites: vec![suite; MAX_CORRELATION_SUITE_REVISIONS + 1],
            evidence: Vec::new(),
        };
        assert_eq!(
            oversized_state.validate().unwrap_err().code,
            CorrelationErrorCode::ResourceLimit
        );
    }

    #[test]
    fn state_and_serde_fail_closed_without_rewriting_history() {
        let source = source(5);
        let suite = suite_with(
            source,
            simulation_csv(0.0),
            metric(
                CorrelationAggregation::EveryPoint,
                CorrelationAlignmentPolicy::ExactOnly,
            ),
            Vec::new(),
        );
        let evaluation = CorrelationEvaluation::evaluate(&suite).unwrap();
        let evidence = CorrelationEvidence::try_new(
            "evidence",
            &evaluation,
            "reviewer",
            CorrelationReviewDecision::Accept,
            "Accepted exact retained evidence.",
            12,
        )
        .unwrap();
        let mut state = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
        state.add_evidence(evidence.clone()).unwrap();
        assert_eq!(
            state.add_evidence(evidence).unwrap_err().code,
            CorrelationErrorCode::ImmutableRecord
        );

        let mut json = serde_json::to_value(&state).unwrap();
        json.as_object_mut()
            .unwrap()
            .insert("unknown".to_owned(), serde_json::json!(true));
        assert!(serde_json::from_value::<ModelCorrelationState>(json).is_err());
        let mut json = serde_json::to_value(&state).unwrap();
        json["schema_version"] = serde_json::json!(99);
        let restored = serde_json::from_value::<ModelCorrelationState>(json).unwrap();
        assert_eq!(
            restored.validate().unwrap_err().code,
            CorrelationErrorCode::UnsupportedSchema
        );
    }
}
