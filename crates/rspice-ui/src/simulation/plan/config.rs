//! Analysis drafts.
//!
//! The in-progress form of each analysis type as the user edits it, before
//! it becomes a validated specification.

use serde::{Deserialize, Serialize};

use crate::services::simulation_runner::PeriodicCarrier;
use crate::simulation::config::{
    AcSweepType, NoiseAnalysisConfig, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
use crate::simulation::dependency_contract::{
    FourierTransientRequirement, PeriodicStateCapability, TransientCapability,
    validate_fourier_transient_contract, validate_harmonic_balance_carrier_contract,
    validate_periodic_state_contract,
};
use crate::simulation::dialog::{
    CornerDialogState, EnvelopeDialogState, FourierDialogState, HbDialogState, McDialogState,
    NoiseReferenceType, OpDialogState, OptimizationDialogState, PacDialogState, PnoiseDialogState,
    PssDialogState, PstbDialogState, PxfDialogState, PzDialogState, ReliabilityDialogState,
    SensDialogState, SoaDialogState, SpDialogState, StbDialogState, TempDialogState, XfDialogState,
};
use crate::simulation::spice_value::parse_spice_value_checked;
use crate::workbench::app_state::{AcSetup, DcSetup, TranSetup};

use super::AnalysisKind;

mod frequency_table;
mod periodic_network;
mod qpac;
mod qpxf;
pub use qpxf::{QpxfSidebandSelection, QpxfSourceSelection, QuasiPeriodicTransferDraft};
mod quasi_periodic;
pub use qpac::QuasiPeriodicAcDraft;
pub use quasi_periodic::QpssDraft;
use quasi_periodic::validate_qpss;

pub use periodic_network::PeriodicNetworkDraft;
use periodic_network::{validate_periodic_network, validate_psp_network};
mod recorded_fft;

pub use frequency_table::AcDataDraft;
pub use recorded_fft::FftDraft;

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
            // The output node and the input source name the user's own
            // circuit. A new draft states neither: `to_config` refuses an
            // empty one with the field that is missing, which is guidance, and
            // a name the design does not carry is a run that fails later.
            output: String::new(),
            reference: default_noise_reference(),
            input: String::new(),
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
    crate::simulation::config::parse_explicit_frequency_list(text)
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
pub struct HbNoiseDraft {
    #[serde(default = "default_noise_sideband")]
    pub input_sideband: String,
    #[serde(default = "default_noise_sideband")]
    pub output_sideband: String,
    #[serde(default)]
    pub source_resistor: String,
    #[serde(default = "default_noise_reference_temperature")]
    pub reference_temperature: String,
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
            input_sideband: default_noise_sideband(),
            output_sideband: default_noise_sideband(),
            source_resistor: String::new(),
            reference_temperature: default_noise_reference_temperature(),
            sweep: FrequencySweepDraft::default(),
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "V1".to_owned(),
            // A ±4 sideband span fits the default nine-harmonic HB producer;
            // larger spans remain available when the producer retains them.
            max_sideband: "4".to_owned(),
            integrated_noise: true,
            // The user must select the circuit's actual source resistor
            // before enabling the reference-temperature calculation.
            noise_figure: false,
            contributor_ranking: true,
        }
    }
}

fn default_noise_reference_temperature() -> String {
    "290".into()
}

fn default_noise_sideband() -> String {
    "0".into()
}

impl HbNoiseDraft {
    pub(crate) fn sidebands(&self) -> Result<(i32, i32), String> {
        let input = self
            .input_sideband
            .trim()
            .parse::<i32>()
            .map_err(|_| "Input sideband must be a signed integer")?;
        let output = self
            .output_sideband
            .trim()
            .parse::<i32>()
            .map_err(|_| "Output sideband must be a signed integer")?;
        let maximum = self
            .max_sideband
            .trim()
            .parse::<usize>()
            .map_err(|_| "Maximum sideband must be a nonnegative integer")?;
        crate::services::simulation_runner::validate_noise_sidebands(input, output, maximum)?;
        Ok((input, output))
    }
    pub(crate) fn noise_reference(
        &self,
    ) -> Result<Option<crate::services::simulation_runner::HbNoiseReference>, String> {
        if !self.noise_figure {
            return Ok(None);
        }
        let reference = crate::services::simulation_runner::HbNoiseReference {
            source_resistor: self.source_resistor.trim().into(),
            temperature_kelvin: parse_positive(
                &self.reference_temperature,
                "noise reference temperature (K)",
            )?,
        };
        reference.validate()?;
        Ok(Some(reference))
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
pub struct TransientNoiseDraft {
    pub stop_time: String,
    pub step_time: String,
    pub start_time: String,
    pub max_step: String,
    pub seed: String,
    pub noise_fmax: String,
    /// Lowest flicker frequency the run represents, in hertz. Empty is the
    /// engine's own derivation — `1/tstop`, the longest period the window can
    /// resolve — and is the default, because a run that has not been told
    /// otherwise should represent every period it can.
    ///
    /// Defaulted on read so a plan saved before this field existed opens with
    /// the derivation it was running under rather than refusing to load.
    #[serde(default)]
    pub noise_fmin: String,
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
            noise_fmin: String::new(),
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
    /// Smallest variance share a contributor must carry to be listed. Empty
    /// is the card's own default of zero, which retains every contributor the
    /// limit above allows, and is what a plan saved before this control
    /// existed opens with.
    #[serde(default)]
    pub share_threshold: String,
    pub include_process: bool,
    pub include_mismatch: bool,
    pub normalized_contributions: bool,
}

/// A fresh draft is the bare card `.DCMATCH OUT=V(out)`.
///
/// Every default here is the engine card's own, read from `DcMatchCard`
/// rather than copied: a default Studio run and a hand-written `.DCMATCH
/// OUT=V(out)` are then the same analysis, and the engine remains the one
/// place a default is decided. The two numbers were `3` and `25`, which named
/// a study the card never described.
impl Default for DcMismatchDraft {
    fn default() -> Self {
        Self {
            output_expression: "V(out)".to_owned(),
            sigma_multiplier: "1".to_owned(),
            contributor_limit: rspice_core::netlist::DcMatchCard::DEFAULT_CONTRIBUTORS.to_string(),
            share_threshold: String::new(),
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
    #[serde(rename = "acdata")]
    AcData(AcDataDraft),
    #[serde(rename = "fft")]
    Fft(FftDraft),
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
            AnalysisKind::AcData => Self::AcData(AcDataDraft::default()),
            AnalysisKind::Fft => Self::Fft(FftDraft::default()),
        }
    }

    /// Construct a semantic draft from a current singleton-model index.
    #[cfg(test)]
    pub fn from_legacy_index(index: usize) -> Option<Self> {
        AnalysisKind::from_legacy_index(index).map(Self::for_kind)
    }

    /// What this instance's own controls assign after the deck is resolved.
    ///
    /// Two kinds answer with something: the operating point carries both an
    /// accuracy tier and a homotopy choice, and the transfer function carries
    /// the tier. Every other kind resolves to the deck and states nothing on
    /// top of it, so it owns nothing and refuses nothing extra.
    ///
    /// Read off the draft's stored index rather than through `to_config`,
    /// because a draft that does not yet validate — a half-typed temperature —
    /// still has a tier, and a refusal that disappeared while a field was
    /// being edited would be a gate nobody could rely on.
    #[must_use]
    pub fn solver_ownership(&self) -> crate::simulation::plan::SolverOwnership {
        use crate::simulation::accuracy::AnalysisAccuracy;
        use crate::simulation::dialog::OpHomotopy;
        use crate::simulation::plan::SolverOwnership;

        match self {
            Self::OperatingPoint(state) => SolverOwnership {
                accuracy: AnalysisAccuracy::ALL.get(state.accuracy_idx).copied(),
                homotopy: OpHomotopy::ALL.get(state.homotopy_idx).copied(),
            },
            Self::TransferFunction(state) => SolverOwnership {
                accuracy: AnalysisAccuracy::ALL.get(state.accuracy_idx).copied(),
                homotopy: None,
            },
            _ => SolverOwnership::NONE,
        }
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
            Self::AcData(_) => AnalysisKind::AcData,
            Self::Fft(_) => AnalysisKind::Fft,
        }
    }

    /// Exact prerequisite roles required by this configured draft.
    ///
    /// Envelope owns its selected periodic initializer as part of one
    /// authenticated execution. Exposing a separate HB/PSS task here would be
    /// incorrect until that task can publish a typed continuation artifact
    /// that Envelope actually consumes.
    ///
    /// The periodic small-signal family is the one place a *draft* moves this
    /// away from its kind's declaration: the carrier the request names is the
    /// family it linearizes around, so a `.PAC` carried by `FROM=HB` requires
    /// a harmonic-balance solve and not a shooting `.PSS`. The third carrier
    /// position names neither family — see [`Self::prerequisite_alternatives`]
    /// — and reads as this kind's declared role until the plan resolves it.
    #[must_use]
    pub fn prerequisite_roles(&self) -> &'static [AnalysisKind] {
        const HB: &[AnalysisKind] = &[AnalysisKind::HarmonicBalance];

        match self.periodic_carrier() {
            Some(PeriodicCarrier::Hb) => HB,
            _ => self.kind().prerequisites(),
        }
    }

    /// The prerequisite kinds that may fill this draft's one declared role
    /// where more than its own kind can, in the order the plan prefers them.
    ///
    /// Empty for everything but a periodic small-signal request whose carrier
    /// is the *preceding* periodic solve. That position writes no `FROM=`
    /// keyword, and `resolve_periodic_source` in
    /// `rspice-core/src/execution/plan.rs` binds such a card to the nearest
    /// preceding `.PSS` **or** `.HB` — whichever the deck wrote last. So the
    /// role is not a property of the request alone, and the plan resolves it
    /// by the same rule against its own order.
    #[must_use]
    pub fn prerequisite_alternatives(&self) -> &'static [AnalysisKind] {
        const EITHER_PERIODIC_SOLVE: &[AnalysisKind] =
            &[AnalysisKind::Pss, AnalysisKind::HarmonicBalance];
        const NONE: &[AnalysisKind] = &[];

        match self.periodic_carrier() {
            Some(PeriodicCarrier::Preceding) => EITHER_PERIODIC_SOLVE,
            _ => NONE,
        }
    }

    /// The carrier a periodic small-signal draft linearizes around, or `None`
    /// for every draft that names no carrier.
    ///
    /// `.PSTB` is deliberately absent: its card has no `FROM=` arm, and the
    /// engine binds it to the preceding `.PSS` unconditionally because only a
    /// shooting solve carries a monodromy matrix.
    #[must_use]
    fn periodic_carrier(&self) -> Option<PeriodicCarrier> {
        match self {
            Self::Pac(draft) => Some(PeriodicCarrier::at(draft.carrier_idx)),
            Self::Pxf(draft) => Some(PeriodicCarrier::at(draft.carrier_idx)),
            Self::Pnoise(draft) => Some(PeriodicCarrier::at(draft.carrier_idx)),
            _ => None,
        }
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
            Self::OperatingPoint(state) => state.prepare_after_restore(),
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
            | Self::DcMismatch(_)
            | Self::AcData(_)
            | Self::Fft(_) => {}
        }
    }

    /// Validate drafts whose engines are not yet available without projecting
    /// them through the retired singleton configuration model.
    pub fn manifest_configuration_error(&self) -> Option<String> {
        match self {
            Self::Noise(draft) => draft.to_config().err(),
            Self::Qpss(draft) => validate_qpss(draft),
            Self::Hbsp(draft) => validate_periodic_network(draft),
            Self::Psp(draft) => validate_psp_network(draft),
            Self::Hbnoise(draft) => validate_hbnoise(draft),
            Self::Qpac(draft) => validate_qpac(draft),
            Self::Qpnoise(draft) => validate_qpnoise(draft),
            Self::Qpxf(draft) => validate_qpxf(draft),
            Self::TransientNoise(draft) => validate_transient_noise(draft),
            Self::DcMismatch(draft) => validate_dc_mismatch(draft),
            Self::AcData(draft) => draft.to_config().err(),
            Self::Fft(draft) => draft.to_request().err(),
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
            Self::Qpxf(draft) => Some(draft.summary()),
            Self::TransientNoise(draft) => Some(format!(
                "stop {} · step {} · seed {}",
                draft.stop_time, draft.step_time, draft.seed
            )),
            Self::DcMismatch(draft) => Some(format!(
                "{} · {} sigma · top {}",
                draft.output_expression, draft.sigma_multiplier, draft.contributor_limit
            )),
            Self::AcData(draft) => Some(draft.summary()),
            Self::Fft(draft) => Some(draft.summary()),
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

    /// Whether this PSS draft's source selection can execute.
    ///
    /// The closed periodic-source contract is a *driven* rule and is asked only
    /// of a driven solve. An autonomous run has no tone list to close over: the
    /// engine's `PssConfig` carries no tone field at all
    /// (`rspice-core/src/analysis/pss/config.rs`), the period comes from the
    /// oscillator node, and `validate_periodic_source_contract` says in its own
    /// first line that it defines "a driven PSS period"
    /// (`rspice-core/src/engine/transient.rs:1349-1352`).
    ///
    /// Asking it anyway made autonomous PSS unsatisfiable on any circuit that
    /// places a transient source, which is every oscillator with a startup
    /// kick: the driven contract refuses an omitted source, and the autonomous
    /// mode refuses a tone list, so neither an empty selection nor a full one
    /// could pass. What the engine actually does with those sources is stated
    /// on the form rather than refused here.
    pub(crate) fn validate_pss_sources(&self, draft: &PssDialogState) -> Result<(), String> {
        let config = draft
            .to_config()
            .map_err(|detail| format!("PSS configuration is invalid: {detail}"))?;
        if config.osc_mode {
            return Ok(());
        }
        self.validate_periodic_source_selection(&config.tone_sources)?;
        let contract = self
            .periodic_sources
            .as_ref()
            .map_err(|detail| detail.clone())?;
        let netlist = rspice_core::Netlist::parse(&contract.netlist_source).map_err(|error| {
            format!("the authenticated periodic-source circuit is no longer parseable: {error}")
        })?;
        rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .validate_pss_source_contract_with_abort(
                &netlist,
                &config.tone_sources,
                &rspice_core::analysis::PssConfig::new(config.fund_freq)
                    .with_points_per_period(config.points_per_period)
                    .with_harmonics(config.num_harmonics.max(1)),
                &rspice_core::abort_signal::NoAbort,
            )
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
                // A PSS this editor built is a shooting solve. The capability
                // stays a field rather than becoming a constant because a
                // *sealed* specification may still carry the retired HB-PSS
                // formulation, and `execution::artifact` judges that one; this
                // is the live draft, which has no way to ask for it.
                shooting: true,
                autonomous: pss.osc_mode,
            },
            require_autonomous,
        )
        .err()
        .map(DependencyConfigurationIssue::Incompatible);
    }

    // The same question asked of the other periodic carrier the engine
    // accepts. `.PSTB` is absent because it cannot reach here: it declares no
    // harmonic-balance role in any carrier position.
    if matches!(
        dependent,
        AnalysisDraft::Pac(_) | AnalysisDraft::Pnoise(_) | AnalysisDraft::Pxf(_)
    ) && matches!(prerequisite, AnalysisDraft::HarmonicBalance(_))
    {
        let (consumer, require_autonomous) = match periodic_state_requirement(dependent) {
            Ok(requirement) => requirement,
            Err(detail) => return Some(DependencyConfigurationIssue::InvalidDependent(detail)),
        };
        return validate_harmonic_balance_carrier_contract(consumer, require_autonomous)
            .err()
            .map(DependencyConfigurationIssue::Incompatible);
    }

    // The engine's own rule, before the run rather than during it: a card
    // whose STOP is past a transient's stop time fails that transient.
    if let (AnalysisDraft::Fft(fft), AnalysisDraft::Transient(transient)) =
        (dependent, prerequisite)
    {
        let request = match fft.to_request() {
            Ok(request) => request,
            Err(detail) => {
                return Some(DependencyConfigurationIssue::InvalidDependent(format!(
                    "FFT configuration is invalid: {detail}"
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
        let stop = request.stop.unwrap_or(capability.stop_time);
        return (stop > capability.stop_time).then(|| {
            DependencyConfigurationIssue::Incompatible(format!(
                "STOP {stop} exceeds transient stop time {}",
                capability.stop_time
            ))
        });
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
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fft(fft) = dependent
    {
        let request = fft
            .to_request()
            .map_err(|detail| format!("FFT configuration is invalid: {detail}"))?;
        // Only an authored STOP can size a transient. Without one the card
        // takes the transient's own stop time, and the default transient is
        // exactly the run the author has not yet constrained.
        let Some(stop) = request.stop else {
            return Ok(AnalysisDraft::for_kind(prerequisite));
        };
        let start = request.start.unwrap_or(0.0);
        let step = (stop - start) / request.points as f64;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{stop:.12e}"),
            step: format!("{step:.12e}"),
            start: format!("{:.12e}", 0.0),
            max_step: format!("{step:.12e}"),
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
    let draft = AnalysisDraft::for_kind(prerequisite);
    // A synthesized carrier is refused here for the same reason a chosen one
    // is: a repair that inserted it would leave the plan holding a dependency
    // its own contract rejects, with no further repair to offer.
    if prerequisite == AnalysisKind::HarmonicBalance
        && let Some(issue) = dependency_configuration_issue(dependent, &draft)
    {
        return Err(issue.detail().to_owned());
    }
    Ok(draft)
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

/// Read an authored DC mismatch share threshold, or `None` for the card's own
/// default.
///
/// Two spellings mean the same card and are canonicalized to one: an empty
/// field, and a threshold authored as exactly zero. The engine's default IS
/// zero (`DcMatchCard::threshold`), so `THRESHOLD=0` is the unauthored card —
/// and if the two specifications differed, one analysis would have two plan
/// digests and a saved plan would re-run as a different request.
///
/// Both the plan draft and the specification builder read a threshold through
/// here so there is one account of that identity.
pub(crate) fn dc_mismatch_share_threshold(text: &str) -> Result<Option<f64>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let value = crate::simulation::dialog::options::parse_si_value(text)
        .map_err(|error| format!("invalid share threshold: {error}"))?;
    if value == 0.0 {
        return Ok(None);
    }
    Ok(Some(value))
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

fn parse_i32_tuple(text: &str, field: &str) -> Result<Vec<i32>, String> {
    let values = text
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("{field} must contain comma-separated integers"))?;
    if values.len() < 2 {
        return Err(format!("{field} must contain at least two integers"));
    }
    Ok(values)
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

fn validate_hbnoise(draft: &HbNoiseDraft) -> Option<String> {
    (|| {
        if draft.sweep.sweep > 2 {
            return Err("frequency sweep mode is outside the supported schema".to_owned());
        }
        let max_sideband =
            draft.max_sideband.trim().parse::<usize>().map_err(|_| {
                "maximum sideband must be an integer from 0 to 2147483647".to_owned()
            })?;
        crate::services::simulation_runner::validate_hbnoise_frequency_options(
            parse_positive(&draft.sweep.start, "start frequency")?,
            parse_positive(&draft.sweep.stop, "stop frequency")?,
            parse_positive_usize(&draft.sweep.points, "sweep point count")?,
            draft.sweep.sweep == 2,
            max_sideband,
            draft.integrated_noise || draft.contributor_ranking,
        )?;
        if draft.output_node.trim().is_empty() {
            return Err("HBNOISE requires an output node".to_owned());
        }
        if draft.input_source.trim().is_empty() {
            return Err("HBNOISE requires an input source".to_owned());
        }
        draft.sidebands()?;
        draft.noise_reference()?;
        Ok(())
    })()
    .err()
}

fn validate_qpac(draft: &QuasiPeriodicAcDraft) -> Option<String> {
    draft.to_spec().err()
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
    draft.to_spec().err()
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
        let fmax = parse_positive(&draft.noise_fmax, "maximum noise frequency")?;
        // An empty floor is the engine's `1/tstop` derivation, not a missing
        // value, so it is not an error. An authored one has to sit inside the
        // band the ceiling opens.
        if !draft.noise_fmin.trim().is_empty() {
            let fmin = parse_positive(&draft.noise_fmin, "minimum noise frequency")?;
            if fmin >= fmax {
                return Err(
                    "minimum noise frequency must be below the maximum noise frequency".to_owned(),
                );
            }
        }
        parse_positive(&draft.scale, "noise scale")?;
        Ok(())
    })()
    .err()
}

/// What a DC mismatch draft refuses on, in the engine card's own words.
///
/// Text that is not a number at all is this layer's own to answer — the
/// engine never sees a half-typed field — but every *range* belongs to the
/// card, so the parsed draft is assembled into the specification the run
/// would carry and that specification is asked. There is then exactly one
/// account of what `.DCMATCH` refuses on, and the form and a hand-written
/// deck are refused by the same sentence.
///
/// A contributor limit of zero is legal here because it is legal on the card:
/// zero is how a deck asks for every contributor.
fn validate_dc_mismatch(draft: &DcMismatchDraft) -> Option<String> {
    (|| {
        let sigma_multiplier =
            crate::simulation::dialog::options::parse_si_value(&draft.sigma_multiplier)
                .map_err(|error| format!("invalid sigma multiplier: {error}"))?;
        let contributor_limit = draft
            .contributor_limit
            .trim()
            .parse::<usize>()
            .map_err(|_| "contributor limit must be a non-negative integer".to_owned())?;
        let contribution_threshold = dc_mismatch_share_threshold(&draft.share_threshold)?;
        crate::simulation::multi_run::AnalysisSpec::DcMismatch {
            output_expression: draft.output_expression.trim().to_owned(),
            sigma_multiplier,
            contributor_limit,
            include_process: draft.include_process,
            include_mismatch: draft.include_mismatch,
            normalized_contributions: draft.normalized_contributions,
            contribution_threshold,
        }
        .validate()
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
        assert!(AnalysisDraft::from_legacy_index(AnalysisKind::ALL.len()).is_none());
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

    /// An empty elaborated source set is a fact; an unreadable one is not.
    ///
    /// Asked of a driven solve, which is the only mode the catalog is
    /// load-bearing for: the driven period is defined by closing over the
    /// elaborated set, so a set that could not be read has to fail closed. An
    /// autonomous solve takes its period from the oscillator node and reads no
    /// tone list, so it needs neither answer.
    #[test]
    fn exact_periodic_source_context_distinguishes_empty_from_unavailable() {
        let exact_empty = AnalysisDependencyRepairContext::exact_periodic_sources(
            "driven fixture\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.end\n",
        )
        .expect("an elaborated source set is authoritative");
        let mut driven = PssDialogState::default();
        driven.tone_sources = "V1".to_owned();
        exact_empty
            .validate_pss_sources(&driven)
            .expect("a driven PSS closing over the exact source set commits");

        let unavailable = AnalysisDependencyRepairContext::default();
        assert!(
            unavailable
                .validate_pss_sources(&driven)
                .unwrap_err()
                .contains("unavailable")
        );

        let mut autonomous = PssDialogState::default();
        autonomous.tone_sources.clear();
        autonomous.osc_mode = true;
        autonomous.osc_node = "out".to_owned();
        unavailable
            .validate_pss_sources(&autonomous)
            .expect("an autonomous solve does not consult the driven-source catalog at all");
    }

    /// An oscillator with a startup kick can run an autonomous PSS.
    ///
    /// It could not run one either way before. The autonomous mode refuses a
    /// tone list, and the driven periodic-source contract refuses an omitted
    /// source — and a kick PULSE is an elaborated transient source, so an empty
    /// selection failed the second rule and a full one failed the first.
    ///
    /// The engine has no such rule. `PssConfig` carries no tone field at all
    /// (`rspice-core/src/analysis/pss/config.rs`), and the shooting solve
    /// evaluates every transient source at the window time whatever the tone
    /// list said (`rspice-core/src/engine/pss.rs`, the `update_transient_rhs`
    /// pair in `pss_stamp_system`). The contract that was being applied says in
    /// its own first line that it defines a *driven* period
    /// (`rspice-core/src/engine/transient.rs:1349-1352`).
    #[test]
    fn an_autonomous_pss_runs_beside_a_startup_kick_that_a_driven_one_refuses() {
        let kicked = AnalysisDependencyRepairContext::exact_periodic_sources(
            "oscillator fixture\nVKICK n1 0 PULSE(0 1 0 1n 1n 10n 1)\nL1 n1 0 1u\nC1 n1 0 1p\n.end\n",
        )
        .expect("the kicked oscillator's source identity elaborates");

        let mut autonomous = PssDialogState::default();
        autonomous.tone_sources.clear();
        autonomous.osc_mode = true;
        autonomous.osc_node = "n1".to_owned();
        kicked
            .validate_pss_sources(&autonomous)
            .expect("an autonomous solve reads no tone list, so the kick omits nothing");

        // The driven rule is unchanged, and still names the source it is about:
        // an omitted transient source there really does drive a solve whose
        // period is defined by the list that leaves it out.
        let mut driven = PssDialogState::default();
        driven.tone_sources.clear();
        let error = driven
            .to_config()
            .expect_err("a driven solve with no tone is refused before the catalog is consulted");
        assert!(error.contains("periodic tone source"), "{error}");

        let mut driven = PssDialogState::default();
        driven.tone_sources = "VSTALE".to_owned();
        let error = kicked
            .validate_pss_sources(&driven)
            .expect_err("a driven solve must close over the elaborated source set");
        assert!(error.contains("omitted: VKICK"), "{error}");

        // And the tone list stays a control that ships connected: under
        // autonomous it is read by nothing, so authoring one is refused rather
        // than silently ignored.
        let mut contradictory = PssDialogState::default();
        contradictory.osc_mode = true;
        contradictory.osc_node = "n1".to_owned();
        contradictory.tone_sources = "VKICK".to_owned();
        let error = contradictory
            .to_config()
            .expect_err("an autonomous solve has no tone list to author");
        assert!(error.contains("reads no tone list"), "{error}");
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
        assert!(error.contains("frequencies must be integer multiples"));

        let pwl = AnalysisDependencyRepairContext::exact_periodic_sources(
            "periodic fixture\nV1 out 0 PWL(0 0 1u 1)\nR1 out 0 1k\n.end\n",
        )
        .expect("PWL source identity elaborates");
        let error = pwl
            .validate_pss_sources(&pss)
            .expect_err("repair cannot claim an unauthenticated PWL period");
        assert!(error.contains("source 'V1' is not certified periodic"));
    }

    #[test]
    fn periodic_source_preflight_resolves_edges_from_the_drafts_actual_grid() {
        let context = AnalysisDependencyRepairContext::exact_periodic_sources(
            "source defaults\nV1 in 0 PULSE(0 1 0.7u 0 0 0.28u 1u)\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        ).unwrap();
        let mut draft = PssDialogState::default();
        draft.fund_freq = "1meg".to_owned();
        draft.tone_sources = "V1".to_owned();
        draft.num_harmonics = "4".to_owned();
        for (points, periodic) in [(32, false), (512, true)] {
            draft.points_per_period = points.to_string();
            let result = context.validate_pss_sources(&draft);
            assert_eq!(result.is_ok(), periodic, "{points}: {result:?}");
        }
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
        // The two fields that name the user's circuit open unset, and
        // conversion says which one it is still waiting for.
        assert_eq!(draft.output, "");
        assert_eq!(draft.input, "");
        assert_eq!(draft.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(draft.integration_mode, NoiseIntegrationMode::Enabled);
        assert_eq!(
            draft.to_config().expect_err("no output node is named"),
            "output node or expression is required"
        );
        let named_output = NoiseDraft {
            output: "out".to_owned(),
            ..NoiseDraft::default()
        };
        assert_eq!(
            named_output
                .to_config()
                .expect_err("no input source is named"),
            "input source is required"
        );
        assert!(
            NoiseDraft {
                input: "V1".to_owned(),
                ..named_output
            }
            .to_config()
            .is_ok()
        );
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
        let AnalysisDraft::TransferFunction(mut draft) =
            AnalysisDraft::for_kind(AnalysisKind::TransferFunction)
        else {
            panic!("expected a transfer-function draft");
        };
        draft.input_source = "V1".to_owned();
        draft.output_expression = "V(out)".to_owned();
        assert_eq!(
            AnalysisDraft::TransferFunction(draft)
                .manifest_summary()
                .as_deref(),
            Some("V(out) <- V1 - DC operating point")
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
            AnalysisKind::AcData,
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

    /// A plan that stated only its frequencies still opens, referring to the
    /// table the writer will generate.
    ///
    /// The draft carries `deny_unknown_fields`, so a key it does not know is
    /// refused and an absent key is the `serde(default)` — this is the test
    /// that the default is the generated table name rather than an empty
    /// string, which would refuse the restored plan at its first validation.
    #[test]
    fn a_saved_frequency_table_plan_reopens_on_the_generated_table_name() {
        let saved = serde_json::json!({ "frequencies": "1k, 10k, 100k" });
        let draft: AcDataDraft =
            serde_json::from_value(saved).expect("a plan that stated only its axis still opens");
        assert_eq!(
            draft.table_name,
            crate::simulation::config::AC_FREQUENCY_TABLE
        );
        assert_eq!(draft.frequencies, "1k, 10k, 100k");
        let config = draft.to_config().expect("the restored plan is executable");
        assert_eq!(config.frequencies, vec![1.0e3, 1.0e4, 1.0e5]);

        // And the tagged draft round-trips under its own serde name, which is
        // what a saved project holds.
        let tagged = serde_json::to_string(&AnalysisDraft::AcData(draft))
            .expect("the tagged draft serializes");
        assert!(tagged.contains("\"acdata\""), "{tagged}");
        let restored: AnalysisDraft =
            serde_json::from_str(&tagged).expect("the tagged draft deserializes");
        assert_eq!(restored.kind(), AnalysisKind::AcData);
    }

    /// A plan saved before the share threshold existed still opens, listing
    /// the contributors it was listing.
    ///
    /// Same shim, same reason as the noise floor below: `deny_unknown_fields`
    /// says nothing about an absent key, so `serde(default)` is what lets the
    /// plan open at all, and this is the test that it is there. Empty rather
    /// than `0` is the value that reopens: both mean the same card, and empty
    /// is the spelling this form canonicalizes to.
    #[test]
    fn a_plan_saved_before_the_share_threshold_field_opens_without_one() {
        let saved = serde_json::json!({
            "output_expression": "V(out)",
            "sigma_multiplier": "1",
            "contributor_limit": "10",
            "include_process": false,
            "include_mismatch": true,
            "normalized_contributions": true
        });
        let draft: DcMismatchDraft =
            serde_json::from_value(saved).expect("a plan saved before the threshold still opens");
        assert!(
            draft.share_threshold.is_empty(),
            "a saved plan must reopen on the untrimmed list it ran: {:?}",
            draft.share_threshold
        );
        assert_eq!(draft.output_expression, "V(out)");
        assert_eq!(draft.contributor_limit, "10");
        assert!(validate_dc_mismatch(&draft).is_none());
    }

    /// An authored zero is the unauthored card, and reaches the run as one.
    ///
    /// The engine's own default threshold is exactly zero. Two
    /// specifications that differ only in how that zero was spelled would
    /// give one analysis two plan digests, so the two spellings are
    /// canonicalized to one before the specification is built.
    #[test]
    fn a_share_threshold_of_zero_is_the_unauthored_card() {
        assert_eq!(dc_mismatch_share_threshold(""), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("   "), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0"), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0.0"), Ok(None));
        assert_eq!(dc_mismatch_share_threshold("0.05"), Ok(Some(0.05)));
        assert_eq!(dc_mismatch_share_threshold("1"), Ok(Some(1.0)));
        assert!(dc_mismatch_share_threshold("half").is_err());

        // And the range belongs to the card, so an out-of-range share is
        // refused by the specification rather than here.
        let mut draft = DcMismatchDraft::default();
        draft.share_threshold = "1.5".to_owned();
        let refusal = validate_dc_mismatch(&draft).expect("a share above one is refused");
        assert!(
            refusal.contains("THRESHOLD must be a variance share in [0, 1]"),
            "{refusal}"
        );
    }

    /// A plan saved before the noise floor field existed still opens, running
    /// the band it was running.
    ///
    /// The draft carries `deny_unknown_fields`, which refuses keys it does not
    /// know and says nothing about keys that are absent — so the shim is the
    /// `serde(default)`, and this is the test that it is there. The restored
    /// value has to be the empty field rather than any frequency: empty is the
    /// engine's `1/tstop` derivation, which is what that saved plan asked for.
    #[test]
    fn a_plan_saved_before_the_noise_floor_field_opens_with_the_engine_default() {
        let saved = serde_json::json!({
            "stop_time": "1u",
            "step_time": "1n",
            "start_time": "0",
            "max_step": "10n",
            "seed": "1",
            "noise_fmax": "10G",
            "scale": "1",
            "use_initial_conditions": false
        });
        let draft: TransientNoiseDraft =
            serde_json::from_value(saved).expect("a plan saved before the floor still opens");
        assert!(
            draft.noise_fmin.is_empty(),
            "a saved plan must reopen on the derivation it ran, not on a frequency: {:?}",
            draft.noise_fmin
        );
        // Every field it did carry is still the field it carried.
        assert_eq!(draft.stop_time, "1u");
        assert_eq!(draft.noise_fmax, "10G");
        assert_eq!(draft.seed, "1");
        assert_eq!(draft.scale, "1");
        assert!(validate_transient_noise(&draft).is_none());
        // And the run that plan resolves to asks for no floor at all, which is
        // the fact the saved bytes were making.
        assert!(
            AnalysisDraft::TransientNoise(draft)
                .manifest_configuration_error()
                .is_none()
        );
    }

    #[test]
    fn manifest_draft_validation_rejects_incomplete_configuration() {
        let mut qpss = QpssDraft::default();
        qpss.tones = "1G".to_owned();
        assert!(validate_qpss(&qpss).is_some());

        let mut network = PeriodicNetworkDraft::default();
        network.ports[0].node_pos.clear();
        assert!(validate_periodic_network(&network).is_some());

        let mut psp = PeriodicNetworkDraft::default();
        psp.mixed_mode = true;
        assert!(
            validate_psp_network(&psp).is_none(),
            "an even equal-impedance port list supports mixed-mode conversion"
        );
        psp.noise_parameters = true;
        assert!(validate_psp_network(&psp).is_none());

        let mut hbsp = PeriodicNetworkDraft::default();
        hbsp.noise_parameters = true;
        assert!(validate_periodic_network(&hbsp).is_none());

        let mut tnoise = TransientNoiseDraft::default();
        tnoise.seed = "0".to_owned();
        assert!(validate_transient_noise(&tnoise).is_some());

        // An empty noise floor is the engine's derivation and refuses
        // nothing; an authored one has to sit under the ceiling.
        let mut floor = TransientNoiseDraft::default();
        assert!(floor.noise_fmin.is_empty());
        assert!(validate_transient_noise(&floor).is_none());
        floor.noise_fmin = "1k".to_owned();
        assert!(validate_transient_noise(&floor).is_none());
        floor.noise_fmin = "100G".to_owned();
        assert!(validate_transient_noise(&floor).is_some());

        let mut mismatch = DcMismatchDraft::default();
        mismatch.include_mismatch = false;
        assert!(validate_dc_mismatch(&mismatch).is_some());

        // Zero contributors is the card's own spelling of "list every one",
        // so the draft accepts it rather than refusing a legal card.
        let mut all = DcMismatchDraft::default();
        all.contributor_limit = "0".to_owned();
        assert!(
            validate_dc_mismatch(&all).is_none(),
            "{:?}",
            validate_dc_mismatch(&all)
        );
    }

    /// A fresh DC mismatch draft is the bare card `.DCMATCH OUT=V(out)`.
    ///
    /// Read from the engine rather than pinned to numbers: the card's
    /// defaults are parsed out of a bare `.DCMATCH` line and compared with
    /// the draft's, so a default Studio run and a hand-written card are the
    /// same analysis by construction. The draft used to open at `3` sigma
    /// over `25` contributors, which named a study no card described.
    #[test]
    fn a_default_dc_mismatch_draft_is_the_bare_engine_card() {
        let draft = DcMismatchDraft::default();
        assert!(validate_dc_mismatch(&draft).is_none());

        let netlist = rspice_core::netlist::Netlist::parse(
            "dc mismatch defaults\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.DCMATCH \
             OUT=V(out)\n.end\n",
        )
        .expect("a bare .DCMATCH card parses");
        let [rspice_core::netlist::AnalysisCommand::DcMatch(card)] = netlist.analyses.as_slice()
        else {
            panic!("the deck holds one .DCMATCH: {:?}", netlist.analyses);
        };

        assert_eq!(
            draft
                .sigma_multiplier
                .parse::<f64>()
                .expect("the default multiplier is a number"),
            card.sigma_multiplier
        );
        assert_eq!(
            draft
                .contributor_limit
                .parse::<usize>()
                .expect("the default limit is a count"),
            card.contributor_limit
        );
        assert_eq!(draft.include_mismatch, card.mismatch);
        assert_eq!(draft.include_process, card.process);
        // The parser canonicalizes the probe to upper case; the probe itself
        // is the same one.
        assert_eq!(
            draft.output_expression.to_ascii_uppercase(),
            format!("V({})", card.output_node)
        );
    }

    #[test]
    fn periodic_network_draft_accepts_discovery_single_ports_and_zero_sideband() {
        let mut draft = PeriodicNetworkDraft::default();
        draft.max_sideband = "0".to_owned();
        draft.ports.truncate(1);
        assert!(validate_periodic_network(&draft).is_none());
        draft.ports.clear();
        assert!(validate_periodic_network(&draft).is_none());
        for invalid in ["-1", "1.5", "2147483648"] {
            draft.max_sideband = invalid.to_owned();
            assert!(validate_periodic_network(&draft).is_some());
        }
    }
    #[test]
    fn hbnoise_reference_drafts_round_trip_and_legacy_defaults_remain_disabled() {
        let mut draft = HbNoiseDraft::default();
        draft.input_sideband = "-2".into();
        draft.output_sideband = "1".into();
        draft.noise_figure = true;
        draft.source_resistor = "Rs".into();
        draft.reference_temperature = "325".into();
        let expected = draft.noise_reference().unwrap();
        let json = serde_json::to_string(&draft).unwrap();
        let ron = ron::to_string(&draft).unwrap();
        let from_json: HbNoiseDraft = serde_json::from_str(&json).unwrap();
        assert_eq!(from_json.sidebands().unwrap(), (-2, 1));
        let from_ron: HbNoiseDraft = ron::from_str(&ron).unwrap();
        assert_eq!(from_json.noise_reference().unwrap(), expected);
        assert_eq!(from_ron.noise_reference().unwrap(), expected);
        let mut legacy = serde_json::to_value(HbNoiseDraft::default()).unwrap();
        legacy.as_object_mut().unwrap().remove("source_resistor");
        legacy
            .as_object_mut()
            .unwrap()
            .remove("reference_temperature");
        legacy.as_object_mut().unwrap().remove("input_sideband");
        legacy.as_object_mut().unwrap().remove("output_sideband");
        let restored: HbNoiseDraft = serde_json::from_value(legacy).unwrap();
        assert_eq!(restored.sidebands().unwrap(), (0, 0));
        assert!(!restored.noise_figure);
        assert_eq!(restored.noise_reference().unwrap(), None);
        assert_eq!(restored.reference_temperature, "290");
    }
}
