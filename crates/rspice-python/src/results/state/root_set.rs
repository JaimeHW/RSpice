//! Pickle state of pole-zero root-set evidence.
//!
//! The tag and the certificate have to agree: the rebuild re-derives the
//! evidence from the certificate and refuses a state whose tag contradicts
//! it, so a qualified root set can never be restored from an approximate one.

use super::*;

/// Stable primitive pickle state for a pole-zero spectrum certificate.
pub(crate) type SpectrumCertificateState = (usize, usize, f64, f64);

/// Stable tagged pickle state for pole/zero root-set evidence.
pub(crate) type RootSetEvidenceState = (String, Option<SpectrumCertificateState>);

pub(crate) fn spectrum_certificate_state(
    certificate: &rspice_core::analysis::SpectrumCertificate,
) -> SpectrumCertificateState {
    (
        certificate.problem_order,
        certificate.infinite_count,
        certificate.max_backward_error,
        certificate.qualification_tolerance,
    )
}

pub(crate) fn spectrum_certificate_from_state(
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

pub(crate) fn root_set_evidence_state(
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

pub(crate) fn root_set_evidence_from_state(
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
