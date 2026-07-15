use serde::{Deserialize, Serialize};

/// Canonical identity of an analysis kind supported by the current engine.
///
/// Serialized names are stable product IDs. Numeric indices exist only for
/// deterministic migration from the current singleton setup model; they are
/// never analysis-instance identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AnalysisKind {
    #[serde(rename = "op")]
    OperatingPoint,
    #[serde(rename = "tran")]
    Transient,
    #[serde(rename = "ac")]
    Ac,
    #[serde(rename = "dc")]
    DcSweep,
    #[serde(rename = "noise")]
    Noise,
    #[serde(rename = "pz")]
    PoleZero,
    #[serde(rename = "sens")]
    Sensitivity,
    #[serde(rename = "mc")]
    MonteCarlo,
    #[serde(rename = "pss")]
    Pss,
    #[serde(rename = "stb")]
    Stb,
    #[serde(rename = "temp")]
    Temperature,
    #[serde(rename = "hb")]
    HarmonicBalance,
    #[serde(rename = "sp")]
    SParameter,
    #[serde(rename = "pac")]
    Pac,
    #[serde(rename = "pnoise")]
    Pnoise,
    #[serde(rename = "pxf")]
    Pxf,
    #[serde(rename = "pstb")]
    Pstb,
    #[serde(rename = "xf")]
    TransferFunction,
    #[serde(rename = "corner")]
    Corner,
    #[serde(rename = "envelope")]
    Envelope,
    #[serde(rename = "fourier")]
    Fourier,
    #[serde(rename = "reliability")]
    Reliability,
    #[serde(rename = "opt")]
    Optimization,
    #[serde(rename = "soa")]
    Soa,
    #[serde(rename = "disto")]
    Disto,
}

impl AnalysisKind {
    /// All currently executable kinds in the legacy index order.
    pub const ALL: [Self; 25] = [
        Self::OperatingPoint,
        Self::Transient,
        Self::Ac,
        Self::DcSweep,
        Self::Noise,
        Self::PoleZero,
        Self::Sensitivity,
        Self::MonteCarlo,
        Self::Pss,
        Self::Stb,
        Self::Temperature,
        Self::HarmonicBalance,
        Self::SParameter,
        Self::Pac,
        Self::Pnoise,
        Self::Pxf,
        Self::Pstb,
        Self::TransferFunction,
        Self::Corner,
        Self::Envelope,
        Self::Fourier,
        Self::Reliability,
        Self::Optimization,
        Self::Soa,
        Self::Disto,
    ];

    /// Stable canonical manifest ID.
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::OperatingPoint => "op",
            Self::Transient => "tran",
            Self::Ac => "ac",
            Self::DcSweep => "dc",
            Self::Noise => "noise",
            Self::PoleZero => "pz",
            Self::Sensitivity => "sens",
            Self::MonteCarlo => "mc",
            Self::Pss => "pss",
            Self::Stb => "stb",
            Self::Temperature => "temp",
            Self::HarmonicBalance => "hb",
            Self::SParameter => "sp",
            Self::Pac => "pac",
            Self::Pnoise => "pnoise",
            Self::Pxf => "pxf",
            Self::Pstb => "pstb",
            Self::TransferFunction => "xf",
            Self::Corner => "corner",
            Self::Envelope => "envelope",
            Self::Fourier => "fourier",
            Self::Reliability => "reliability",
            Self::Optimization => "opt",
            Self::Soa => "soa",
            Self::Disto => "disto",
        }
    }

    /// Legacy singleton configuration index. This is migration metadata only.
    pub const fn legacy_index(self) -> usize {
        match self {
            Self::OperatingPoint => 0,
            Self::Transient => 1,
            Self::Ac => 2,
            Self::DcSweep => 3,
            Self::Noise => 4,
            Self::PoleZero => 5,
            Self::Sensitivity => 6,
            Self::MonteCarlo => 7,
            Self::Pss => 8,
            Self::Stb => 9,
            Self::Temperature => 10,
            Self::HarmonicBalance => 11,
            Self::SParameter => 12,
            Self::Pac => 13,
            Self::Pnoise => 14,
            Self::Pxf => 15,
            Self::Pstb => 16,
            Self::TransferFunction => 17,
            Self::Corner => 18,
            Self::Envelope => 19,
            Self::Fourier => 20,
            Self::Reliability => 21,
            Self::Optimization => 22,
            Self::Soa => 23,
            Self::Disto => 24,
        }
    }

    /// User-facing canonical label from the mockup product manifest.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OperatingPoint => "Operating point",
            Self::Transient => "Transient",
            Self::Ac => "AC response",
            Self::DcSweep => "DC sweep",
            Self::Noise => "Noise",
            Self::PoleZero => "Pole-zero",
            Self::Sensitivity => "Sensitivity",
            Self::MonteCarlo => "Monte Carlo",
            Self::Pss => "Periodic steady state",
            Self::Stb => "Loop stability",
            Self::Temperature => "Temperature sweep",
            Self::HarmonicBalance => "Harmonic balance",
            Self::SParameter => "S-parameters",
            Self::Pac => "Periodic AC",
            Self::Pnoise => "Periodic noise",
            Self::Pxf => "Periodic transfer",
            Self::Pstb => "Periodic stability",
            Self::TransferFunction => "Transfer function",
            Self::Corner => "Process corners",
            Self::Envelope => "Envelope",
            Self::Fourier => "Fourier measurements",
            Self::Reliability => "Reliability & aging",
            Self::Optimization => "Optimization",
            Self::Soa => "Safe operating area",
            Self::Disto => "Distortion compatibility",
        }
    }

    /// Exact prerequisite kinds declared by the canonical mockup manifest.
    pub const fn prerequisites(self) -> &'static [Self] {
        const OP: &[AnalysisKind] = &[AnalysisKind::OperatingPoint];
        const TRAN: &[AnalysisKind] = &[AnalysisKind::Transient];
        const AC: &[AnalysisKind] = &[AnalysisKind::Ac];
        const PSS: &[AnalysisKind] = &[AnalysisKind::Pss];
        const HB: &[AnalysisKind] = &[AnalysisKind::HarmonicBalance];
        const NONE: &[AnalysisKind] = &[];

        match self {
            Self::Ac
            | Self::Noise
            | Self::PoleZero
            | Self::Sensitivity
            | Self::Pss
            | Self::Stb
            | Self::HarmonicBalance
            | Self::SParameter
            | Self::TransferFunction => OP,
            Self::Pac | Self::Pnoise | Self::Pxf | Self::Pstb => PSS,
            Self::Envelope => HB,
            Self::Fourier => TRAN,
            Self::Disto => AC,
            Self::OperatingPoint
            | Self::Transient
            | Self::DcSweep
            | Self::MonteCarlo
            | Self::Temperature
            | Self::Corner
            | Self::Reliability
            | Self::Optimization
            | Self::Soa => NONE,
        }
    }

    /// Resolve a legacy singleton index without inventing a fallback kind.
    pub const fn from_legacy_index(index: usize) -> Option<Self> {
        if index < Self::ALL.len() {
            Some(Self::ALL[index])
        } else {
            None
        }
    }

    /// Resolve an exact canonical stable ID.
    pub fn from_stable_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.stable_id() == id)
    }
}

impl std::fmt::Display for AnalysisKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.stable_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_ids_indices_and_serde_are_bijective() {
        let mut ids = std::collections::HashSet::new();
        for (index, kind) in AnalysisKind::ALL.into_iter().enumerate() {
            assert_eq!(kind.legacy_index(), index);
            assert_eq!(AnalysisKind::from_legacy_index(index), Some(kind));
            assert_eq!(AnalysisKind::from_stable_id(kind.stable_id()), Some(kind));
            assert!(ids.insert(kind.stable_id()));
            assert_eq!(
                serde_json::to_string(&kind).expect("kind serializes"),
                format!("\"{}\"", kind.stable_id())
            );
        }
        assert_eq!(ids.len(), AnalysisKind::ALL.len());
        assert_eq!(AnalysisKind::from_legacy_index(25), None);
        assert_eq!(AnalysisKind::from_stable_id("AC"), None);
    }

    #[test]
    fn prerequisite_map_matches_the_canonical_current_kind_contract() {
        let expected = [
            (AnalysisKind::Ac, AnalysisKind::OperatingPoint),
            (AnalysisKind::Noise, AnalysisKind::OperatingPoint),
            (AnalysisKind::PoleZero, AnalysisKind::OperatingPoint),
            (AnalysisKind::Sensitivity, AnalysisKind::OperatingPoint),
            (AnalysisKind::Stb, AnalysisKind::OperatingPoint),
            (AnalysisKind::TransferFunction, AnalysisKind::OperatingPoint),
            (AnalysisKind::Pss, AnalysisKind::OperatingPoint),
            (AnalysisKind::HarmonicBalance, AnalysisKind::OperatingPoint),
            (AnalysisKind::SParameter, AnalysisKind::OperatingPoint),
            (AnalysisKind::Pac, AnalysisKind::Pss),
            (AnalysisKind::Pnoise, AnalysisKind::Pss),
            (AnalysisKind::Pxf, AnalysisKind::Pss),
            (AnalysisKind::Pstb, AnalysisKind::Pss),
            (AnalysisKind::Envelope, AnalysisKind::HarmonicBalance),
            (AnalysisKind::Fourier, AnalysisKind::Transient),
            (AnalysisKind::Disto, AnalysisKind::Ac),
        ];
        for (kind, prerequisite) in expected {
            assert_eq!(kind.prerequisites(), &[prerequisite]);
        }
        assert_eq!(
            AnalysisKind::ALL
                .into_iter()
                .filter(|kind| !kind.prerequisites().is_empty())
                .count(),
            expected.len()
        );
    }
}
