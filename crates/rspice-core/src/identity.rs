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
    Pxf,
    PNoise,
    Pstb,
    HarmonicBalance,
    Envelope,
    MonteCarlo,
    Fourier,
    Fft,
    Soa,
    Optimize,
    Psp,
    Hbsp,
    HbNoise,
    Qpss,
    Qpac,
    Qpnoise,
    Qpxf,
    DcMatch,
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
            Self::Pxf => "pxf",
            Self::PNoise => "pnoise",
            Self::Pstb => "pstb",
            Self::HarmonicBalance => "hb",
            Self::Envelope => "env",
            Self::MonteCarlo => "mc",
            Self::Fourier => "four",
            Self::Fft => "fft",
            Self::Soa => "soa",
            Self::Optimize => "optimize",
            Self::Psp => "psp",
            Self::Hbsp => "hbsp",
            Self::HbNoise => "hbnoise",
            Self::Qpss => "qpss",
            Self::Qpac => "qpac",
            Self::Qpnoise => "qpnoise",
            Self::Qpxf => "qpxf",
            Self::DcMatch => "dcmatch",
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
    /// Construct an authored analysis identity from its zero-based card ordinal.
    pub const fn new(kind: AnalysisKind, ordinal: u32) -> Self {
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

    /// Every kind's tag is a distinct lower-case token.
    ///
    /// The tag is the kind's external spelling: it names a result document's
    /// analysis on the wire and an instance in every output name, so two
    /// kinds sharing one — or one carrying a capital that a reader would
    /// type in lower case — is a collision rather than a cosmetic slip.
    /// There is no parse-from-tag to round-trip through; this is the
    /// property such a parser would need.
    #[test]
    fn every_analysis_kind_tag_is_a_distinct_lower_case_token() {
        let kinds = [
            AnalysisKind::ImplicitOp,
            AnalysisKind::Op,
            AnalysisKind::Dc,
            AnalysisKind::Ac,
            AnalysisKind::Tran,
            AnalysisKind::Noise,
            AnalysisKind::Sp,
            AnalysisKind::Stb,
            AnalysisKind::Distortion,
            AnalysisKind::PoleZero,
            AnalysisKind::Sensitivity,
            AnalysisKind::TransferFunction,
            AnalysisKind::Pss,
            AnalysisKind::Pac,
            AnalysisKind::Pxf,
            AnalysisKind::PNoise,
            AnalysisKind::Pstb,
            AnalysisKind::HarmonicBalance,
            AnalysisKind::Envelope,
            AnalysisKind::MonteCarlo,
            AnalysisKind::Fourier,
            AnalysisKind::Fft,
            AnalysisKind::Soa,
            AnalysisKind::Optimize,
            AnalysisKind::Psp,
            AnalysisKind::Hbsp,
            AnalysisKind::HbNoise,
            AnalysisKind::Qpss,
            AnalysisKind::Qpac,
            AnalysisKind::Qpnoise,
            AnalysisKind::Qpxf,
            AnalysisKind::DcMatch,
        ];

        let mut tags = kinds.map(AnalysisKind::tag).to_vec();
        tags.sort_unstable();
        let distinct = tags.len();
        tags.dedup();
        assert_eq!(tags.len(), distinct, "two kinds share a tag: {tags:?}");
        for tag in &tags {
            assert!(
                !tag.is_empty()
                    && tag
                        .chars()
                        .all(|character| character.is_ascii_lowercase() || character == '-'),
                "{tag} is not a lower-case token"
            );
        }
    }

    /// The ten kinds this release names for the surfaces above it.
    ///
    /// They exist so a plan, a project and a result document can carry the
    /// kind while the engine learns to run it. Pinned by tag because the tag
    /// is what persists: renaming one would silently orphan every saved
    /// document that named it.
    #[test]
    fn the_newly_named_kinds_keep_the_tags_they_were_declared_with() {
        assert_eq!(
            [
                AnalysisKind::Soa,
                AnalysisKind::Optimize,
                AnalysisKind::Psp,
                AnalysisKind::Hbsp,
                AnalysisKind::HbNoise,
                AnalysisKind::Qpss,
                AnalysisKind::Qpac,
                AnalysisKind::Qpnoise,
                AnalysisKind::Qpxf,
                AnalysisKind::DcMatch,
            ]
            .map(AnalysisKind::tag),
            [
                "soa", "optimize", "psp", "hbsp", "hbnoise", "qpss", "qpac", "qpnoise", "qpxf",
                "dcmatch",
            ]
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
