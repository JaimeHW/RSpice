use serde::{Deserialize, Serialize};

use crate::common::app::{AcSetup, DcSetup, NoiseSetup, TranSetup};
use crate::simulation::dialog::{
    CornerDialogState, EnvelopeDialogState, FourierDialogState, HbDialogState, McDialogState,
    OpDialogState, OptimizationDialogState, PacDialogState, PnoiseDialogState, PssDialogState,
    PstbDialogState, PxfDialogState, PzDialogState, ReliabilityDialogState, SensDialogState,
    SoaDialogState, SpDialogState, StbDialogState, TempDialogState, XfDialogState,
};

use super::AnalysisKind;

/// AC sweep draft shared structurally by AC and DISTO, but never shared by
/// identity. Each analysis instance owns a deep copy.
pub type AcDraft = AcSetup;

/// Noise draft with an independent sweep density and mode.
///
/// The legacy singleton setup borrowed these two values from AC. Keeping them
/// here removes that hidden cross-analysis mutation while preserving raw text
/// buffers that may be temporarily incomplete during editing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoiseDraft {
    pub output: String,
    pub reference: String,
    pub input: String,
    pub fstart: String,
    pub fstop: String,
    pub points: String,
    /// 0 = decade, 1 = octave, 2 = linear.
    pub sweep: usize,
}

impl Default for NoiseDraft {
    fn default() -> Self {
        let noise = NoiseSetup::default();
        let ac = AcSetup::default();
        Self {
            output: noise.output,
            reference: noise.reference,
            input: noise.input,
            fstart: noise.fstart,
            fstop: noise.fstop,
            points: ac.points,
            sweep: ac.sweep,
        }
    }
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
            Self::TransferFunction(state) => state.initialized = true,
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
            _ => None,
        }
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
        assert_eq!(noise.points, "101");
        assert_eq!(disto.sweep.points, "101");
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
