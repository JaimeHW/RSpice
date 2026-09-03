//! Pickle state of the Floquet evidence a PSS result carries.
//!
//! The evidence is a tagged union, not a number: a spectrum that was never
//! computed, one with no dynamic modes, and one qualified by a backward-error
//! certificate are three different claims, and the round trip keeps them
//! apart rather than collapsing them into a missing value.

use super::*;

/// Stable primitive pickle state for a qualified Floquet eigenspectrum.
pub(crate) type FloquetSpectrumCertificateState = (usize, f64, f64);

/// Stable tagged pickle state for the provenance of a Floquet spectrum.
pub(crate) type FloquetSpectrumEvidenceState = (String, Option<FloquetSpectrumCertificateState>);

/// Versioned Floquet contract appended to the legacy six-argument PSS state.
pub(crate) type PssFloquetContractState =
    (usize, FloquetSpectrumEvidenceState, String, Option<usize>);

pub(crate) const PSS_FLOQUET_CONTRACT_STATE_VERSION: usize = 1;

pub(crate) fn floquet_spectrum_certificate_state(
    certificate: &rspice_core::analysis::FloquetSpectrumCertificate,
) -> FloquetSpectrumCertificateState {
    (
        certificate.problem_order,
        certificate.max_backward_error,
        certificate.qualification_tolerance,
    )
}

pub(crate) fn floquet_spectrum_certificate_from_state(
    state: FloquetSpectrumCertificateState,
) -> PyResult<rspice_core::analysis::FloquetSpectrumCertificate> {
    let (problem_order, max_backward_error, qualification_tolerance) = state;
    rspice_core::analysis::FloquetSpectrumCertificate::new(
        problem_order,
        max_backward_error,
        qualification_tolerance,
    )
    .ok_or_else(|| {
        crate::errors::value_error(
            "invalid Floquet spectrum certificate in pickled PSS result".to_string(),
        )
    })
}

pub(crate) fn floquet_spectrum_evidence_state(
    evidence: &rspice_core::analysis::FloquetSpectrumEvidence,
) -> PyResult<FloquetSpectrumEvidenceState> {
    use rspice_core::analysis::FloquetSpectrumEvidence;

    match evidence {
        FloquetSpectrumEvidence::NotComputed => Ok(("not_computed".to_string(), None)),
        FloquetSpectrumEvidence::NoDynamicModes => Ok(("no_dynamic_modes".to_string(), None)),
        FloquetSpectrumEvidence::Qualified { certificate } => Ok((
            "qualified".to_string(),
            Some(floquet_spectrum_certificate_state(certificate)),
        )),
        FloquetSpectrumEvidence::LegacyUnknown => Ok(("legacy_unknown".to_string(), None)),
        _ => Err(crate::errors::value_error(
            "unsupported Floquet spectrum evidence variant".to_string(),
        )),
    }
}

pub(crate) fn floquet_spectrum_evidence_from_state(
    state: FloquetSpectrumEvidenceState,
) -> PyResult<rspice_core::analysis::FloquetSpectrumEvidence> {
    use rspice_core::analysis::FloquetSpectrumEvidence;

    let (kind, certificate) = state;
    match (kind.as_str(), certificate) {
        ("not_computed", None) => Ok(FloquetSpectrumEvidence::NotComputed),
        ("no_dynamic_modes", None) => Ok(FloquetSpectrumEvidence::NoDynamicModes),
        ("legacy_unknown", None) => Ok(FloquetSpectrumEvidence::LegacyUnknown),
        ("qualified", Some(certificate)) => {
            let certificate = floquet_spectrum_certificate_from_state(certificate)?;
            FloquetSpectrumEvidence::qualified(certificate).ok_or_else(|| {
                crate::errors::value_error(
                    "inconsistent qualified Floquet evidence in pickled PSS result".to_string(),
                )
            })
        }
        ("not_computed" | "no_dynamic_modes" | "legacy_unknown", Some(_)) => {
            Err(crate::errors::value_error(format!(
                "Floquet evidence tag '{kind}' cannot carry a certificate"
            )))
        }
        ("qualified", None) => Err(crate::errors::value_error(
            "Floquet evidence tag 'qualified' requires a certificate".to_string(),
        )),
        (other, _) => Err(crate::errors::value_error(format!(
            "unknown Floquet evidence tag '{other}' in pickled PSS state"
        ))),
    }
}

pub(crate) fn floquet_orbit_kind_state(
    orbit_kind: rspice_core::analysis::FloquetOrbitKind,
) -> PyResult<String> {
    use rspice_core::analysis::FloquetOrbitKind;

    match orbit_kind {
        FloquetOrbitKind::Driven => Ok("driven".to_string()),
        FloquetOrbitKind::Autonomous => Ok("autonomous".to_string()),
        _ => Err(crate::errors::value_error(
            "unsupported Floquet orbit kind".to_string(),
        )),
    }
}

pub(crate) fn floquet_orbit_kind_from_state(
    orbit_kind: &str,
) -> PyResult<rspice_core::analysis::FloquetOrbitKind> {
    match orbit_kind {
        "driven" => Ok(rspice_core::analysis::FloquetOrbitKind::Driven),
        "autonomous" => Ok(rspice_core::analysis::FloquetOrbitKind::Autonomous),
        other => Err(crate::errors::value_error(format!(
            "unknown Floquet orbit kind '{other}' in pickled PSS state"
        ))),
    }
}
