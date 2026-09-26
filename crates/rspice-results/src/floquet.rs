//! Retained Floquet qualification, orbit provenance, and exact PSS/PSTB validation.

use crate::simulation_values::ComplexResultValue;
use crate::validation::{same_retained_float, validate_complex_values};

/// Strict residual certificate for one complete retained Floquet spectrum.
///
/// This result-owned representation is the durable serde contract. Validation is
/// delegated to the core constructor so a project cannot authenticate an
/// inflated qualification tolerance after the numerical contract changes.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FloquetSpectrumCertificateEvidence {
    pub problem_order: u64,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

impl FloquetSpectrumCertificateEvidence {
    #[must_use]
    pub fn canonical_qualification_tolerance(problem_order: u64) -> Option<f64> {
        let problem_order = usize::try_from(problem_order).ok()?;
        Some(
            rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(
                problem_order,
            ),
        )
    }

    fn as_core(self) -> Option<rspice_core::analysis::FloquetSpectrumCertificate> {
        rspice_core::analysis::FloquetSpectrumCertificate::new(
            usize::try_from(self.problem_order).ok()?,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    #[must_use]
    pub fn is_strictly_qualified(self) -> bool {
        self.as_core().is_some()
    }
}

/// Provenance for a durable Floquet multiplier vector.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum FloquetSpectrumEvidence {
    /// Stability post-processing was not performed.
    NotComputed,
    /// The periodic map is an authenticated zero-order driven map.
    NoDynamicModes,
    /// Every multiplier belongs to a complete strict eigenspectrum.
    Qualified {
        certificate: FloquetSpectrumCertificateEvidence,
    },
    /// Truthful state for a project written before Floquet certificates were
    /// retained. It never proves a stability classification.
    #[default]
    LegacyUnknown,
}

impl FloquetSpectrumEvidence {
    #[must_use]
    pub const fn certificate(&self) -> Option<FloquetSpectrumCertificateEvidence> {
        match self {
            Self::Qualified { certificate } => Some(*certificate),
            Self::NotComputed | Self::NoDynamicModes | Self::LegacyUnknown => None,
        }
    }

    fn is_consistent_with_count(&self, multiplier_count: usize) -> bool {
        match self {
            Self::NotComputed | Self::NoDynamicModes | Self::LegacyUnknown => multiplier_count == 0,
            Self::Qualified { certificate } => {
                multiplier_count > 0
                    && certificate.is_strictly_qualified()
                    && u64::try_from(multiplier_count).ok() == Some(certificate.problem_order)
            }
        }
    }

    fn as_core(&self) -> Option<rspice_core::analysis::FloquetSpectrumEvidence> {
        match self {
            Self::NotComputed => Some(rspice_core::analysis::FloquetSpectrumEvidence::NotComputed),
            Self::NoDynamicModes => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes)
            }
            Self::Qualified { certificate } => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::Qualified {
                    certificate: certificate.as_core()?,
                })
            }
            Self::LegacyUnknown => {
                Some(rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown)
            }
        }
    }
}

/// Orbit policy used to interpret a retained Floquet spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloquetOrbitKindEvidence {
    Driven,
    Autonomous,
    /// The producing project did not retain an orbit policy.
    #[default]
    LegacyUnknown,
}

/// Evidence-aware periodic stability verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FloquetStabilityVerdictEvidence {
    Stable,
    Unstable,
    Marginal,
    Indeterminate,
}

/// Rich PSTB classification refining the shared four-state verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PstbStabilityClassificationEvidence {
    Stable,
    UnstableReal,
    UnstableComplex,
    PeriodDoubling,
    NeimarkSacker,
    SaddleNode,
    Marginal,
    Indeterminate,
}

/// One exact multiplier in the complete PSS Floquet vector.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PssFloquetMultiplierEvidence {
    pub multiplier: ComplexResultValue,
}

/// One complete PSTB mode. The containing vector is authoritative and is
/// never truncated by presentation limits.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PstbFloquetModeEvidence {
    pub multiplier: ComplexResultValue,
    pub exponent: ComplexResultValue,
    pub probe_participation: f64,
    pub is_unstable: bool,
    pub is_trivial: bool,
    pub subharmonic_order: Option<u64>,
}

/// Borrowed inputs for validating retained PSS Floquet evidence.
pub struct PssFloquetRef<'a> {
    pub period_s: Option<f64>,
    pub fundamental_frequency_hz: Option<f64>,
    pub iterations: Option<u64>,
    pub residual_norm: Option<f64>,
    pub multipliers: &'a [PssFloquetMultiplierEvidence],
    pub evidence: &'a FloquetSpectrumEvidence,
    pub orbit_kind: FloquetOrbitKindEvidence,
    pub trivial_multiplier_index: Option<u64>,
    pub verdict: FloquetStabilityVerdictEvidence,
}

impl PssFloquetRef<'_> {
    pub fn validate(self) -> Result<(), String> {
        let Self {
            period_s,
            fundamental_frequency_hz,
            iterations: _iterations,
            residual_norm,
            multipliers,
            evidence,
            orbit_kind,
            trivial_multiplier_index,
            verdict,
        } = self;
        if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
        {
            let exact_legacy_marker = period_s.is_none()
                && fundamental_frequency_hz.is_none()
                && _iterations.is_none()
                && residual_norm.is_none()
                && multipliers.is_empty()
                && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
                && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
                && trivial_multiplier_index.is_none()
                && verdict == FloquetStabilityVerdictEvidence::Indeterminate;
            return exact_legacy_marker.then_some(()).ok_or_else(|| {
                "legacy PSS Floquet evidence is not an exact migration marker".to_owned()
            });
        }

        let (Some(period_s), Some(frequency_hz), Some(_), Some(residual_norm)) = (
            period_s,
            fundamental_frequency_hz,
            _iterations,
            residual_norm,
        ) else {
            return Err("current PSS Floquet payload is missing global metrics".to_owned());
        };
        if !period_s.is_finite()
            || period_s <= 0.0
            || !frequency_hz.is_finite()
            || frequency_hz <= 0.0
            || !same_retained_float(frequency_hz, 1.0 / period_s)
            || !residual_norm.is_finite()
            || residual_norm < 0.0
        {
            return Err("PSS Floquet period, frequency, or residual is invalid".to_owned());
        }

        let values = multipliers
            .iter()
            .map(|mode| mode.multiplier)
            .collect::<Vec<_>>();
        validate_complex_values(&values, "PSS Floquet multiplier")?;
        if !evidence.is_consistent_with_count(values.len()) {
            return Err(
                "PSS Floquet certificate does not cover the complete multiplier vector".to_owned(),
            );
        }
        if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
            && orbit_kind != FloquetOrbitKindEvidence::Driven
        {
            return Err(
                "a zero-order PSS Floquet spectrum must use the driven orbit policy".to_owned(),
            );
        }

        let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
        if trivial_multiplier_index != expected_trivial {
            return Err(
                "PSS autonomous phase-mode index is inconsistent with the spectrum".to_owned(),
            );
        }
        let expected_verdict = derive_floquet_verdict(
            &values,
            evidence,
            orbit_kind,
            trivial_multiplier_index,
            rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
        )?;
        if verdict != expected_verdict {
            return Err(
                "PSS stability verdict is inconsistent with its Floquet evidence".to_owned(),
            );
        }
        Ok(())
    }
}

/// Borrowed inputs for validating complete PSTB spectrum evidence.
pub struct PstbRef<'a> {
    pub period_s: Option<f64>,
    pub fundamental_frequency_hz: Option<f64>,
    pub stability_threshold: Option<f64>,
    pub probe_instance: Option<&'a str>,
    pub detect_subharmonics: Option<bool>,
    pub modes: &'a [PstbFloquetModeEvidence],
    pub evidence: &'a FloquetSpectrumEvidence,
    pub orbit_kind: FloquetOrbitKindEvidence,
    pub trivial_multiplier_index: Option<u64>,
    pub verdict: FloquetStabilityVerdictEvidence,
    pub classification: PstbStabilityClassificationEvidence,
    pub min_stability_margin_db: Option<f64>,
    pub max_multiplier_magnitude: Option<f64>,
    pub num_unstable: Option<u64>,
    pub subharmonics: &'a [u64],
    pub converged: Option<bool>,
    pub iterations: Option<u64>,
}

impl PstbRef<'_> {
    pub fn validate(self) -> Result<(), String> {
        let Self {
            period_s,
            fundamental_frequency_hz,
            stability_threshold,
            probe_instance,
            detect_subharmonics,
            modes,
            evidence,
            orbit_kind,
            trivial_multiplier_index,
            verdict,
            classification,
            min_stability_margin_db,
            max_multiplier_magnitude,
            num_unstable,
            subharmonics,
            converged,
            iterations,
        } = self;
        if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
        {
            let exact_legacy_marker = period_s.is_none()
                && fundamental_frequency_hz.is_none()
                && stability_threshold.is_none()
                && probe_instance.is_none()
                && detect_subharmonics.is_none()
                && modes.is_empty()
                && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
                && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
                && trivial_multiplier_index.is_none()
                && verdict == FloquetStabilityVerdictEvidence::Indeterminate
                && classification == PstbStabilityClassificationEvidence::Indeterminate
                && min_stability_margin_db.is_none()
                && max_multiplier_magnitude.is_none()
                && num_unstable.is_none()
                && subharmonics.is_empty()
                && converged.is_none()
                && iterations.is_none();
            return exact_legacy_marker
                .then_some(())
                .ok_or_else(|| "legacy PSTB evidence is not an exact migration marker".to_owned());
        }

        let (
            Some(period_s),
            Some(frequency_hz),
            Some(stability_threshold),
            Some(probe_instance),
            Some(detect_subharmonics),
            Some(max_multiplier_magnitude),
            Some(num_unstable),
            Some(true),
            Some(_),
        ) = (
            period_s,
            fundamental_frequency_hz,
            stability_threshold,
            probe_instance,
            detect_subharmonics,
            max_multiplier_magnitude,
            num_unstable,
            converged,
            iterations,
        )
        else {
            return Err(
                "current PSTB payload is missing provenance, convergence, or global metrics"
                    .to_owned(),
            );
        };
        if !period_s.is_finite()
            || period_s <= 0.0
            || !frequency_hz.is_finite()
            || frequency_hz <= 0.0
            || !same_retained_float(frequency_hz, 1.0 / period_s)
            || !stability_threshold.is_finite()
            || stability_threshold < 1.0
            || probe_instance.is_empty()
            || probe_instance.trim() != probe_instance
            || probe_instance
                .chars()
                .any(|character| character.is_control() || character.is_whitespace())
        {
            return Err(
                "PSTB period, frequency, stability boundary, or probe identity is invalid"
                    .to_owned(),
            );
        }
        if !matches!(
            evidence,
            FloquetSpectrumEvidence::NoDynamicModes | FloquetSpectrumEvidence::Qualified { .. }
        ) || !evidence.is_consistent_with_count(modes.len())
        {
            return Err("PSTB requires a complete current Floquet spectrum".to_owned());
        }
        if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
            && orbit_kind != FloquetOrbitKindEvidence::Driven
        {
            return Err("a zero-order PSTB spectrum must use the driven orbit policy".to_owned());
        }

        let values = modes.iter().map(|mode| mode.multiplier).collect::<Vec<_>>();
        validate_complex_values(&values, "PSTB Floquet multiplier")?;
        if !pstb_modes_are_canonically_sorted(modes) {
            return Err("PSTB Floquet modes are not in canonical sorted order".to_owned());
        }
        let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
        if trivial_multiplier_index != expected_trivial {
            return Err(
                "PSTB autonomous phase-mode index is inconsistent with the spectrum".to_owned(),
            );
        }
        let trivial_index = trivial_multiplier_index
            .map(usize::try_from)
            .transpose()
            .map_err(|_| "PSTB phase-mode index does not fit this platform".to_owned())?;

        let mut expected_subharmonics = Vec::new();
        let mut expected_unstable_count = 0_u64;
        let mut expected_min_margin: Option<f64> = None;
        for (index, mode) in modes.iter().enumerate() {
            let value = num_complex::Complex64::from(mode.multiplier);
            let magnitude = value.norm();
            if !magnitude.is_finite()
                || magnitude <= 0.0
                || !mode.exponent.real.is_finite()
                || !mode.exponent.imaginary.is_finite()
                || !mode.probe_participation.is_finite()
                || !(0.0..=1.0).contains(&mode.probe_participation)
            {
                return Err(format!(
                    "PSTB Floquet mode {index} contains invalid numerical data"
                ));
            }
            let expected_exponent = value.ln() / period_s;
            if !same_retained_float(mode.exponent.real, expected_exponent.re)
                || !same_retained_float(mode.exponent.imaginary, expected_exponent.im)
            {
                return Err(format!(
                    "PSTB Floquet mode {index} has an inconsistent exponent"
                ));
            }
            let expected_trivial_flag = trivial_index == Some(index);
            let expected_unstable = !expected_trivial_flag && magnitude > stability_threshold;
            if mode.is_trivial != expected_trivial_flag || mode.is_unstable != expected_unstable {
                return Err(format!(
                    "PSTB Floquet mode {index} has inconsistent stability flags"
                ));
            }
            expected_unstable_count += u64::from(expected_unstable);

            if !expected_trivial_flag {
                let margin = -20.0 * magnitude.log10();
                if !margin.is_finite() {
                    return Err(format!("PSTB Floquet mode {index} has a non-finite margin"));
                }
                expected_min_margin = Some(match expected_min_margin {
                    Some(current) if current.total_cmp(&margin).is_le() => current,
                    _ => margin,
                });
            }

            let detected_order = detect_subharmonics
                .then(|| detected_subharmonic_order(value))
                .flatten();
            if mode.subharmonic_order != detected_order {
                return Err(format!(
                    "PSTB Floquet mode {index} has inconsistent subharmonic evidence"
                ));
            }
            if let Some(order) = detected_order {
                expected_subharmonics.push(order);
            }
        }

        let expected_max_magnitude = modes.first().map_or(0.0, |mode| {
            num_complex::Complex64::from(mode.multiplier).norm()
        });
        if !max_multiplier_magnitude.is_finite()
            || !same_retained_float(max_multiplier_magnitude, expected_max_magnitude)
            || num_unstable != expected_unstable_count
            || !same_optional_retained_float(min_stability_margin_db, expected_min_margin)
            || subharmonics != expected_subharmonics
        {
            return Err(
                "PSTB aggregate counts, margins, or subharmonics contradict the complete spectrum"
                    .to_owned(),
            );
        }

        let expected_verdict = derive_floquet_verdict(
            &values,
            evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_threshold - 1.0,
        )?;
        if verdict != expected_verdict {
            return Err(
                "PSTB stability verdict contradicts the complete Floquet spectrum".to_owned(),
            );
        }
        let expected_classification = classify_pstb_modes(modes, verdict, trivial_index)?;
        if classification != expected_classification {
            return Err(
                "PSTB rich stability classification contradicts the complete spectrum".to_owned(),
            );
        }
        Ok(())
    }
}

fn expected_trivial_floquet_index(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
) -> Result<Option<u64>, String> {
    match orbit_kind {
        FloquetOrbitKindEvidence::Driven => Ok(None),
        FloquetOrbitKindEvidence::Autonomous => {
            if !matches!(evidence, FloquetSpectrumEvidence::Qualified { .. }) {
                return Ok(None);
            }
            let values = values
                .iter()
                .copied()
                .map(num_complex::Complex64::from)
                .collect::<Vec<_>>();
            rspice_core::analysis::select_autonomous_phase_mode(&values)
                .map(u64::try_from)
                .transpose()
                .map_err(|_| {
                    "Floquet phase-mode index does not fit the durable contract".to_owned()
                })
        }
        FloquetOrbitKindEvidence::LegacyUnknown => {
            Err("current Floquet evidence has an unknown orbit policy".to_owned())
        }
    }
}

fn derive_floquet_verdict(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    band: f64,
) -> Result<FloquetStabilityVerdictEvidence, String> {
    let values = values
        .iter()
        .copied()
        .map(num_complex::Complex64::from)
        .collect::<Vec<_>>();
    let evidence = evidence
        .as_core()
        .ok_or_else(|| "Floquet certificate is not core-authentic".to_owned())?;
    let orbit_kind = match orbit_kind {
        FloquetOrbitKindEvidence::Driven => rspice_core::analysis::FloquetOrbitKind::Driven,
        FloquetOrbitKindEvidence::Autonomous => rspice_core::analysis::FloquetOrbitKind::Autonomous,
        FloquetOrbitKindEvidence::LegacyUnknown => {
            return Ok(FloquetStabilityVerdictEvidence::Indeterminate);
        }
    };
    let trivial_multiplier_index = trivial_multiplier_index
        .map(usize::try_from)
        .transpose()
        .map_err(|_| "Floquet phase-mode index does not fit this platform".to_owned())?;
    let verdict = match rspice_core::analysis::classify_floquet_stability(
        &values,
        &evidence,
        orbit_kind,
        trivial_multiplier_index,
        band,
    ) {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => {
            FloquetStabilityVerdictEvidence::Stable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => {
            FloquetStabilityVerdictEvidence::Unstable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => {
            FloquetStabilityVerdictEvidence::Marginal
        }
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => {
            FloquetStabilityVerdictEvidence::Indeterminate
        }
        // The durable schema must be deliberately revised before it can
        // authenticate a future core semantic state.
        _ => return Err("unsupported core Floquet stability verdict".to_owned()),
    };
    Ok(verdict)
}

fn classify_pstb_modes(
    modes: &[PstbFloquetModeEvidence],
    verdict: FloquetStabilityVerdictEvidence,
    trivial_index: Option<usize>,
) -> Result<PstbStabilityClassificationEvidence, String> {
    match verdict {
        FloquetStabilityVerdictEvidence::Stable => Ok(PstbStabilityClassificationEvidence::Stable),
        FloquetStabilityVerdictEvidence::Indeterminate => {
            Ok(PstbStabilityClassificationEvidence::Indeterminate)
        }
        FloquetStabilityVerdictEvidence::Unstable => {
            let dominant = modes.iter().find(|mode| mode.is_unstable).ok_or_else(|| {
                "PSTB unstable verdict has no mode outside the stability boundary".to_owned()
            })?;
            if dominant.multiplier.imaginary.abs() > 0.01 {
                Ok(PstbStabilityClassificationEvidence::UnstableComplex)
            } else {
                Ok(PstbStabilityClassificationEvidence::UnstableReal)
            }
        }
        FloquetStabilityVerdictEvidence::Marginal => {
            for (index, mode) in modes.iter().enumerate() {
                if trivial_index == Some(index) {
                    continue;
                }
                let value = num_complex::Complex64::from(mode.multiplier);
                if (value + num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::PeriodDoubling);
                }
                if (value - num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::SaddleNode);
                }
                if (value.norm() - 1.0).abs() < 0.01 && value.im.abs() > 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::NeimarkSacker);
                }
            }
            Ok(PstbStabilityClassificationEvidence::Marginal)
        }
    }
}

fn pstb_modes_are_canonically_sorted(modes: &[PstbFloquetModeEvidence]) -> bool {
    modes.windows(2).all(|pair| {
        let left = &pair[0].multiplier;
        let right = &pair[1].multiplier;
        num_complex::Complex64::from(*right)
            .norm()
            .total_cmp(&num_complex::Complex64::from(*left).norm())
            .then_with(|| left.real.total_cmp(&right.real))
            .then_with(|| left.imaginary.total_cmp(&right.imaginary))
            .is_le()
    })
}

fn detected_subharmonic_order(value: num_complex::Complex64) -> Option<u64> {
    if (value.norm() - 1.0).abs() > 0.01 {
        return None;
    }
    let angle = value.arg().abs();
    (2_u64..=8).find(|order| {
        let expected_angle = 2.0 * std::f64::consts::PI / *order as f64;
        (angle - expected_angle).abs() < 0.01
    })
}

fn same_optional_retained_float(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_retained_float(left, right),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}
