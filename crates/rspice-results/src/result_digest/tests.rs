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
            .analyze_with_abort(
                &[1.0],
                &[rspice_core::Complex64::new(0.5, 0.0)],
                &rspice_core::NoAbort,
            )
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
        .analyze_with_abort(
            &[1.0],
            &[rspice_core::Complex64::new(0.5, 0.0)],
            &rspice_core::NoAbort,
        )
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
            hdl_sources: Vec::new(),
        };
        let mut writer =
            ResultDigestWriter::new("test-current-derivatives", ResultDigestEncoding::CURRENT);
        encode_result_payload(&mut writer, &payload);
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

/// A stored column or event signal in memory that counts its reads; `identity` is what its store
/// states of it.
struct Held {
    values: Vec<f64>,
    identity: Option<([u8; 32], u64)>,
    reads: std::sync::atomic::AtomicUsize,
}

impl Held {
    fn new(values: &[f64], identity: Option<([u8; 32], u64)>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            values: values.to_vec(),
            identity,
            reads: std::sync::atomic::AtomicUsize::new(0),
        })
    }

    fn reads(&self) -> usize {
        self.reads.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn change(&self, index: usize) -> rspice_results_store::EventChange {
        rspice_results_store::EventChange {
            signal: 0,
            time: index as f64,
            sequence: index as u32,
            value: rspice_results_store::EventValue::Real(self.values[index]),
            strength: 0,
        }
    }
}

impl crate::waveform::ColumnSource for Held {
    fn name(&self) -> &str {
        "v(out)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        self.identity
    }
}

impl crate::events::points::EventHistorySource for Held {
    fn name(&self) -> &str {
        "e(out)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn count(&self) -> Result<u64, String> {
        Ok(self.values.len() as u64)
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        Ok((0..self.values.len()).filter(|&i| (i as f64) < t).count() as u64)
    }

    fn span(&self, t0: f64, t1: f64) -> Result<rspice_results_store::EventSpan, String> {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let times = 0..self.values.len();
        Ok(rspice_results_store::EventSpan {
            before: times
                .clone()
                .rev()
                .find(|&i| (i as f64) < t0)
                .map(|i| self.change(i)),
            within: times
                .clone()
                .filter(|&i| (i as f64) >= t0 && (i as f64) <= t1)
                .map(|i| self.change(i))
                .collect(),
            after: times
                .clone()
                .find(|&i| (i as f64) > t1)
                .map(|i| self.change(i)),
        })
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        self.identity
    }
}

/// Opening a stored run digests it to bind its views: a digest that read every row would read
/// the whole run before its first paint. A stored window or event history whose store states its
/// digest is identified by the store, its column or signal and its window, and reads nothing; one
/// whose store states none (a run recovered without its end) digests as its values in memory do.
#[test]
fn a_stored_result_is_identified_by_its_store_without_reading_it() {
    use crate::events::points::EventPoints;
    use crate::waveform::{StoredWindow, WaveformValues, WindowLead};

    let writer = || ResultDigestWriter::new("test-stored", ResultDigestEncoding::CURRENT);
    let values = |values: &WaveformValues| {
        let mut writer = writer();
        writer.waveform_values(values);
        (writer.retained_bytes(), writer.finish())
    };
    let events = |points: &EventPoints<crate::events::RealEventPointEvidence>| {
        let mut writer = writer();
        writer.event_points(points, 16, |writer, point| {
            writer.f64(point.time_s);
            writer.f64(point.value);
        });
        (writer.retained_bytes(), writer.finish())
    };
    let window = |first, rows, lead| StoredWindow { first, rows, lead };
    let samples = [0.0, 1.0, 2.0, 3.0, 4.0];

    let column = Held::new(&samples, Some(([7; 32], 1)));
    let stored = WaveformValues::stored(column.clone(), window(1, 3, WindowLead::None)).unwrap();
    let (bytes, identified) = values(&stored);
    assert_eq!(
        column.reads(),
        0,
        "the digest read a row of a stored window"
    );
    assert!(
        bytes >= 3 * 8,
        "the window's rows count as retained data: {bytes}"
    );
    for other in [
        WaveformValues::stored(column.clone(), window(0, 3, WindowLead::None)),
        WaveformValues::stored(column.clone(), window(1, 2, WindowLead::None)),
        WaveformValues::stored(column.clone(), window(1, 3, WindowLead::Value(0.5))),
        WaveformValues::stored(column.clone(), window(1, 3, WindowLead::Interpolated(0.5))),
        WaveformValues::stored(
            Held::new(&samples, Some(([7; 32], 2))),
            window(1, 3, WindowLead::None),
        ),
        WaveformValues::stored(
            Held::new(&samples, Some(([8; 32], 1))),
            window(1, 3, WindowLead::None),
        ),
    ] {
        assert_ne!(values(&other.unwrap()).1, identified);
    }
    assert_eq!(column.reads(), 0);
    let unstated =
        WaveformValues::stored(Held::new(&samples, None), window(1, 3, WindowLead::None)).unwrap();
    assert_eq!(
        values(&unstated).1,
        values(&WaveformValues::from(vec![1.0, 2.0, 3.0])).1
    );

    let signal = Held::new(&[0.5, 1.5, 2.5], Some(([7; 32], 0)));
    let history = EventPoints::stored(signal.clone(), 1.0).unwrap();
    let (bytes, identified) = events(&history);
    assert_eq!(
        signal.reads(),
        0,
        "the digest read a change of a stored history"
    );
    assert!(
        bytes >= 2 * 16,
        "the history's changes count as retained data: {bytes}"
    );
    assert_ne!(
        events(&EventPoints::stored(signal.clone(), 0.0).unwrap()).1,
        identified
    );
    assert_ne!(
        events(&EventPoints::stored(Held::new(&[0.5, 1.5, 2.5], Some(([7; 32], 1))), 1.0).unwrap())
            .1,
        identified
    );
    let unstated = EventPoints::stored(Held::new(&[0.5, 1.5, 2.5], None), 1.0).unwrap();
    let in_memory: EventPoints<_> = [(1.0, 1.5), (2.0, 2.5)]
        .into_iter()
        .map(|(time_s, value)| crate::events::RealEventPointEvidence { time_s, value })
        .collect();
    assert_eq!(events(&unstated).1, events(&in_memory).1);
}
