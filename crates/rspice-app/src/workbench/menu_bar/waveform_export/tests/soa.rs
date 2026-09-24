//! Retained SOA evidence in engineering-data exports.

use super::*;

#[test]
fn soa_derating_csv_preserves_each_sample_temperature_and_limit() {
    use crate::results::safety::{SoaPowerDerating, SoaPowerDeratingEvidence};
    let mut traces = Vec::new();
    for (name, unit, values) in [
        ("SOA_PDISS(Q1)", "W", vec![0.8, 0.5, 0.2]),
        ("SOA_PDISS_LIMIT(Q1)", "W", vec![1.0, 0.5, 0.0]),
        ("SOA_PDISS_TEMPERATURE(Q1)", "K", vec![290.0, 350.0, 400.0]),
    ] {
        let mut trace = waveform(name, vec![0.0, 1.0, 2.0], values);
        trace.unit = Some(unit.into());
        traces.push(trace);
    }
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "Derated SOA")
        .with_waveforms(traces)
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0, 2.0],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: Some(SoaPowerDeratingEvidence {
                    rated_power_w: 1.0,
                    curve: SoaPowerDerating {
                        reference_temperature_kelvin: 300.0,
                        watts_per_kelvin: 0.01,
                    },
                }),
                device_id: "Q1".into(),
                parameter: SoaParameterEvidence::PowerDissipation,
                limit_value: 0.0,
                worst_actual_value: 0.2,
                worst_time_s: 2.0,
                sample_count: 3,
                unit: "W".into(),
                description: "Derated power".into(),
                verdict: SoaRuleVerdictEvidence::Critical,
            }],
            violations: vec![
                SoaViolationEvidence {
                    device_id: "Q1".into(),
                    parameter: SoaParameterEvidence::PowerDissipation,
                    limit_value: 0.5,
                    actual_value: 0.5,
                    time_s: 1.0,
                    severity: SoaViolationSeverityEvidence::Warning,
                },
                SoaViolationEvidence {
                    device_id: "Q1".into(),
                    parameter: SoaParameterEvidence::PowerDissipation,
                    limit_value: 0.0,
                    actual_value: 0.2,
                    time_s: 2.0,
                    severity: SoaViolationSeverityEvidence::Critical,
                },
            ],
        });
    analysis.validate_retained_evidence().unwrap();
    let mut custom = analysis.clone();
    let Some(AnalysisResultPayload::Soa {
        source_history: _,
        evaluations,
        violations,
    }) = custom.result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations[0].thresholds = crate::results::safety::SoaThresholds {
        warning_fraction: None,
        critical_fraction: None,
    };
    evaluations[0].verdict = SoaRuleVerdictEvidence::Violation;
    violations.remove(0);
    violations[0].severity = SoaViolationSeverityEvidence::Violation;
    custom.validate_retained_evidence().unwrap();
    let custom_csv = prepare_typed_result_csv(&custom).unwrap().contents;
    assert!(
        custom_csv
            .lines()
            .next()
            .unwrap()
            .ends_with(",warning_fraction,critical_fraction")
    );
    for row in custom_csv.lines().skip(1) {
        assert_eq!(row.split(',').count(), 16);
        assert!(row.ends_with(",off,off"));
    }
    // Reporting cadence must not thin the separate SOA evidence CSV.
    let mut reported = analysis.clone();
    let mut source = crate::state::SoaSourceHistory {
        time: vec![0.0, 1.0, 2.0],
        waveforms: analysis
            .waveforms
            .iter()
            .map(|wave| crate::state::SoaSourceWaveform {
                name: wave.name.clone(),
                unit: wave.unit.clone().unwrap(),
                values: wave.y.as_ref().clone(),
            })
            .collect(),
    };
    source.waveforms.push(crate::state::SoaSourceWaveform {
        name: "SOA_VIOLATION_COUNT".into(),
        unit: "count".into(),
        values: vec![0.0, 1.0, 2.0],
    });
    let projection =
        rspice_core::analysis::transient::TransientOutputProjection::interpolate_times(
            &source.time,
            &[0.5, 2.0],
            2,
        )
        .unwrap();
    reported.waveforms = source
        .waveforms
        .iter()
        .map(|wave| {
            WaveformData::new(
                &wave.name,
                projection.times().to_vec(),
                source.report_values(wave, &projection).unwrap(),
                "#00aaff",
            )
            .with_unit(&wave.unit)
        })
        .collect();
    let Some(AnalysisResultPayload::Soa { source_history, .. }) = &mut reported.result_payload
    else {
        unreachable!()
    };
    *source_history = Some(std::sync::Arc::new(source));
    reported.validate_retained_evidence().unwrap();
    assert_eq!(
        prepare_typed_result_csv(&reported).unwrap().contents,
        prepare_typed_result_csv(&analysis).unwrap().contents
    );
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);
    let files = io.text_files.borrow();
    assert_eq!(files.len(), 1);
    let csv = &files[0].1;
    assert!(csv.lines().next().unwrap().ends_with(
        "temperature_kelvin,rated_power_w,reference_temperature_kelvin,watts_per_kelvin"
    ));
    let samples = csv
        .lines()
        .filter(|line| line.starts_with("sample,"))
        .collect::<Vec<_>>();
    assert_eq!(samples.len(), 3);
    for (index, line) in samples.iter().enumerate() {
        let fields = line.split(',').collect::<Vec<_>>();
        assert_eq!(fields.len(), 14);
        assert_eq!(fields[3].parse::<f64>().unwrap(), [1.0, 0.5, 0.0][index]);
        assert_eq!(
            fields[10].parse::<f64>().unwrap(),
            [290.0, 350.0, 400.0][index]
        );
    }
}

#[test]
fn soa_current_envelope_csv_retains_voltage_limits_and_authored_curves() {
    use crate::results::safety::SoaCurrentEnvelopeEvidence;
    let curve = crate::results::safety::soa_current_envelope_test_fixture();
    let mut traces = Vec::new();
    for (name, unit, values) in [
        ("SOA_ID(M1)", "A", vec![0.001, 0.002, 0.003]),
        ("SOA_ID_CURVE_VOLTAGE(M1)", "V", vec![1.0, 5.0, 12.0]),
        (
            "SOA_ID_CURVE_LIMIT(M1)",
            "A",
            [1.0, 5.0, 12.0]
                .map(|v| curve.limit(v).unwrap().min(0.015))
                .to_vec(),
        ),
    ] {
        let mut trace = waveform(name, vec![0.0, 1e-9, 2e-9], values);
        trace.unit = Some(unit.into());
        traces.push(trace);
    }
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "Current curve SOA")
        .with_waveforms(traces)
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1e-9, 2e-9],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                envelope: Some(SoaCurrentEnvelopeEvidence {
                    maximum_current_a: 0.015,
                    curve,
                }),
                duration: None,
                thresholds: Default::default(),
                derating: None,
                device_id: "M1".into(),
                parameter: SoaParameterEvidence::DrainCurrent,
                limit_value: 0.0,
                worst_actual_value: 0.003,
                worst_time_s: 2e-9,
                sample_count: 3,
                unit: "A".into(),
                description: "Synthetic current curve".into(),
                verdict: SoaRuleVerdictEvidence::Critical,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".into(),
                parameter: SoaParameterEvidence::DrainCurrent,
                limit_value: 0.0,
                actual_value: 0.003,
                time_s: 2e-9,
                severity: SoaViolationSeverityEvidence::Critical,
            }],
        });
    analysis.validate_retained_evidence().unwrap();
    let csv = prepare_typed_result_csv(&analysis).unwrap().contents;
    assert!(
        csv.lines()
            .next()
            .unwrap()
            .ends_with("curve_voltage_v,curve_maximum_current_a,curve_definition_json")
    );
    assert!(csv.contains("Synthetic SOA fixture"));
    assert!(csv.contains("\"\"voltages_v\"\":[0.0,1.0,10.0]"));
    assert_eq!(csv.lines().filter(|l| l.starts_with("sample,")).count(), 3);
    assert!(
        csv.lines()
            .rfind(|l| l.starts_with("sample,"))
            .unwrap()
            .contains(&format!(",{:.17e},", 12.0))
    );
}

#[test]
fn soa_duration_csv_preserves_short_spikes_and_qualified_excursions() {
    check_soa_duration_csv(None);
}

#[test]
fn soa_duration_cumulative_csv_preserves_policy_and_exposure() {
    use crate::results::safety::SoaCumulativeDurationEvidence;
    check_soa_duration_csv(Some(SoaCumulativeDurationEvidence {
        recovery_time_s: None,
        peak_exposure_s: 4.5,
        final_exposure_s: 4.5,
    }));
    let peak = 3. + 1.5 * (-0.25_f64).exp();
    check_soa_duration_csv(Some(SoaCumulativeDurationEvidence {
        recovery_time_s: Some(3.),
        peak_exposure_s: peak,
        final_exposure_s: peak * (-0.5_f64 / 3.).exp(),
    }));
}

fn check_soa_duration_csv(
    cumulative: Option<crate::results::safety::SoaCumulativeDurationEvidence>,
) {
    let mut stress = waveform(
        "SOA_VDS(M1)",
        vec![0., 1., 2., 3., 4., 5., 6.],
        vec![0., 4., 0., 2., 2., 2., 0.],
    );
    stress.unit = Some("V".into());
    let duration = crate::results::safety::SoaDurationEvidence {
        cumulative,
        minimum_duration_s: if cumulative.is_some() { 4. } else { 2. },
        total_exceedance_s: 4.5,
        longest_excursion_s: 3.,
        qualified_excursions: 1,
        rejected_excursions: 1,
        clipped_excursions: 0,
    };
    let mut events = vec![];
    for (time_s, actual_value, severity) in [
        (1., 4., SoaViolationSeverityEvidence::Warning),
        (3., 2., SoaViolationSeverityEvidence::Critical),
        (4., 2., SoaViolationSeverityEvidence::Critical),
        (5., 2., SoaViolationSeverityEvidence::Critical),
    ] {
        events.push(SoaViolationEvidence {
            device_id: "M1".into(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 1.,
            actual_value,
            time_s,
            severity,
        });
    }
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA duration")
        .with_waveforms(vec![stress])
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: vec![0., 1., 2., 3., 4., 5., 6.],
        })
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: Some(duration),
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".into(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 1.,
                worst_actual_value: 2.,
                worst_time_s: 3.,
                sample_count: 7,
                unit: "V".into(),
                description: "Drain voltage".into(),
                verdict: SoaRuleVerdictEvidence::Critical,
            }],
            violations: events,
        });
    analysis.validate_retained_evidence().unwrap();
    let mut forged = analysis.clone();
    let Some(AnalysisResultPayload::Soa { violations, .. }) = &mut forged.result_payload else {
        unreachable!()
    };
    violations[0].severity = SoaViolationSeverityEvidence::Critical;
    assert!(forged.validate_retained_evidence().is_err());
    let mut state = state_with_typed_result(analysis);
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state, &io);
    let files = io.text_files.borrow();
    let rows: Vec<_> = files[0]
        .1
        .lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .collect();
    assert_eq!(rows.len(), 13);
    assert!(
        rows.iter()
            .all(|row| row.len() == if cumulative.is_some() { 20 } else { 16 })
    );
    assert_eq!(rows[0][10], "minimum_duration_s");
    assert_eq!(rows.iter().filter(|row| row[0] == "sample").count(), 7);
    assert!(
        rows.iter()
            .any(|row| row[0] == "sample" && row[4].parse::<f64>().unwrap() == 4.)
    );
    for row in rows.iter().skip(1) {
        assert_eq!(row[10].parse::<f64>().unwrap(), duration.minimum_duration_s);
        assert_eq!(row[11].parse::<f64>().unwrap(), 4.5);
        assert_eq!(row[12].parse::<f64>().unwrap(), 3.);
        if let Some(cumulative) = cumulative {
            assert_eq!(rows[0][14], "unqualified_excursions");
            assert_eq!(
                rows[0][16..],
                [
                    "duration_policy",
                    "recovery_time_s",
                    "peak_exposure_s",
                    "final_exposure_s"
                ]
            );
            assert_eq!(row[16], "cumulative");
            if let Some(tau) = cumulative.recovery_time_s {
                assert_eq!(row[17].parse::<f64>().unwrap(), tau);
            } else {
                assert_eq!(row[17], "off");
            }
            assert_eq!(row[18].parse::<f64>().unwrap(), cumulative.peak_exposure_s);
            assert_eq!(row[19].parse::<f64>().unwrap(), cumulative.final_exposure_s);
        }
    }
}
