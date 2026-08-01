//! One qualification vector: what it asks for, and when it is well-formed.
//!
//! A vector names an analysis, the probes and samples taken from it, and the
//! reference values with the tolerance they must be met within.  Probe and
//! sample compatibility is checked against the analysis rather than assumed,
//! so a vector that could never produce the quantity it compares against is
//! rejected at definition time instead of failing as a qualification result.

use super::*;

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
#[cfg(test)]
pub struct ReferenceTolerance {
    pub quantity: String,
    pub absolute: NonNegativeFinite,
    pub relative: NonNegativeFinite,
}

#[cfg(test)]
impl ReferenceTolerance {
    #[cfg(test)]
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
    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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
    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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

    pub(super) fn validate(&self, path: &str) -> QualificationResult<()> {
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

    pub(super) fn validate_source_binding(
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
