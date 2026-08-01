//! Analysis drafts.
//!
//! The in-progress form of each analysis type as the user edits it, before
//! it becomes a validated specification.

use serde::{Deserialize, Serialize};

use crate::simulation::config::{
    AcSweepType, NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
use crate::simulation::dependency_contract::{
    FourierTransientRequirement, PeriodicStateCapability, TransientCapability,
    validate_fourier_transient_contract, validate_periodic_state_contract,
};
use crate::simulation::dialog::{
    CornerDialogState, EnvelopeDialogState, FourierDialogState, HbDialogState, McDialogState,
    NoiseReferenceType, OpDialogState, OptimizationDialogState, PacDialogState, PnoiseDialogState,
    PssDialogState, PssSolverMethod, PstbDialogState, PxfDialogState, PzDialogState,
    ReliabilityDialogState, SensDialogState, SoaDialogState, SpDialogState, StbDialogState,
    TempDialogState, XfDialogState,
};
use crate::simulation::spice_value::parse_spice_value_checked;
use crate::workbench::app_state::{AcSetup, DcSetup, TranSetup};

use super::AnalysisKind;

/// AC sweep draft shared structurally by AC and DISTO, but never shared by
/// identity. Each analysis instance owns a deep copy.
pub type AcDraft = AcSetup;

/// Noise draft with an independent, losslessly persisted analysis contract.
///
/// The legacy singleton setup borrowed these two values from AC. Keeping them
/// here removes that hidden cross-analysis mutation. Raw text buffers are
/// retained so an unfinished edit can still round-trip through project save.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseDraft {
    pub output: String,
    /// Legacy output reference. New drafts use `output` for either a
    /// single-ended node or a comma-separated differential expression.
    #[serde(default = "default_noise_reference")]
    pub reference: String,
    pub input: String,
    pub fstart: String,
    pub fstop: String,
    pub points: String,
    pub sweep: NoiseSweepType,
    /// Comma/space-separated frequency values used only by the explicit mode.
    #[serde(default = "default_noise_frequency_list")]
    pub explicit_frequencies: String,
    #[serde(default)]
    pub contribution_detail: NoiseContributionDetail,
    #[serde(default)]
    pub integration_mode: NoiseIntegrationMode,
}

impl Default for NoiseDraft {
    fn default() -> Self {
        Self {
            output: "afe_out".to_owned(),
            reference: default_noise_reference(),
            input: "VIN_DIFF".to_owned(),
            fstart: "10".to_owned(),
            fstop: "1Meg".to_owned(),
            points: "30".to_owned(),
            sweep: NoiseSweepType::Decade,
            explicit_frequencies: default_noise_frequency_list(),
            contribution_detail: NoiseContributionDetail::Top50,
            integration_mode: NoiseIntegrationMode::Enabled,
        }
    }
}

fn default_noise_reference() -> String {
    "0".to_owned()
}

fn default_noise_frequency_list() -> String {
    "10, 100, 1k, 10k, 100k, 1Meg".to_owned()
}

impl NoiseDraft {
    /// Parse this raw draft into a complete executable configuration.
    pub fn to_config(&self) -> Result<NoiseAnalysisConfig, String> {
        let (output_node, reference_node) =
            parse_noise_output_expression(&self.output, &self.reference)?;
        let input_source = self.input.trim().to_owned();
        if input_source.is_empty() {
            return Err("input source is required".to_owned());
        }

        let (sweep_type, num_points, start_freq, stop_freq, explicit_frequencies) = match self.sweep
        {
            NoiseSweepType::Decade | NoiseSweepType::Octave | NoiseSweepType::Linear => {
                let num_points = parse_positive_usize(&self.points, "noise point count")?;
                let start_freq = parse_positive(&self.fstart, "noise start frequency")?;
                let stop_freq = parse_positive(&self.fstop, "noise stop frequency")?;
                if stop_freq <= start_freq {
                    return Err(
                        "noise stop frequency must be greater than start frequency".to_owned()
                    );
                }
                let sweep_type = match self.sweep {
                    NoiseSweepType::Decade => AcSweepType::Decade,
                    NoiseSweepType::Octave => AcSweepType::Octave,
                    NoiseSweepType::Linear => AcSweepType::Linear,
                    _ => unreachable!("matched fixed noise sweep"),
                };
                (sweep_type, num_points, start_freq, stop_freq, None)
            }
            NoiseSweepType::ExplicitFrequencyList => {
                let frequencies = parse_noise_frequency_list(&self.explicit_frequencies)?;
                let start = frequencies.first().copied().unwrap_or_default();
                let stop = frequencies.last().copied().unwrap_or_default();
                (
                    AcSweepType::Decade,
                    frequencies.len(),
                    start,
                    stop,
                    Some(frequencies),
                )
            }
            NoiseSweepType::Unsupported(index) => {
                return Err(format!(
                    "noise sweep mode {index} is outside the supported schema"
                ));
            }
        };

        let config = NoiseAnalysisConfig {
            output_node,
            reference_node,
            input_source,
            sweep_type,
            num_points,
            start_freq,
            stop_freq,
            explicit_frequencies,
            data_table_name: None,
            contribution_detail: self.contribution_detail,
            integration_mode: self.integration_mode,
            temperature_kelvin: rspice_core::constants::TEMP_REFERENCE,
        };
        config
            .validate()
            .map_err(|errors| errors.join("; "))
            .map(|()| config)
    }
}

fn parse_noise_output_expression(
    output: &str,
    legacy_reference: &str,
) -> Result<(String, String), String> {
    let output = output.trim();
    if output.is_empty() {
        return Err("output node or expression is required".to_owned());
    }
    let has_voltage_prefix = output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("v("));
    if has_voltage_prefix != output.ends_with(')') {
        return Err("voltage output expressions must use V(node) or V(node,reference)".to_owned());
    }
    let inner = if has_voltage_prefix {
        &output[2..output.len() - 1]
    } else {
        output
    };
    let mut nodes = inner.split(',').map(str::trim);
    let positive = nodes.next().unwrap_or_default();
    let explicit_reference = nodes.next();
    if positive.is_empty() || nodes.next().is_some() {
        return Err(
            "output expression must name one node or one differential node pair".to_owned(),
        );
    }
    let reference = explicit_reference.unwrap_or_else(|| legacy_reference.trim());
    if reference.contains(',') || explicit_reference.is_some_and(str::is_empty) {
        return Err("output reference must name exactly one node".to_owned());
    }
    Ok((
        positive.to_owned(),
        if reference.is_empty() {
            default_noise_reference()
        } else {
            reference.to_owned()
        },
    ))
}

fn parse_noise_frequency_list(text: &str) -> Result<Vec<f64>, String> {
    let values = text
        .split(|character: char| character == ',' || character == ';' || character.is_whitespace())
        .filter(|value| !value.is_empty())
        .map(|value| {
            parse_spice_value_checked(value)
                .map_err(|error| format!("invalid explicit noise frequency '{value}': {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if values.is_empty() {
        return Err("explicit frequency list must contain at least one value".to_owned());
    }
    let mut previous = None;
    for value in &values {
        if *value <= 0.0 {
            return Err("explicit noise frequencies must be greater than zero".to_owned());
        }
        if previous.is_some_and(|previous| *value <= previous) {
            return Err(
                "explicit noise frequencies must be strictly increasing without duplicates"
                    .to_owned(),
            );
        }
        previous = Some(*value);
    }
    Ok(values)
}

/// DISTO draft with its own AC sweep and optional second-tone ratio.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistoDraft {
    pub sweep: AcDraft,
    /// Empty or `auto` selects single-tone harmonic distortion.
    pub f2_over_f1: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencySweepDraft {
    pub start: String,
    pub stop: String,
    pub points: String,
    /// 0 = decade, 1 = octave, 2 = linear.
    pub sweep: usize,
}

impl Default for FrequencySweepDraft {
    fn default() -> Self {
        Self {
            start: "1k".to_owned(),
            stop: "10G".to_owned(),
            points: "101".to_owned(),
            sweep: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkPortDraft {
    pub node_pos: String,
    pub node_neg: String,
    pub z0: String,
}

impl Default for NetworkPortDraft {
    fn default() -> Self {
        Self {
            node_pos: "in".to_owned(),
            node_neg: "0".to_owned(),
            z0: "50".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssDraft {
    pub tones: String,
    pub harmonics: String,
    pub max_iterations: String,
    pub relative_tolerance: String,
    pub autonomous: bool,
    pub oscillator_node: String,
}

impl Default for QpssDraft {
    fn default() -> Self {
        Self {
            tones: "1G, 1.001G".to_owned(),
            harmonics: "7, 7".to_owned(),
            max_iterations: "100".to_owned(),
            relative_tolerance: "1e-6".to_owned(),
            autonomous: false,
            oscillator_node: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNetworkDraft {
    pub sweep: FrequencySweepDraft,
    pub ports: Vec<NetworkPortDraft>,
    pub max_sideband: String,
    pub mixed_mode: bool,
    pub noise_parameters: bool,
}

impl Default for PeriodicNetworkDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            ports: vec![NetworkPortDraft::default()],
            max_sideband: "7".to_owned(),
            mixed_mode: false,
            noise_parameters: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HbNoiseDraft {
    pub sweep: FrequencySweepDraft,
    pub output_node: String,
    pub output_ref: String,
    pub input_source: String,
    pub max_sideband: String,
    pub integrated_noise: bool,
    pub noise_figure: bool,
    pub contributor_ranking: bool,
}

impl Default for HbNoiseDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "V1".to_owned(),
            max_sideband: "7".to_owned(),
            integrated_noise: true,
            noise_figure: true,
            contributor_ranking: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicAcDraft {
    pub sweep: FrequencySweepDraft,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: String,
    pub input_lattice: String,
    pub output_lattice: String,
}

impl Default for QuasiPeriodicAcDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            input_source: "V1".to_owned(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_lattice: "0, 0".to_owned(),
            output_lattice: "0, 0".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicNoiseDraft {
    pub sweep: FrequencySweepDraft,
    pub output_node: String,
    pub output_ref: String,
    pub input_source: String,
    pub lattice_products: String,
    pub integrated_noise: bool,
    pub contributor_ranking: bool,
}

impl Default for QuasiPeriodicNoiseDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "V1".to_owned(),
            lattice_products: "-3:3, -3:3".to_owned(),
            integrated_noise: true,
            contributor_ranking: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicTransferDraft {
    pub sweep: FrequencySweepDraft,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: String,
    pub input_lattice: String,
    pub output_lattice: String,
    pub group_delay: bool,
}

impl Default for QuasiPeriodicTransferDraft {
    fn default() -> Self {
        Self {
            sweep: FrequencySweepDraft::default(),
            input_source: "V1".to_owned(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_lattice: "0, 0".to_owned(),
            output_lattice: "0, 0".to_owned(),
            group_delay: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientNoiseDraft {
    pub stop_time: String,
    pub step_time: String,
    pub start_time: String,
    pub max_step: String,
    pub seed: String,
    pub noise_fmax: String,
    pub scale: String,
    pub use_initial_conditions: bool,
}

impl Default for TransientNoiseDraft {
    fn default() -> Self {
        Self {
            stop_time: "1u".to_owned(),
            step_time: "1n".to_owned(),
            start_time: "0".to_owned(),
            max_step: "10n".to_owned(),
            seed: "1".to_owned(),
            noise_fmax: "10G".to_owned(),
            scale: "1".to_owned(),
            use_initial_conditions: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcMismatchDraft {
    pub output_expression: String,
    pub sigma_multiplier: String,
    pub contributor_limit: String,
    pub include_process: bool,
    pub include_mismatch: bool,
    pub normalized_contributions: bool,
}

impl Default for DcMismatchDraft {
    fn default() -> Self {
        Self {
            output_expression: "V(out)".to_owned(),
            sigma_multiplier: "3".to_owned(),
            contributor_limit: "25".to_owned(),
            include_process: false,
            include_mismatch: true,
            normalized_contributions: true,
        }
    }
}

/// Raw, lossless configuration draft for one executable analysis instance.
///
/// Drafts intentionally retain strings rather than parsed engine values so a
/// project can round-trip an in-progress edit without silently rewriting it.
/// The enum tag is the stable analysis ID; legacy numeric indices are accepted
/// only through [`Self::from_legacy_index`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "draft")]
pub enum AnalysisDraft {
    #[serde(rename = "op")]
    OperatingPoint(OpDialogState),
    #[serde(rename = "tran")]
    Transient(TranSetup),
    #[serde(rename = "ac")]
    Ac(AcDraft),
    #[serde(rename = "dc")]
    DcSweep(DcSetup),
    #[serde(rename = "noise")]
    Noise(NoiseDraft),
    #[serde(rename = "pz")]
    PoleZero(PzDialogState),
    #[serde(rename = "sens")]
    Sensitivity(SensDialogState),
    #[serde(rename = "mc")]
    MonteCarlo(McDialogState),
    #[serde(rename = "pss")]
    Pss(PssDialogState),
    #[serde(rename = "stb")]
    Stb(StbDialogState),
    #[serde(rename = "temp")]
    Temperature(TempDialogState),
    #[serde(rename = "hb")]
    HarmonicBalance(HbDialogState),
    #[serde(rename = "sp")]
    SParameter(SpDialogState),
    #[serde(rename = "pac")]
    Pac(PacDialogState),
    #[serde(rename = "pnoise")]
    Pnoise(PnoiseDialogState),
    #[serde(rename = "pxf")]
    Pxf(PxfDialogState),
    #[serde(rename = "pstb")]
    Pstb(PstbDialogState),
    #[serde(rename = "xf")]
    TransferFunction(XfDialogState),
    #[serde(rename = "corner")]
    Corner(CornerDialogState),
    #[serde(rename = "envelope")]
    Envelope(EnvelopeDialogState),
    #[serde(rename = "fourier")]
    Fourier(FourierDialogState),
    #[serde(rename = "reliability")]
    Reliability(ReliabilityDialogState),
    #[serde(rename = "opt")]
    Optimization(OptimizationDialogState),
    #[serde(rename = "soa")]
    Soa(SoaDialogState),
    #[serde(rename = "disto")]
    Disto(DistoDraft),
    #[serde(rename = "qpss")]
    Qpss(QpssDraft),
    #[serde(rename = "hbsp")]
    Hbsp(PeriodicNetworkDraft),
    #[serde(rename = "hbnoise")]
    Hbnoise(HbNoiseDraft),
    #[serde(rename = "psp")]
    Psp(PeriodicNetworkDraft),
    #[serde(rename = "qpac")]
    Qpac(QuasiPeriodicAcDraft),
    #[serde(rename = "qpnoise")]
    Qpnoise(QuasiPeriodicNoiseDraft),
    #[serde(rename = "qpxf")]
    Qpxf(QuasiPeriodicTransferDraft),
    #[serde(rename = "tnoise")]
    TransientNoise(TransientNoiseDraft),
    #[serde(rename = "dcmatch")]
    DcMismatch(DcMismatchDraft),
}

macro_rules! initialized_default {
    ($type:ty) => {{
        let mut draft = <$type>::default();
        draft.ensure_initialized();
        draft
    }};
}

impl AnalysisDraft {
    /// Construct the semantic fresh-dialog default for a kind.
    #[must_use]
    pub fn for_kind(kind: AnalysisKind) -> Self {
        match kind {
            AnalysisKind::OperatingPoint => {
                Self::OperatingPoint(initialized_default!(OpDialogState))
            }
            AnalysisKind::Transient => Self::Transient(TranSetup::default()),
            AnalysisKind::Ac => Self::Ac(AcDraft::default()),
            AnalysisKind::DcSweep => Self::DcSweep(DcSetup::default()),
            AnalysisKind::Noise => Self::Noise(NoiseDraft::default()),
            AnalysisKind::PoleZero => Self::PoleZero(initialized_default!(PzDialogState)),
            AnalysisKind::Sensitivity => Self::Sensitivity(initialized_default!(SensDialogState)),
            AnalysisKind::MonteCarlo => Self::MonteCarlo(initialized_default!(McDialogState)),
            AnalysisKind::Pss => Self::Pss(initialized_default!(PssDialogState)),
            AnalysisKind::Stb => Self::Stb(initialized_default!(StbDialogState)),
            AnalysisKind::Temperature => Self::Temperature(initialized_default!(TempDialogState)),
            AnalysisKind::HarmonicBalance => {
                Self::HarmonicBalance(initialized_default!(HbDialogState))
            }
            AnalysisKind::SParameter => Self::SParameter(initialized_default!(SpDialogState)),
            AnalysisKind::Pac => Self::Pac(initialized_default!(PacDialogState)),
            AnalysisKind::Pnoise => Self::Pnoise(initialized_default!(PnoiseDialogState)),
            AnalysisKind::Pxf => Self::Pxf(initialized_default!(PxfDialogState)),
            AnalysisKind::Pstb => Self::Pstb(initialized_default!(PstbDialogState)),
            AnalysisKind::TransferFunction => {
                Self::TransferFunction(initialized_default!(XfDialogState))
            }
            AnalysisKind::Corner => Self::Corner(initialized_default!(CornerDialogState)),
            AnalysisKind::Envelope => Self::Envelope(initialized_default!(EnvelopeDialogState)),
            AnalysisKind::Fourier => Self::Fourier(initialized_default!(FourierDialogState)),
            AnalysisKind::Reliability => {
                Self::Reliability(initialized_default!(ReliabilityDialogState))
            }
            AnalysisKind::Optimization => {
                Self::Optimization(initialized_default!(OptimizationDialogState))
            }
            AnalysisKind::Soa => Self::Soa(initialized_default!(SoaDialogState)),
            AnalysisKind::Disto => Self::Disto(DistoDraft::default()),
            AnalysisKind::Qpss => Self::Qpss(QpssDraft::default()),
            AnalysisKind::Hbsp => Self::Hbsp(PeriodicNetworkDraft::default()),
            AnalysisKind::Hbnoise => Self::Hbnoise(HbNoiseDraft::default()),
            AnalysisKind::Psp => Self::Psp(PeriodicNetworkDraft::default()),
            AnalysisKind::Qpac => Self::Qpac(QuasiPeriodicAcDraft::default()),
            AnalysisKind::Qpnoise => Self::Qpnoise(QuasiPeriodicNoiseDraft::default()),
            AnalysisKind::Qpxf => Self::Qpxf(QuasiPeriodicTransferDraft::default()),
            AnalysisKind::TransientNoise => Self::TransientNoise(TransientNoiseDraft::default()),
            AnalysisKind::DcMismatch => Self::DcMismatch(DcMismatchDraft::default()),
        }
    }

    /// Construct a semantic draft from a current singleton-model index.
    #[cfg(test)]
    pub fn from_legacy_index(index: usize) -> Option<Self> {
        AnalysisKind::from_legacy_index(index).map(Self::for_kind)
    }

    /// Exact kind carried by this tagged draft.
    #[must_use]
    pub const fn kind(&self) -> AnalysisKind {
        match self {
            Self::OperatingPoint(_) => AnalysisKind::OperatingPoint,
            Self::Transient(_) => AnalysisKind::Transient,
            Self::Ac(_) => AnalysisKind::Ac,
            Self::DcSweep(_) => AnalysisKind::DcSweep,
            Self::Noise(_) => AnalysisKind::Noise,
            Self::PoleZero(_) => AnalysisKind::PoleZero,
            Self::Sensitivity(_) => AnalysisKind::Sensitivity,
            Self::MonteCarlo(_) => AnalysisKind::MonteCarlo,
            Self::Pss(_) => AnalysisKind::Pss,
            Self::Stb(_) => AnalysisKind::Stb,
            Self::Temperature(_) => AnalysisKind::Temperature,
            Self::HarmonicBalance(_) => AnalysisKind::HarmonicBalance,
            Self::SParameter(_) => AnalysisKind::SParameter,
            Self::Pac(_) => AnalysisKind::Pac,
            Self::Pnoise(_) => AnalysisKind::Pnoise,
            Self::Pxf(_) => AnalysisKind::Pxf,
            Self::Pstb(_) => AnalysisKind::Pstb,
            Self::TransferFunction(_) => AnalysisKind::TransferFunction,
            Self::Corner(_) => AnalysisKind::Corner,
            Self::Envelope(_) => AnalysisKind::Envelope,
            Self::Fourier(_) => AnalysisKind::Fourier,
            Self::Reliability(_) => AnalysisKind::Reliability,
            Self::Optimization(_) => AnalysisKind::Optimization,
            Self::Soa(_) => AnalysisKind::Soa,
            Self::Disto(_) => AnalysisKind::Disto,
            Self::Qpss(_) => AnalysisKind::Qpss,
            Self::Hbsp(_) => AnalysisKind::Hbsp,
            Self::Hbnoise(_) => AnalysisKind::Hbnoise,
            Self::Psp(_) => AnalysisKind::Psp,
            Self::Qpac(_) => AnalysisKind::Qpac,
            Self::Qpnoise(_) => AnalysisKind::Qpnoise,
            Self::Qpxf(_) => AnalysisKind::Qpxf,
            Self::TransientNoise(_) => AnalysisKind::TransientNoise,
            Self::DcMismatch(_) => AnalysisKind::DcMismatch,
        }
    }

    /// Exact prerequisite roles required by this configured draft.
    ///
    /// Envelope owns its selected periodic initializer as part of one
    /// authenticated execution. Exposing a separate HB/PSS task here would be
    /// incorrect until that task can publish a typed continuation artifact
    /// that Envelope actually consumes.
    #[must_use]
    pub fn prerequisite_roles(&self) -> &'static [AnalysisKind] {
        self.kind().prerequisites()
    }

    /// Current singleton-model index, for deterministic migration only.
    #[must_use]
    pub const fn legacy_index(&self) -> usize {
        self.kind().legacy_index()
    }

    /// Restore runtime-only lazy initialization sentinels after deserialization.
    ///
    /// This sets only the skipped sentinel and never calls `ensure_initialized`,
    /// because doing so would overwrite persisted raw edits with defaults.
    pub fn prepare_after_restore(&mut self) {
        match self {
            Self::OperatingPoint(state) => state.initialized = true,
            Self::PoleZero(state) => state.initialized = true,
            Self::Sensitivity(state) => state.initialized = true,
            Self::MonteCarlo(state) => state.initialized = true,
            Self::Pss(state) => state.initialized = true,
            Self::Stb(state) => state.initialized = true,
            Self::Temperature(state) => state.initialized = true,
            Self::HarmonicBalance(state) => state.initialized = true,
            Self::SParameter(state) => state.initialized = true,
            Self::Pac(state) => state.initialized = true,
            Self::Pnoise(state) => state.initialized = true,
            Self::Pxf(state) => state.initialized = true,
            Self::Pstb(state) => state.initialized = true,
            Self::TransferFunction(state) => state.prepare_after_restore(),
            Self::Corner(state) => state.initialized = true,
            Self::Envelope(state) => state.initialized = true,
            Self::Fourier(state) => state.initialized = true,
            Self::Reliability(state) => state.initialized = true,
            Self::Optimization(state) => state.initialized = true,
            Self::Soa(state) => state.initialized = true,
            Self::Transient(_)
            | Self::Ac(_)
            | Self::DcSweep(_)
            | Self::Noise(_)
            | Self::Disto(_)
            | Self::Qpss(_)
            | Self::Hbsp(_)
            | Self::Hbnoise(_)
            | Self::Psp(_)
            | Self::Qpac(_)
            | Self::Qpnoise(_)
            | Self::Qpxf(_)
            | Self::TransientNoise(_)
            | Self::DcMismatch(_) => {}
        }
    }

    /// Validate drafts whose engines are not yet available without projecting
    /// them through the retired singleton configuration model.
    pub fn manifest_configuration_error(&self) -> Option<String> {
        match self {
            Self::Noise(draft) => draft.to_config().err(),
            Self::Qpss(draft) => validate_qpss(draft),
            Self::Hbsp(draft) | Self::Psp(draft) => validate_periodic_network(draft),
            Self::Hbnoise(draft) => validate_hbnoise(draft),
            Self::Qpac(draft) => validate_qpac(draft),
            Self::Qpnoise(draft) => validate_qpnoise(draft),
            Self::Qpxf(draft) => validate_qpxf(draft),
            Self::TransientNoise(draft) => validate_transient_noise(draft),
            Self::DcMismatch(draft) => validate_dc_mismatch(draft),
            _ => None,
        }
    }

    pub fn manifest_summary(&self) -> Option<String> {
        match self {
            Self::Qpss(draft) => Some(format!(
                "tones {} · harmonics {}",
                draft.tones, draft.harmonics
            )),
            Self::Hbsp(draft) | Self::Psp(draft) => Some(format!(
                "{}…{} · {} points · {} port{}",
                draft.sweep.start,
                draft.sweep.stop,
                draft.sweep.points,
                draft.ports.len(),
                if draft.ports.len() == 1 { "" } else { "s" }
            )),
            Self::Hbnoise(draft) => Some(format!(
                "{}→{} · {}…{} · sidebands {}",
                draft.input_source,
                draft.output_node,
                draft.sweep.start,
                draft.sweep.stop,
                draft.max_sideband
            )),
            Self::Qpac(draft) => Some(format!(
                "{}→{} · {}…{}",
                draft.input_source, draft.output_node, draft.sweep.start, draft.sweep.stop
            )),
            Self::Qpnoise(draft) => Some(format!(
                "{} · {}…{} · lattice {}",
                draft.output_node, draft.sweep.start, draft.sweep.stop, draft.lattice_products
            )),
            Self::Qpxf(draft) => Some(format!(
                "{}→{} · {} to {}",
                draft.input_source, draft.output_node, draft.input_lattice, draft.output_lattice
            )),
            Self::TransientNoise(draft) => Some(format!(
                "stop {} · step {} · seed {}",
                draft.stop_time, draft.step_time, draft.seed
            )),
            Self::DcMismatch(draft) => Some(format!(
                "{} · {}σ · top {}",
                draft.output_expression, draft.sigma_multiplier, draft.contributor_limit
            )),
            Self::TransferFunction(draft) => Some(format!(
                "{} <- {} - DC operating point",
                draft.output_expression, draft.input_source
            )),
            _ => None,
        }
    }
}

fn fourier_requirement(draft: &FourierDialogState) -> Result<FourierTransientRequirement, String> {
    let config = draft.to_config()?;
    Ok(FourierTransientRequirement {
        start_time: config.start_time,
        stop_time: config.stop_time,
        fundamental_freq: config.fundamental_freq,
        num_harmonics: config.num_harmonics,
    })
}

fn transient_capability(draft: &TranSetup) -> Result<TransientCapability, String> {
    let max_timestep = match draft.max_step.trim() {
        "" => None,
        value if value.eq_ignore_ascii_case("auto") => None,
        value => Some(parse_spice_value_checked(value)?),
    };
    Ok(TransientCapability {
        start_time: parse_spice_value_checked(&draft.start)?,
        stop_time: parse_spice_value_checked(&draft.stop)?,
        step_time: parse_spice_value_checked(&draft.step)?,
        max_timestep,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DependencyConfigurationIssue {
    InvalidDependent(String),
    InvalidPrerequisite(String),
    Incompatible(String),
}

/// Circuit-derived inputs required to synthesize or reuse prerequisites whose
/// configuration depends on elaborated source identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisDependencyRepairContext {
    periodic_sources: Result<ExactPeriodicSourceContract, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExactPeriodicSourceContract {
    names: Vec<String>,
    netlist_source: String,
}

impl AnalysisDependencyRepairContext {
    /// Context for callers that cannot authenticate the circuit source set.
    #[must_use]
    pub fn periodic_sources_unavailable(detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let detail = detail.trim();
        Self {
            periodic_sources: Err(if detail.is_empty() {
                "the exact elaborated periodic-source catalog is unavailable".to_owned()
            } else {
                detail.to_owned()
            }),
        }
    }

    /// Authenticate and retain the exact elaborated source contract used by
    /// PSS. Keeping the executable source with the discovered identities lets
    /// repair validate waveform periodicity and commensurability against the
    /// candidate fundamental before committing a lifecycle transaction.
    ///
    /// An empty, successfully elaborated catalog is authoritative: it can
    /// validate an autonomous PSS with no driven tones. It must remain
    /// distinct from an unavailable catalog, because only the latter prevents
    /// safe reuse of an existing oscillator prerequisite.
    pub fn exact_periodic_sources(netlist_source: impl Into<String>) -> Result<Self, String> {
        let netlist_source = netlist_source.into();
        let netlist = rspice_core::Netlist::parse(&netlist_source)
            .map_err(|error| format!("the periodic-source circuit could not be parsed: {error}"))?;
        let names = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .transient_source_names(&netlist)
            .map_err(|error| {
                format!("the periodic-source circuit could not be elaborated: {error}")
            })?;
        Ok(Self {
            periodic_sources: Ok(ExactPeriodicSourceContract {
                names,
                netlist_source,
            }),
        })
    }

    fn periodic_sources(&self) -> Result<&[String], String> {
        self.periodic_sources
            .as_ref()
            .map(|contract| contract.names.as_slice())
            .map_err(|detail| detail.clone())
    }

    pub(super) fn availability_error(&self) -> Option<&str> {
        self.periodic_sources.as_ref().err().map(String::as_str)
    }

    pub(crate) fn validate_pss_sources(&self, draft: &PssDialogState) -> Result<(), String> {
        let config = draft
            .to_config()
            .map_err(|detail| format!("PSS configuration is invalid: {detail}"))?;
        self.validate_periodic_source_selection(&config.tone_sources)?;
        let contract = self
            .periodic_sources
            .as_ref()
            .map_err(|detail| detail.clone())?;
        let netlist = rspice_core::Netlist::parse(&contract.netlist_source).map_err(|error| {
            format!("the authenticated periodic-source circuit is no longer parseable: {error}")
        })?;
        rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .validate_periodic_source_contract(&netlist, &config.tone_sources, config.fund_freq)
            .map_err(|error| format!("PSS periodic-source contract is invalid: {error}"))
    }

    fn validate_periodic_source_selection(&self, selected: &[String]) -> Result<(), String> {
        let expected = self.periodic_sources()?;
        let missing = selected
            .iter()
            .filter(|requested| {
                !expected
                    .iter()
                    .any(|available| available.eq_ignore_ascii_case(requested))
            })
            .cloned()
            .collect::<Vec<_>>();
        let omitted = expected
            .iter()
            .filter(|available| {
                !selected
                    .iter()
                    .any(|requested| requested.eq_ignore_ascii_case(available))
            })
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() || !omitted.is_empty() {
            let mut details = Vec::new();
            if !missing.is_empty() {
                details.push(format!("unknown: {}", missing.join(", ")));
            }
            if !omitted.is_empty() {
                details.push(format!("omitted: {}", omitted.join(", ")));
            }
            return Err(format!(
                "PSS tones do not match the exact elaborated periodic-source catalog ({})",
                details.join("; ")
            ));
        }
        Ok(())
    }
}

impl Default for AnalysisDependencyRepairContext {
    fn default() -> Self {
        Self::periodic_sources_unavailable(
            "the exact elaborated periodic-source catalog is unavailable",
        )
    }
}

impl DependencyConfigurationIssue {
    pub(super) fn detail(&self) -> &str {
        match self {
            Self::InvalidDependent(detail)
            | Self::InvalidPrerequisite(detail)
            | Self::Incompatible(detail) => detail,
        }
    }
}

pub(super) fn dependency_configuration_issue(
    dependent: &AnalysisDraft,
    prerequisite: &AnalysisDraft,
) -> Option<DependencyConfigurationIssue> {
    if matches!(
        dependent,
        AnalysisDraft::Pac(_)
            | AnalysisDraft::Pnoise(_)
            | AnalysisDraft::Pxf(_)
            | AnalysisDraft::Pstb(_)
    ) && let AnalysisDraft::Pss(pss) = prerequisite
    {
        let (consumer, require_autonomous) = match periodic_state_requirement(dependent) {
            Ok(requirement) => requirement,
            Err(detail) => return Some(DependencyConfigurationIssue::InvalidDependent(detail)),
        };
        let pss = match pss.to_config() {
            Ok(pss) => pss,
            Err(detail) => {
                return Some(DependencyConfigurationIssue::InvalidPrerequisite(format!(
                    "PSS configuration is invalid: {detail}"
                )));
            }
        };
        return validate_periodic_state_contract(
            consumer,
            PeriodicStateCapability {
                shooting: pss.method == PssSolverMethod::Shooting,
                autonomous: pss.osc_mode,
            },
            require_autonomous,
        )
        .err()
        .map(DependencyConfigurationIssue::Incompatible);
    }

    let (AnalysisDraft::Fourier(fourier), AnalysisDraft::Transient(transient)) =
        (dependent, prerequisite)
    else {
        return None;
    };
    let requirement = match fourier_requirement(fourier) {
        Ok(requirement) => requirement,
        Err(detail) => {
            return Some(DependencyConfigurationIssue::InvalidDependent(format!(
                "Fourier configuration is invalid: {detail}"
            )));
        }
    };
    let capability = match transient_capability(transient) {
        Ok(capability) => capability,
        Err(detail) => {
            return Some(DependencyConfigurationIssue::InvalidPrerequisite(format!(
                "Transient configuration is invalid: {detail}"
            )));
        }
    };
    validate_fourier_transient_contract(requirement, capability)
        .err()
        .map(DependencyConfigurationIssue::Incompatible)
}

pub(super) fn prerequisite_draft_for(
    dependent: &AnalysisDraft,
    prerequisite: AnalysisKind,
    context: &AnalysisDependencyRepairContext,
) -> Result<AnalysisDraft, String> {
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fourier(fourier) = dependent
    {
        let requirement = fourier_requirement(fourier)
            .map_err(|detail| format!("Fourier configuration is invalid: {detail}"))?;
        let required_interval = requirement.required_sample_interval()?;
        // Preserve margin against text round-tripping and future solver output
        // interpolation by targeting 10 samples for every highest-basis cycle.
        let interval = required_interval * 0.8;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{:.12e}", requirement.stop_time),
            step: format!("{interval:.12e}"),
            start: format!("{:.12e}", requirement.start_time),
            max_step: format!("{interval:.12e}"),
            uic: false,
        }));
    }
    if prerequisite == AnalysisKind::Pss {
        if matches!(
            dependent,
            AnalysisDraft::Pac(_)
                | AnalysisDraft::Pnoise(_)
                | AnalysisDraft::Pxf(_)
                | AnalysisDraft::Pstb(_)
        ) {
            periodic_state_requirement(dependent)?;
        }
        let sources = context.periodic_sources()?;
        let draft = PssDialogState {
            tone_sources: sources.join(", "),
            ..Default::default()
        };
        context.validate_pss_sources(&draft)?;
        let draft = AnalysisDraft::Pss(draft);
        if let Some(issue) = dependency_configuration_issue(dependent, &draft) {
            return Err(issue.detail().to_owned());
        }
        return Ok(draft);
    }
    Ok(AnalysisDraft::for_kind(prerequisite))
}

pub(super) fn dependency_candidate_context_issue(
    prerequisite: AnalysisKind,
    candidate: &AnalysisDraft,
    context: &AnalysisDependencyRepairContext,
) -> Option<String> {
    if prerequisite != AnalysisKind::Pss {
        return None;
    }
    let AnalysisDraft::Pss(pss) = candidate else {
        return Some("the prerequisite does not contain a PSS draft".to_owned());
    };
    context.validate_pss_sources(pss).err()
}

fn periodic_state_requirement(dependent: &AnalysisDraft) -> Result<(&'static str, bool), String> {
    match dependent {
        AnalysisDraft::Pac(draft) => draft
            .to_config()
            .map(|_| ("PAC", false))
            .map_err(|detail| format!("PAC configuration is invalid: {detail}")),
        AnalysisDraft::Pxf(draft) => draft
            .to_config()
            .map(|_| ("PXF", false))
            .map_err(|detail| format!("PXF configuration is invalid: {detail}")),
        AnalysisDraft::Pstb(draft) => draft
            .to_config()
            .map(|_| ("PSTB", false))
            .map_err(|detail| format!("PSTB configuration is invalid: {detail}")),
        AnalysisDraft::Pnoise(draft) => draft
            .to_config()
            .map(|config| ("PNOISE", config.noise_ref == NoiseReferenceType::Phase))
            .map_err(|detail| format!("PNOISE configuration is invalid: {detail}")),
        _ => Err(format!(
            "{} does not consume a PSS periodic-state prerequisite",
            dependent.kind().label()
        )),
    }
}

fn parse_positive(text: &str, field: &str) -> Result<f64, String> {
    let value = crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))?;
    if value <= 0.0 {
        Err(format!("{field} must be greater than zero"))
    } else {
        Ok(value)
    }
}

fn parse_nonnegative(text: &str, field: &str) -> Result<f64, String> {
    let value = crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|error| format!("invalid {field}: {error}"))?;
    if value < 0.0 {
        Err(format!("{field} must not be negative"))
    } else {
        Ok(value)
    }
}

fn parse_positive_usize(text: &str, field: &str) -> Result<usize, String> {
    let value = text
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("{field} must be a positive integer"))?;
    if value == 0 {
        Err(format!("{field} must be greater than zero"))
    } else {
        Ok(value)
    }
}

fn parse_i32_pair(text: &str, field: &str) -> Result<[i32; 2], String> {
    let values = text
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("{field} must contain two comma-separated integers"))?;
    values
        .try_into()
        .map_err(|_| format!("{field} must contain exactly two integers"))
}

fn validate_sweep(draft: &FrequencySweepDraft) -> Result<(), String> {
    let start = parse_positive(&draft.start, "start frequency")?;
    let stop = parse_positive(&draft.stop, "stop frequency")?;
    if stop <= start {
        return Err("stop frequency must be greater than start frequency".to_owned());
    }
    parse_positive_usize(&draft.points, "sweep point count")?;
    if draft.sweep > 2 {
        return Err("frequency sweep mode is outside the supported schema".to_owned());
    }
    Ok(())
}

fn validate_qpss(draft: &QpssDraft) -> Option<String> {
    (|| {
        let tones = draft
            .tones
            .split(',')
            .map(str::trim)
            .map(|value| parse_positive(value, "QPSS tone"))
            .collect::<Result<Vec<_>, _>>()?;
        if tones.len() < 2 {
            return Err("QPSS requires at least two incommensurate tones".to_owned());
        }
        if tones
            .iter()
            .enumerate()
            .any(|(index, tone)| tones[..index].contains(tone))
        {
            return Err("QPSS tone frequencies must be distinct".to_owned());
        }
        let harmonics = draft
            .harmonics
            .split(',')
            .map(str::trim)
            .map(|value| parse_positive_usize(value, "QPSS harmonic order"))
            .collect::<Result<Vec<_>, _>>()?;
        if harmonics.len() != tones.len() {
            return Err("QPSS must declare one harmonic order per tone".to_owned());
        }
        parse_positive_usize(&draft.max_iterations, "maximum iteration count")?;
        parse_positive(&draft.relative_tolerance, "relative tolerance")?;
        if draft.autonomous && draft.oscillator_node.trim().is_empty() {
            return Err("autonomous QPSS requires an oscillator node".to_owned());
        }
        Ok(())
    })()
    .err()
}

fn validate_periodic_network(draft: &PeriodicNetworkDraft) -> Option<String> {
    (|| {
        validate_sweep(&draft.sweep)?;
        parse_positive_usize(&draft.max_sideband, "maximum sideband")?;
        if draft.ports.is_empty() {
            return Err("at least one network port is required".to_owned());
        }
        for (index, port) in draft.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() {
                return Err(format!("port {} requires a positive node", index + 1));
            }
            parse_positive(&port.z0, &format!("port {} reference impedance", index + 1))?;
        }
        Ok(())
    })()
    .err()
}

fn validate_hbnoise(draft: &HbNoiseDraft) -> Option<String> {
    (|| {
        validate_sweep(&draft.sweep)?;
        if draft.output_node.trim().is_empty() {
            return Err("HBNOISE requires an output node".to_owned());
        }
        if draft.input_source.trim().is_empty() {
            return Err("HBNOISE requires an input source".to_owned());
        }
        parse_positive_usize(&draft.max_sideband, "maximum sideband")?;
        Ok(())
    })()
    .err()
}

fn validate_qpac(draft: &QuasiPeriodicAcDraft) -> Option<String> {
    (|| {
        validate_sweep(&draft.sweep)?;
        if draft.input_source.trim().is_empty() || draft.output_node.trim().is_empty() {
            return Err("QPAC requires an input source and output node".to_owned());
        }
        parse_i32_pair(&draft.input_lattice, "input lattice product")?;
        parse_i32_pair(&draft.output_lattice, "output lattice product")?;
        Ok(())
    })()
    .err()
}

fn validate_qpnoise(draft: &QuasiPeriodicNoiseDraft) -> Option<String> {
    (|| {
        validate_sweep(&draft.sweep)?;
        if draft.output_node.trim().is_empty() || draft.input_source.trim().is_empty() {
            return Err("QPNOISE requires an output node and input source".to_owned());
        }
        let ranges = draft
            .lattice_products
            .split(',')
            .map(|range| {
                let values = range
                    .split(':')
                    .map(|bound| bound.trim().parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| "lattice products must use integer min:max bounds".to_owned())?;
                let [minimum, maximum]: [i32; 2] = values
                    .try_into()
                    .map_err(|_| "each lattice product must contain one min:max pair".to_owned())?;
                if minimum > maximum {
                    return Err("lattice product minimum must not exceed maximum".to_owned());
                }
                Ok([minimum, maximum])
            })
            .collect::<Result<Vec<_>, String>>()?;
        if ranges.len() != 2 {
            return Err("lattice products must contain exactly two ranges".to_owned());
        }
        Ok(())
    })()
    .err()
}

fn validate_qpxf(draft: &QuasiPeriodicTransferDraft) -> Option<String> {
    (|| {
        validate_sweep(&draft.sweep)?;
        if draft.input_source.trim().is_empty() || draft.output_node.trim().is_empty() {
            return Err("QPXF requires an input source and output node".to_owned());
        }
        parse_i32_pair(&draft.input_lattice, "input lattice product")?;
        parse_i32_pair(&draft.output_lattice, "output lattice product")?;
        Ok(())
    })()
    .err()
}

fn validate_transient_noise(draft: &TransientNoiseDraft) -> Option<String> {
    (|| {
        let stop = parse_positive(&draft.stop_time, "stop time")?;
        let step = parse_positive(&draft.step_time, "step time")?;
        let start = parse_nonnegative(&draft.start_time, "start time")?;
        let max_step = parse_positive(&draft.max_step, "maximum step")?;
        if start >= stop {
            return Err("start time must be less than stop time".to_owned());
        }
        if step > stop || max_step > stop {
            return Err("time steps must not exceed stop time".to_owned());
        }
        parse_positive_usize(&draft.seed, "random seed")?;
        parse_positive(&draft.noise_fmax, "maximum noise frequency")?;
        parse_positive(&draft.scale, "noise scale")?;
        Ok(())
    })()
    .err()
}

fn validate_dc_mismatch(draft: &DcMismatchDraft) -> Option<String> {
    (|| {
        if draft.output_expression.trim().is_empty() {
            return Err("DC mismatch requires an output expression".to_owned());
        }
        parse_positive(&draft.sigma_multiplier, "sigma multiplier")?;
        parse_positive_usize(&draft.contributor_limit, "contributor limit")?;
        if !draft.include_process && !draft.include_mismatch {
            return Err("enable process or mismatch contributions".to_owned());
        }
        Ok(())
    })()
    .err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_kinds_have_exact_tagged_drafts_and_legacy_mapping() {
        for (index, kind) in AnalysisKind::ALL.into_iter().enumerate() {
            let draft = AnalysisDraft::for_kind(kind);
            assert_eq!(draft.kind(), kind);
            assert_eq!(draft.legacy_index(), index);
            assert_eq!(
                AnalysisDraft::from_legacy_index(index)
                    .expect("known index")
                    .kind(),
                kind
            );
            let value = serde_json::to_value(&draft).expect("draft serializes");
            assert_eq!(value["kind"], kind.stable_id());
        }
        assert!(AnalysisDraft::from_legacy_index(34).is_none());
    }

    #[test]
    fn restore_preserves_raw_edits_and_only_repairs_lazy_sentinel() {
        let mut draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
        let AnalysisDraft::OperatingPoint(state) = &mut draft else {
            panic!("expected OP draft");
        };
        state.temperature = "unfinished(".to_owned();
        let json = serde_json::to_string(&draft).expect("draft serializes");
        assert!(!json.contains("initialized"));

        let mut restored: AnalysisDraft = serde_json::from_str(&json).expect("draft restores");
        let AnalysisDraft::OperatingPoint(state) = &restored else {
            panic!("expected OP draft");
        };
        assert!(!state.initialized);
        assert_eq!(state.temperature, "unfinished(");

        restored.prepare_after_restore();
        let AnalysisDraft::OperatingPoint(state) = restored else {
            panic!("expected OP draft");
        };
        assert!(state.initialized);
        assert_eq!(state.temperature, "unfinished(");
    }

    #[test]
    fn envelope_periodic_initializer_is_owned_by_the_envelope_task() {
        let mut draft = AnalysisDraft::for_kind(AnalysisKind::Envelope);
        for selection in 0..=2 {
            let AnalysisDraft::Envelope(state) = &mut draft else {
                panic!("expected Envelope draft");
            };
            state.initial_periodic_solve_idx = selection;
            assert!(draft.prerequisite_roles().is_empty());
        }
    }

    #[test]
    fn exact_periodic_source_context_distinguishes_empty_from_unavailable() {
        let exact_empty = AnalysisDependencyRepairContext::exact_periodic_sources(
            "autonomous fixture\nR1 out 0 1k\n.end\n",
        )
        .expect("an elaborated empty source set is authoritative");
        let mut autonomous = PssDialogState::default();
        autonomous.tone_sources.clear();
        autonomous.osc_mode = true;
        autonomous.osc_node = "out".to_owned();
        exact_empty
            .validate_pss_sources(&autonomous)
            .expect("an autonomous PSS can use the exact empty driven-source set");

        let unavailable = AnalysisDependencyRepairContext::default();
        assert!(
            unavailable
                .validate_pss_sources(&autonomous)
                .unwrap_err()
                .contains("unavailable")
        );
    }

    #[test]
    fn exact_periodic_source_context_rejects_unknown_and_omitted_tones() {
        let context = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nVLO lo 0 SIN(0 1 1k)\nVRF rf 0 SIN(0 1 2k)\nR1 lo 0 1k\nR2 rf 0 1k\n.end\n",
        )
        .expect("test source catalog is exact");
        let mut pss = PssDialogState::default();
        pss.tone_sources = "vlo, VSTALE".to_owned();

        let error = context
            .validate_pss_sources(&pss)
            .expect_err("unknown and omitted tones fail closed");
        assert!(error.contains("unknown: VSTALE"));
        assert!(error.contains("omitted: VRF"));
    }

    #[test]
    fn exact_periodic_source_context_rejects_nonperiodic_or_incommensurate_waveforms() {
        let incommensurate = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nV1 out 0 SIN(0 1 1.1k)\nR1 out 0 1k\n.end\n",
        )
        .expect("source identity elaborates");
        let mut pss = PssDialogState::default();
        pss.tone_sources = "V1".to_owned();
        let error = incommensurate
            .validate_pss_sources(&pss)
            .expect_err("repair cannot commit an incommensurate PSS");
        assert!(error.contains("not an integer multiple"));

        let pwl = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nV1 out 0 PWL(0 0 1u 1)\nR1 out 0 1k\n.end\n",
        )
        .expect("PWL source identity elaborates");
        let error = pwl
            .validate_pss_sources(&pss)
            .expect_err("repair cannot claim an unauthenticated PWL period");
        assert!(error.contains("uses PWL"));
    }

    #[test]
    fn noise_and_disto_own_independent_sweep_drafts() {
        let mut ac = AnalysisDraft::for_kind(AnalysisKind::Ac);
        let noise = AnalysisDraft::for_kind(AnalysisKind::Noise);
        let disto = AnalysisDraft::for_kind(AnalysisKind::Disto);
        let AnalysisDraft::Ac(ac_draft) = &mut ac else {
            panic!("expected AC draft");
        };
        ac_draft.points = "777".to_owned();

        let AnalysisDraft::Noise(noise) = noise else {
            panic!("expected noise draft");
        };
        let AnalysisDraft::Disto(disto) = disto else {
            panic!("expected DISTO draft");
        };
        assert_eq!(noise.points, "30");
        assert_eq!(noise.sweep, NoiseSweepType::Decade);
        assert_eq!(disto.sweep.points, "101");
    }

    #[test]
    fn noise_defaults_match_the_eight_field_mockup_contract() {
        let draft = NoiseDraft::default();
        assert_eq!(draft.sweep, NoiseSweepType::Decade);
        assert_eq!(draft.points, "30");
        assert_eq!(draft.fstart, "10");
        assert_eq!(draft.fstop, "1Meg");
        assert_eq!(draft.output, "afe_out");
        assert_eq!(draft.input, "VIN_DIFF");
        assert_eq!(draft.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(draft.integration_mode, NoiseIntegrationMode::Enabled);
        assert!(draft.to_config().is_ok());
    }

    #[test]
    fn legacy_noise_draft_migrates_without_inventing_active_settings() {
        let json = r#"{
            "kind":"noise",
            "draft":{
                "output":"legacy_out",
                "reference":"legacy_ref",
                "input":"VLEGACY",
                "fstart":"2",
                "fstop":"2Meg",
                "points":"17",
                "sweep":1
            }
        }"#;
        let restored: AnalysisDraft = serde_json::from_str(json).expect("legacy draft migrates");
        let AnalysisDraft::Noise(restored) = restored else {
            panic!("expected migrated noise draft");
        };
        assert_eq!(restored.sweep, NoiseSweepType::Octave);
        assert_eq!(restored.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(restored.integration_mode, NoiseIntegrationMode::Enabled);
        let config = restored
            .to_config()
            .expect("migrated draft remains executable");
        assert_eq!(config.output_node, "legacy_out");
        assert_eq!(config.reference_node, "legacy_ref");
        assert_eq!(config.input_source, "VLEGACY");
        assert_eq!(config.sweep_type, AcSweepType::Octave);
        assert_eq!(config.num_points, 17);
    }

    #[test]
    fn current_noise_draft_round_trips_every_owned_field() {
        let draft = NoiseDraft {
            output: "sensor_p,sensor_n".to_owned(),
            reference: "legacy_reference_is_retained".to_owned(),
            input: "IIN_CAL".to_owned(),
            fstart: "11".to_owned(),
            fstop: "9Meg".to_owned(),
            points: "41".to_owned(),
            sweep: NoiseSweepType::ExplicitFrequencyList,
            explicit_frequencies: "11 1k 9Meg".to_owned(),
            contribution_detail: NoiseContributionDetail::SummaryOnly,
            integration_mode: NoiseIntegrationMode::Disabled,
        };
        let json = serde_json::to_string(&draft).expect("current draft serializes");
        assert!(json.contains("\"sweep\":\"explicit_frequency_list\""));
        let restored: NoiseDraft = serde_json::from_str(&json).expect("current draft restores");
        assert_eq!(restored, draft);
    }

    #[test]
    fn invalid_legacy_noise_sweep_is_retained_and_rejected() {
        let json = r#"{
            "kind":"noise",
            "draft":{
                "output":"out",
                "reference":"0",
                "input":"V1",
                "fstart":"1",
                "fstop":"1Meg",
                "points":"10",
                "sweep":99
            }
        }"#;
        let restored: AnalysisDraft =
            serde_json::from_str(json).expect("invalid index is retained");
        let AnalysisDraft::Noise(restored) = restored else {
            panic!("expected noise draft");
        };
        assert_eq!(restored.sweep, NoiseSweepType::Unsupported(99));
        assert!(restored.to_config().is_err());
    }

    #[test]
    fn noise_differential_output_and_explicit_axis_convert_exactly() {
        let draft = NoiseDraft {
            output: "V(sensor_p, sensor_n)".to_owned(),
            input: "IIN_CAL".to_owned(),
            sweep: NoiseSweepType::ExplicitFrequencyList,
            explicit_frequencies: "10, 1k; 1Meg".to_owned(),
            contribution_detail: NoiseContributionDetail::AllContributors,
            integration_mode: NoiseIntegrationMode::OutputNoiseOnly,
            ..NoiseDraft::default()
        };
        let config = draft.to_config().expect("exact draft converts");
        assert_eq!(config.output_node, "sensor_p");
        assert_eq!(config.reference_node, "sensor_n");
        assert_eq!(config.input_source, "IIN_CAL");
        assert_eq!(config.explicit_frequencies, Some(vec![10.0, 1.0e3, 1.0e6]));
        assert_eq!(
            config.contribution_detail,
            NoiseContributionDetail::AllContributors
        );
        assert_eq!(
            config.integration_mode,
            NoiseIntegrationMode::OutputNoiseOnly
        );
    }

    #[test]
    fn noise_validation_rejects_invalid_output_input_and_frequency_axes() {
        for draft in [
            NoiseDraft {
                output: "V(a,b,c)".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                input: " ".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                output: "V(out\n)\n.op".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                input: "VIN\n.tran 1n 1u".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                fstop: "1".to_owned(),
                ..NoiseDraft::default()
            },
            NoiseDraft {
                sweep: NoiseSweepType::ExplicitFrequencyList,
                explicit_frequencies: "10, 10".to_owned(),
                ..NoiseDraft::default()
            },
        ] {
            assert!(draft.to_config().is_err());
            assert!(
                AnalysisDraft::Noise(draft.clone())
                    .manifest_configuration_error()
                    .is_some()
            );
        }
    }

    #[test]
    fn transfer_function_summary_uses_only_the_current_dc_contract() {
        let draft = AnalysisDraft::for_kind(AnalysisKind::TransferFunction);
        assert_eq!(
            draft.manifest_summary().as_deref(),
            Some("V(afe_out) <- VIN_DIFF - DC operating point")
        );
    }

    #[test]
    fn missing_manifest_analysis_drafts_are_typed_validated_and_round_trip() {
        for kind in [
            AnalysisKind::Qpss,
            AnalysisKind::Hbsp,
            AnalysisKind::Hbnoise,
            AnalysisKind::Psp,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
            AnalysisKind::TransientNoise,
            AnalysisKind::DcMismatch,
        ] {
            let draft = AnalysisDraft::for_kind(kind);
            assert_eq!(draft.kind(), kind);
            assert!(draft.manifest_configuration_error().is_none(), "{kind}");
            assert!(draft.manifest_summary().is_some(), "{kind}");
            let bytes = serde_json::to_vec(&draft).expect("draft serializes");
            let restored: AnalysisDraft =
                serde_json::from_slice(&bytes).expect("draft deserializes");
            assert_eq!(restored.kind(), kind);
        }
    }

    #[test]
    fn manifest_draft_validation_rejects_incomplete_configuration() {
        let mut qpss = QpssDraft::default();
        qpss.tones = "1G".to_owned();
        assert!(validate_qpss(&qpss).is_some());

        let mut network = PeriodicNetworkDraft::default();
        network.ports.clear();
        assert!(validate_periodic_network(&network).is_some());

        let mut tnoise = TransientNoiseDraft::default();
        tnoise.seed = "0".to_owned();
        assert!(validate_transient_noise(&tnoise).is_some());

        let mut mismatch = DcMismatchDraft::default();
        mismatch.include_mismatch = false;
        assert!(validate_dc_mismatch(&mismatch).is_some());
    }
}
