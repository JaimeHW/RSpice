//! Validated periodic stability evidence from the authoritative engine.

use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::floquet::FloquetOrbitKindEvidence;
use rspice_results::floquet::FloquetSpectrumCertificateEvidence;
use rspice_results::floquet::FloquetSpectrumEvidence;
use rspice_results::floquet::FloquetStabilityVerdictEvidence;
use rspice_results::floquet::PssFloquetMultiplierEvidence;
use rspice_results::floquet::PstbStabilityClassificationEvidence;
use rspice_results::simulation_values::ComplexResultValue;

pub(super) fn retain_floquet_evidence(
    evidence: &rspice_core::analysis::FloquetSpectrumEvidence,
) -> Result<FloquetSpectrumEvidence, String> {
    use rspice_core::analysis::FloquetSpectrumEvidence as CoreEvidence;

    match evidence {
        CoreEvidence::NotComputed => Ok(FloquetSpectrumEvidence::NotComputed),
        CoreEvidence::NoDynamicModes => Ok(FloquetSpectrumEvidence::NoDynamicModes),
        CoreEvidence::Qualified { certificate } => Ok(FloquetSpectrumEvidence::Qualified {
            certificate: FloquetSpectrumCertificateEvidence {
                problem_order: u64::try_from(certificate.problem_order)
                    .map_err(|_| "Floquet problem order does not fit durable storage")?,
                max_backward_error: certificate.max_backward_error,
                qualification_tolerance: certificate.qualification_tolerance,
            },
        }),
        CoreEvidence::LegacyUnknown => Ok(FloquetSpectrumEvidence::LegacyUnknown),
        _ => Err("unsupported Floquet evidence variant".to_owned()),
    }
}

pub(super) fn retain_floquet_orbit_kind(
    orbit_kind: rspice_core::analysis::FloquetOrbitKind,
) -> Result<FloquetOrbitKindEvidence, String> {
    match orbit_kind {
        rspice_core::analysis::FloquetOrbitKind::Driven => Ok(FloquetOrbitKindEvidence::Driven),
        rspice_core::analysis::FloquetOrbitKind::Autonomous => {
            Ok(FloquetOrbitKindEvidence::Autonomous)
        }
        _ => Err("unsupported Floquet orbit policy".to_owned()),
    }
}

pub(super) fn retain_floquet_verdict(
    verdict: rspice_core::analysis::FloquetStabilityVerdict,
) -> Result<FloquetStabilityVerdictEvidence, String> {
    match verdict {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => {
            Ok(FloquetStabilityVerdictEvidence::Stable)
        }
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => {
            Ok(FloquetStabilityVerdictEvidence::Unstable)
        }
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => {
            Ok(FloquetStabilityVerdictEvidence::Marginal)
        }
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => {
            Ok(FloquetStabilityVerdictEvidence::Indeterminate)
        }
        _ => Err("unsupported Floquet stability verdict".to_owned()),
    }
}

pub(super) fn retain_pstb_classification(
    classification: rspice_core::analysis::pstb::StabilityType,
) -> Result<PstbStabilityClassificationEvidence, String> {
    use rspice_core::analysis::pstb::StabilityType as CoreClassification;

    match classification {
        CoreClassification::Stable => Ok(PstbStabilityClassificationEvidence::Stable),
        CoreClassification::UnstableReal => Ok(PstbStabilityClassificationEvidence::UnstableReal),
        CoreClassification::UnstableComplex => {
            Ok(PstbStabilityClassificationEvidence::UnstableComplex)
        }
        CoreClassification::PeriodDoubling => {
            Ok(PstbStabilityClassificationEvidence::PeriodDoubling)
        }
        CoreClassification::NeimarkSacker => Ok(PstbStabilityClassificationEvidence::NeimarkSacker),
        CoreClassification::SaddleNode => Ok(PstbStabilityClassificationEvidence::SaddleNode),
        CoreClassification::Marginal => Ok(PstbStabilityClassificationEvidence::Marginal),
        CoreClassification::Indeterminate => Ok(PstbStabilityClassificationEvidence::Indeterminate),
        _ => Err("unsupported PSTB stability classification".to_owned()),
    }
}

pub(super) fn retain_pss_floquet_payload(
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> Result<AnalysisResultPayload, String> {
    let result = &operating_point.analysis().result;
    if !result.has_consistent_floquet_contract() {
        return Err("PSS Floquet runtime contract is inconsistent".to_owned());
    }
    Ok(AnalysisResultPayload::PssFloquet {
        period_s: Some(result.period),
        fundamental_frequency_hz: Some(result.frequency),
        iterations: Some(
            u64::try_from(result.iterations)
                .map_err(|_| "PSS iteration count does not fit durable storage")?,
        ),
        residual_norm: Some(result.residual_norm),
        multipliers: result
            .floquet_multipliers
            .iter()
            .map(|multiplier| PssFloquetMultiplierEvidence {
                multiplier: ComplexResultValue {
                    real: multiplier.re,
                    imaginary: multiplier.im,
                },
            })
            .collect(),
        floquet_evidence: retain_floquet_evidence(&result.floquet_evidence)?,
        orbit_kind: retain_floquet_orbit_kind(result.floquet_orbit_kind)?,
        trivial_multiplier_index: result
            .trivial_floquet_multiplier_index
            .map(u64::try_from)
            .transpose()
            .map_err(|_| "PSS trivial Floquet index does not fit durable storage")?,
        stability_verdict: retain_floquet_verdict(result.stability_verdict())?,
    })
}
