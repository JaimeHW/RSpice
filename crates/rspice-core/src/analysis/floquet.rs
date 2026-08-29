//! Shared evidence and stability contracts for Floquet spectra.

use crate::Value;
pub use crate::numerics::FloquetSpectrumCertificate;
use num_complex::Complex64;

/// Numerical half-width used around the physical unit-circle boundary.
pub const FLOQUET_UNIT_CIRCLE_BAND: Value = 1.0e-6;

/// Provenance for a retained Floquet multiplier vector.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "veriloga", serde(rename_all = "snake_case"))]
pub enum FloquetSpectrumEvidence {
    /// Stability post-processing was not performed.
    NotComputed,
    /// The periodic map has no dynamic state and therefore no multipliers.
    NoDynamicModes,
    /// Every multiplier belongs to a complete, strictly qualified spectrum.
    Qualified {
        /// Certificate covering the complete associated multiplier vector.
        certificate: FloquetSpectrumCertificate,
    },
    /// Legacy data retained multipliers without a numerical certificate.
    LegacyUnknown,
}

impl Default for FloquetSpectrumEvidence {
    /// Missing durable evidence is legacy-unknown, never newly qualified.
    fn default() -> Self {
        Self::LegacyUnknown
    }
}

impl FloquetSpectrumEvidence {
    /// Construct qualified evidence from a valid strict certificate.
    pub fn qualified(certificate: FloquetSpectrumCertificate) -> Option<Self> {
        certificate
            .is_valid()
            .then_some(Self::Qualified { certificate })
    }

    /// Return the numerical certificate, when present.
    pub fn certificate(&self) -> Option<&FloquetSpectrumCertificate> {
        match self {
            Self::Qualified { certificate } => Some(certificate),
            Self::NotComputed | Self::NoDynamicModes | Self::LegacyUnknown => None,
        }
    }

    /// Check that evidence, cardinality, and retained values agree.
    pub fn is_consistent_with(&self, multipliers: &[Complex64]) -> bool {
        let all_finite = multipliers
            .iter()
            .all(|value| value.re.is_finite() && value.im.is_finite());
        if !all_finite {
            return false;
        }
        match self {
            Self::NotComputed | Self::NoDynamicModes => multipliers.is_empty(),
            Self::Qualified { certificate } => {
                certificate.is_valid()
                    && certificate.problem_order == multipliers.len()
                    && !multipliers.is_empty()
            }
            Self::LegacyUnknown => true,
        }
    }
}

/// Periodic-orbit policy used when interpreting a unity multiplier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "veriloga", serde(rename_all = "snake_case"))]
pub enum FloquetOrbitKind {
    /// Externally driven orbit; no multiplier is automatically exempted.
    #[default]
    Driven,
    /// Autonomous orbit; at most one explicitly selected phase mode may be
    /// exempted.
    Autonomous,
}

/// Evidence-aware stability verdict shared by PSS and PSTB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "veriloga", serde(rename_all = "snake_case"))]
pub enum FloquetStabilityVerdict {
    /// Every applicable multiplier is strictly inside the inner boundary.
    Stable,
    /// At least one applicable multiplier is outside the outer boundary.
    Unstable,
    /// No multiplier is outside, but at least one lies in the uncertainty band.
    Marginal,
    /// Evidence or orbit-policy data is absent or internally inconsistent.
    Indeterminate,
}

/// Select the closest autonomous phase-mode candidate within the canonical
/// unit-circle qualification band.
pub fn select_autonomous_phase_mode(multipliers: &[Complex64]) -> Option<usize> {
    multipliers
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            let distance = (*value - Complex64::new(1.0, 0.0)).norm();
            (distance.is_finite() && distance <= FLOQUET_UNIT_CIRCLE_BAND)
                .then_some((index, distance))
        })
        .min_by(|left, right| left.1.total_cmp(&right.1))
        .map(|(index, _)| index)
}

/// Derive a stability verdict from a complete qualified Floquet spectrum.
pub fn classify_floquet_stability(
    multipliers: &[Complex64],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKind,
    trivial_multiplier_index: Option<usize>,
    band: Value,
) -> FloquetStabilityVerdict {
    if !band.is_finite() || band < 0.0 || !evidence.is_consistent_with(multipliers) {
        return FloquetStabilityVerdict::Indeterminate;
    }

    if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes) {
        return if orbit_kind == FloquetOrbitKind::Driven
            && trivial_multiplier_index.is_none()
            && multipliers.is_empty()
        {
            FloquetStabilityVerdict::Stable
        } else {
            FloquetStabilityVerdict::Indeterminate
        };
    }
    if !matches!(evidence, FloquetSpectrumEvidence::Qualified { .. }) {
        return FloquetStabilityVerdict::Indeterminate;
    }

    let phase_mode = match orbit_kind {
        FloquetOrbitKind::Driven => {
            if trivial_multiplier_index.is_some() {
                return FloquetStabilityVerdict::Indeterminate;
            }
            None
        }
        FloquetOrbitKind::Autonomous => {
            let Some(index) = trivial_multiplier_index else {
                return FloquetStabilityVerdict::Indeterminate;
            };
            if index >= multipliers.len()
                || (multipliers[index] - Complex64::new(1.0, 0.0)).norm() > FLOQUET_UNIT_CIRCLE_BAND
            {
                return FloquetStabilityVerdict::Indeterminate;
            }
            Some(index)
        }
    };

    let outer = 1.0 + band;
    let inner = (1.0 - band).max(0.0);
    let mut marginal = false;
    for (index, multiplier) in multipliers.iter().enumerate() {
        if phase_mode == Some(index) {
            continue;
        }
        let magnitude = multiplier.norm();
        if magnitude > outer {
            return FloquetStabilityVerdict::Unstable;
        }
        if magnitude >= inner {
            marginal = true;
        }
    }

    if marginal {
        FloquetStabilityVerdict::Marginal
    } else {
        FloquetStabilityVerdict::Stable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn certificate(order: usize) -> FloquetSpectrumCertificate {
        FloquetSpectrumCertificate::new(
            order,
            0.0,
            FloquetSpectrumCertificate::canonical_qualification_tolerance(order),
        )
        .unwrap()
    }

    #[test]
    fn certificate_rejects_inflated_or_non_strict_data() {
        let tolerance = FloquetSpectrumCertificate::canonical_qualification_tolerance(2);
        assert!(FloquetSpectrumCertificate::new(2, tolerance, tolerance).is_some());
        assert!(FloquetSpectrumCertificate::new(0, 0.0, tolerance).is_none());
        assert!(FloquetSpectrumCertificate::new(2, tolerance * 2.0, tolerance).is_none());
        assert!(FloquetSpectrumCertificate::new(2, 0.0, tolerance * 2.0).is_none());
    }

    #[test]
    fn stability_requires_qualified_complete_evidence() {
        let roots = [Complex64::new(0.5, 0.0)];
        assert_eq!(
            classify_floquet_stability(
                &roots,
                &FloquetSpectrumEvidence::LegacyUnknown,
                FloquetOrbitKind::Driven,
                None,
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Indeterminate
        );
        assert_eq!(
            classify_floquet_stability(
                &roots,
                &FloquetSpectrumEvidence::Qualified {
                    certificate: certificate(2),
                },
                FloquetOrbitKind::Driven,
                None,
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Indeterminate
        );
    }

    #[test]
    fn authenticated_driven_zero_order_spectrum_is_stable() {
        assert_eq!(
            classify_floquet_stability(
                &[],
                &FloquetSpectrumEvidence::NoDynamicModes,
                FloquetOrbitKind::Driven,
                None,
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Stable
        );
        assert_eq!(
            classify_floquet_stability(
                &[],
                &FloquetSpectrumEvidence::NoDynamicModes,
                FloquetOrbitKind::Autonomous,
                None,
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Indeterminate
        );
    }

    #[test]
    fn autonomous_policy_exempts_exactly_one_explicit_phase_mode() {
        let roots = [Complex64::new(1.0, 0.0), Complex64::new(0.5, 0.0)];
        let evidence = FloquetSpectrumEvidence::Qualified {
            certificate: certificate(2),
        };
        assert_eq!(
            classify_floquet_stability(
                &roots,
                &evidence,
                FloquetOrbitKind::Autonomous,
                Some(0),
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Stable
        );
        assert_eq!(
            classify_floquet_stability(
                &roots,
                &evidence,
                FloquetOrbitKind::Autonomous,
                None,
                FLOQUET_UNIT_CIRCLE_BAND,
            ),
            FloquetStabilityVerdict::Indeterminate
        );
    }

    #[test]
    fn wide_classification_band_cannot_fabricate_an_autonomous_phase_mode() {
        let roots = [Complex64::new(1.5, 0.0), Complex64::new(0.5, 0.0)];
        let evidence = FloquetSpectrumEvidence::Qualified {
            certificate: certificate(2),
        };
        assert_eq!(
            classify_floquet_stability(
                &roots,
                &evidence,
                FloquetOrbitKind::Autonomous,
                Some(0),
                1.0,
            ),
            FloquetStabilityVerdict::Indeterminate
        );
    }
}
