//! Canonical payload tags remain unique across independently added analyses.

use super::*;

#[test]
fn pole_stability_proofs_change_digests_and_legacy_bytes_stay_unchanged() {
    use rspice_core::analysis::pole_zero::{PoleSpectrum, RootSetEvidence, SpectrumCertificate};
    use rspice_core::analysis::stb::{CircuitPoleEvidence, StbAnalyzer};
    let core_certificate = SpectrumCertificate::exact(1, 0).unwrap();
    let mut certificate = PoleZeroSpectrumCertificate {
        problem_order: 1,
        infinite_count: 0,
        max_backward_error: 0.0,
        qualification_tolerance: core_certificate.qualification_tolerance,
        asymptotically_stable: None,
    };
    let writer = || ResultDigestWriter::new("test-stability-proof", ResultDigestEncoding::CURRENT);
    let digest = |certificate| {
        let mut w = writer();
        encode_pole_zero_root_evidence(&mut w, &PoleZeroRootSetEvidence::Qualified { certificate });
        w.finish()
    };
    let legacy = digest(certificate);
    let mut old_writer = writer();
    old_writer.u8(2);
    old_writer.u64(1);
    old_writer.u64(0);
    old_writer.f64(0.0);
    old_writer.f64(certificate.qualification_tolerance);
    assert_eq!(legacy, old_writer.finish());
    let mut hashes = vec![legacy];
    let mut stb_hashes = Vec::new();
    for proof in [None, Some(false), Some(true)] {
        certificate.asymptotically_stable = proof;
        if proof.is_some() {
            let hash = digest(certificate);
            assert!(!hashes.contains(&hash));
            hashes.push(hash);
        }
        let mut core_certificate = core_certificate;
        core_certificate.asymptotically_stable = proof;
        let mut response = StbAnalyzer::new(Default::default())
            .analyze(&[1.0], &[rspice_core::Complex64::new(0.5, 0.0)])
            .unwrap();
        response.circuit_poles = CircuitPoleEvidence::Available {
            spectrum: PoleSpectrum {
                poles: vec![rspice_core::Complex64::new(-1e-18, 1.0)],
                evidence: RootSetEvidence::Qualified {
                    certificate: core_certificate,
                },
            },
        };
        let mut w = writer();
        encode_stb_response(&mut w, &response);
        let hash = w.finish();
        assert!(!stb_hashes.contains(&hash));
        stb_hashes.push(hash);
    }
}

#[test]
fn stb_circuit_modes_and_unavailability_participate_in_the_digest() {
    use rspice_core::analysis::pole_zero::{Matrix, PoleZeroAnalyzer};
    use rspice_core::analysis::stb::{
        CircuitPoleEvidence, CircuitPoleFailure, StbAnalyzer, StbConfig,
    };
    let response = StbAnalyzer::new(StbConfig::new())
        .analyze(&[1.0], &[rspice_core::Complex64::new(0.5, 0.0)])
        .unwrap();
    let digest = |evidence| {
        let mut response = response.clone();
        response.circuit_poles = evidence;
        let mut writer = ResultDigestWriter::new("test-stb-poles", ResultDigestEncoding::CURRENT);
        encode_result_payload(
            &mut writer,
            &AnalysisResultPayload::Stb {
                response: std::sync::Arc::new(response),
            },
            ResultDigestEncoding::CURRENT,
        );
        writer.finish()
    };
    let empty = digest(CircuitPoleEvidence::NotComputed);
    let mut seen = vec![empty];
    for g in [-1.0, 1.0, 2.0] {
        let spectrum =
            PoleZeroAnalyzer::new(Matrix::from_dense(vec![vec![g]]), Matrix::identity(1))
                .pole_spectrum()
                .unwrap();
        let hash = digest(CircuitPoleEvidence::Available { spectrum });
        assert!(!seen.contains(&hash));
        seen.push(hash);
    }
    for cause in [
        CircuitPoleFailure::Numerical {
            detail: "irregular descriptor".into(),
        },
        CircuitPoleFailure::ResourceLimit {
            resource: "result_values".into(),
            requested: 100,
            limit: 80,
        },
        CircuitPoleFailure::ResourceLimit {
            resource: "result_values".into(),
            requested: 101,
            limit: 80,
        },
    ] {
        let hash = digest(CircuitPoleEvidence::Unavailable { cause });
        assert!(!seen.contains(&hash));
        seen.push(hash);
    }
}

#[test]
fn current_impulse_derivatives_participate_in_authenticated_identity() {
    let point = rspice_core::CurrentImpulseDerivative {
        time: 0.3,
        order: 1,
        coefficient: -1e-21,
    };
    let digest = |point: Option<rspice_core::CurrentImpulseDerivative>| {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![],
            real_traces: vec![],
            digital_buses: vec![],
            voltage_impulses: None,
            current_impulses: Some(crate::current_impulses::CurrentImpulseHistoryEvidence {
                start_time_s: 0.0,
                stop_time_s: 1.0,
                delivery_complete: true,
                traces: vec![rspice_core::CurrentImpulseTrace {
                    owner: rspice_core::CurrentImpulseOwner::Branch {
                        branch_name: "H1".into(),
                    },
                    complete: true,
                    points: vec![],
                    derivatives: point.into_iter().collect(),
                }],
            }),
        };
        let mut writer =
            ResultDigestWriter::new("test-current-derivatives", ResultDigestEncoding::CURRENT);
        encode_result_payload(&mut writer, &payload, ResultDigestEncoding::CURRENT);
        writer.finish()
    };
    let original = digest(Some(point));
    assert_ne!(original, digest(None));
    for changed in [
        rspice_core::CurrentImpulseDerivative { time: 0.4, ..point },
        rspice_core::CurrentImpulseDerivative { order: 2, ..point },
        rspice_core::CurrentImpulseDerivative {
            coefficient: 1e-21,
            ..point
        },
    ] {
        assert_ne!(original, digest(Some(changed)));
    }
}

/// Every typed payload arm opens with a tag no other arm uses.
///
/// Two lanes added a payload on the same day and each took "the next unused
/// tag" from its own base: both wrote 11, nothing conflicted textually, and
/// every gate passed. The tags are read out of the encoder's own source so the
/// list cannot fall behind it.
#[test]
fn every_typed_payload_digests_under_its_own_tag() {
    let source = include_str!("../result_digest.rs");
    let mut tags: Vec<(u32, String)> = Vec::new();
    let mut arm: Option<String> = None;
    for line in source.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("AnalysisResultPayload::") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            arm = Some(name);
        } else if let (Some(name), Some(rest)) = (arm.as_ref(), line.strip_prefix("writer.u8("))
            && let Some(tag) = rest
                .strip_suffix(");")
                .and_then(|tag| tag.parse::<u32>().ok())
        {
            tags.push((tag, name.clone()));
            arm = None;
        }
    }
    assert!(
        tags.len() >= 14,
        "the scan stopped finding the payload arms it is here to compare: {tags:?}"
    );
    let mut seen = std::collections::BTreeMap::new();
    for (tag, name) in &tags {
        if let Some(first) = seen.insert(*tag, name.clone()) {
            assert_eq!(
                &first, name,
                "payload tag {tag} is written by both {first} and {name}"
            );
        }
    }
}
