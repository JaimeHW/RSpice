//! Table-driven frequency analyses: physical row identity and bounded execution.
use rspice_core::{
    Engine, Netlist, ResourceKind, ResourceLimits, SimulationConfig, SimulationError,
};

const DECK: &str = "Frequency table\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance}\nC1 out 0 1u\n.data points FREQ resistance\n10 1k\n100 2k\n.enddata\n.end\n";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn borrowed_preflight_checks_exact_admission_limits_without_solving_rows() {
    let netlist = Netlist::parse(DECK).unwrap();
    let request = rspice_core::netlist::AnalysisCommand::AcData {
        table_name: "POINTS".into(),
    };
    let mut limits = ResourceLimits::default();
    limits.max_analysis_points = 2;
    limits.max_batch_runs = 2;
    limits.max_result_values = 4;
    Engine::validate_frequency_data_request_with_abort(
        &netlist,
        &request,
        limits,
        &rspice_core::NoAbort,
    )
    .unwrap();
    // These exact limits admit the four coordinates, without allocating or
    // charging for row solutions that checking does not produce.
    for resource in [
        ResourceKind::AnalysisPoints,
        ResourceKind::BatchRuns,
        ResourceKind::ResultValues,
    ] {
        let mut bounded = limits;
        let (requested, limit) = match resource {
            ResourceKind::AnalysisPoints => {
                bounded.max_analysis_points = 1;
                (2, 1)
            }
            ResourceKind::BatchRuns => {
                bounded.max_batch_runs = 1;
                (2, 1)
            }
            _ => {
                bounded.max_result_values = 3;
                (4, 3)
            }
        };
        let error = Engine::validate_frequency_data_request_with_abort(
            &netlist,
            &request,
            bounded,
            &rspice_core::NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::ResourceLimit(found) if found.resource == resource && found.requested == requested && found.limit == limit)
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn borrowed_preflight_observes_cancellation_during_row_validation() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct CancelAfter(AtomicUsize);
    impl rspice_core::AbortSignal for CancelAfter {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, Ordering::Relaxed) >= 2
        }
    }
    let netlist = Netlist::parse(DECK).unwrap();
    let request = rspice_core::netlist::AnalysisCommand::AcData {
        table_name: "points".into(),
    };
    assert!(matches!(
        Engine::validate_frequency_data_request_with_abort(
            &netlist,
            &request,
            ResourceLimits::default(),
            &CancelAfter(AtomicUsize::new(0))
        ),
        Err(SimulationError::Aborted)
    ));
}

fn engine(limits: ResourceLimits) -> Engine {
    Engine::new(SimulationConfig {
        resource_limits: limits,
        spice_dialect: rspice_core::SpiceDialect::BestAvailable,
        ..SimulationConfig::default()
    })
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-30,
        "{actual:e} != {expected:e}"
    );
}

fn assert_limit(error: SimulationError, kind: ResourceKind, limit: usize) {
    let SimulationError::ResourceLimit(error) = error else {
        panic!("expected {kind} limit, got {error}")
    };
    assert_eq!(error.resource, kind);
    assert_eq!(error.limit, limit);
    assert!(error.requested > limit);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn compact_coordinates_and_row_solutions_match_independent_rc_equations() {
    use rspice_core::engine::FrequencyDataTarget;
    let netlist = Netlist::parse("RC table\n.param resistance=1k\n.options temp=27\nV1 in 0 AC 1\nR1 in out {resistance}\nC1 out 0 1u\n.data MixedRows resistance HERTZ C1:CAP TEMP\n1k 100 1u 27\n2k 10 2u 127\n3k 100 3u -73\n.enddata\n.end\n").unwrap();
    let engine = engine(ResourceLimits::default());
    let ac = engine.run_ac_table(&netlist, "mixedrows").unwrap();
    let noise = engine
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "mixedrows", 999.0)
        .unwrap();
    assert_eq!(ac.table_name, "MixedRows");
    assert_eq!(ac.requested_rows, 3);
    assert!(ac.finish.is_none());
    assert_eq!(ac.columns, noise.columns);
    assert_eq!(
        ac.columns
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>(),
        ["resistance", "HERTZ", "C1:CAP", "TEMP"]
    );
    assert_eq!(
        ac.columns[0].target,
        FrequencyDataTarget::Parameter("RESISTANCE".into())
    );
    assert_eq!(ac.columns[1].target, FrequencyDataTarget::Frequency);
    assert_eq!(
        ac.columns[2].target,
        FrequencyDataTarget::DeviceParameter {
            device_name: "C1".into(),
            parameter_name: "C".into(),
        }
    );
    assert_eq!(ac.columns[1].values, [100.0, 10.0, 100.0]);
    assert_eq!(ac.columns[0].values, [1000.0, 2000.0, 3000.0]);
    assert_eq!(ac.columns[2].values, [1e-6, 2e-6, 3e-6]);
    assert_eq!(ac.columns[3].values, [27.0, 127.0, -73.0]);
    for (index, (a, n)) in ac.points.iter().zip(&noise.points).enumerate() {
        let resistance = ac.columns[0].values[index];
        let frequency = ac.columns[1].values[index];
        let capacitance = ac.columns[2].values[index];
        let temperature = ac.columns[3].values[index] + 273.15;
        let wrc = std::f64::consts::TAU * frequency * resistance * capacitance;
        let out = a
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        close(a.voltages[out].re, 1.0 / (1.0 + wrc * wrc));
        close(a.voltages[out].im, -wrc / (1.0 + wrc * wrc));
        close(
            n.input_referred_density,
            4.0 * 1.380649e-23 * temperature * resistance,
        );
        close(
            n.output_noise_density,
            n.input_referred_density / (1.0 + wrc * wrc),
        );
        assert_eq!(a.frequency, frequency);
        assert_eq!(n.frequency, frequency);
    }
    let (rows, legacy) = engine.run_ac_data(&netlist, "MixedRows").unwrap();
    assert_eq!(format!("{legacy:?}"), format!("{:?}", ac.points));
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(
            row.params.get("resistance"),
            Some(ac.columns[0].values[index])
        );
        assert_eq!(row.options.temp, Some(ac.columns[3].values[index]));
    }
    assert_eq!(netlist.params.get("resistance"), Some(1000.0));
    assert_eq!(netlist.options.temp, Some(27.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn declared_parameter_target_wins_over_an_identically_named_device() {
    use rspice_core::engine::FrequencyDataTarget;
    let netlist = Netlist::parse("Parameter precedence\n.param R1=1k\nI1 out 0 AC 1\nR1 out 0 {R1}\n.data points FREQ r1\n100 2k\n10 3k\n.enddata\n.end\n").unwrap();
    let result = Engine::default().run_ac_table(&netlist, "points").unwrap();
    assert_eq!(
        result.columns[1].target,
        FrequencyDataTarget::Parameter("R1".into())
    );
    close(result.points[0].voltages[0].re, -2000.0);
    close(result.points[1].voltages[0].re, -3000.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn numeric_budget_covers_coordinates_and_every_row_in_both_apis() {
    let netlist = Netlist::parse(DECK).unwrap();
    let ordinary = engine(ResourceLimits::default());
    let ac = ordinary.run_ac_table(&netlist, "points").unwrap();
    let noise = ordinary
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
        .unwrap();
    for (is_noise, required) in [
        (false, ac.retained_value_count()),
        (true, noise.retained_value_count()),
    ] {
        for (limit, succeeds) in [
            (required, true),
            (required - 1, false),
            (3, false),
            (4, false),
        ] {
            let mut limits = ResourceLimits::default();
            limits.max_result_values = limit;
            let bounded = engine(limits);
            for legacy in [false, true] {
                let result = match (is_noise, legacy) {
                    (false, false) => bounded
                        .run_ac_table(&netlist, "points")
                        .map(|r| r.points.len()),
                    (false, true) => bounded.run_ac_data(&netlist, "points").map(|r| r.1.len()),
                    (true, false) => bounded
                        .run_noise_table_named_with_input_source(
                            &netlist, "out", None, "V1", "points", 300.15,
                        )
                        .map(|r| r.points.len()),
                    (true, true) => bounded
                        .run_noise_data_named_with_input_source(
                            &netlist, "out", None, "V1", "points", 300.15,
                        )
                        .map(|r| r.1.len()),
                };
                if succeeds {
                    assert_eq!(result.unwrap(), 2);
                } else {
                    assert_limit(result.unwrap_err(), ResourceKind::ResultValues, limit);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn compatibility_netlists_have_an_aggregate_source_budget() {
    let netlist = Netlist::parse(DECK).unwrap();
    // Retained root text plus typed FREQ and RESISTANCE bindings in each row.
    let one_row = DECK.len() + "FREQ".len() + "RESISTANCE".len() + 2 * size_of::<f64>();
    for limit in [one_row, one_row * 2 - 1, one_row * 2] {
        let mut limits = ResourceLimits::default();
        limits.max_expanded_source_bytes = limit;
        let bounded = engine(limits);
        assert_eq!(
            bounded
                .run_ac_table(&netlist, "points")
                .unwrap()
                .points
                .len(),
            2
        );
        assert_eq!(
            bounded
                .run_noise_table_named_with_input_source(
                    &netlist, "out", None, "V1", "points", 300.15
                )
                .unwrap()
                .points
                .len(),
            2
        );
        for result in [
            bounded.run_ac_data(&netlist, "points").map(|r| r.1.len()),
            bounded
                .run_noise_data_named_with_input_source(
                    &netlist, "out", None, "V1", "points", 300.15,
                )
                .map(|r| r.1.len()),
        ] {
            if limit == one_row * 2 {
                assert_eq!(result.unwrap(), 2);
            } else {
                assert_limit(
                    result.unwrap_err(),
                    ResourceKind::ExpandedSourceBytes,
                    limit,
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn row_counts_and_all_table_values_are_checked_before_solving() {
    let netlist = Netlist::parse(DECK).unwrap();
    for kind in [ResourceKind::AnalysisPoints, ResourceKind::BatchRuns] {
        let mut limits = ResourceLimits::default();
        if kind == ResourceKind::AnalysisPoints {
            limits.max_analysis_points = 1;
        } else {
            limits.max_batch_runs = 1;
        }
        let bounded = engine(limits);
        assert_limit(
            bounded.run_ac_table(&netlist, "points").unwrap_err(),
            kind,
            1,
        );
        assert_limit(
            bounded
                .run_noise_table_named_with_input_source(
                    &netlist, "out", None, "V1", "points", 300.15,
                )
                .unwrap_err(),
            kind,
            1,
        );
    }
    let engine = engine(ResourceLimits::default());
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut modified = netlist.clone();
        modified.data_tables[0].rows[1][1] = invalid;
        assert!(
            engine
                .run_ac_table(&modified, "points")
                .unwrap_err()
                .to_string()
                .contains("must be finite")
        );
    }
    let mut zero = netlist.clone();
    zero.data_tables[0].rows[1][0] = 0.0;
    assert_eq!(
        engine.run_ac_table(&zero, "points").unwrap().points[1].frequency,
        0.0
    );
    assert!(
        engine
            .run_noise_table_named_with_input_source(&zero, "out", None, "V1", "points", 300.15)
            .unwrap_err()
            .to_string()
            .contains("strictly positive")
    );
    let mut malformed = netlist.clone();
    malformed.data_tables[0].rows[1].pop();
    assert!(
        engine
            .run_ac_table(&malformed, "points")
            .unwrap_err()
            .to_string()
            .contains("row 2")
    );
    for (column, message) in [
        ("absent", "does not resolve"),
        ("R1:bogus", "Unsupported resistor"),
        ("R1::R", "invalid device-parameter target"),
    ] {
        let mut modified = netlist.clone();
        modified.data_tables[0].params[1] = column.into();
        let error = engine.run_ac_table(&modified, "points").unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cancellation_at_admission_mid_run_and_publication_returns_no_partial_result() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct CancelAfter {
        polls: AtomicUsize,
        limit: usize,
    }
    impl rspice_core::AbortSignal for CancelAfter {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
        }
    }
    let netlist = Netlist::parse(DECK).unwrap();
    let engine = engine(ResourceLimits::default());
    for noise in [false, true] {
        let run = |abort: &dyn rspice_core::AbortSignal| {
            if noise {
                engine
                    .run_noise_table_named_with_input_source_and_abort(
                        &netlist, "out", None, "V1", "points", 300.15, abort,
                    )
                    .map(|r| r.points.len())
            } else {
                engine
                    .run_ac_table_with_abort(&netlist, "points", abort)
                    .map(|r| r.points.len())
            }
        };
        let count = CancelAfter {
            polls: AtomicUsize::new(0),
            limit: usize::MAX,
        };
        assert_eq!(run(&count).unwrap(), 2);
        let total = count.polls.load(Ordering::Relaxed);
        assert!(total > 20);
        for limit in [0, total / 2, total - 1] {
            let cancel = CancelAfter {
                polls: AtomicUsize::new(0),
                limit,
            };
            assert!(matches!(run(&cancel), Err(SimulationError::Aborted)));
            assert_eq!(cancel.polls.load(Ordering::Relaxed), limit + 1);
        }
        assert_eq!(run(&rspice_core::NoAbort).unwrap(), 2);
    }
    assert_eq!(netlist.params.get("resistance"), Some(1000.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn row_replay_honors_the_callers_source_limit_for_ac_and_noise() {
    let netlist = Netlist::parse(DECK).unwrap();
    // Parsing already succeeded. Replaying even one row must honor the new
    // engine policy rather than silently reverting to the default parser budget.
    let limit = DECK.len() - 1;
    let mut limits = ResourceLimits::default();
    limits.max_netlist_bytes = limit;
    let bounded = engine(limits);
    assert_limit(
        bounded.run_ac_data(&netlist, "points").unwrap_err(),
        ResourceKind::NetlistBytes,
        limit,
    );
    assert_limit(
        bounded
            .run_noise_data_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
            .unwrap_err(),
        ResourceKind::NetlistBytes,
        limit,
    );
    let ordinary = engine(ResourceLimits::default());
    assert_eq!(ordinary.run_ac_data(&netlist, "points").unwrap().1.len(), 2);
    assert_eq!(
        ordinary
            .run_noise_data_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
            .unwrap()
            .1
            .len(),
        2,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn table_measurement_adapters_share_resolved_complex_parameter_environments() {
    use rspice_core::NoAbort;
    use rspice_core::analysis::*;
    let mut source = String::from(
        "Resolved rows\n.param P=1 Q={2*P} C={P*1j} AC_P={P} NOISE_P={P}\n.func row_value(x) {x+Q+imag(C)}\nV1 out 0 DC 1 AC 1\nR1 out 0 {P*1k}\n.data points FREQ P\n10 3\n20 5\n10 7\n.enddata\n",
    );
    for family in ["AC", "NOISE"] {
        source.push_str(&format!(".meas {family} {family}_P FIND {{Q}} AT=20\n.meas {family} {family}_post PARAM='{family}_P+row_value(2)'\n.meas {family} {family}_equation EQN {{row_value(2)}}\n.meas {family} {family}_maximum MAX {{imag(C)}}\n.meas {family}_CONT {family}_event FIND {{row_value(2)}} AT=20\n"));
    }
    source.push_str(".end\n");
    let netlist = Netlist::parse(&source).unwrap();
    let engine = Engine::default();
    let ac = engine.run_ac_table(&netlist, "points").unwrap();
    let noise = engine
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
        .unwrap();
    for measures in [
        evaluate_ac_table_measurements_with_abort(&netlist, &ac, &NoAbort).unwrap(),
        evaluate_noise_table_measurements_with_abort(&netlist, &noise, &NoAbort).unwrap(),
    ] {
        assert_eq!(
            measures
                .iter()
                .map(|measure| measure.value)
                .collect::<Vec<_>>(),
            [Some(10.0), Some(33.0), Some(23.0), Some(7.0)]
        );
        assert!(
            measures.iter().all(|measure| measure.passed),
            "{measures:?}"
        );
    }
    for events in [
        evaluate_ac_table_continuous_measurements_with_limits_and_abort(
            &netlist,
            &ac,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
        evaluate_noise_table_continuous_measurements_with_limits_and_abort(
            &netlist,
            &noise,
            &ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap(),
    ] {
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].records.len(), 1);
        assert_eq!(events[0].records[0].value, 17.0);
    }
    let abort = rspice_core::abort_signal::CountingAbort::new(0);
    assert!(matches!(
        evaluate_ac_table_measurements_with_abort(&netlist, &ac, &abort),
        Err(SimulationError::Aborted)
    ));
}
