//! Versioned, source-bound measurement-correlation contracts.
//!
//! This module owns imported reference/simulation datasets, explicit
//! alignment and metric definitions, governed outlier decisions, deterministic
//! evaluation, and immutable reviewer evidence. It contains no egui code and
//! makes no instrument, network, or release-authority claims.

use std::collections::{BTreeMap, BTreeSet};
mod ingest;
mod metrics;

use ingest::*;
use metrics::*;

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

#[cfg(test)]
mod tests;
