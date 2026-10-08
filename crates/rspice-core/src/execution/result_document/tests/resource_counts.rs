use super::*;

fn result_limit(document: &AnalysisResultDocument, count: usize) {
    let limits = crate::ResourceLimits {
        max_result_values: count,
        ..Default::default()
    };
    document
        .validate_with_limits_and_abort(&limits, &NoAbort)
        .unwrap();
    AnalysisResultDocument::from_json_with_limits_and_abort(
        &document.to_json().unwrap(),
        &limits,
        &NoAbort,
        u64::MAX,
    )
    .unwrap();
    let lower = crate::ResourceLimits {
        max_result_values: count - 1,
        ..limits
    };
    let error = document
        .validate_with_limits_and_abort(&lower, &NoAbort)
        .unwrap_err();
    assert!(
        matches!(error, ResultDocumentError::ResourceLimit(ref error)
        if error.resource == crate::ResourceKind::ResultValues && error.requested == count && error.limit == count - 1),
        "{error}"
    );
}

#[test]
fn rf_and_monte_carlo_budgets_include_every_retained_data_array() {
    let sp = document_for(AnalysisResultKind::SParameters);
    // One port number and impedance, plus the separately retained rad/s axis.
    assert_eq!(sp.payload.value_count(), 3);
    result_limit(
        &sp,
        sp.values_per_point() * sp.point_count + sp.scalars.len() + 3,
    );

    let mut monte_carlo = document_for(AnalysisResultKind::MonteCarlo);
    let ResultPayload::MonteCarlo(payload) = &mut monte_carlo.payload else {
        panic!("Monte Carlo")
    };
    payload.successful_trial_indices = Some(vec![0, 1, 2]);
    assert_eq!(payload.statistics[0].histogram.len(), 2);
    assert_eq!(payload.statistics[0].bin_edges.len(), 3);
    // Three trial identities, three samples, two histogram bins, three bin
    // edges, and four statistic slots (including any unavailable statistics).
    assert_eq!(monte_carlo.payload.value_count(), 15);
    result_limit(&monte_carlo, monte_carlo.scalars.len() + 15);

    let ResultPayload::PortNoise(mut payload) = document_for(AnalysisResultKind::PortNoise).payload
    else {
        panic!("port noise")
    };
    payload.port_count = 2;
    payload.two_port = vec![TwoPortNoiseEntry {
        frequency: 1e9,
        noise_resistance_ohm: 50.0,
        noise_factor: 2.0,
        minimum_noise_factor: 1.0,
        optimum_source_reflection: ComplexSample {
            real: 0.1,
            imaginary: -0.2,
        },
    }];
    // Frequency, resistance, factor, minimum factor, and both complex components.
    assert_eq!(ResultPayload::PortNoise(payload).value_count(), 6);
}

#[test]
fn malformed_primary_and_pac_series_cannot_hide_allocated_values() {
    for signal in [false, true] {
        let mut document = document_for(AnalysisResultKind::Ac);
        if signal {
            let SeriesValues::Complex { samples } = &mut document.signals[0].values else {
                panic!("complex")
            };
            samples.resize(100, None);
        } else {
            let AxisValues::Real { values } = &mut document.axes[0].values else {
                panic!("real")
            };
            values.resize(100, 0.0);
        }
        let error = document
            .validate_with_limits_and_abort(
                &crate::ResourceLimits {
                    max_result_values: 20,
                    ..Default::default()
                },
                &NoAbort,
            )
            .unwrap_err();
        assert!(
            matches!(error, ResultDocumentError::ResourceLimit(_)),
            "{error}"
        );
    }
    let mut document = document_for(AnalysisResultKind::Pac);
    let original = document.total_value_count();
    let ResultPayload::Pac(payload) = &mut document.payload else {
        panic!("PAC")
    };
    payload.sidebands[0].frequency_offsets.extend([0.0; 100]);
    assert_eq!(document.total_value_count(), original + 100);
    let error = document
        .validate_with_limits_and_abort(
            &crate::ResourceLimits {
                max_result_values: original,
                ..Default::default()
            },
            &NoAbort,
        )
        .unwrap_err();
    assert!(
        matches!(error, ResultDocumentError::ResourceLimit(_)),
        "{error}"
    );
}
