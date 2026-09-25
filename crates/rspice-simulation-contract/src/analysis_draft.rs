//! Canonical persisted analysis drafts and their portable plan semantics.

use serde::{Deserialize, Serialize};

use crate::analysis_kind::AnalysisKind;
use crate::corner_draft::CornerDialogState;
use crate::drafts::{
    AcDataDraft, AcSetup, DcMismatchDraft, DcSetup, DistoDraft, FftDraft, NoiseDraft, TranSetup,
    TransientNoiseDraft, validate_dc_mismatch, validate_transient_noise,
};
use crate::envelope_draft::EnvelopeDialogState;
use crate::fourier_draft::FourierDialogState;
use crate::hb_draft::HbDialogState;
use crate::hbnoise_draft::{HbNoiseDraft, validate_hbnoise};
use crate::mc_draft::McDialogState;
use crate::op_draft::OpDialogState;
use crate::optimization_draft::OptimizationDialogState;
use crate::pac_draft::PacDialogState;
use crate::periodic_carrier::PeriodicCarrier;
use crate::periodic_network_draft::{
    PeriodicNetworkDraft, validate_periodic_network, validate_psp_network,
};
use crate::pnoise_draft::PnoiseDialogState;
use crate::pss_draft::PssDialogState;
use crate::pstb_draft::PstbDialogState;
use crate::pxf_draft::PxfDialogState;
use crate::pz_draft::PzDialogState;
use crate::quasi_periodic_draft::{
    QpssDraft, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft,
    validate_qpss,
};
use crate::sens_draft::SensDialogState;
use crate::soa_draft::SoaDialogState;
use crate::sp_draft::SpDialogState;
use crate::stb_draft::StbDialogState;
use crate::temp_draft::TempDialogState;
use crate::xf_draft::XfDialogState;

/// AC sweep draft shared structurally by AC and DISTO, but never shared by
/// identity. Each analysis instance owns a deep copy.
pub type AcDraft = AcSetup;

/// Raw, lossless configuration draft for one executable analysis instance.
///
/// Drafts intentionally retain strings rather than parsed engine values so a
/// project can round-trip an in-progress edit without silently rewriting it.
/// The enum tag is the stable analysis ID; legacy numeric indices are resolved
/// through [`AnalysisKind::from_legacy_index`] before a draft is constructed.
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
    Envelope(Box<EnvelopeDialogState>),
    #[serde(rename = "fourier")]
    Fourier(FourierDialogState),
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
    /// Saved base used by a Temperature or Corner point expansion.
    pub fn pvt_base_analysis(&self) -> Option<rspice_app_types::product::AnalysisInstanceId> {
        match self {
            Self::Temperature(state) => state.base_analysis,
            Self::Corner(state) => state.base_analysis,
            _ => None,
        }
    }

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
            AnalysisKind::Envelope => {
                Self::Envelope(Box::new(initialized_default!(EnvelopeDialogState)))
            }
            AnalysisKind::Fourier => Self::Fourier(initialized_default!(FourierDialogState)),
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

    /// What this instance's own controls assign after the deck is resolved.
    ///
    /// Operating point and transfer function carry numerical policies;
    /// Envelope additionally decides whether an HB initializer will run.
    ///
    /// Read off the draft's stored index rather than through `to_config`,
    /// because a draft that does not yet validate — a half-typed temperature —
    /// still has a tier, and a refusal that disappeared while a field was
    /// being edited would be a gate nobody could rely on.
    #[must_use]
    pub fn solver_ownership(&self) -> crate::numeric_override::SolverOwnership {
        use crate::accuracy::AnalysisAccuracy;
        use crate::config::OpHomotopy;
        use crate::numeric_override::SolverOwnership;

        match self {
            Self::OperatingPoint(state) => SolverOwnership {
                accuracy: AnalysisAccuracy::ALL.get(state.accuracy_idx).copied(),
                homotopy: OpHomotopy::ALL.get(state.homotopy_idx).copied(),
                ..SolverOwnership::NONE
            },
            Self::TransferFunction(state) => SolverOwnership {
                accuracy: AnalysisAccuracy::ALL.get(state.accuracy_idx).copied(),
                homotopy: None,
                ..SolverOwnership::NONE
            },
            Self::Temperature(state) if state.base_analysis.is_none() => SolverOwnership {
                time_integration: Some(state.base_idx == 1),
                ..SolverOwnership::NONE
            },
            Self::Corner(state) if state.base_analysis.is_none() => SolverOwnership {
                time_integration: Some(matches!(
                    state.base_analysis(),
                    crate::corner_config::CornerBaseAnalysis::Transient
                )),
                ..SolverOwnership::NONE
            },
            Self::MonteCarlo(state) if state.base_analysis.is_none() => SolverOwnership {
                time_integration: Some(false),
                ..SolverOwnership::NONE
            },
            Self::Optimization(state) if state.base_analysis.is_none() => SolverOwnership {
                time_integration: Some(false),
                ..SolverOwnership::NONE
            },
            Self::Envelope(state) => SolverOwnership {
                multirate_envelope_dc: state
                    .multirate_enabled
                    .then_some(state.multirate.dc_initialization),
                hb_initializer: Some(
                    !state.multirate_enabled && state.initial_periodic_solve_idx == 0,
                ),
                ..SolverOwnership::NONE
            },
            _ => SolverOwnership::NONE,
        }
    }

    /// Solver ownership including choices stored in the numerical options.
    #[must_use]
    pub fn solver_ownership_with_options(
        &self,
        record: Option<&crate::numeric_override::AnalysisNumericOverride>,
    ) -> crate::numeric_override::SolverOwnership {
        use crate::numeric_override::{NumericOverrideOption, OverrideValue};
        use crate::options::HbTimeDomainMode;

        let mut ownership = self.solver_ownership();
        if matches!(self, Self::HarmonicBalance(_)) {
            ownership.time_integration = Some(matches!(
                record.and_then(|record| record.stated(NumericOverrideOption::HbInitialState)),
                Some(OverrideValue::TimeDomainMode(
                    HbTimeDomainMode::TransientAssisted
                ))
            ));
        }
        ownership
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
            Self::Qpac(draft) => draft.to_spec().err(),
            Self::Qpnoise(draft) => draft.to_spec().err(),
            Self::Qpxf(draft) => draft.to_spec().err(),
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
            Self::Qpnoise(draft) => Some(draft.summary()),
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
