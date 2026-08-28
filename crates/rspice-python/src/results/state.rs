//! Pickle state encoding shared by every result type.
//!
//! A result pickles as a plain tuple of primitives rather than through PyO3's
//! derive, because the core types it wraps are not themselves Python objects.
//! Two conventions here are load-bearing across versions:
//!
//! - Complex values travel as `(real, imaginary)` pairs. `Complex64` has no
//!   PyO3 scalar conversion, and a pair is the lossless encoding a NumPy-free
//!   consumer can read too.
//! - Enums travel as stable string labels, never ordinals, so state pickled by
//!   one build still reads correctly under a later one that reorders a variant.

use super::*;

/// Rebuild a core `SimulationResult` from its Python-visible state.
///
/// `SimulationResult::new` leaves its private observable index empty, and
/// core's observable lookup falls back to a linear scan in exactly that
/// case, so DC observables still resolve after a round-trip.
pub(super) fn rebuild_simulation_result(state: SimulationResultState) -> SimulationResult {
    let (node_voltages, node_names, branch_currents, branch_names, dc_observables) = state;
    let mut result =
        SimulationResult::new(node_voltages.len().saturating_sub(1), branch_currents.len());
    result.node_voltages = node_voltages;
    result.node_names = node_names;
    result.branch_currents = branch_currents;
    result.branch_names = branch_names;
    result.dc_observables = dc_observables;
    result
}

/// Complete Python-visible state of a core `SimulationResult`.
pub(super) type SimulationResultState = (
    Vec<f64>,
    Vec<String>,
    Vec<f64>,
    Vec<String>,
    Vec<(String, f64)>,
);

pub(super) fn simulation_result_state(result: &SimulationResult) -> SimulationResultState {
    (
        result.node_voltages.clone(),
        result.node_names.clone(),
        result.branch_currents.clone(),
        result.branch_names.clone(),
        result.dc_observables.clone(),
    )
}

/// Complex values in pickled state travel as `(real, imaginary)` pairs.
///
/// `num_complex::Complex64` has no PyO3 scalar conversion, and a pair is the
/// lossless encoding NumPy-free consumers can read too.
pub(super) fn complex_state(values: &[Complex64]) -> Vec<(f64, f64)> {
    values.iter().map(|value| (value.re, value.im)).collect()
}

pub(super) fn complex_from_state(values: Vec<(f64, f64)>) -> Vec<Complex64> {
    values
        .into_iter()
        .map(|(re, im)| Complex64::new(re, im))
        .collect()
}

/// Stable primitive pickle state for a pole-zero spectrum certificate.
pub(super) type SpectrumCertificateState = (usize, usize, f64, f64);

/// Stable tagged pickle state for pole/zero root-set evidence.
pub(super) type RootSetEvidenceState = (String, Option<SpectrumCertificateState>);

pub(super) fn spectrum_certificate_state(
    certificate: &rspice_core::analysis::SpectrumCertificate,
) -> SpectrumCertificateState {
    (
        certificate.problem_order,
        certificate.infinite_count,
        certificate.max_backward_error,
        certificate.qualification_tolerance,
    )
}

pub(super) fn spectrum_certificate_from_state(
    state: SpectrumCertificateState,
) -> PyResult<rspice_core::analysis::SpectrumCertificate> {
    let (problem_order, infinite_count, max_backward_error, qualification_tolerance) = state;
    rspice_core::analysis::SpectrumCertificate::new(
        problem_order,
        infinite_count,
        max_backward_error,
        qualification_tolerance,
    )
    .ok_or_else(|| {
        crate::errors::value_error(
            "invalid spectrum certificate in pickled pole-zero result".to_string(),
        )
    })
}

pub(super) fn root_set_evidence_state(
    evidence: &rspice_core::analysis::RootSetEvidence,
) -> PyResult<RootSetEvidenceState> {
    use rspice_core::analysis::RootSetEvidence;

    match evidence {
        RootSetEvidence::NotRequested => Ok(("not_requested".to_string(), None)),
        RootSetEvidence::QualifiedEmpty { certificate } => Ok((
            "qualified_empty".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::Qualified { certificate } => Ok((
            "qualified".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::Approximate { certificate } => Ok((
            "approximate".to_string(),
            Some(spectrum_certificate_state(certificate)),
        )),
        RootSetEvidence::LegacyUnknown => Ok(("legacy_unknown".to_string(), None)),
        _ => Err(crate::errors::value_error(
            "unsupported root-set evidence variant".to_string(),
        )),
    }
}

pub(super) fn root_set_evidence_from_state(
    state: RootSetEvidenceState,
) -> PyResult<rspice_core::analysis::RootSetEvidence> {
    use rspice_core::analysis::RootSetEvidence;

    let (kind, certificate) = state;
    match (kind.as_str(), certificate) {
        ("not_requested", None) => Ok(RootSetEvidence::NotRequested),
        ("legacy_unknown", None) => Ok(RootSetEvidence::LegacyUnknown),
        ("qualified_empty" | "qualified" | "approximate", Some(certificate)) => {
            let certificate = spectrum_certificate_from_state(certificate)?;
            let derived =
                RootSetEvidence::from_certificate(certificate.finite_count(), certificate)
                    .ok_or_else(|| {
                        crate::errors::value_error(
                            "inconsistent root-set certificate in pickled pole-zero result"
                                .to_string(),
                        )
                    })?;
            let tag_matches = matches!(
                (kind.as_str(), &derived),
                ("qualified_empty", RootSetEvidence::QualifiedEmpty { .. })
                    | ("qualified", RootSetEvidence::Qualified { .. })
                    | ("approximate", RootSetEvidence::Approximate { .. })
            );
            if tag_matches {
                Ok(derived)
            } else {
                Err(crate::errors::value_error(format!(
                    "root-set evidence tag '{kind}' disagrees with its certificate"
                )))
            }
        }
        ("not_requested" | "legacy_unknown", Some(_)) => Err(crate::errors::value_error(format!(
            "root-set evidence tag '{kind}' cannot carry a certificate"
        ))),
        ("qualified_empty" | "qualified" | "approximate", None) => Err(crate::errors::value_error(
            format!("root-set evidence tag '{kind}' requires a certificate"),
        )),
        (other, _) => Err(crate::errors::value_error(format!(
            "unknown root-set evidence tag '{other}' in pickled state"
        ))),
    }
}

/// The `(absolute, normalized)` complex derivative pair of an AC sensitivity
/// trace, in pickled `(real, imaginary)` form.
pub(super) type ComplexSeriesPair = (Vec<(f64, f64)>, Vec<(f64, f64)>);

/// Complete Python-visible state of one core `AcResult` row.
pub(super) type AcRowState = (
    f64,
    Vec<String>,
    Vec<String>,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
);

pub(super) fn ac_row_state(row: &AcResult) -> AcRowState {
    (
        row.frequency,
        row.node_names.clone(),
        row.branch_names.clone(),
        complex_state(&row.voltages),
        complex_state(&row.currents),
    )
}

pub(super) fn rebuild_ac_row(state: AcRowState) -> AcResult {
    let (frequency, node_names, branch_names, voltages, currents) = state;
    AcResult {
        frequency,
        node_names,
        branch_names,
        voltages: complex_from_state(voltages),
        currents: complex_from_state(currents),
    }
}

/// One named complex spectrum: `(name, coefficients, harmonic frequencies)`.
///
/// `SpectralVoltage` and `SpectralBranchCurrent` share this shape.
pub(super) type SpectralSeriesState = (String, Vec<(f64, f64)>, Vec<f64>);

/// One reactive element's retained HB spectra:
/// `(device, kind, voltage phasors, current phasors, DC current is exact)`.
pub(super) type HbReactiveState = (String, String, Vec<(f64, f64)>, Vec<(f64, f64)>, bool);

/// Complex values indexed `[frequency][sideband][node]` or
/// `[frequency][output sideband][input sideband]`.
pub(super) type ComplexGridState = Vec<Vec<Vec<(f64, f64)>>>;

/// One distortion product and its per-F1-point rows, keyed by stable label.
pub(super) type DistortionProductState = (String, Vec<AcRowState>);

pub(super) fn spectral_series_state(
    name: &str,
    coefficients: &[Complex64],
    frequencies: &[f64],
) -> SpectralSeriesState {
    (
        name.to_string(),
        complex_state(coefficients),
        frequencies.to_vec(),
    )
}

/// `HbReactiveKind` travels as its lowercase element name.
pub(super) fn hb_reactive_kind_label(kind: HbReactiveKind) -> &'static str {
    match kind {
        HbReactiveKind::Capacitor => "capacitor",
        HbReactiveKind::Inductor => "inductor",
    }
}

pub(super) fn rebuild_hb_reactive(
    state: HbReactiveState,
) -> PyResult<rspice_core::analysis::HbReactiveSpectrum> {
    let (device_name, kind, voltage, current, dc_current_is_exact) = state;
    Ok(rspice_core::analysis::HbReactiveSpectrum {
        device_name,
        kind: hb_reactive_kind_from_label(&kind)?,
        voltage_coefficients: complex_from_state(voltage),
        current_coefficients: complex_from_state(current),
        dc_current_is_exact,
    })
}

fn hb_reactive_kind_from_label(label: &str) -> PyResult<HbReactiveKind> {
    match label {
        "capacitor" => Ok(HbReactiveKind::Capacitor),
        "inductor" => Ok(HbReactiveKind::Inductor),
        other => Err(crate::errors::value_error(format!(
            "unknown reactive element kind '{other}' in pickled state"
        ))),
    }
}

/// Continuation limitations travel as stable snake-case tags rather than
/// ordinals, so a state pickled by one build still reads correctly under a
/// later one that reorders the enum.
pub(super) fn hb_limitation_label(limitation: &HbContinuationLimitation) -> &'static str {
    match limitation {
        HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent => {
            "nonlinear_voltage_sources_use_norton_equivalent"
        }
        HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate => {
            "inductor_dc_current_uses_short_surrogate"
        }
        HbContinuationLimitation::VerilogAInternalStateNotRetained => {
            "veriloga_internal_state_not_retained"
        }
    }
}

pub(super) fn hb_limitation_from_label(label: &str) -> PyResult<HbContinuationLimitation> {
    match label {
        "nonlinear_voltage_sources_use_norton_equivalent" => {
            Ok(HbContinuationLimitation::NonlinearVoltageSourcesUseNortonEquivalent)
        }
        "inductor_dc_current_uses_short_surrogate" => {
            Ok(HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate)
        }
        "veriloga_internal_state_not_retained" => {
            Ok(HbContinuationLimitation::VerilogAInternalStateNotRetained)
        }
        other => Err(crate::errors::value_error(format!(
            "unknown continuation limitation '{other}' in pickled state"
        ))),
    }
}

/// Distortion products travel as the same stable labels the accessors accept.
pub(super) fn distortion_product_from_label(label: &str) -> PyResult<DistortionProduct> {
    match label {
        "2f1" => Ok(DistortionProduct::SecondHarmonic),
        "3f1" => Ok(DistortionProduct::ThirdHarmonic),
        "f1+f2" => Ok(DistortionProduct::Sum),
        "f1-f2" => Ok(DistortionProduct::Difference),
        "2f1-f2" => Ok(DistortionProduct::ThirdOrderDifference),
        other => Err(crate::errors::value_error(format!(
            "unknown distortion product '{other}' in pickled state"
        ))),
    }
}

/// The `_unpickle` callable bound to a pyclass, referenced by `__reduce__`.
///
/// Pickling a bound staticmethod resolves by qualified name, which keeps each
/// helper namespaced on its own class instead of adding private names to the
/// module surface.
pub(super) fn unpickler<'py, T: pyo3::PyTypeInfo>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    py.get_type::<T>().getattr("_unpickle")
}
