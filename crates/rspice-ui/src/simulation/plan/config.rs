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
            | Self::Disto(_) => {}
        }
    }
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
        assert!(AnalysisDraft::from_legacy_index(25).is_none());
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
}
