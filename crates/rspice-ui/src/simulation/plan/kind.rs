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
    #[serde(rename = "qpss")]
    Qpss,
    #[serde(rename = "hbsp")]
    Hbsp,
    #[serde(rename = "hbnoise")]
    Hbnoise,
    #[serde(rename = "psp")]
    Psp,
    #[serde(rename = "qpac")]
    Qpac,
    #[serde(rename = "qpnoise")]
    Qpnoise,
    #[serde(rename = "qpxf")]
    Qpxf,
    #[serde(rename = "tnoise")]
    TransientNoise,
    #[serde(rename = "dcmatch")]
    DcMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisAvailability {
    Production,
    Preview,
    Compatibility,
}

impl AnalysisAvailability {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Production => "Production",
            Self::Preview => "Preview · non-sign-off",
            Self::Compatibility => "Compatibility · non-sign-off",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnalysisCategory {
    pub id: &'static str,
    pub label: &'static str,
    pub glyph: &'static str,
    pub detail: &'static str,
}

impl AnalysisKind {
    /// All plan-recognized kinds in the stable historical-index order.
    pub const ALL: [Self; 34] = [
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
        Self::Qpss,
        Self::Hbsp,
        Self::Hbnoise,
        Self::Psp,
        Self::Qpac,
        Self::Qpnoise,
        Self::Qpxf,
        Self::TransientNoise,
        Self::DcMismatch,
    ];

    /// Canonical display order from `product-manifest.js`. This is separate
    /// from [`Self::ALL`] so the historical singleton indices remain stable.
    pub const MANIFEST_ORDER: [Self; 34] = [
        Self::OperatingPoint,
        Self::Transient,
        Self::Ac,
        Self::DcSweep,
        Self::Noise,
        Self::PoleZero,
        Self::Sensitivity,
        Self::Stb,
        Self::TransferFunction,
        Self::Pss,
        Self::Qpss,
        Self::HarmonicBalance,
        Self::SParameter,
        Self::Hbsp,
        Self::Hbnoise,
        Self::Envelope,
        Self::Pac,
        Self::Pnoise,
        Self::Pxf,
        Self::Pstb,
        Self::Psp,
        Self::Qpac,
        Self::Qpnoise,
        Self::Qpxf,
        Self::TransientNoise,
        Self::MonteCarlo,
        Self::Temperature,
        Self::Corner,
        Self::DcMismatch,
        Self::Fourier,
        Self::Disto,
        Self::Reliability,
        Self::Soa,
        Self::Optimization,
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
            Self::Qpss => "qpss",
            Self::Hbsp => "hbsp",
            Self::Hbnoise => "hbnoise",
            Self::Psp => "psp",
            Self::Qpac => "qpac",
            Self::Qpnoise => "qpnoise",
            Self::Qpxf => "qpxf",
            Self::TransientNoise => "tnoise",
            Self::DcMismatch => "dcmatch",
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
            Self::Qpss => 25,
            Self::Hbsp => 26,
            Self::Hbnoise => 27,
            Self::Psp => 28,
            Self::Qpac => 29,
            Self::Qpnoise => 30,
            Self::Qpxf => 31,
            Self::TransientNoise => 32,
            Self::DcMismatch => 33,
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
            Self::Qpss => "Quasi-periodic steady state",
            Self::Hbsp => "Large-signal S-parameters",
            Self::Hbnoise => "Harmonic-balance noise",
            Self::Psp => "Periodic S-parameters",
            Self::Qpac => "Quasi-periodic AC",
            Self::Qpnoise => "Quasi-periodic noise",
            Self::Qpxf => "Quasi-periodic transfer",
            Self::TransientNoise => "Transient noise",
            Self::DcMismatch => "DC mismatch contribution",
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::OperatingPoint => "OP",
            Self::Transient => "TRAN",
            Self::Ac => "AC",
            Self::DcSweep => "DC",
            Self::Noise => "NOISE",
            Self::PoleZero => "PZ",
            Self::Sensitivity => "SENS",
            Self::Stb => "STB",
            Self::TransferFunction => "XF",
            Self::Pss => "PSS",
            Self::Qpss => "QPSS",
            Self::HarmonicBalance => "HB",
            Self::SParameter => "SP",
            Self::Hbsp => "HBSP",
            Self::Hbnoise => "HBNOISE",
            Self::Envelope => "ENV",
            Self::Pac => "PAC",
            Self::Pnoise => "PNOISE",
            Self::Pxf => "PXF",
            Self::Pstb => "PSTB",
            Self::Psp => "PSP",
            Self::Qpac => "QPAC",
            Self::Qpnoise => "QPNOISE",
            Self::Qpxf => "QPXF",
            Self::TransientNoise => "TNOISE",
            Self::MonteCarlo => "MC",
            Self::Temperature => "TEMP",
            Self::Corner => "CORNER",
            Self::DcMismatch => "DCMATCH",
            Self::Fourier => "FOUR",
            Self::Disto => "DISTO",
            Self::Reliability => "REL",
            Self::Soa => "SOA",
            Self::Optimization => "OPT",
        }
    }

    pub const fn glyph(self) -> &'static str {
        match self {
            Self::Transient => "TR",
            Self::Noise => "N",
            Self::Sensitivity => "SE",
            Self::Qpss => "QPSS",
            Self::Hbsp => "HSP",
            Self::Hbnoise => "HN",
            Self::Envelope => "ENV",
            Self::Pnoise => "PN",
            Self::Pstb => "PST",
            Self::Qpnoise => "QPN",
            Self::TransientNoise => "TN",
            Self::Temperature => "T",
            Self::Corner => "CO",
            Self::DcMismatch => "DM",
            Self::Fourier => "FO",
            Self::Disto => "DI",
            Self::Reliability => "REL",
            Self::Soa => "SOA",
            Self::Optimization => "OPT",
            other => other.code(),
        }
    }

    pub const fn detail(self) -> &'static str {
        match self {
            Self::OperatingPoint => "Bias solution, annotations, and device operating points.",
            Self::Transient => {
                "Adaptive time-domain integration, events, breakpoints, and checkpoints."
            }
            Self::Ac => "Linearized complex frequency response and family sweeps.",
            Self::DcSweep => {
                "Single, nested, list, and parameterized bias sweeps with continuation."
            }
            Self::Noise => {
                "Input/output-referred noise, correlation, integration, and contributor ranking."
            }
            Self::PoleZero => "Balanced pole, zero, residual, and stability extraction.",
            Self::Sensitivity => "Direct and adjoint parameter influence with normalized ranking.",
            Self::Stb => "Loop gain, gain margin, phase margin, and instability detection.",
            Self::TransferFunction => "DC transfer gain plus input and output resistance.",
            Self::Pss => "Shooting-Newton driven or autonomous periodic operating point.",
            Self::Qpss => {
                "Incommensurate multi-tone operating state with an explicit spectral lattice."
            }
            Self::HarmonicBalance => "Single- and multi-tone frequency-domain nonlinear solution.",
            Self::SParameter => {
                "Multiport network analysis, conversions, de-embedding, and Touchstone export."
            }
            Self::Hbsp => {
                "Multiport small-signal network response linearized around an immutable harmonic-balance state."
            }
            Self::Hbnoise => {
                "Large-signal noise, folding, correlation, and noise-figure metrics around harmonic balance."
            }
            Self::Envelope => {
                "Slow modulation envelope around a harmonic-balance carrier solution."
            }
            Self::Pac => "Frequency-translated small-signal response around a periodic solution.",
            Self::Pnoise => "Cyclostationary noise and folding around a periodic operating point.",
            Self::Pxf => "Frequency-conversion paths and sideband transfer around PSS.",
            Self::Pstb => "Monodromy, Floquet, and sideband stability around a periodic solution.",
            Self::Psp => {
                "Frequency-translated multiport network response around an immutable periodic steady state."
            }
            Self::Qpac => "Small-signal conversion matrix around an immutable QPSS state.",
            Self::Qpnoise => "Noise folding and correlation across a multi-tone spectral lattice.",
            Self::Qpxf => "Translated transfer paths indexed by input and output lattice products.",
            Self::TransientNoise => "Reproducible stochastic time-domain device-noise simulation.",
            Self::MonteCarlo => {
                "Process and mismatch variation with explicit distributions, correlations, and seeds."
            }
            Self::Temperature => {
                "List or range temperature dimension with warm-start and failure policy."
            }
            Self::Corner => "Explicit model sections crossed with supply and temperature axes.",
            Self::DcMismatch => {
                "Linearized local mismatch variance, contribution ranking, and sigma estimates around a declared operating point."
            }
            Self::Fourier => {
                "Harmonics, THD, SINAD, SFDR, and ENOB from compatible time-domain data."
            }
            Self::Disto => {
                "Legacy AC distortion compatibility with explicit THD/IMD post-processing."
            }
            Self::Reliability => {
                "Mission-profile stress, degradation models, and lifetime margins."
            }
            Self::Soa => "Electrical terminal, current, power, and temperature-aware stress rules.",
            Self::Optimization => {
                "Bounded variables, objectives, hard constraints, algorithms, and candidate history."
            }
        }
    }

    pub const fn category(self) -> AnalysisCategory {
        match self {
            Self::OperatingPoint | Self::Transient | Self::Ac | Self::DcSweep | Self::Noise => {
                AnalysisCategory {
                    id: "core",
                    label: "Core analyses",
                    glyph: "DC",
                    detail: "Bias, time, AC, DC and noise",
                }
            }
            Self::PoleZero | Self::Sensitivity | Self::Stb | Self::TransferFunction => {
                AnalysisCategory {
                    id: "transfer",
                    label: "Transfer & linearization",
                    glyph: "ƒ",
                    detail: "PZ, sensitivity, STB and XF",
                }
            }
            Self::Pss
            | Self::Qpss
            | Self::HarmonicBalance
            | Self::SParameter
            | Self::Hbsp
            | Self::Hbnoise
            | Self::Envelope => AnalysisCategory {
                id: "rf",
                label: "RF & periodic solvers",
                glyph: "RF",
                detail: "PSS, QPSS, HB, SP, HB network analyses and envelope",
            },
            Self::Pac | Self::Pnoise | Self::Pxf | Self::Pstb | Self::Psp => AnalysisCategory {
                id: "periodic",
                label: "Periodic small-signal",
                glyph: "PX",
                detail: "PAC, PNoise, PXF, PSTB and periodic S-parameters",
            },
            Self::Qpac | Self::Qpnoise | Self::Qpxf => AnalysisCategory {
                id: "quasi-periodic",
                label: "Quasi-periodic small-signal",
                glyph: "QP",
                detail: "QPAC, QPNoise and QPXF",
            },
            Self::TransientNoise => AnalysisCategory {
                id: "stochastic",
                label: "Time-domain stochastic",
                glyph: "η",
                detail: "Transient device-noise simulation",
            },
            Self::MonteCarlo | Self::Temperature | Self::Corner | Self::DcMismatch => {
                AnalysisCategory {
                    id: "sweep",
                    label: "Sweeps & variation",
                    glyph: "σ",
                    detail: "Monte Carlo, mismatch contribution, corners and temperature",
                }
            }
            Self::Fourier | Self::Disto => AnalysisCategory {
                id: "measurement",
                label: "Measurements",
                glyph: "Σ",
                detail: "Fourier and compatibility post-processing",
            },
            Self::Reliability | Self::Soa => AnalysisCategory {
                id: "verification",
                label: "Verification checks",
                glyph: "✓",
                detail: "Reliability, aging and electrical SOA",
            },
            Self::Optimization => AnalysisCategory {
                id: "optimization",
                label: "Optimization workflow",
                glyph: "◎",
                detail: "Variables, goals and constraints",
            },
        }
    }

    pub const fn availability(self) -> AnalysisAvailability {
        match self {
            Self::Disto => AnalysisAvailability::Compatibility,
            Self::Qpss
            | Self::Hbsp
            | Self::Hbnoise
            | Self::Envelope
            | Self::Pac
            | Self::Pnoise
            | Self::Pxf
            | Self::Pstb
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

    /// Current engine limitation. The identity and configuration remain
    /// persistent, but execution must reject these kinds until a real solver
    /// is integrated.
    pub const fn execution_blocker(self) -> Option<&'static str> {
        match self {
            Self::Qpss => {
                Some("the QPSS spectral-lattice solver is not available in this engine build")
            }
            Self::Hbsp => Some(
                "large-signal S-parameter linearization around HB is not available in this engine build",
            ),
            Self::Hbnoise => Some(
                "harmonic-balance noise and correlation are not available in this engine build",
            ),
            Self::Psp => {
                Some("periodic S-parameter conversion is not available in this engine build")
            }
            Self::Qpac => {
                Some("QPAC conversion-matrix execution is not available in this engine build")
            }
            Self::Qpnoise => {
                Some("quasi-periodic noise execution is not available in this engine build")
            }
            Self::Qpxf => Some(
                "quasi-periodic translated-transfer execution is not available in this engine build",
            ),
            Self::TransientNoise => Some(
                "stochastic transient device-noise execution is not available in this engine build",
            ),
            Self::DcMismatch => {
                Some("DC mismatch contribution extraction is not available in this engine build")
            }
            _ => None,
        }
    }

    /// Exact prerequisite kinds declared by the canonical mockup manifest.
    pub const fn prerequisites(self) -> &'static [Self] {
        const OP: &[AnalysisKind] = &[AnalysisKind::OperatingPoint];
        const TRAN: &[AnalysisKind] = &[AnalysisKind::Transient];
        const AC: &[AnalysisKind] = &[AnalysisKind::Ac];
        const PSS: &[AnalysisKind] = &[AnalysisKind::Pss];
        const HB: &[AnalysisKind] = &[AnalysisKind::HarmonicBalance];
        const QPSS: &[AnalysisKind] = &[AnalysisKind::Qpss];
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
            Self::Qpss => OP,
            Self::Pac | Self::Pnoise | Self::Pxf | Self::Pstb | Self::Psp => PSS,
            Self::Envelope | Self::Hbsp | Self::Hbnoise => HB,
            Self::Qpac | Self::Qpnoise | Self::Qpxf => QPSS,
            Self::Fourier | Self::TransientNoise => TRAN,
            Self::DcMismatch => OP,
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
        assert_eq!(AnalysisKind::from_legacy_index(34), None);
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
            (AnalysisKind::Qpss, AnalysisKind::OperatingPoint),
            (AnalysisKind::Hbsp, AnalysisKind::HarmonicBalance),
            (AnalysisKind::Hbnoise, AnalysisKind::HarmonicBalance),
            (AnalysisKind::Psp, AnalysisKind::Pss),
            (AnalysisKind::Qpac, AnalysisKind::Qpss),
            (AnalysisKind::Qpnoise, AnalysisKind::Qpss),
            (AnalysisKind::Qpxf, AnalysisKind::Qpss),
            (AnalysisKind::TransientNoise, AnalysisKind::Transient),
            (AnalysisKind::DcMismatch, AnalysisKind::OperatingPoint),
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

    #[test]
    fn canonical_manifest_order_and_metadata_are_complete() {
        let manifest = AnalysisKind::MANIFEST_ORDER;
        assert_eq!(manifest.len(), 34);
        assert_eq!(manifest[10], AnalysisKind::Qpss);
        assert_eq!(manifest[13], AnalysisKind::Hbsp);
        assert_eq!(manifest[24], AnalysisKind::TransientNoise);
        assert_eq!(manifest[28], AnalysisKind::DcMismatch);
        let identities = manifest
            .into_iter()
            .map(AnalysisKind::stable_id)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(identities.len(), AnalysisKind::ALL.len());
        for kind in manifest {
            assert!(!kind.code().is_empty());
            assert!(!kind.glyph().is_empty());
            assert!(!kind.label().is_empty());
            assert!(!kind.detail().is_empty());
            assert!(!kind.category().label.is_empty());
        }
    }
}
