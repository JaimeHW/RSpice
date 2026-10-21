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
    /// Canonical inventory for capability discovery and persisted plan inspection.
    /// Entries retain their identity even when a frontend cannot execute them.
    pub const ALL: [Self; 32] = [
        Self::ImplicitOp,
        Self::Op,
        Self::Dc,
        Self::Ac,
        Self::Tran,
        Self::Noise,
        Self::Sp,
        Self::Stb,
        Self::Distortion,
        Self::PoleZero,
        Self::Sensitivity,
        Self::TransferFunction,
        Self::Pss,
        Self::Pac,
        Self::Pxf,
        Self::PNoise,
        Self::Pstb,
        Self::HarmonicBalance,
        Self::Envelope,
        Self::MonteCarlo,
        Self::Fourier,
        Self::Fft,
        Self::Soa,
        Self::Optimize,
        Self::Psp,
        Self::Hbsp,
        Self::HbNoise,
        Self::Qpss,
        Self::Qpac,
        Self::Qpnoise,
        Self::Qpxf,
        Self::DcMatch,
    ];

    /// The control command that runs this analysis directly, if the control
    /// host has one; `ControlCircuit::analysis_command_name` publishes it.
    ///
    /// The table sits beside the enum because the match is exhaustive only
    /// here: a kind added later fails to compile until it says whether the
    /// control host runs it.
    #[doc(hidden)]
    pub const fn control_command(self) -> Option<&'static str> {
        match self {
            Self::Op => Some("op"),
            Self::Dc => Some("dc"),
            Self::Ac => Some("ac"),
            Self::Noise => Some("noise"),
            Self::Tran => Some("tran"),
            Self::TransferFunction => Some("tf"),
            Self::PoleZero => Some("pz"),
            Self::Sensitivity => Some("sens"),
            Self::Distortion => Some("disto"),
            Self::Stb => Some("stb"),
            Self::DcMatch => Some("dcmatch"),
            Self::ImplicitOp
            | Self::Sp
            | Self::Pss
            | Self::Pac
            | Self::Pxf
            | Self::PNoise
            | Self::Pstb
            | Self::HarmonicBalance
            | Self::Envelope
            | Self::MonteCarlo
            | Self::Fourier
            | Self::Fft
            | Self::Soa
            | Self::Optimize
            | Self::Psp
            | Self::Hbsp
            | Self::HbNoise
            | Self::Qpss
            | Self::Qpac
            | Self::Qpnoise
            | Self::Qpxf => None,
        }
    }
}

/// Declare each kind's tag once: the spelling and its reading back are the
/// same table, so a kind added later cannot compile without a tag, and no tag
/// can name a kind that does not read back as it.
macro_rules! analysis_kind_tags {
    ($($variant:ident => $tag:literal,)+) => {
        impl AnalysisKind {
            pub const fn tag(self) -> &'static str {
                match self {
                    $(Self::$variant => $tag,)+
                }
            }

            /// The kind [`Self::tag`] spells as `tag`, or `None` for a tag no
            /// kind carries. Result documents read their analysis back
            /// through it.
            #[doc(hidden)]
            pub fn from_tag(tag: &str) -> Option<Self> {
                match tag {
                    $($tag => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

analysis_kind_tags! {
    ImplicitOp => "implicit-op",
    Op => "op",
    Dc => "dc",
    Ac => "ac",
    Tran => "tran",
    Noise => "noise",
    Sp => "sp",
    Stb => "stb",
    Distortion => "disto",
    PoleZero => "pz",
    Sensitivity => "sens",
    TransferFunction => "tf",
    Pss => "pss",
    Pac => "pac",
    Pxf => "pxf",
    PNoise => "pnoise",
    Pstb => "pstb",
    HarmonicBalance => "hb",
    Envelope => "env",
    MonteCarlo => "mc",
    Fourier => "four",
    Fft => "fft",
    Soa => "soa",
    Optimize => "optimize",
    Psp => "psp",
    Hbsp => "hbsp",
    HbNoise => "hbnoise",
    Qpss => "qpss",
    Qpac => "qpac",
    Qpnoise => "qpnoise",
    Qpxf => "qpxf",
    DcMatch => "dcmatch",
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
    #[doc(hidden)]
    pub const fn from_parts(semantic: [u8; 16], occurrence: u32) -> Self {
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
    /// Every tag also reads back as its own kind.
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

        assert_eq!(AnalysisKind::ALL, kinds);
        for kind in kinds {
            assert_eq!(AnalysisKind::from_tag(kind.tag()), Some(kind));
        }
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
