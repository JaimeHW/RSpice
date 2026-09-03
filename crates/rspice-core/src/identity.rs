//! Stable identities for authored analyses and run coordinates.
//!
//! These three types name *which* analysis instance and *which* sweep point a
//! piece of evidence belongs to. Deck planning mints them, output naming
//! consumes them, and — since a typed error is evidence too — the engine's
//! error taxonomy carries them.
//!
//! They live in their own layer-0 leaf for exactly that reason. The planner in
//! `crate::execution` sits above the engine, so an error type in
//! `crate::engine` cannot reach up for the identity of the analysis it is
//! reporting on. Identity is vocabulary, not orchestration: it depends on
//! nothing and both layers read down into it.

use std::fmt;

/// Physical or post-processing analysis identity used by every frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum AnalysisKind {
    ImplicitOp,
    Op,
    Dc,
    Ac,
    Tran,
    Noise,
    Sp,
    Stb,
    Distortion,
    PoleZero,
    Sensitivity,
    TransferFunction,
    Pss,
    Pac,
    PNoise,
    HarmonicBalance,
    Envelope,
    MonteCarlo,
    Fourier,
    Fft,
}

impl AnalysisKind {
    pub const fn tag(self) -> &'static str {
        match self {
            Self::ImplicitOp => "implicit-op",
            Self::Op => "op",
            Self::Dc => "dc",
            Self::Ac => "ac",
            Self::Tran => "tran",
            Self::Noise => "noise",
            Self::Sp => "sp",
            Self::Stb => "stb",
            Self::Distortion => "disto",
            Self::PoleZero => "pz",
            Self::Sensitivity => "sens",
            Self::TransferFunction => "tf",
            Self::Pss => "pss",
            Self::Pac => "pac",
            Self::PNoise => "pnoise",
            Self::HarmonicBalance => "hb",
            Self::Envelope => "env",
            Self::MonteCarlo => "mc",
            Self::Fourier => "four",
            Self::Fft => "fft",
        }
    }
}

/// Stable identity of one authored analysis card.
///
/// `ordinal` is zero-based in memory. Its external tag is one-based so a
/// repeated pair of `.AC` cards becomes `ac-001` and `ac-002`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisInstanceId {
    kind: AnalysisKind,
    ordinal: u32,
}

impl AnalysisInstanceId {
    pub(crate) const fn new(kind: AnalysisKind, ordinal: u32) -> Self {
        Self { kind, ordinal }
    }

    pub const fn kind(self) -> AnalysisKind {
        self.kind
    }

    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub fn tag(self) -> String {
        format!("{}-{:03}", self.kind.tag(), u64::from(self.ordinal) + 1)
    }
}

impl fmt::Display for AnalysisInstanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.tag())
    }
}

/// Stable identity of one run coordinate produced by the deck's run axes.
///
/// `semantic` digests the coordinate's axis assignments so the same point in
/// the same deck keeps its name across runs; `occurrence` disambiguates the
/// rare digest collision within one plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RunCoordinateId {
    semantic: [u8; 16],
    occurrence: u32,
}

impl RunCoordinateId {
    pub(crate) const fn from_parts(semantic: [u8; 16], occurrence: u32) -> Self {
        Self {
            semantic,
            occurrence,
        }
    }

    pub const fn semantic_bytes(self) -> [u8; 16] {
        self.semantic
    }

    pub const fn occurrence(self) -> u32 {
        self.occurrence
    }
}

impl fmt::Display for RunCoordinateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.semantic {
            write!(formatter, "{byte:02x}")?;
        }
        write!(formatter, "-{:03}", u64::from(self.occurrence) + 1)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_instance_tags_are_one_based_per_kind() {
        assert_eq!(AnalysisInstanceId::new(AnalysisKind::Ac, 0).tag(), "ac-001");
        assert_eq!(AnalysisInstanceId::new(AnalysisKind::Ac, 1).tag(), "ac-002");
        assert_eq!(
            AnalysisInstanceId::new(AnalysisKind::ImplicitOp, 0).to_string(),
            "implicit-op-001"
        );
    }

    #[test]
    fn coordinate_ids_render_digest_then_one_based_occurrence() {
        let id = RunCoordinateId::from_parts([0x0a; 16], 2);
        assert_eq!(id.to_string(), format!("{}-003", "0a".repeat(16)));
        assert_eq!(id.semantic_bytes(), [0x0a; 16]);
        assert_eq!(id.occurrence(), 2);
    }
}
