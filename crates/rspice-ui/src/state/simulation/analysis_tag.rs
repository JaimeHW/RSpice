//! The canonical analysis-tag protocol.
//!
//! A prepared task carries one numeric tag. Three separate questions are asked
//! about that number, and they must never disagree:
//!
//! 1. Which tag does dispatch stamp onto a task? The tag is part of the config
//!    digest, so a tag can be appended but never reused or renumbered — doing
//!    either would silently redefine every prepared snapshot already on disk.
//! 2. May a receipt accept the tag? The protocol is closed: a tag this binary
//!    cannot produce must be refused rather than allowed to masquerade as a
//!    task this binary executed.
//! 3. Which result family does the task produce? A retained result is matched
//!    to its authenticated task by that family, so a wrong answer rejects a
//!    correct run.
//!
//! Answering them from three independent tables is how the protocol broke
//! before: dispatch learned tags 27, 28, 29 and 35 while the receipt layer
//! kept refusing everything above 25, so preflight reported a plan runnable
//! and dispatch then refused the receipt it had just authorized. This enum is
//! the one place all three answers live, and every consumer derives from it.

use super::AnalysisType;

/// What a run of a given analysis kind may be cited for.
///
/// One owner, read by the studio's analysis catalog before a run and by the
/// prepared receipt's sign-off verdict after one. A preview engine still
/// produces real numbers and is never hidden or blocked; it is stamped, so a
/// preview result cannot be carried into a sign-off package as though it had
/// cleared the production bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisAvailability {
    Production,
    Preview,
}

impl AnalysisAvailability {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Production => "Production",
            Self::Preview => "Preview · non-sign-off",
        }
    }

    /// Whether a result produced by this tier may be cited as sign-off.
    #[must_use]
    pub const fn blocks_sign_off(self) -> bool {
        matches!(self, Self::Preview)
    }
}

/// One analysis kind of the canonical execution-tag protocol.
///
/// Variants are ordered by tag for readability only; [`Self::tag`] is the
/// authority and is append-only. A variant is finer-grained than a plan
/// analysis kind where the execution payload differs — `Ac` and `AcData` are
/// two tags because a table-driven AC sweep is a different sealed request —
/// and coarser where two specifications execute identically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalAnalysisKind {
    DcOp,
    DcSweep,
    Ac,
    AcData,
    Disto,
    Transient,
    Noise,
    Pss,
    HarmonicBalance,
    Tf,
    Sensitivity,
    PoleZero,
    Pac,
    Pnoise,
    Pxf,
    Pstb,
    Stb,
    MonteCarlo,
    Parametric,
    Corner,
    Reliability,
    Optimization,
    Soa,
    SParameter,
    Envelope,
    Fourier,
    Qpss,
    Hbsp,
    Hbnoise,
    Psp,
    Qpac,
    Qpnoise,
    Qpxf,
    TransientNoise,
    DcMismatch,
    PssSpectrum,
}

impl CanonicalAnalysisKind {
    /// Every kind the protocol defines, in tag order.
    ///
    /// This is what makes the protocol closed without a second literal:
    /// [`Self::from_tag`] searches this array, so a tag is acceptable exactly
    /// when some kind claims it.
    pub const ALL: [Self; 36] = [
        Self::DcOp,
        Self::DcSweep,
        Self::Ac,
        Self::AcData,
        Self::Disto,
        Self::Transient,
        Self::Noise,
        Self::Pss,
        Self::HarmonicBalance,
        Self::Tf,
        Self::Sensitivity,
        Self::PoleZero,
        Self::Pac,
        Self::Pnoise,
        Self::Pxf,
        Self::Pstb,
        Self::Stb,
        Self::MonteCarlo,
        Self::Parametric,
        Self::Corner,
        Self::Reliability,
        Self::Optimization,
        Self::Soa,
        Self::SParameter,
        Self::Envelope,
        Self::Fourier,
        Self::Qpss,
        Self::Hbsp,
        Self::Hbnoise,
        Self::Psp,
        Self::Qpac,
        Self::Qpnoise,
        Self::Qpxf,
        Self::TransientNoise,
        Self::DcMismatch,
        Self::PssSpectrum,
    ];

    /// The numeric tag stamped into prepared snapshots and receipts.
    ///
    /// Appended, never renumbered: these tags are part of the config digest,
    /// so reusing or reordering one would silently redefine every prepared
    /// snapshot already on disk. Tags 26 and 30..=34 name kinds whose solver
    /// is not present in this engine build; the tag still exists because the
    /// specification is transportable and its identity must stay stable, and
    /// [`crate::simulation::plan::AnalysisKind::execution_blocker`] is what
    /// keeps them from being dispatched.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::DcOp => 0,
            Self::DcSweep => 1,
            Self::Ac => 2,
            Self::AcData => 3,
            Self::Disto => 4,
            Self::Transient => 5,
            Self::Noise => 6,
            Self::Pss => 7,
            Self::HarmonicBalance => 8,
            Self::Tf => 9,
            Self::Sensitivity => 10,
            Self::PoleZero => 11,
            Self::Pac => 12,
            Self::Pnoise => 13,
            Self::Pxf => 14,
            Self::Pstb => 15,
            Self::Stb => 16,
            Self::MonteCarlo => 17,
            Self::Parametric => 18,
            Self::Corner => 19,
            Self::Reliability => 20,
            Self::Optimization => 21,
            Self::Soa => 22,
            Self::SParameter => 23,
            Self::Envelope => 24,
            Self::Fourier => 25,
            Self::Qpss => 26,
            Self::Hbsp => 27,
            Self::Hbnoise => 28,
            Self::Psp => 29,
            Self::Qpac => 30,
            Self::Qpnoise => 31,
            Self::Qpxf => 32,
            Self::TransientNoise => 33,
            Self::DcMismatch => 34,
            Self::PssSpectrum => 35,
        }
    }

    /// The kind that claims `tag`, or `None` when no kind does.
    ///
    /// `None` is the closed-protocol refusal: a persisted tag this binary does
    /// not define is not a task this binary produced.
    #[must_use]
    pub const fn from_tag(tag: u8) -> Option<Self> {
        let mut index = 0;
        while index < Self::ALL.len() {
            let kind = Self::ALL[index];
            if kind.tag() == tag {
                return Some(kind);
            }
            index += 1;
        }
        None
    }

    /// What this kind's engine is fit to be cited for.
    ///
    /// The tag is what a sealed receipt retains, so this is the only spelling
    /// of the question a *finished* run can be asked — the plan's
    /// `AnalysisKind::availability` reads it through `canonical_kind`, and
    /// `PreparedRunReceipt::is_sign_off_eligible` reads it through the tasks it
    /// authenticated. Keeping the answer on the tag is what stops the studio's
    /// catalog, Verify's sign-off tile and the Results manifest from holding
    /// three opinions about one run.
    ///
    /// A tag with no plan kind of its own inherits the tier of the kind that
    /// produces it: `AcData` is an `.ac` sweep read from a table, `Parametric`
    /// is what a temperature sweep executes as, and `PssSpectrum` is the
    /// spectral projection of a PSS solve.
    #[must_use]
    pub const fn availability(self) -> AnalysisAvailability {
        match self {
            Self::Qpss
            | Self::Hbsp
            | Self::Hbnoise
            | Self::Envelope
            | Self::Psp
            | Self::Qpac
            | Self::Qpnoise
            | Self::Qpxf
            | Self::TransientNoise
            | Self::DcMismatch
            | Self::Reliability => AnalysisAvailability::Preview,
            _ => AnalysisAvailability::Production,
        }
    }

    /// The result family a task of this kind produces.
    ///
    /// This is the same answer the execution controller gives when it labels a
    /// finished result, because the controller derives it from here. The two
    /// are compared directly by
    /// [`PreparedRunReceipt::validate_result_prefix`](super::PreparedRunReceipt::validate_result_prefix),
    /// so a second opinion would reject correct runs.
    #[must_use]
    pub const fn result_analysis_type(self) -> AnalysisType {
        match self {
            Self::DcOp => AnalysisType::DcOp,
            Self::DcSweep => AnalysisType::DcSweep,
            Self::Ac | Self::AcData => AnalysisType::Ac,
            Self::Disto => AnalysisType::Disto,
            Self::Transient => AnalysisType::Transient,
            Self::Noise => AnalysisType::Noise,
            Self::Pss => AnalysisType::Pss,
            // A retained coefficient spectrum is exactly what the harmonic
            // balance viewer already draws, so the spectrum reuses that owner
            // rather than introducing a second one that renders the same fact.
            Self::HarmonicBalance | Self::PssSpectrum => AnalysisType::HarmonicBalance,
            Self::Tf => AnalysisType::Tf,
            Self::Sensitivity => AnalysisType::Sensitivity,
            Self::PoleZero => AnalysisType::PoleZero,
            Self::Pac => AnalysisType::Pac,
            Self::Pnoise => AnalysisType::Pnoise,
            Self::Pxf => AnalysisType::Pxf,
            Self::Pstb => AnalysisType::Pstb,
            Self::Stb => AnalysisType::Stb,
            Self::MonteCarlo => AnalysisType::MonteCarlo,
            Self::Parametric => AnalysisType::Parametric,
            Self::Corner => AnalysisType::Corner,
            Self::Reliability => AnalysisType::Reliability,
            Self::Optimization => AnalysisType::Optimization,
            Self::Soa => AnalysisType::Soa,
            Self::SParameter => AnalysisType::SParameter,
            Self::Envelope => AnalysisType::Envelope,
            Self::Fourier => AnalysisType::Fourier,
            Self::Qpss => AnalysisType::Qpss,
            Self::Hbsp => AnalysisType::Hbsp,
            Self::Hbnoise => AnalysisType::Hbnoise,
            Self::Psp => AnalysisType::Psp,
            Self::Qpac => AnalysisType::Qpac,
            Self::Qpnoise => AnalysisType::Qpnoise,
            Self::Qpxf => AnalysisType::Qpxf,
            Self::TransientNoise => AnalysisType::TransientNoise,
            Self::DcMismatch => AnalysisType::DcMismatch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_defined_tag_is_distinct_and_round_trips() {
        let mut seen = Vec::with_capacity(CanonicalAnalysisKind::ALL.len());
        for kind in CanonicalAnalysisKind::ALL {
            let tag = kind.tag();
            assert!(
                !seen.contains(&tag),
                "tag {tag} is claimed by more than one canonical kind"
            );
            seen.push(tag);
            assert_eq!(
                CanonicalAnalysisKind::from_tag(tag),
                Some(kind),
                "tag {tag} does not resolve back to the kind that claims it"
            );
        }
    }

    #[test]
    fn the_protocol_is_contiguous_and_closed_above_its_high_water_mark() {
        // Contiguity is not a rule the protocol needs, but a gap would mean a
        // retired tag, and retiring one redefines historical snapshots. Assert
        // it so a deletion has to argue with a test rather than pass quietly.
        for (index, kind) in CanonicalAnalysisKind::ALL.into_iter().enumerate() {
            assert_eq!(
                u8::try_from(index).expect("the protocol is far smaller than 256 kinds"),
                kind.tag(),
                "canonical tags must stay contiguous in ALL order"
            );
        }
        let highest = CanonicalAnalysisKind::ALL[CanonicalAnalysisKind::ALL.len() - 1].tag();
        assert_eq!(highest, 35);
        assert!(CanonicalAnalysisKind::from_tag(highest + 1).is_none());
        assert!(CanonicalAnalysisKind::from_tag(u8::MAX).is_none());
    }

    #[test]
    fn the_four_tags_dispatch_gained_after_the_cap_shipped_are_accepted() {
        // Tags 27, 28, 29 and 35 were emitted by dispatch while the receipt
        // layer still refused every tag above 25. Name them explicitly: this
        // is the exact regression the single table exists to prevent.
        for (tag, expected) in [
            (27, AnalysisType::Hbsp),
            (28, AnalysisType::Hbnoise),
            (29, AnalysisType::Psp),
            (35, AnalysisType::HarmonicBalance),
        ] {
            let kind = CanonicalAnalysisKind::from_tag(tag)
                .unwrap_or_else(|| panic!("tag {tag} must be part of the closed protocol"));
            assert_eq!(kind.result_analysis_type(), expected, "tag {tag}");
        }
    }
}
