//! Complete-result round trips and rejection of corrupted transfer evidence.
use super::*;
use crate::abort_signal::CountingAbort;

#[test]
fn qpxf_retained_result_round_trip_and_corruption_boundaries() {
    let netlist = Netlist::parse(
        "QPXF saved result\nV1 in 0 DC 1\nI1 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(Default::default());
    let point = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(vec![1000.0, std::f64::consts::SQRT_2 * 1000.0], vec![1, 1]),
        )
        .unwrap();
    let request = QpxfRequest {
        frequencies_hz: vec![-37.0, 0.0, 127.0],
        frequency_axis: QpxfFrequencyAxis::Output,
        input_sources: QpxfSources::AllIndependent,
        input_lattices: QpxfInputLattices::AllRetained,
        output: QpxfOutput::Voltage {
            positive: "out".into(),
            negative: "GND".into(),
        },
        output_lattice: vec![1, -1],
        linear: Default::default(),
        group_delay: true,
        group_delay_magnitude_floor: 1e-8,
    };
    let result = engine
        .run_qpxf_from_qpss(&netlist, request, &point)
        .unwrap();
    let limits = ResourceLimits::default();
    assert_eq!(result.transfers.len(), 18);
    assert!(result.transfers.iter().any(|t| {
        t.group_delay
            .as_ref()
            .unwrap()
            .iter()
            .all(|d| matches!(d, QpxfGroupDelay::BelowMagnitudeFloor))
    }));
    assert!(result.transfers.iter().any(|t| {
        t.group_delay
            .as_ref()
            .unwrap()
            .iter()
            .all(|d| matches!(d, QpxfGroupDelay::Finite(_)))
    }));
    let text = serde_json::to_string(&result).unwrap();
    let decoded: QpxfAnalysisResult = serde_json::from_str(&text).unwrap();
    decoded
        .validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    assert_eq!(decoded, result);
    let (metadata, rows) = result.clone().into_transfer_parts();
    let lengths: Vec<_> = rows.iter().map(Vec::len).collect();
    assert_eq!(
        QpxfAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            rows.clone(),
            &limits,
            &NoAbort
        )
        .unwrap(),
        result
    );
    let mut short = rows.clone();
    short[0].pop();
    assert!(
        QpxfAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            short,
            &limits,
            &NoAbort
        )
        .is_err()
    );
    let mut corrupt = rows;
    corrupt[0][0].re += 0.1;
    assert!(
        QpxfAnalysisResult::from_transfer_parts_with_abort(
            metadata.clone(),
            corrupt,
            &limits,
            &NoAbort
        )
        .is_err()
    );
    let mut limited = limits;
    limited.max_result_values = 1;
    assert!(matches!(
        metadata.validate_transfer_layout_with_abort(&lengths, &limited, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        result.validate_retained_payload_with_abort(&limits, &CountingAbort::new(0)),
        Err(SimulationError::Aborted)
    ));
    let mutations: &[fn(&mut QpxfAnalysisResult)] = &[
        |r| r.metadata.version += 1,
        |r| r.metadata.operating_point_identity = "not-an-identity".into(),
        |r| r.metadata.normalized_residuals[0] = 2.0,
        |r| r.metadata.request.output_lattice[0] = 9,
        |r| r.metadata.output_frequencies_hz[0] += 1.0,
        |r| r.metadata.probe_offsets_hz[0] += 1.0,
        |r| r.metadata.input_lattices.swap(0, 1),
        |r| r.metadata.input_sources.swap(0, 1),
        |r| r.metadata.input_sources[0].injections[0].0 = usize::MAX,
        |r| r.metadata.observation[0].1 = -r.metadata.observation[0].1,
        |r| r.metadata.ground_policy = GroundPolicy::OnlyZero,
        |r| r.solutions[0].frequency_hz += 1.0,
        |r| r.solutions[0].frequency_lattice[0] += 1,
        |r| r.solutions[0].normalized_residual = 0.5,
        |r| r.solutions[0].sensitivities[0][0].re = Value::NAN,
        |r| r.transfers[0].input_source = 1,
        |r| r.transfers[0].input_lattice[0] = 9,
        |r| r.transfers[0].input_frequencies_hz[0] += 1.0,
        |r| r.transfers[0].values[0].re += 0.1,
        |r| r.transfers[0].group_delay = None,
        |r| r.transfers[0].group_delay.as_mut().unwrap()[0] = QpxfGroupDelay::Finite(0.0),
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut changed = result.clone();
        mutate(&mut changed);
        // Structural/derived evidence must be rejected independently of a
        // matching checksum, including payloads authored by an older worker.
        changed.metadata.retained_identity = changed.payload_identity(&NoAbort).unwrap();
        assert!(
            changed
                .validate_retained_payload_with_abort(&limits, &NoAbort)
                .is_err(),
            "mutation {index}"
        );
    }
    let mut changed = result.clone();
    changed.metadata.request.linear.relative_tolerance *= 2.0;
    assert!(
        changed
            .validate_retained_payload_with_abort(&limits, &NoAbort)
            .is_err(),
        "request identity is bound even when shape is unchanged"
    );
}
