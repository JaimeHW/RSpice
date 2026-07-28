//! Versioned model-qualification and model-release contracts.
//!
//! This module is intentionally independent of egui.  It is the domain model
//! behind the Model Editor's existing **Tests** and **Release** surfaces:
//! golden vectors, Desktop/WebAssembly parity, immutable source-bound evidence,
//! release declarations, and fail-closed promotion.  A failed qualification is
//! valid evidence (and remains inspectable); it is never release-eligible.

mod promotion;

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
#[serde(deny_unknown_fields)]
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

/// Analysis that the qualification runner must execute for one vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QualificationAnalysis {
    DcOperatingPoint,
    DcSweep {
        source: String,
        start: FiniteValue,
        stop: FiniteValue,
        step: FiniteValue,
    },
    /// Small-signal AC/C-V qualification at an exact, explicitly retained
    /// frequency axis. Explicit points avoid target-dependent sweep expansion
    /// and make Desktop/WebAssembly evidence directly comparable.
    AcSweep {
        frequencies: Vec<FiniteValue>,
    },
    /// Input- and output-referred small-signal noise qualification. Named
    /// nodes and the independent input source are resolved by the same
    /// canonical topology used by the noise solver.
    Noise {
        output_node: String,
        output_reference: Option<String>,
        input_source: String,
        frequencies: Vec<FiniteValue>,
        temperature_kelvin: FiniteValue,
    },
    /// Time-domain qualification using the production transient integrator.
    Transient {
        stop_time: FiniteValue,
        max_step: FiniteValue,
    },
}

impl QualificationAnalysis {
    fn validate(&self, path: &str) -> QualificationResult<()> {
        match self {
            Self::DcOperatingPoint => Ok(()),
            Self::DcSweep {
                source,
                start,
                stop,
                step,
            } => {
                require_text(&format!("{path}.source"), source)?;
                let start = start.get();
                let stop = stop.get();
                let step = step.get();
                if step == 0.0 || (stop > start && step < 0.0) || (stop < start && step > 0.0) {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::InvalidExecutionDefinition,
                        format!("{path}.step"),
                        "DC sweep step must be non-zero and advance from start toward stop",
                    ));
                }
                Ok(())
            }
            Self::AcSweep { frequencies } => {
                validate_frequency_axis(&format!("{path}.frequencies"), frequencies)
            }
            Self::Noise {
                output_node,
                output_reference,
                input_source,
                frequencies,
                temperature_kelvin,
            } => {
                require_text(&format!("{path}.output_node"), output_node)?;
                if let Some(reference) = output_reference {
                    require_text(&format!("{path}.output_reference"), reference)?;
                    if reference.eq_ignore_ascii_case(output_node) {
                        return Err(QualificationValidationError::new(
                            QualificationErrorCode::InvalidExecutionDefinition,
                            format!("{path}.output_reference"),
                            "noise output and reference nodes must be distinct",
                        ));
                    }
                }
                require_text(&format!("{path}.input_source"), input_source)?;
                validate_frequency_axis(&format!("{path}.frequencies"), frequencies)?;
                if temperature_kelvin.get() <= 0.0 {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::InvalidExecutionDefinition,
                        format!("{path}.temperature_kelvin"),
                        "noise temperature must be greater than zero kelvin",
                    ));
                }
                Ok(())
            }
            Self::Transient {
                stop_time,
                max_step,
            } => {
                let stop_time = stop_time.get();
                let max_step = max_step.get();
                if stop_time <= 0.0 || max_step <= 0.0 || max_step > stop_time {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::InvalidExecutionDefinition,
                        path,
                        "transient stop time and maximum step must be positive, with maximum step no greater than stop time",
                    ));
                }
                Ok(())
            }
        }
    }
}

fn validate_frequency_axis(path: &str, frequencies: &[FiniteValue]) -> QualificationResult<()> {
    if frequencies.is_empty() {
        return Err(missing(
            path,
            "AC and noise qualification require at least one exact frequency point",
        ));
    }
    let mut previous = None;
    for (index, frequency) in frequencies.iter().enumerate() {
        let frequency = frequency.get();
        if frequency <= 0.0 {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                format!("{path}[{index}]"),
                "frequency must be greater than zero",
            ));
        }
        if previous.is_some_and(|previous| frequency <= previous) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                format!("{path}[{index}]"),
                "frequency points must be strictly increasing and unique",
            ));
        }
        previous = Some(frequency);
    }
    Ok(())
}

/// Solver result channel sampled by a qualification output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QualificationProbe {
    NodeVoltage {
        node: String,
    },
    BranchCurrent {
        branch: String,
    },
    DcObservable {
        expression: String,
    },
    SweepValue,
    AcNodeVoltageMagnitude {
        node: String,
    },
    AcNodeVoltagePhaseDegrees {
        node: String,
    },
    AcNodeVoltageReal {
        node: String,
    },
    AcNodeVoltageImaginary {
        node: String,
    },
    AcBranchCurrentMagnitude {
        branch: String,
    },
    AcBranchCurrentPhaseDegrees {
        branch: String,
    },
    AcBranchCurrentReal {
        branch: String,
    },
    AcBranchCurrentImaginary {
        branch: String,
    },
    /// Effective small-signal capacitance, `Im(I)/(2*pi*f*Vac)`, preserving
    /// branch-current orientation and therefore its sign.
    AcEffectiveCapacitance {
        branch: String,
        excitation_magnitude: FiniteValue,
    },
    FrequencyValue,
    NoiseOutputDensity,
    NoiseInputReferredDensity,
    NoiseOutputAmplitude,
    NoiseInputReferredAmplitude,
    TransientNodeVoltage {
        node: String,
    },
    TransientBranchCurrent {
        branch: String,
    },
    TimeValue,
}

impl QualificationProbe {
    fn validate(&self, path: &str) -> QualificationResult<()> {
        match self {
            Self::NodeVoltage { node } => require_text(&format!("{path}.node"), node),
            Self::BranchCurrent { branch } => require_text(&format!("{path}.branch"), branch),
            Self::DcObservable { expression } => {
                require_text(&format!("{path}.expression"), expression)
            }
            Self::AcNodeVoltageMagnitude { node }
            | Self::AcNodeVoltagePhaseDegrees { node }
            | Self::AcNodeVoltageReal { node }
            | Self::AcNodeVoltageImaginary { node }
            | Self::TransientNodeVoltage { node } => require_text(&format!("{path}.node"), node),
            Self::AcBranchCurrentMagnitude { branch }
            | Self::AcBranchCurrentPhaseDegrees { branch }
            | Self::AcBranchCurrentReal { branch }
            | Self::AcBranchCurrentImaginary { branch }
            | Self::TransientBranchCurrent { branch } => {
                require_text(&format!("{path}.branch"), branch)
            }
            Self::AcEffectiveCapacitance {
                branch,
                excitation_magnitude,
            } => {
                require_text(&format!("{path}.branch"), branch)?;
                if excitation_magnitude.get() <= 0.0 {
                    return Err(QualificationValidationError::new(
                        QualificationErrorCode::InvalidExecutionDefinition,
                        format!("{path}.excitation_magnitude"),
                        "effective-capacitance excitation magnitude must be greater than zero",
                    ));
                }
                Ok(())
            }
            Self::SweepValue
            | Self::FrequencyValue
            | Self::NoiseOutputDensity
            | Self::NoiseInputReferredDensity
            | Self::NoiseOutputAmplitude
            | Self::NoiseInputReferredAmplitude
            | Self::TimeValue => Ok(()),
        }
    }
}

/// Exact result point selected from the declared analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum QualificationSample {
    OperatingPoint,
    FirstSweepPoint,
    LastSweepPoint,
    SweepPoint { index: usize },
    FirstFrequencyPoint,
    LastFrequencyPoint,
    FrequencyPoint { index: usize },
    FirstTimePoint,
    LastTimePoint,
    TimePoint { index: usize },
}

/// One named value extracted from a simulation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationOutputDefinition {
    pub quantity: String,
    pub probe: QualificationProbe,
    pub sample: QualificationSample,
}

impl QualificationOutputDefinition {
    pub fn try_new(
        quantity: impl Into<String>,
        probe: QualificationProbe,
        sample: QualificationSample,
    ) -> QualificationResult<Self> {
        let value = Self {
            quantity: quantity.into(),
            probe,
            sample,
        };
        value.validate("qualification_output")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.quantity"), &self.quantity)?;
        self.probe.validate(&format!("{path}.probe"))
    }
}

/// Golden value and acceptance tolerance for one declared output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationReference {
    pub quantity: String,
    pub expected: FiniteValue,
    pub absolute_tolerance: NonNegativeFinite,
    pub relative_tolerance: NonNegativeFinite,
}

impl QualificationReference {
    pub fn try_new(
        quantity: impl Into<String>,
        expected: f64,
        absolute_tolerance: f64,
        relative_tolerance: f64,
    ) -> QualificationResult<Self> {
        let value = Self {
            quantity: quantity.into(),
            expected: FiniteValue::new(expected)
                .map_err(|error| at_path(error, "qualification_reference.expected"))?,
            absolute_tolerance: NonNegativeFinite::new(absolute_tolerance)
                .map_err(|error| at_path(error, "qualification_reference.absolute_tolerance"))?,
            relative_tolerance: NonNegativeFinite::new(relative_tolerance)
                .map_err(|error| at_path(error, "qualification_reference.relative_tolerance"))?,
        };
        value.validate("qualification_reference")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.quantity"), &self.quantity)
    }
}

/// One immutable, replayable test input and its complete execution contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationVector {
    pub id: String,
    pub name: String,
    /// Exact source revision incorporated into `executable_input`.
    pub source: ModelSourceEvidenceBinding,
    /// Exact canonical project-owned model source incorporated into the
    /// executable testbench. Its digest must equal `source.source_digest`, and
    /// the parsed testbench must instantiate `source.model_id`.
    #[serde(default)]
    pub model_source: Vec<u8>,
    /// Optional named `.lib` section selected for this vector. `None` executes
    /// the top-level base card. A named section must resolve exactly in
    /// `model_source`; source-wide evidence cannot qualify a section by proxy.
    #[serde(default)]
    pub model_section: Option<String>,
    /// Exact standalone model card extracted from the selected base/section
    /// of `model_source` and incorporated into `executable_input`.
    #[serde(default)]
    pub execution_model_source: Vec<u8>,
    /// Exact bytes passed to the SPICE parser. No include or source content is
    /// reconstructed by the qualification runner.
    pub executable_input: Vec<u8>,
    pub input_digest: ContentDigest,
    pub analysis: QualificationAnalysis,
    pub outputs: Vec<QualificationOutputDefinition>,
    pub references: Vec<QualificationReference>,
}

impl QualificationVector {
    /// Legacy compatibility entry point. A caller that supplies no separate
    /// candidate-source snapshot cannot create current qualification evidence.
    /// Use [`Self::try_new_source_bound`] for all new authoring paths.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        id: impl Into<String>,
        name: impl Into<String>,
        source: ModelSourceEvidenceBinding,
        executable_input: Vec<u8>,
        analysis: QualificationAnalysis,
        outputs: Vec<QualificationOutputDefinition>,
        references: Vec<QualificationReference>,
    ) -> QualificationResult<Self> {
        Self::try_new_source_bound(
            id,
            name,
            source,
            Vec::new(),
            executable_input,
            analysis,
            outputs,
            references,
        )
    }

    /// Construct an executable vector from separately retained canonical model
    /// bytes and the complete testbench bytes. Validation proves that the
    /// canonical bytes match the exact project source identity, occur in the
    /// testbench, parse as the named model, and are actually instantiated.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_source_bound(
        id: impl Into<String>,
        name: impl Into<String>,
        source: ModelSourceEvidenceBinding,
        model_source: Vec<u8>,
        executable_input: Vec<u8>,
        analysis: QualificationAnalysis,
        outputs: Vec<QualificationOutputDefinition>,
        references: Vec<QualificationReference>,
    ) -> QualificationResult<Self> {
        let execution_model_source = model_source.clone();
        Self::try_new_source_section_bound(
            id,
            name,
            source,
            model_source,
            None,
            execution_model_source,
            executable_input,
            analysis,
            outputs,
            references,
        )
    }

    /// Construct a vector that proves and executes one exact base or named
    /// section from the retained canonical model source.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_source_section_bound(
        id: impl Into<String>,
        name: impl Into<String>,
        source: ModelSourceEvidenceBinding,
        model_source: Vec<u8>,
        model_section: Option<String>,
        execution_model_source: Vec<u8>,
        executable_input: Vec<u8>,
        analysis: QualificationAnalysis,
        mut outputs: Vec<QualificationOutputDefinition>,
        mut references: Vec<QualificationReference>,
    ) -> QualificationResult<Self> {
        outputs.sort_by(|left, right| normalized(&left.quantity).cmp(&normalized(&right.quantity)));
        references
            .sort_by(|left, right| normalized(&left.quantity).cmp(&normalized(&right.quantity)));
        let input_digest = digest_bytes(&executable_input);
        let value = Self {
            id: id.into(),
            name: name.into(),
            source,
            model_source,
            model_section,
            execution_model_source,
            executable_input,
            input_digest,
            analysis,
            outputs,
            references,
        };
        value.validate("vector")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.id"), &self.id)?;
        require_text(&format!("{path}.name"), &self.name)?;
        self.source.validate(&format!("{path}.source"))?;
        self.source
            .require_project_bound(&format!("{path}.source"))?;
        if let Some(section) = &self.model_section {
            require_text(&format!("{path}.model_section"), section)?;
        }
        validate_vector_model_source(self, path)?;
        if self.executable_input.is_empty() {
            return Err(missing(
                format!("{path}.executable_input"),
                "a qualification vector requires exact executable input bytes",
            ));
        }
        if digest_bytes(&self.executable_input) != self.input_digest {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InputDigestMismatch,
                format!("{path}.input_digest"),
                "retained executable input does not match its SHA-256 digest",
            ));
        }
        self.analysis.validate(&format!("{path}.analysis"))?;
        if self.outputs.is_empty() {
            return Err(missing(
                format!("{path}.outputs"),
                "a qualification vector requires at least one declared output",
            ));
        }
        if self.references.is_empty() {
            return Err(missing(
                format!("{path}.references"),
                "a qualification vector requires at least one golden reference",
            ));
        }
        if self.outputs.len() != self.references.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                format!("{path}.references"),
                "every declared output requires exactly one golden reference",
            ));
        }
        for (index, output) in self.outputs.iter().enumerate() {
            output.validate(&format!("{path}.outputs[{index}]"))?;
            if !probe_is_compatible(&self.analysis, &output.probe) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::InvalidExecutionDefinition,
                    format!("{path}.outputs[{index}].probe"),
                    "output probe is incompatible with the declared analysis",
                ));
            }
            if !sample_is_compatible(&self.analysis, output.sample) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::InvalidExecutionDefinition,
                    format!("{path}.outputs[{index}].sample"),
                    "output sample is incompatible with the declared analysis",
                ));
            }
        }
        for (index, reference) in self.references.iter().enumerate() {
            reference.validate(&format!("{path}.references[{index}]"))?;
        }
        ensure_unique(
            &format!("{path}.outputs"),
            self.outputs.iter().map(|value| value.quantity.as_str()),
            QualificationErrorCode::DuplicateName,
            "output quantity",
        )?;
        ensure_unique(
            &format!("{path}.references"),
            self.references.iter().map(|value| value.quantity.as_str()),
            QualificationErrorCode::DuplicateName,
            "reference quantity",
        )?;
        for output in &self.outputs {
            if find_ci(&self.references, &output.quantity, |value| &value.quantity).is_none() {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::InvalidExecutionDefinition,
                    format!("{path}.references"),
                    format!("missing golden reference for output {:?}", output.quantity),
                ));
            }
        }
        Ok(())
    }

    fn validate_source_binding(
        &self,
        expected_source: &ModelSourceEvidenceBinding,
        path: &str,
    ) -> QualificationResult<()> {
        if &self.source != expected_source {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                path,
                "qualification input was not retained from the expected model source revision",
            ));
        }
        Ok(())
    }
}

fn probe_is_compatible(analysis: &QualificationAnalysis, probe: &QualificationProbe) -> bool {
    match analysis {
        QualificationAnalysis::DcOperatingPoint => matches!(
            probe,
            QualificationProbe::NodeVoltage { .. }
                | QualificationProbe::BranchCurrent { .. }
                | QualificationProbe::DcObservable { .. }
        ),
        QualificationAnalysis::DcSweep { .. } => matches!(
            probe,
            QualificationProbe::NodeVoltage { .. }
                | QualificationProbe::BranchCurrent { .. }
                | QualificationProbe::DcObservable { .. }
                | QualificationProbe::SweepValue
        ),
        QualificationAnalysis::AcSweep { .. } => matches!(
            probe,
            QualificationProbe::AcNodeVoltageMagnitude { .. }
                | QualificationProbe::AcNodeVoltagePhaseDegrees { .. }
                | QualificationProbe::AcNodeVoltageReal { .. }
                | QualificationProbe::AcNodeVoltageImaginary { .. }
                | QualificationProbe::AcBranchCurrentMagnitude { .. }
                | QualificationProbe::AcBranchCurrentPhaseDegrees { .. }
                | QualificationProbe::AcBranchCurrentReal { .. }
                | QualificationProbe::AcBranchCurrentImaginary { .. }
                | QualificationProbe::AcEffectiveCapacitance { .. }
                | QualificationProbe::FrequencyValue
        ),
        QualificationAnalysis::Noise { .. } => matches!(
            probe,
            QualificationProbe::FrequencyValue
                | QualificationProbe::NoiseOutputDensity
                | QualificationProbe::NoiseInputReferredDensity
                | QualificationProbe::NoiseOutputAmplitude
                | QualificationProbe::NoiseInputReferredAmplitude
        ),
        QualificationAnalysis::Transient { .. } => matches!(
            probe,
            QualificationProbe::TransientNodeVoltage { .. }
                | QualificationProbe::TransientBranchCurrent { .. }
                | QualificationProbe::TimeValue
        ),
    }
}

fn sample_is_compatible(analysis: &QualificationAnalysis, sample: QualificationSample) -> bool {
    match analysis {
        QualificationAnalysis::DcOperatingPoint => {
            matches!(sample, QualificationSample::OperatingPoint)
        }
        QualificationAnalysis::DcSweep { .. } => matches!(
            sample,
            QualificationSample::FirstSweepPoint
                | QualificationSample::LastSweepPoint
                | QualificationSample::SweepPoint { .. }
        ),
        QualificationAnalysis::AcSweep { .. } | QualificationAnalysis::Noise { .. } => matches!(
            sample,
            QualificationSample::FirstFrequencyPoint
                | QualificationSample::LastFrequencyPoint
                | QualificationSample::FrequencyPoint { .. }
        ),
        QualificationAnalysis::Transient { .. } => matches!(
            sample,
            QualificationSample::FirstTimePoint
                | QualificationSample::LastTimePoint
                | QualificationSample::TimePoint { .. }
        ),
    }
}

fn validate_vector_model_source(
    vector: &QualificationVector,
    path: &str,
) -> QualificationResult<()> {
    if vector.model_source.is_empty() {
        return Err(missing(
            format!("{path}.model_source"),
            "exact canonical candidate-model source bytes are required",
        ));
    }
    if digest_bytes(&vector.model_source) != vector.source.source_digest {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::SourceBindingMismatch,
            format!("{path}.model_source"),
            "retained candidate-model bytes do not match the bound source digest",
        ));
    }
    let model_source =
        rspice_core::netlist::decode_source_bytes(&vector.model_source).map_err(|error| {
            QualificationValidationError::new(
                QualificationErrorCode::InvalidExecutionDefinition,
                format!("{path}.model_source"),
                format!("retained candidate-model source cannot be decoded: {error}"),
            )
        })?;
    let mut library_parser = rspice_core::library::LibParser::new(Path::new("."));
    let parsed_library = library_parser.parse_string(&model_source);
    let model_names = parsed_library
        .top_level_models
        .iter()
        .map(|model| model.name.as_str())
        .chain(
            parsed_library
                .sections
                .iter()
                .flat_map(|section| section.models.iter().map(|model| model.name.as_str())),
        )
        .collect::<Vec<_>>();
    if !parsed_library.is_ok()
        || model_names.is_empty()
        || model_names
            .iter()
            .any(|name| !name.eq_ignore_ascii_case(&vector.source.model_id))
    {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::InvalidExecutionDefinition,
            format!("{path}.model_source"),
            "retained candidate-model source must parse as only the bound model identity",
        ));
    }

    let expected_execution_source =
        selected_execution_model_source(&model_source, vector.model_section.as_deref())
            .ok_or_else(|| {
                QualificationValidationError::new(
                    QualificationErrorCode::InvalidExecutionDefinition,
                    format!("{path}.model_section"),
                    vector.model_section.as_ref().map_or_else(
                        || "the canonical source has no executable top-level model card".to_owned(),
                        |section| {
                            format!("model section {section:?} does not resolve exactly once")
                        },
                    ),
                )
            })?;
    let execution_model_source = rspice_core::netlist::decode_source_bytes(
        &vector.execution_model_source,
    )
    .map_err(|error| {
        QualificationValidationError::new(
            QualificationErrorCode::InvalidExecutionDefinition,
            format!("{path}.execution_model_source"),
            format!("selected model source cannot be decoded: {error}"),
        )
    })?;
    if execution_model_source != expected_execution_source {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::SourceBindingMismatch,
            format!("{path}.execution_model_source"),
            "executed model bytes are not the exact selected base or section card from the retained canonical source",
        ));
    }
    if !contains_exact_bytes(&vector.executable_input, &vector.execution_model_source) {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::SourceBindingMismatch,
            format!("{path}.executable_input"),
            "the executable testbench does not contain the exact selected model card",
        ));
    }

    let executable = std::str::from_utf8(&vector.executable_input).map_err(|error| {
        QualificationValidationError::new(
            QualificationErrorCode::InvalidExecutionDefinition,
            format!("{path}.executable_input"),
            format!("executable testbench is not UTF-8: {error}"),
        )
    })?;
    let netlist = rspice_core::Netlist::parse_validated(executable).map_err(|error| {
        QualificationValidationError::new(
            QualificationErrorCode::InvalidExecutionDefinition,
            format!("{path}.executable_input"),
            format!("executable testbench is not a valid netlist: {error}"),
        )
    })?;
    let model_is_present = netlist
        .models
        .iter()
        .any(|model| model.name.eq_ignore_ascii_case(&vector.source.model_id));
    let model_is_instantiated = netlist.elements.iter().any(|element| {
        element_kind_model_name(&element.kind)
            .is_some_and(|name| name.eq_ignore_ascii_case(&vector.source.model_id))
    });
    if !model_is_present || !model_is_instantiated {
        return Err(QualificationValidationError::new(
            QualificationErrorCode::SourceBindingMismatch,
            format!("{path}.executable_input"),
            "the executable testbench must parse and instantiate the exact bound candidate model",
        ));
    }
    Ok(())
}

fn selected_execution_model_source<'a>(
    canonical_source: &'a str,
    section: Option<&str>,
) -> Option<&'a str> {
    if let Some(section) = section {
        let mut selected = None;
        let mut active_start = None;
        let mut offset = 0;
        for line in canonical_source.split_inclusive('\n') {
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if let Some(name) = trimmed.strip_prefix(".lib ") {
                if name.eq_ignore_ascii_case(section) {
                    if active_start.is_some() || selected.is_some() {
                        return None;
                    }
                    active_start = Some(offset + line.len());
                }
            } else if let Some(name) = trimmed.strip_prefix(".endl ")
                && name.eq_ignore_ascii_case(section)
            {
                let start = active_start.take()?;
                selected = Some(&canonical_source[start..offset]);
            }
            offset += line.len();
        }
        return active_start.is_none().then_some(selected).flatten();
    }

    let end = canonical_source
        .split_inclusive('\n')
        .scan(0usize, |offset, line| {
            let start = *offset;
            *offset += line.len();
            Some((start, line))
        })
        .find_map(|(start, line)| line.trim_start().starts_with(".lib ").then_some(start))
        .unwrap_or(canonical_source.len());
    (end > 0).then_some(&canonical_source[..end])
}

fn contains_exact_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

fn element_kind_model_name(kind: &rspice_core::netlist::ElementKind) -> Option<&str> {
    use rspice_core::netlist::ElementKind;
    match kind {
        ElementKind::Resistor {
            model: Some(model), ..
        }
        | ElementKind::Capacitor {
            model: Some(model), ..
        }
        | ElementKind::Inductor {
            model: Some(model), ..
        }
        | ElementKind::TransmissionLine {
            model: Some(model), ..
        }
        | ElementKind::JilesAthertonInductor { model, .. }
        | ElementKind::Diode { model, .. }
        | ElementKind::Bjt { model, .. }
        | ElementKind::Mosfet { model, .. }
        | ElementKind::Jfet { model, .. }
        | ElementKind::Mesfet { model, .. }
        | ElementKind::XyceMemristor { model, .. }
        | ElementKind::VSwitch { model, .. }
        | ElementKind::ISwitch { model, .. }
        | ElementKind::GenericSwitch { model, .. }
        | ElementKind::Xspice { model, .. } => Some(model),
        _ => None,
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

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.model_id"), &self.model_id)
    }

    fn require_project_bound(&self, path: &str) -> QualificationResult<ModelSourceId> {
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
    fn try_new(
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

    fn validate(&self, path: &str) -> QualificationResult<()> {
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
    fn try_new(
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

    fn validate(&self, path: &str) -> QualificationResult<()> {
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
    fn try_new(
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

    fn try_failed(
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

    fn validate(&self, path: &str) -> QualificationResult<()> {
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
    fn try_new(
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
    fn try_new(
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
    fn try_new(
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

/// Observable suite progress. Emitting progress never publishes partial
/// qualification evidence; cancellation discards the in-progress run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationExecutionProgress {
    pub platform: QualificationPlatform,
    pub completed_vectors: usize,
    pub total_vectors: usize,
    pub current_vector_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum QualificationExecutionError {
    #[error(transparent)]
    InvalidContract(#[from] QualificationValidationError),
    #[error("qualification execution was cancelled; no partial platform run was published")]
    Cancelled,
    #[error("qualification execution session has already finished")]
    SessionFinished,
}

/// Result of one cooperative session step. An in-progress step never exposes
/// partial evidence. The terminal variant contains the first publishable,
/// fully validated platform run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualificationExecutionStep {
    InProgress(QualificationExecutionProgress),
    Complete {
        progress: QualificationExecutionProgress,
        run: QualificationPlatformRun,
    },
}

/// Owned, frame-safe execution state. `step` executes at most one vector, so
/// callers can yield to egui or a browser event loop between vectors.
#[derive(Debug, Clone)]
pub struct QualificationExecutionSession {
    suite: QualificationSuite,
    source: ModelSourceEvidenceBinding,
    platform: QualificationPlatform,
    next_vector: usize,
    vector_outcomes: Vec<QualificationPlatformVectorOutcome>,
    cancelled: bool,
    finished: bool,
}

impl QualificationExecutionSession {
    pub fn try_new(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
    ) -> Result<Self, QualificationExecutionError> {
        validate_execution_contract(suite, source)?;
        Ok(Self {
            suite: suite.clone(),
            source: source.clone(),
            platform: current_qualification_platform(),
            next_vector: 0,
            vector_outcomes: Vec::with_capacity(suite.vectors.len()),
            cancelled: false,
            finished: false,
        })
    }

    #[must_use]
    pub fn progress(&self) -> QualificationExecutionProgress {
        QualificationExecutionProgress {
            platform: self.platform,
            completed_vectors: self.next_vector,
            total_vectors: self.suite.vectors.len(),
            current_vector_id: self
                .suite
                .vectors
                .get(self.next_vector)
                .map(|value| value.id.clone()),
        }
    }

    /// Request cancellation before another vector begins. No partial run can
    /// be retrieved from a cancelled session.
    pub fn cancel(&mut self) {
        if !self.finished {
            self.cancelled = true;
        }
    }

    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }

    /// Execute at most one vector and yield a progress or terminal record.
    pub fn step(
        &mut self,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationExecutionStep, QualificationExecutionError> {
        if self.finished {
            return Err(QualificationExecutionError::SessionFinished);
        }
        if self.cancelled || abort.is_aborted() {
            self.cancelled = true;
            return Err(QualificationExecutionError::Cancelled);
        }
        let vector = self
            .suite
            .vectors
            .get(self.next_vector)
            .expect("a validated qualification suite is non-empty and unfinished");
        let outcome = match execute_qualification_vector(vector, self.platform, abort) {
            Ok(value) => value,
            Err(QualificationExecutionError::Cancelled) => {
                self.cancelled = true;
                return Err(QualificationExecutionError::Cancelled);
            }
            Err(error) => return Err(error),
        };
        if abort.is_aborted() {
            self.cancelled = true;
            return Err(QualificationExecutionError::Cancelled);
        }
        self.vector_outcomes
            .push(QualificationPlatformVectorOutcome {
                vector_id: vector.id.clone(),
                input_digest: vector.input_digest,
                outcome,
            });
        self.next_vector += 1;
        let progress = self.progress();
        abort.observe_progress(
            progress.completed_vectors as f64 / progress.total_vectors.max(1) as f64,
        );
        if self.next_vector < self.suite.vectors.len() {
            return Ok(QualificationExecutionStep::InProgress(progress));
        }

        let run = QualificationPlatformRun::try_new(
            self.platform,
            self.source.clone(),
            &self.suite,
            self.vector_outcomes.clone(),
        )?;
        self.finished = true;
        Ok(QualificationExecutionStep::Complete { progress, run })
    }
}

/// Deterministic, synchronous qualification executor shared by desktop and
/// WebAssembly builds. It uses only `rspice-core` APIs available on both
/// targets and publishes the runtime selected by the compilation target.
#[derive(Debug, Default, Clone, Copy)]
pub struct QualificationExecutionService;

impl QualificationExecutionService {
    pub fn execute_current_platform(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationPlatformRun, QualificationExecutionError> {
        let mut ignore_progress = |_progress: &QualificationExecutionProgress| {};
        Self::execute_current_platform_with_progress(suite, source, abort, &mut ignore_progress)
    }

    pub fn execute_current_platform_with_progress(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        abort: &dyn rspice_core::AbortSignal,
        progress: &mut dyn FnMut(&QualificationExecutionProgress),
    ) -> Result<QualificationPlatformRun, QualificationExecutionError> {
        let mut session = QualificationExecutionSession::try_new(suite, source)?;
        progress(&session.progress());
        loop {
            match session.step(abort)? {
                QualificationExecutionStep::InProgress(step_progress) => {
                    progress(&step_progress);
                }
                QualificationExecutionStep::Complete {
                    progress: step_progress,
                    run,
                } => {
                    progress(&step_progress);
                    return Ok(run);
                }
            }
        }
    }

    /// Assemble immutable parity evidence only after independently produced
    /// Desktop and WebAssembly runs provide exact coverage.
    pub fn assemble_evidence(
        evidence_id: impl Into<String>,
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        runs: Vec<QualificationPlatformRun>,
    ) -> QualificationResult<QualificationEvidence> {
        validate_execution_contract(suite, source)?;
        if runs.len() != QualificationPlatform::REQUIRED.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "platform_runs",
                "exactly one real Desktop run and one real WebAssembly run are required",
            ));
        }
        let mut platforms = BTreeSet::new();
        for (index, run) in runs.iter().enumerate() {
            if !platforms.insert(run.platform) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("platform_runs[{index}].platform"),
                    "qualification platform run is duplicated",
                ));
            }
            run.validate_bound(suite, source)?;
        }
        if !QualificationPlatform::REQUIRED
            .iter()
            .all(|platform| platforms.contains(platform))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "platform_runs",
                "Desktop and WebAssembly platform runs are both required",
            ));
        }

        let mut vector_outcomes = Vec::with_capacity(suite.vectors.len());
        for vector in &suite.vectors {
            let mut outcomes = Vec::with_capacity(QualificationPlatform::REQUIRED.len());
            for platform in QualificationPlatform::REQUIRED {
                let run = runs
                    .iter()
                    .find(|value| value.platform == platform)
                    .expect("required platform coverage checked above");
                let platform_vector =
                    find_ci(&run.vector_outcomes, &vector.id, |value| &value.vector_id)
                        .expect("platform run coverage validated above");
                outcomes.push(platform_vector.outcome.clone());
            }
            vector_outcomes.push(QualificationVectorOutcome::try_new(
                vector.id.clone(),
                vector.input_digest,
                outcomes,
            )?);
        }
        let evidence = QualificationEvidence::try_new(
            evidence_id,
            source.clone(),
            suite.id.clone(),
            suite.revision,
            vector_outcomes,
        )?;
        evidence.validate_bound(suite, source)?;
        Ok(evidence)
    }
}

enum ExecutedQualificationAnalysis {
    OperatingPoint(rspice_core::SimulationResult),
    DcSweep(Vec<(f64, rspice_core::SimulationResult)>),
    AcSweep(Vec<rspice_core::analysis::AcResult>),
    Noise(Vec<rspice_core::analysis::NoiseResult>),
    Transient(rspice_core::engine::TransientResult),
}

fn validate_execution_contract(
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
) -> QualificationResult<()> {
    suite.validate()?;
    source.validate("execution.source")?;
    source.require_project_bound("execution.source")?;
    for (index, vector) in suite.vectors.iter().enumerate() {
        vector.validate_source_binding(source, &format!("suite.vectors[{index}].source"))?;
    }
    Ok(())
}

fn execute_qualification_vector(
    vector: &QualificationVector,
    platform: QualificationPlatform,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<PlatformQualificationOutcome, QualificationExecutionError> {
    let input = match std::str::from_utf8(&vector.executable_input) {
        Ok(value) => value,
        Err(error) => {
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Input,
                "input-not-utf8",
                format!("retained executable input is not UTF-8: {error}"),
            );
        }
    };
    let netlist = match rspice_core::Netlist::parse_validated_with_abort(input, abort) {
        Ok(value) => value,
        Err(error) if error.is_aborted() || abort.is_aborted() => {
            return Err(QualificationExecutionError::Cancelled);
        }
        Err(error) => {
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Parse,
                "netlist-parse-failed",
                error.to_string(),
            );
        }
    };
    let engine = rspice_core::Engine::default();
    let executed = match &vector.analysis {
        QualificationAnalysis::DcOperatingPoint => engine
            .run_dc_op_with_abort(&netlist, abort)
            .map(ExecutedQualificationAnalysis::OperatingPoint),
        QualificationAnalysis::DcSweep {
            source,
            start,
            stop,
            step,
        } => engine
            .run_dc_sweep_with_abort(&netlist, source, start.get(), stop.get(), step.get(), abort)
            .map(ExecutedQualificationAnalysis::DcSweep),
        QualificationAnalysis::AcSweep { frequencies } => {
            let frequencies = frequencies
                .iter()
                .map(|frequency| frequency.get())
                .collect::<Vec<_>>();
            engine
                .run_ac_with_abort(&netlist, &frequencies, abort)
                .map(ExecutedQualificationAnalysis::AcSweep)
        }
        QualificationAnalysis::Noise {
            output_node,
            output_reference,
            input_source,
            frequencies,
            temperature_kelvin,
        } => {
            let frequencies = frequencies
                .iter()
                .map(|frequency| frequency.get())
                .collect::<Vec<_>>();
            engine
                .run_noise_named_with_input_source_and_abort(
                    &netlist,
                    output_node,
                    output_reference.as_deref(),
                    input_source,
                    &frequencies,
                    temperature_kelvin.get(),
                    abort,
                )
                .map(ExecutedQualificationAnalysis::Noise)
        }
        QualificationAnalysis::Transient {
            stop_time,
            max_step,
        } => engine
            .run_tran_with_abort(&netlist, stop_time.get(), max_step.get(), abort)
            .map(ExecutedQualificationAnalysis::Transient),
    };
    let executed = match executed {
        Ok(value) => value,
        Err(error) if error.descriptor().code == rspice_core::SimulationErrorCode::Aborted => {
            return Err(QualificationExecutionError::Cancelled);
        }
        Err(error) => {
            let code = error.descriptor().code.to_string();
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Simulation,
                code,
                error.to_string(),
            );
        }
    };
    if abort.is_aborted() {
        return Err(QualificationExecutionError::Cancelled);
    }

    let mut references = Vec::with_capacity(vector.outputs.len());
    for output in &vector.outputs {
        let reference = find_ci(&vector.references, &output.quantity, |value| {
            &value.quantity
        })
        .expect("vector output/reference coverage validated before execution");
        let observed = match measure_qualification_output(output, &executed) {
            Ok(value) if value.is_finite() => value,
            Ok(_) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "non-finite-output",
                    format!("output {:?} produced a non-finite value", output.quantity),
                );
            }
            Err(message) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "output-unavailable",
                    message,
                );
            }
        };
        let evidence = match ReferenceErrorEvidence::try_new(
            output.quantity.clone(),
            reference.expected.get(),
            observed,
            reference.absolute_tolerance.get(),
            reference.relative_tolerance.get(),
        ) {
            Ok(value) => value,
            Err(error) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "error-computation-failed",
                    error.to_string(),
                );
            }
        };
        references.push(evidence);
    }
    PlatformQualificationOutcome::try_new(platform, references).map_err(Into::into)
}

fn failed_platform_outcome(
    platform: QualificationPlatform,
    stage: QualificationFailureStage,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<PlatformQualificationOutcome, QualificationExecutionError> {
    let failure = QualificationExecutionFailure::try_new(stage, code, message)?;
    PlatformQualificationOutcome::try_failed(platform, failure).map_err(Into::into)
}

fn measure_qualification_output(
    output: &QualificationOutputDefinition,
    executed: &ExecutedQualificationAnalysis,
) -> Result<f64, String> {
    let (sweep_value, result) = match (executed, output.sample) {
        (
            ExecutedQualificationAnalysis::OperatingPoint(result),
            QualificationSample::OperatingPoint,
        ) => (None, result),
        (ExecutedQualificationAnalysis::DcSweep(points), QualificationSample::FirstSweepPoint) => {
            let (sweep, result) = points
                .first()
                .ok_or_else(|| "DC sweep completed without producing a result point".to_owned())?;
            (Some(*sweep), result)
        }
        (ExecutedQualificationAnalysis::DcSweep(points), QualificationSample::LastSweepPoint) => {
            let (sweep, result) = points
                .last()
                .ok_or_else(|| "DC sweep completed without producing a result point".to_owned())?;
            (Some(*sweep), result)
        }
        (
            ExecutedQualificationAnalysis::DcSweep(points),
            QualificationSample::SweepPoint { index },
        ) => {
            let (sweep, result) = points.get(index).ok_or_else(|| {
                format!(
                    "DC sweep output requests point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            (Some(*sweep), result)
        }
        (ExecutedQualificationAnalysis::AcSweep(points), sample) => {
            let index = selected_frequency_index(sample, points.len(), "AC sweep")?;
            let point = points.get(index).ok_or_else(|| {
                format!(
                    "AC output requests frequency point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            return measure_ac_output(&output.probe, point);
        }
        (ExecutedQualificationAnalysis::Noise(points), sample) => {
            let index = selected_frequency_index(sample, points.len(), "noise sweep")?;
            let point = points.get(index).ok_or_else(|| {
                format!(
                    "noise output requests frequency point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            return measure_noise_output(&output.probe, point);
        }
        (ExecutedQualificationAnalysis::Transient(result), sample) => {
            let index = selected_time_index(sample, result.time.len())?;
            return measure_transient_output(&output.probe, result, index);
        }
        _ => return Err("output sample is incompatible with the executed analysis".to_owned()),
    };

    match &output.probe {
        QualificationProbe::NodeVoltage { node } => result
            .try_voltage_named(node)
            .ok_or_else(|| format!("node voltage {:?} is unavailable", node)),
        QualificationProbe::BranchCurrent { branch } => result
            .branch_current_named(branch)
            .ok_or_else(|| format!("branch current {:?} is unavailable", branch)),
        QualificationProbe::DcObservable { expression } => result
            .try_dc_observable_named(expression)
            .ok_or_else(|| format!("DC observable {:?} is unavailable", expression)),
        QualificationProbe::SweepValue => sweep_value
            .ok_or_else(|| "sweep value is unavailable for an operating-point result".to_owned()),
        _ => Err("output probe is incompatible with a DC result".to_owned()),
    }
}

fn selected_frequency_index(
    sample: QualificationSample,
    point_count: usize,
    analysis: &str,
) -> Result<usize, String> {
    if point_count == 0 {
        return Err(format!(
            "{analysis} completed without producing a result point"
        ));
    }
    match sample {
        QualificationSample::FirstFrequencyPoint => Ok(0),
        QualificationSample::LastFrequencyPoint => Ok(point_count - 1),
        QualificationSample::FrequencyPoint { index } if index < point_count => Ok(index),
        QualificationSample::FrequencyPoint { index } => Err(format!(
            "{analysis} output requests point {index}, but only {point_count} points were produced"
        )),
        _ => Err(format!(
            "output sample is incompatible with the executed {analysis}"
        )),
    }
}

fn selected_time_index(sample: QualificationSample, point_count: usize) -> Result<usize, String> {
    if point_count == 0 {
        return Err("transient analysis completed without producing a result point".to_owned());
    }
    match sample {
        QualificationSample::FirstTimePoint => Ok(0),
        QualificationSample::LastTimePoint => Ok(point_count - 1),
        QualificationSample::TimePoint { index } if index < point_count => Ok(index),
        QualificationSample::TimePoint { index } => Err(format!(
            "transient output requests point {index}, but only {point_count} points were produced"
        )),
        _ => Err("output sample is incompatible with the executed transient analysis".to_owned()),
    }
}

fn measure_ac_output(
    probe: &QualificationProbe,
    point: &rspice_core::analysis::AcResult,
) -> Result<f64, String> {
    let node_voltage = |node: &str| -> Result<num_complex::Complex64, String> {
        if is_ground_name(node) {
            return Ok(num_complex::Complex64::new(0.0, 0.0));
        }
        let index = point
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(node))
            .ok_or_else(|| format!("AC node voltage {node:?} is unavailable"))?;
        point
            .voltages
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC node voltage {node:?} is unavailable"))
    };
    let branch_current = |branch: &str| -> Result<num_complex::Complex64, String> {
        let index = point
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(branch))
            .ok_or_else(|| format!("AC branch current {branch:?} is unavailable"))?;
        point
            .currents
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC branch current {branch:?} is unavailable"))
    };

    match probe {
        QualificationProbe::AcNodeVoltageMagnitude { node } => Ok(node_voltage(node)?.norm()),
        QualificationProbe::AcNodeVoltagePhaseDegrees { node } => {
            Ok(node_voltage(node)?.arg().to_degrees())
        }
        QualificationProbe::AcNodeVoltageReal { node } => Ok(node_voltage(node)?.re),
        QualificationProbe::AcNodeVoltageImaginary { node } => Ok(node_voltage(node)?.im),
        QualificationProbe::AcBranchCurrentMagnitude { branch } => {
            Ok(branch_current(branch)?.norm())
        }
        QualificationProbe::AcBranchCurrentPhaseDegrees { branch } => {
            Ok(branch_current(branch)?.arg().to_degrees())
        }
        QualificationProbe::AcBranchCurrentReal { branch } => Ok(branch_current(branch)?.re),
        QualificationProbe::AcBranchCurrentImaginary { branch } => Ok(branch_current(branch)?.im),
        QualificationProbe::AcEffectiveCapacitance {
            branch,
            excitation_magnitude,
        } => Ok(branch_current(branch)?.im
            / (std::f64::consts::TAU * point.frequency * excitation_magnitude.get())),
        QualificationProbe::FrequencyValue => Ok(point.frequency),
        _ => Err("output probe is incompatible with an AC result".to_owned()),
    }
}

fn measure_noise_output(
    probe: &QualificationProbe,
    point: &rspice_core::analysis::NoiseResult,
) -> Result<f64, String> {
    match probe {
        QualificationProbe::FrequencyValue => Ok(point.frequency),
        QualificationProbe::NoiseOutputDensity => Ok(point.output_noise_density),
        QualificationProbe::NoiseInputReferredDensity => Ok(point.input_referred_density),
        QualificationProbe::NoiseOutputAmplitude => point
            .output_noise_density
            .is_sign_positive()
            .then(|| point.output_noise_density.sqrt())
            .ok_or_else(|| "output-noise density is negative".to_owned()),
        QualificationProbe::NoiseInputReferredAmplitude => point
            .input_referred_density
            .is_sign_positive()
            .then(|| point.input_referred_density.sqrt())
            .ok_or_else(|| "input-referred-noise density is negative".to_owned()),
        _ => Err("output probe is incompatible with a noise result".to_owned()),
    }
}

fn measure_transient_output(
    probe: &QualificationProbe,
    result: &rspice_core::engine::TransientResult,
    index: usize,
) -> Result<f64, String> {
    match probe {
        QualificationProbe::TransientNodeVoltage { node } => {
            result.try_voltage_at_named(node, index).ok_or_else(|| {
                format!("transient node voltage {node:?} is unavailable at point {index}")
            })
        }
        QualificationProbe::TransientBranchCurrent { branch } => result
            .try_branch_current_waveform_named(branch)
            .and_then(|waveform| waveform.get(index))
            .copied()
            .ok_or_else(|| {
                format!("transient branch current {branch:?} is unavailable at point {index}")
            }),
        QualificationProbe::TimeValue => result
            .time
            .get(index)
            .copied()
            .ok_or_else(|| format!("transient time is unavailable at point {index}")),
        _ => Err("output probe is incompatible with a transient result".to_owned()),
    }
}

fn is_ground_name(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "gnd!"
    )
}

#[cfg(target_arch = "wasm32")]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::WebAssembly
}

#[cfg(not(target_arch = "wasm32"))]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::Desktop
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
mod tests {
    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn model_source(byte: u8) -> Vec<u8> {
        format!("* retained qualification model\n.model demo180_nch NMOS (VTO=0.{byte})\n")
            .into_bytes()
    }

    fn source(byte: u8) -> ModelSourceEvidenceBinding {
        ModelSourceEvidenceBinding::try_new_project_bound(
            "demo180_nch",
            "11111111-1111-4111-8111-111111111111".parse().unwrap(),
            digest_bytes(&model_source(byte)),
            ObjectRevision::new(7).unwrap(),
        )
        .unwrap()
    }

    fn vector(id: &str, name: &str, byte: u8) -> QualificationVector {
        vector_for_source(id, name, byte, 7)
    }

    fn vector_for_source(
        id: &str,
        name: &str,
        input_variant: u8,
        source_variant: u8,
    ) -> QualificationVector {
        let retained_model = model_source(source_variant);
        let input = format!(
            "qualification {input_variant}\n{}V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            String::from_utf8(retained_model.clone()).unwrap()
        )
        .into_bytes();
        QualificationVector::try_new_source_bound(
            id,
            name,
            source(source_variant),
            retained_model,
            input,
            QualificationAnalysis::DcOperatingPoint,
            vec![
                QualificationOutputDefinition::try_new(
                    "drain_current",
                    QualificationProbe::NodeVoltage {
                        node: "output".into(),
                    },
                    QualificationSample::OperatingPoint,
                )
                .unwrap(),
            ],
            vec![QualificationReference::try_new("drain_current", 1.0, 0.01, 0.005).unwrap()],
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
                ReferenceErrorEvidence::try_new("drain_current", 1.0, 1.0 + error, 0.01, 0.005)
                    .unwrap(),
            ],
        )
        .unwrap()
    }

    fn executable_vector(
        id: &str,
        name: &str,
        testbench: &str,
        analysis: QualificationAnalysis,
        outputs: Vec<QualificationOutputDefinition>,
        references: Vec<QualificationReference>,
    ) -> QualificationVector {
        let retained_model = model_source(7);
        let input = format!(
            "qualification {id}\n{}{testbench}",
            String::from_utf8(retained_model.clone()).unwrap()
        )
        .into_bytes();
        QualificationVector::try_new_source_bound(
            id,
            name,
            source(7),
            retained_model,
            input,
            analysis,
            outputs,
            references,
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

    fn imported_platform_run(
        run: &QualificationPlatformRun,
        platform: QualificationPlatform,
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
    ) -> QualificationPlatformRun {
        let outcomes = run
            .vector_outcomes
            .iter()
            .map(|value| {
                let mut value = value.clone();
                value.outcome.platform = platform;
                value
            })
            .collect();
        QualificationPlatformRun::try_new(platform, source.clone(), suite, outcomes).unwrap()
    }

    fn failed_platform_run(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        platform: QualificationPlatform,
    ) -> QualificationPlatformRun {
        let outcomes = suite
            .vectors
            .iter()
            .map(|vector| QualificationPlatformVectorOutcome {
                vector_id: vector.id.clone(),
                input_digest: vector.input_digest,
                outcome: PlatformQualificationOutcome::try_failed(
                    platform,
                    QualificationExecutionFailure::try_new(
                        QualificationFailureStage::Measurement,
                        "reference-mismatch",
                        "retained test failure",
                    )
                    .unwrap(),
                )
                .unwrap(),
            })
            .collect();
        QualificationPlatformRun::try_new(platform, source.clone(), suite, outcomes).unwrap()
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
        candidate_with_snapshot(
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
            .unwrap(),
        )
    }

    fn candidate_with_snapshot(mut candidate: ModelReleaseCandidate) -> ModelReleaseCandidate {
        candidate.definition_source = model_source(7);
        candidate.definition_metadata = Some(ModelDefinitionMetadata::default());
        candidate
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
        let candidate = candidate_with_snapshot(
            ModelReleaseCandidate::try_new(
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
            .unwrap(),
        );
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::DocumentationIncomplete
        );

        let candidate = candidate_with_snapshot(
            ModelReleaseCandidate::try_new(
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
            .unwrap(),
        );
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::LicenseIncomplete
        );

        let candidate = candidate_with_snapshot(
            ModelReleaseCandidate::try_new(
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
            .unwrap(),
        );
        assert_eq!(
            candidate
                .validate_for_promotion(&suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::ConsumerImpactIncomplete
        );

        let candidate = candidate_with_snapshot(
            ModelReleaseCandidate::try_new(
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
            .unwrap(),
        );
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
        let evidence_digest = evidence.content_digest().unwrap();
        assert!(record.checklist.is_complete());
        assert_eq!(release.promotion_record_id, record.id);
        assert_eq!(candidate.evidence_digest, Some(evidence_digest));
        assert_eq!(record.evidence_digest, Some(evidence_digest));
        assert_eq!(release.evidence_digest, Some(evidence_digest));
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
    fn governed_records_reject_missing_or_mismatched_evidence_digests() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &evidence, &source);
        let mut serialized = serde_json::to_value(&candidate).unwrap();
        serialized
            .as_object_mut()
            .unwrap()
            .remove("evidence_digest");
        let legacy: ModelReleaseCandidate = serde_json::from_value(serialized).unwrap();
        assert_eq!(legacy.evidence_digest, None);
        assert_eq!(
            legacy.validate_bound(&suite, &evidence).unwrap_err().code,
            QualificationErrorCode::MissingRequiredValue
        );

        let (mut promotion, release) = ModelPromotionRecord::promote(
            "promotion-digest",
            ModelReleaseIdentity {
                id: "release-digest".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3".into(),
            },
            &candidate,
            &suite,
            &evidence,
        )
        .unwrap();
        promotion.evidence_digest = Some(digest(99));
        assert_eq!(
            promotion
                .validate_bound(&release, &candidate, &suite, &evidence)
                .unwrap_err()
                .code,
            QualificationErrorCode::SourceBindingMismatch
        );
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
                Vec::new(),
            )
            .unwrap_err()
            .code,
            QualificationErrorCode::DuplicateName
        );
    }

    #[test]
    fn suite_only_state_is_rejected_under_the_wrong_model_key() {
        let state = ModelQualificationState::try_new(
            vec![suite()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        state.validate_for_model("demo180_nch").unwrap();
        assert_eq!(
            state
                .validate_for_model("different_model")
                .unwrap_err()
                .code,
            QualificationErrorCode::SourceBindingMismatch
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
            Vec::new(),
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

    #[test]
    fn qualified_section_digest_resolves_only_exact_passing_evidence() {
        let suite = suite();
        let bound_source = source(7);
        let base_evidence = evidence(&suite, &bound_source, true);
        let evidence_digest = base_evidence.content_digest().unwrap();
        let state = ModelQualificationState::try_new(
            vec![suite],
            Vec::new(),
            vec![base_evidence],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        state
            .validate_exact_evidence_digest(&bound_source, evidence_digest)
            .unwrap();
        assert_eq!(
            state
                .validate_exact_evidence_digest(&source(8), evidence_digest)
                .unwrap_err()
                .code,
            QualificationErrorCode::SourceBindingMismatch
        );
        assert_eq!(
            state
                .validate_exact_evidence_digest(&bound_source, digest(99))
                .unwrap_err()
                .code,
            QualificationErrorCode::MissingRequiredValue
        );

        let complete_source = b"* base\n.model demo180_nch NMOS (VTO=0.7)\n.lib TT\n.model demo180_nch NMOS (VTO=0.8)\n.endl TT\n".to_vec();
        let section_source = b".model demo180_nch NMOS (VTO=0.8)\n".to_vec();
        let section_binding = ModelSourceEvidenceBinding::try_new_project_bound(
            "demo180_nch",
            "11111111-1111-4111-8111-111111111111".parse().unwrap(),
            digest_bytes(&complete_source),
            ObjectRevision::new(7).unwrap(),
        )
        .unwrap();
        let mut executable = b"RSpice model qualification\n".to_vec();
        executable.extend_from_slice(&section_source);
        executable.extend_from_slice(b"V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n");
        let section_vector = QualificationVector::try_new_source_section_bound(
            "tt-vector",
            "TT operating point",
            section_binding.clone(),
            complete_source,
            Some("TT".to_owned()),
            section_source,
            executable,
            QualificationAnalysis::DcOperatingPoint,
            vec![
                QualificationOutputDefinition::try_new(
                    "drain_current",
                    QualificationProbe::NodeVoltage {
                        node: "output".into(),
                    },
                    QualificationSample::OperatingPoint,
                )
                .unwrap(),
            ],
            vec![QualificationReference::try_new("drain_current", 1.0, 0.01, 0.005).unwrap()],
        )
        .unwrap();
        let section_suite = QualificationSuite::try_new(
            "tt-suite",
            "TT qualification",
            ObjectRevision::INITIAL,
            vec![section_vector],
        )
        .unwrap();
        let section_evidence = evidence(&section_suite, &section_binding, true);
        let section_digest = section_evidence.content_digest().unwrap();
        let section_state = ModelQualificationState::try_new(
            vec![section_suite],
            Vec::new(),
            vec![section_evidence],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        section_state
            .validate_exact_section_evidence_digest(&section_binding, "TT", section_digest)
            .unwrap();
        assert_eq!(
            section_state
                .validate_exact_section_evidence_digest(&section_binding, "FF", section_digest,)
                .unwrap_err()
                .code,
            QualificationErrorCode::EvidenceCoverageMismatch
        );
    }

    #[test]
    fn source_revision_reconciliation_prunes_stale_work_but_keeps_release_lineage() {
        let promoted_suite = suite();
        let promoted_source = source(7);
        let promoted_evidence = evidence(&promoted_suite, &promoted_source, true);
        let promoted_candidate = candidate(&promoted_suite, &promoted_evidence, &promoted_source);
        let (promotion, release) = ModelPromotionRecord::promote(
            "promotion-retained",
            ModelReleaseIdentity {
                id: "release-retained".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3".into(),
            },
            &promoted_candidate,
            &promoted_suite,
            &promoted_evidence,
        )
        .unwrap();

        let stale_suite = QualificationSuite::try_new(
            "stale-suite",
            "Stale suite",
            ObjectRevision::INITIAL,
            vec![vector_for_source("stale-vector", "Stale vector", 3, 8)],
        )
        .unwrap();
        let mut stale_evidence = evidence(&stale_suite, &source(8), true);
        stale_evidence.id = "stale-evidence".into();
        stale_evidence.validate_internal().unwrap();
        let state = ModelQualificationState::try_new(
            vec![promoted_suite.clone(), stale_suite],
            Vec::new(),
            vec![promoted_evidence.clone(), stale_evidence],
            vec![promoted_candidate.clone()],
            vec![release.clone()],
            vec![promotion.clone()],
        )
        .unwrap();

        let current = source(9);
        let reconciled = state.reconcile_after_source_revision(&current).unwrap();
        assert_eq!(reconciled.suites, vec![promoted_suite]);
        assert_eq!(reconciled.evidence, vec![promoted_evidence]);
        assert_eq!(reconciled.candidates, vec![promoted_candidate]);
        assert_eq!(reconciled.releases, vec![release]);
        assert_eq!(reconciled.promotions, vec![promotion]);
        assert!(
            reconciled
                .exact_suites_for_source(&current)
                .unwrap()
                .is_empty()
        );
        reconciled.validate().unwrap();
    }

    #[test]
    fn retained_input_digest_tampering_is_rejected() {
        let mut suite = suite();
        suite.vectors[0].executable_input.push(b' ');
        let error = suite.validate().unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::InputDigestMismatch);
    }

    #[test]
    fn executable_vectors_require_the_exact_bound_candidate_source() {
        let mut wrong_digest = vector("digest", "Digest", 1);
        wrong_digest.model_source = model_source(8);
        assert_eq!(
            wrong_digest.validate("vector").unwrap_err().code,
            QualificationErrorCode::SourceBindingMismatch
        );

        let mut not_embedded = vector("embedded", "Embedded", 2);
        let retained = not_embedded.model_source.clone();
        let executable = String::from_utf8(not_embedded.executable_input.clone())
            .unwrap()
            .replace(&String::from_utf8(retained).unwrap(), "");
        not_embedded.executable_input = executable.into_bytes();
        not_embedded.input_digest = digest_bytes(&not_embedded.executable_input);
        assert_eq!(
            not_embedded.validate("vector").unwrap_err().code,
            QualificationErrorCode::SourceBindingMismatch
        );

        let legacy = ModelSourceEvidenceBinding::try_new(
            "demo180_nch",
            digest_bytes(&model_source(7)),
            ObjectRevision::new(7).unwrap(),
        )
        .unwrap();
        assert_eq!(
            validate_execution_contract(&suite(), &legacy)
                .unwrap_err()
                .code,
            QualificationErrorCode::SourceBindingMismatch
        );
    }

    #[test]
    fn suites_cannot_mix_source_revisions() {
        let error = QualificationSuite::try_new(
            "mixed",
            "Mixed",
            ObjectRevision::INITIAL,
            vec![
                vector_for_source("one", "One", 1, 7),
                vector_for_source("two", "Two", 2, 8),
            ],
        )
        .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::SourceBindingMismatch);
    }

    #[test]
    fn native_execution_measures_outputs_and_applies_declared_tolerances() {
        let source = source(7);
        let passing_suite = suite();
        let run = QualificationExecutionService::execute_current_platform(
            &passing_suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        assert_eq!(run.platform, QualificationPlatform::Desktop);
        assert!(run.passed);
        assert_eq!(run.vector_outcomes.len(), passing_suite.vectors.len());
        let reference = &run.vector_outcomes[0].outcome.references[0];
        assert_eq!(reference.expected_value, FiniteValue::new(1.0).unwrap());
        assert_eq!(reference.observed_value, FiniteValue::new(1.0).unwrap());

        let mut failing_suite = passing_suite.clone();
        for vector in &mut failing_suite.vectors {
            vector.references[0].expected = FiniteValue::new(2.0).unwrap();
        }
        let run = QualificationExecutionService::execute_current_platform(
            &failing_suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        assert!(!run.passed);
        assert!(
            run.vector_outcomes
                .iter()
                .all(|value| !value.outcome.passed)
        );
    }

    #[test]
    fn ac_cv_noise_and_transient_vectors_execute_real_solver_results() {
        let ac_vector = executable_vector(
            "ac-cv",
            "AC and effective capacitance",
            "V1 output 0 DC 0 AC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            QualificationAnalysis::AcSweep {
                frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
            },
            vec![
                QualificationOutputDefinition::try_new(
                    "capacitance",
                    QualificationProbe::AcEffectiveCapacitance {
                        branch: "V1".to_owned(),
                        excitation_magnitude: FiniteValue::new(1.0).unwrap(),
                    },
                    QualificationSample::FirstFrequencyPoint,
                )
                .unwrap(),
                QualificationOutputDefinition::try_new(
                    "voltage-magnitude",
                    QualificationProbe::AcNodeVoltageMagnitude {
                        node: "output".to_owned(),
                    },
                    QualificationSample::FirstFrequencyPoint,
                )
                .unwrap(),
            ],
            vec![
                QualificationReference::try_new("capacitance", 0.0, 1.0, 0.0).unwrap(),
                QualificationReference::try_new("voltage-magnitude", 1.0, 1.0e-10, 1.0e-10)
                    .unwrap(),
            ],
        );
        let noise_vector = executable_vector(
            "noise",
            "Output noise",
            "V1 input 0 DC 0 AC 1\nR1 input output 1k\nR2 output 0 1k\nM1 output input 0 0 demo180_nch\n.end\n",
            QualificationAnalysis::Noise {
                output_node: "output".to_owned(),
                output_reference: None,
                input_source: "V1".to_owned(),
                frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
                temperature_kelvin: FiniteValue::new(300.15).unwrap(),
            },
            vec![
                QualificationOutputDefinition::try_new(
                    "output-noise-density",
                    QualificationProbe::NoiseOutputDensity,
                    QualificationSample::FirstFrequencyPoint,
                )
                .unwrap(),
            ],
            vec![QualificationReference::try_new("output-noise-density", 0.0, 1.0, 0.0).unwrap()],
        );
        let transient_vector = executable_vector(
            "transient",
            "Transient voltage",
            "V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            QualificationAnalysis::Transient {
                stop_time: FiniteValue::new(1.0e-6).unwrap(),
                max_step: FiniteValue::new(1.0e-7).unwrap(),
            },
            vec![
                QualificationOutputDefinition::try_new(
                    "output-voltage",
                    QualificationProbe::TransientNodeVoltage {
                        node: "output".to_owned(),
                    },
                    QualificationSample::LastTimePoint,
                )
                .unwrap(),
            ],
            vec![QualificationReference::try_new("output-voltage", 1.0, 1.0e-8, 1.0e-8).unwrap()],
        );
        let suite = QualificationSuite::try_new(
            "advanced",
            "Advanced qualification",
            ObjectRevision::INITIAL,
            vec![ac_vector, noise_vector, transient_vector],
        )
        .unwrap();

        let run = QualificationExecutionService::execute_current_platform(
            &suite,
            &source(7),
            &rspice_core::NoAbort,
        )
        .unwrap();
        assert!(run.passed, "{run:#?}");
        for outcome in &run.vector_outcomes {
            assert!(outcome.outcome.failure.is_none(), "{outcome:#?}");
            assert!(
                outcome
                    .outcome
                    .references
                    .iter()
                    .all(|reference| reference.observed_value.get().is_finite()),
                "{outcome:#?}"
            );
        }
        let ac = run
            .vector_outcomes
            .iter()
            .find(|outcome| outcome.vector_id == "ac-cv")
            .unwrap();
        let voltage = ac
            .outcome
            .references
            .iter()
            .find(|reference| reference.quantity == "voltage-magnitude")
            .unwrap();
        assert!((voltage.observed_value.get() - 1.0).abs() <= 1.0e-10);
    }

    #[test]
    fn advanced_analysis_axes_and_probe_domains_fail_closed() {
        let mut vector = executable_vector(
            "axis",
            "Frequency axis",
            "V1 output 0 DC 0 AC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            QualificationAnalysis::AcSweep {
                frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
            },
            vec![
                QualificationOutputDefinition::try_new(
                    "frequency",
                    QualificationProbe::FrequencyValue,
                    QualificationSample::FirstFrequencyPoint,
                )
                .unwrap(),
            ],
            vec![QualificationReference::try_new("frequency", 1.0e3, 0.0, 0.0).unwrap()],
        );
        vector.analysis = QualificationAnalysis::AcSweep {
            frequencies: vec![
                FiniteValue::new(1.0e3).unwrap(),
                FiniteValue::new(1.0e3).unwrap(),
            ],
        };
        let error = QualificationSuite::try_new(
            "invalid-axis",
            "Invalid frequency axis",
            ObjectRevision::INITIAL,
            vec![vector.clone()],
        )
        .unwrap_err();
        assert_eq!(
            error.code,
            QualificationErrorCode::InvalidExecutionDefinition
        );

        vector.analysis = QualificationAnalysis::AcSweep {
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
        };
        vector.outputs[0].probe = QualificationProbe::NoiseOutputDensity;
        let error = QualificationSuite::try_new(
            "invalid-probe",
            "Invalid probe domain",
            ObjectRevision::INITIAL,
            vec![vector],
        )
        .unwrap_err();
        assert_eq!(
            error.code,
            QualificationErrorCode::InvalidExecutionDefinition
        );
    }

    #[test]
    fn execution_failures_are_retained_as_failing_platform_outcomes() {
        let source = source(7);
        let mut suite = suite();
        suite.vectors[0].outputs[0].probe = QualificationProbe::NodeVoltage {
            node: "missing-node".into(),
        };
        let run = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let failed = run
            .vector_outcomes
            .iter()
            .find(|value| value.vector_id == "dc-001")
            .unwrap();
        assert!(!failed.outcome.passed);
        assert_eq!(
            failed.outcome.failure.as_ref().unwrap().stage,
            QualificationFailureStage::Measurement
        );
        assert!(failed.outcome.references.is_empty());
        assert!(run.validate_bound(&suite, &source).is_ok());
    }

    #[test]
    fn missing_runtime_parity_cannot_be_promoted_to_evidence() {
        let source = source(7);
        let suite = suite();
        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let error = QualificationExecutionService::assemble_evidence(
            "evidence",
            &suite,
            &source,
            vec![desktop],
        )
        .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::EvidenceCoverageMismatch);
    }

    #[test]
    fn cancellation_publishes_no_partial_platform_run() {
        let result = QualificationExecutionService::execute_current_platform(
            &suite(),
            &source(7),
            &rspice_core::abort_signal::ImmediateAbort,
        );
        assert!(matches!(
            result,
            Err(QualificationExecutionError::Cancelled)
        ));
    }

    #[test]
    fn cooperative_session_runs_one_vector_per_step_and_cancels_atomically() {
        let source = source(7);
        let suite = suite();
        let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
        assert_eq!(session.progress().completed_vectors, 0);
        let step = session.step(&rspice_core::NoAbort).unwrap();
        let QualificationExecutionStep::InProgress(progress) = step else {
            panic!("a two-vector suite must not publish a run after one step");
        };
        assert_eq!(progress.completed_vectors, 1);
        assert!(!session.is_finished());
        session.cancel();
        assert!(session.is_cancelled());
        assert!(matches!(
            session.step(&rspice_core::NoAbort),
            Err(QualificationExecutionError::Cancelled)
        ));
    }

    #[test]
    fn cooperative_session_publishes_only_the_terminal_validated_run() {
        let source = source(7);
        let suite = suite();
        let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
        assert!(matches!(
            session.step(&rspice_core::NoAbort).unwrap(),
            QualificationExecutionStep::InProgress(_)
        ));
        let terminal = session.step(&rspice_core::NoAbort).unwrap();
        let QualificationExecutionStep::Complete { progress, run } = terminal else {
            panic!("the final vector must publish the complete platform run");
        };
        assert_eq!(progress.completed_vectors, suite.vectors.len());
        run.validate_bound(&suite, &source).unwrap();
        assert!(session.is_finished());
        assert!(matches!(
            session.step(&rspice_core::NoAbort),
            Err(QualificationExecutionError::SessionFinished)
        ));
    }

    #[test]
    fn assembled_evidence_is_deterministic_and_requires_real_run_records() {
        let source = source(7);
        let suite = suite();
        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let wasm_outcomes = desktop
            .vector_outcomes
            .iter()
            .map(|value| {
                let mut value = value.clone();
                value.outcome.platform = QualificationPlatform::WebAssembly;
                value
            })
            .collect();
        // Imported platform runs use the same validator as locally-produced
        // runs. Production code cannot relabel `execute_current_platform`.
        let wasm = QualificationPlatformRun::try_new(
            QualificationPlatform::WebAssembly,
            source.clone(),
            &suite,
            wasm_outcomes,
        )
        .unwrap();
        let first = QualificationExecutionService::assemble_evidence(
            "evidence",
            &suite,
            &source,
            vec![desktop.clone(), wasm.clone()],
        )
        .unwrap();
        let second = QualificationExecutionService::assemble_evidence(
            "evidence",
            &suite,
            &source,
            vec![wasm, desktop],
        )
        .unwrap();
        assert_eq!(first, second);
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap()
        );
    }

    #[test]
    fn schema_one_inputs_are_not_migrated_by_inventing_execution_contracts() {
        let legacy = serde_json::json!({
            "schema_version": 1,
            "id": "legacy",
            "name": "Legacy",
            "revision": 1,
            "vectors": [{
                "id": "legacy-vector",
                "name": "Legacy vector",
                "input_digest": digest(1).to_string(),
                "tolerances": [{"quantity": "out", "absolute": 0.1, "relative": 0.1}]
            }]
        });
        assert!(serde_json::from_value::<QualificationSuite>(legacy).is_err());
    }

    #[test]
    fn empty_legacy_state_migrates_without_inventing_evidence() {
        for version in [2, 3] {
            let mut serialized = serde_json::to_value(ModelQualificationState::default()).unwrap();
            serialized["schema_version"] = serde_json::json!(version);
            if version == 2 {
                serialized.as_object_mut().unwrap().remove("platform_runs");
            }
            let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
            assert_eq!(restored.schema_version, MODEL_QUALIFICATION_SCHEMA_VERSION);
            assert!(restored.platform_runs.is_empty());
            restored.validate().unwrap();
        }
    }

    #[test]
    fn populated_schema_two_state_is_loaded_for_diagnosis_but_not_upgraded() {
        let mut serialized = serde_json::to_value(
            ModelQualificationState::try_new(
                vec![suite()],
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
            .unwrap(),
        )
        .unwrap();
        serialized["schema_version"] = serde_json::json!(2);
        let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
        assert_eq!(restored.schema_version, 2);
        assert_eq!(
            restored.validate().unwrap_err().code,
            QualificationErrorCode::UnsupportedSchema
        );
    }

    #[test]
    fn populated_schema_three_state_is_not_upgraded_by_inventing_evidence_digests() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &evidence, &source);
        let mut serialized = serde_json::to_value(
            ModelQualificationState::try_new(
                vec![suite],
                Vec::new(),
                vec![evidence],
                vec![candidate],
                Vec::new(),
                Vec::new(),
            )
            .unwrap(),
        )
        .unwrap();
        serialized["schema_version"] = serde_json::json!(3);
        serialized["candidates"][0]
            .as_object_mut()
            .unwrap()
            .remove("evidence_digest");

        let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
        assert_eq!(restored.schema_version, 3);
        assert_eq!(restored.candidates[0].evidence_digest, None);
        assert_eq!(
            restored.validate().unwrap_err().code,
            QualificationErrorCode::UnsupportedSchema
        );
    }

    #[test]
    fn persisted_qualification_contracts_reject_unknown_fields() {
        let mut state = serde_json::to_value(ModelQualificationState::default()).unwrap();
        state["future_field"] = serde_json::json!(true);
        assert!(serde_json::from_value::<ModelQualificationState>(state).is_err());

        let mut suite_value = serde_json::to_value(suite()).unwrap();
        suite_value["vectors"][0]["future_field"] = serde_json::json!(true);
        assert!(serde_json::from_value::<QualificationSuite>(suite_value).is_err());
    }

    #[test]
    fn evidence_upsert_is_append_only_idempotent_and_atomic() {
        let suite = suite();
        let source = source(7);
        let original = evidence(&suite, &source, true);
        let release_candidate = candidate(&suite, &original, &source);
        let mut state = ModelQualificationState::try_new(
            vec![suite.clone()],
            Vec::new(),
            vec![original.clone()],
            vec![release_candidate],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let before = state.clone();
        let mut invalid = original.clone();
        invalid.vector_outcomes.pop();
        assert_eq!(
            state.upsert_evidence_atomically(invalid).unwrap_err().code,
            QualificationErrorCode::ImmutableRecord
        );
        assert_eq!(state, before);

        let replacement = evidence(&suite, &source, false);
        assert_eq!(
            state
                .upsert_evidence_atomically(replacement)
                .unwrap_err()
                .code,
            QualificationErrorCode::ImmutableRecord
        );
        assert_eq!(state, before);
        state.upsert_evidence_atomically(original).unwrap();
        assert_eq!(state, before);
        assert!(state.evidence[0].passed);
        state.validate().unwrap();
    }

    #[test]
    fn atomic_promotion_commits_release_and_record_or_rolls_back_duplicates() {
        let suite = suite();
        let source = source(7);
        let evidence = evidence(&suite, &source, true);
        let candidate = candidate(&suite, &evidence, &source);
        let candidate_id = candidate.identity.id.clone();
        let mut state = ModelQualificationState::try_new(
            vec![suite],
            Vec::new(),
            vec![evidence],
            vec![candidate],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let release_identity = ModelReleaseIdentity {
            id: "release-atomic".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        };
        state
            .promote_candidate_atomically(
                "promotion-atomic",
                release_identity.clone(),
                &candidate_id,
            )
            .unwrap();
        assert_eq!(state.releases.len(), 1);
        assert_eq!(state.promotions.len(), 1);
        state.validate().unwrap();

        let committed = state.clone();
        let repeated_candidate = state.promote_candidate_atomically(
            "promotion-second",
            ModelReleaseIdentity {
                id: "release-second".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.4".into(),
            },
            &candidate_id,
        );
        assert_eq!(
            repeated_candidate.unwrap_err().code,
            QualificationErrorCode::DuplicateId
        );
        assert_eq!(state, committed);

        let duplicate_promotion = state.promote_candidate_atomically(
            "promotion-atomic",
            ModelReleaseIdentity {
                id: "release-other".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.4".into(),
            },
            &candidate_id,
        );
        assert_eq!(
            duplicate_promotion.unwrap_err().code,
            QualificationErrorCode::DuplicateId
        );
        assert_eq!(state, committed);

        let duplicate_release =
            state.promote_candidate_atomically("promotion-other", release_identity, &candidate_id);
        assert_eq!(
            duplicate_release.unwrap_err().code,
            QualificationErrorCode::DuplicateId
        );
        assert_eq!(state, committed);
    }

    #[test]
    fn platform_runs_persist_and_exact_pair_assembles_evidence_atomically() {
        let suite = suite();
        let source = source(7);
        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let wasm = imported_platform_run(
            &desktop,
            QualificationPlatform::WebAssembly,
            &suite,
            &source,
        );
        let state = ModelQualificationState::try_new(
            vec![suite.clone()],
            vec![wasm, desktop],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        assert_eq!(
            state.platform_runs[0].platform,
            QualificationPlatform::Desktop
        );
        let json = serde_json::to_string(&state).unwrap();
        let mut restored: ModelQualificationState = serde_json::from_str(&json).unwrap();
        restored.validate().unwrap();
        assert_eq!(restored, state);
        let [desktop, wasm] = restored
            .exact_platform_run_pair(&suite.id, &source)
            .unwrap();
        assert_eq!(desktop.platform, QualificationPlatform::Desktop);
        assert_eq!(wasm.platform, QualificationPlatform::WebAssembly);

        let assembled = restored
            .assemble_and_upsert_evidence_atomically("assembled", &suite.id, &source)
            .unwrap();
        assert!(assembled.passed);
        assert_eq!(restored.evidence, vec![assembled]);
        restored.validate().unwrap();
    }

    #[test]
    fn stale_tampered_or_incomplete_platform_runs_roll_back() {
        let suite = suite();
        let source = source(7);
        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let mut state = ModelQualificationState::try_new(
            vec![suite.clone()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let empty = state.clone();

        let mut stale = desktop.clone();
        stale.suite_revision = stale.suite_revision.next().unwrap();
        assert_eq!(
            state
                .upsert_platform_run_atomically(stale)
                .unwrap_err()
                .code,
            QualificationErrorCode::SuiteBindingMismatch
        );
        assert_eq!(state, empty);

        let mut tampered = desktop.clone();
        tampered.vector_outcomes[0].input_digest = digest(99);
        assert_eq!(
            state
                .upsert_platform_run_atomically(tampered)
                .unwrap_err()
                .code,
            QualificationErrorCode::EvidenceCoverageMismatch
        );
        assert_eq!(state, empty);

        state
            .upsert_platform_run_atomically(desktop.clone())
            .unwrap();
        let desktop_only = state.clone();
        assert_eq!(
            state
                .assemble_and_upsert_evidence_atomically("incomplete", &suite.id, &source)
                .unwrap_err()
                .code,
            QualificationErrorCode::EvidenceCoverageMismatch
        );
        assert_eq!(state, desktop_only);

        let duplicate = ModelQualificationState::try_new(
            vec![suite],
            vec![desktop.clone(), desktop],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap_err();
        assert_eq!(duplicate.code, QualificationErrorCode::DuplicateId);
    }

    #[test]
    fn suite_and_vector_lifecycle_is_atomic_revisioned_and_evidence_safe() {
        let suite = suite();
        let source = source(7);
        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let mut state = ModelQualificationState::try_new(
            vec![suite.clone()],
            vec![desktop],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        assert_eq!(
            state
                .qualification_vector(&suite.id, "dc-001")
                .unwrap()
                .name,
            "Transfer sweep"
        );

        let replacement = vector_for_source("dc-001", "Transfer sweep revised", 9, 7);
        state
            .replace_vector_atomically(&suite.id, "dc-001", replacement)
            .unwrap();
        assert_eq!(
            state.qualification_suite(&suite.id).unwrap().revision,
            suite.revision.next().unwrap()
        );
        assert_eq!(
            state
                .qualification_vector(&suite.id, "dc-001")
                .unwrap()
                .name,
            "Transfer sweep revised"
        );
        assert!(state.platform_runs.is_empty());

        state.delete_vector_atomically(&suite.id, "dc-002").unwrap();
        assert_eq!(
            state.qualification_suite(&suite.id).unwrap().vectors.len(),
            1
        );
        let retained = state.clone();
        let error = state
            .delete_vector_atomically(&suite.id, "dc-001")
            .unwrap_err();
        assert_eq!(
            error.code,
            QualificationErrorCode::InvalidExecutionDefinition
        );
        assert_eq!(state, retained);

        state.delete_suite_atomically(&suite.id).unwrap();
        assert!(state.suites.is_empty());

        let evidence = evidence(&suite, &source, true);
        let release_candidate = candidate(&suite, &evidence, &source);
        let mut immutable = ModelQualificationState::try_new(
            vec![suite.clone()],
            Vec::new(),
            vec![evidence],
            vec![release_candidate],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let retained = immutable.clone();
        let error = immutable
            .replace_vector_atomically(
                &suite.id,
                "dc-001",
                vector_for_source("dc-001", "Forbidden edit", 9, 7),
            )
            .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::ImmutableRecord);
        assert_eq!(immutable, retained);
        assert_eq!(
            immutable
                .delete_suite_atomically(&suite.id)
                .unwrap_err()
                .code,
            QualificationErrorCode::ImmutableRecord
        );
        assert_eq!(immutable, retained);
    }

    #[test]
    fn failed_disposition_blocks_promotion_until_exact_cross_platform_rerun_passes() {
        let suite = suite();
        let source = source(7);
        let passing_evidence = evidence(&suite, &source, true);
        let release_candidate = candidate(&suite, &passing_evidence, &source);
        let candidate_id = release_candidate.identity.id.clone();
        let failed = failed_platform_run(&suite, &source, QualificationPlatform::Desktop);
        let mut state = ModelQualificationState::try_new(
            vec![suite.clone()],
            vec![failed],
            vec![passing_evidence],
            vec![release_candidate],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        state
            .record_vector_disposition_atomically(
                "disp-failed-dc-001",
                &suite.id,
                "dc-001",
                &source,
                QualificationVectorDispositionCause::Failed,
                QualificationVectorRequiredAction::Rerun,
                "Reference mismatch requires an exact parity rerun",
            )
            .unwrap();
        let retained = state.clone();
        let error = state
            .promote_candidate_atomically(
                "promotion-blocked",
                ModelReleaseIdentity {
                    id: "release-blocked".to_owned(),
                    model_id: source.model_id.clone(),
                    version: "4.8.3".to_owned(),
                },
                &candidate_id,
            )
            .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::PromotionBlocked);
        assert_eq!(state, retained);
        assert_eq!(
            state
                .resolve_vector_disposition_by_rerun_atomically("disp-failed-dc-001")
                .unwrap_err()
                .code,
            QualificationErrorCode::DispositionInvalid
        );
        assert_eq!(state, retained);

        let desktop = QualificationExecutionService::execute_current_platform(
            &suite,
            &source,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let wasm = imported_platform_run(
            &desktop,
            QualificationPlatform::WebAssembly,
            &suite,
            &source,
        );
        state.upsert_platform_run_atomically(desktop).unwrap();
        state.upsert_platform_run_atomically(wasm).unwrap();
        state
            .resolve_vector_disposition_by_rerun_atomically("disp-failed-dc-001")
            .unwrap();
        assert!(!state.vector_dispositions[0].is_open());
        state
            .promote_candidate_atomically(
                "promotion-after-rerun",
                ModelReleaseIdentity {
                    id: "release-after-rerun".to_owned(),
                    model_id: source.model_id.clone(),
                    version: "4.8.3".to_owned(),
                },
                &candidate_id,
            )
            .unwrap();
        assert_eq!(state.promotions.len(), 1);
    }

    #[test]
    fn stale_dispositions_cannot_be_waived_by_rerun_and_retirement_is_explicit() {
        let suite = suite();
        let current_source = source(8);
        let mut state = ModelQualificationState::try_new(
            vec![suite.clone()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let empty = state.clone();
        let error = state
            .record_vector_disposition_atomically(
                "stale-rerun",
                &suite.id,
                "dc-001",
                &current_source,
                QualificationVectorDispositionCause::Stale,
                QualificationVectorRequiredAction::Rerun,
                "Source revision changed",
            )
            .unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::DispositionInvalid);
        assert_eq!(state, empty);

        state
            .record_vector_disposition_atomically(
                "stale-retire",
                &suite.id,
                "dc-001",
                &current_source,
                QualificationVectorDispositionCause::Stale,
                QualificationVectorRequiredAction::Retire,
                "The old source-bound vector is outside the current candidate",
            )
            .unwrap();
        assert!(state.vector_dispositions[0].is_open());
        state.delete_suite_atomically(&suite.id).unwrap();
        assert_eq!(
            state.vector_dispositions[0].resolution,
            Some(QualificationVectorDispositionResolution::Retired)
        );
        state.validate().unwrap();
    }
}
