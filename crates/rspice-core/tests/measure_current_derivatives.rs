//! Scalar and live measurements of generalized current primitives.
use rspice_core::analysis::measure::MeasureResult;
use rspice_core::analysis::measure_signals::{
    evaluate_tran_equation_measurements, evaluate_tran_measurements,
};
use rspice_core::analysis::transient::TransientResult;
use rspice_core::{CurrentImpulseDerivative, CurrentImpulseOwner, CurrentImpulseTrace, Netlist};

fn fixture(time: f64, order: u32, coefficient: f64) -> TransientResult {
    TransientResult {
        time: (0..=8).map(|j| j as f64 / 8.0).collect(),
        step_sizes: vec![0.125; 9],
        voltages: vec![vec![3.0; 9]],
        branch_currents: vec![vec![1.0; 9], vec![2.0; 9]],
        num_nodes: 1,
        node_names: vec!["out".into()],
        branch_names: vec!["V1".into(), "V2".into()],
        digital_traces: vec![],
        digital_buses: vec![],
        real_traces: vec![],
        device_op_traces: vec![],
        store_traces: vec![],
        fft_results: vec![],
        voltage_impulses: None,
        current_impulses: Some(vec![
            CurrentImpulseTrace {
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "V1".into(),
                },
                complete: true,
                points: vec![],
                derivatives: vec![CurrentImpulseDerivative {
                    time,
                    order,
                    coefficient,
                }],
            },
            CurrentImpulseTrace {
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "V2".into(),
                },
                complete: true,
                points: vec![],
                derivatives: vec![],
            },
        ]),
    }
}
fn deck(cards: &str) -> Netlist {
    Netlist::parse(&format!(
        "Current primitive measurements\nV1 out 0 0\nV2 aux 0 0\nR1 out 0 1k\n{cards}\n.end\n"
    ))
    .unwrap()
}
fn measured<'a>(results: &'a [MeasureResult], name: &str) -> &'a MeasureResult {
    results
        .iter()
        .find(|result| result.name.eq_ignore_ascii_case(name))
        .unwrap()
}
fn value(results: &[MeasureResult], name: &str, expected: f64) {
    let result = measured(results, name);
    assert!(result.passed, "{result:?}");
    assert!(
        (result.value.unwrap() - expected).abs() < 2e-14 * expected.abs().max(1.0),
        "{result:?} vs {expected}"
    );
}
fn singular(results: &[MeasureResult], name: &str) {
    let result = measured(results, name);
    assert!(!result.passed && result.value.is_none(), "{result:?}");
    assert!(
        result.error.as_ref().unwrap().contains("no finite scalar"),
        "{result:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn integral_derivatives_have_zero_interior_regular_part_at_every_order() {
    let netlist = deck(
        ".meas tran charge INTEG I(V1)\n.meas tran mean AVG I(V1)\n.meas tran weighted INTEG {3*I(V1)+V(out)}\n.meas tran before INTEG I(V1) TO=.25\n.meas tran after INTEG I(V1) FROM=.75",
    );
    for time in [0.3125, 0.5] {
        for order in [1, 2, 3, 128, u32::MAX] {
            for coefficient in [-1e300, 1e-300] {
                let results =
                    evaluate_tran_measurements(&netlist, &fixture(time, order, coefficient));
                value(&results, "charge", 1.0);
                value(&results, "mean", 1.0);
                value(&results, "weighted", 6.0);
                value(&results, "before", 0.25);
                value(&results, "after", 0.25);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn integral_boundaries_are_exact_and_regular_point_queries_remain_available() {
    let netlist = deck(
        ".meas tran charge INTEG I(V1)\n.meas tran stop INTEG I(V1) TO=.5\n.meas tran mean_stop AVG I(V1) TO=.5\n.meas tran start INTEG I(V1) FROM=.5\n.meas tran at_event FIND charge AT=.5\n.meas tran away FIND charge AT=.6\n.meas tran final PARAM {charge}",
    );
    let results = evaluate_tran_measurements(&netlist, &fixture(0.5, 2, -1e-20));
    for name in ["stop", "mean_stop", "start", "at_event"] {
        singular(&results, name);
    }
    value(&results, "charge", 1.0);
    value(&results, "away", 0.6);
    value(&results, "final", 1.0);
    assert_eq!(measured(&results, "away").event_axis, Some(0.6));
    assert_eq!(
        measured(&results, "charge").value_in_unit("mC").unwrap(),
        Some(1000.0)
    );
    assert!(
        (measured(&results, "away")
            .value_in_unit("mC")
            .unwrap()
            .unwrap()
            - 600.0)
            .abs()
            < 1e-11
    );
    let netlist = deck(
        ".meas tran charge INTEG I(V1)\n.meas tran off_grid FIND charge AT=.3125\n.meas tran nearby FIND charge AT=.312500000001",
    );
    let results = evaluate_tran_measurements(&netlist, &fixture(0.3125, 1, 1e-20));
    singular(&results, "off_grid");
    value(&results, "nearby", 0.312500000001);
    let netlist = deck(
        ".meas tran empty INTEG I(V1) FROM=.5 TO=.5\n.meas tran tiny INTEG {1e-100*I(V1)} TO=.5",
    );
    let results = evaluate_tran_measurements(&netlist, &fixture(0.5, 3, 1e-300));
    value(&results, "empty", 0.0);
    singular(&results, "tiny");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_primitive_live_validity_recovers_without_normalizing_singular_values() {
    let netlist = deck(
        ".meas tran before EQN {charge}\n.meas tran charge INTEG I(V1)\n.meas tran after EQN {charge+1}\n.meas tran lazy EQN {IF(TIME==.5,7,charge)}\n.meas tran held EQN {charge} TO=.5\n.meas tran independent AVG V(out)",
    );
    let result = fixture(0.5, 1, -1e-20);
    let results = evaluate_tran_measurements(&netlist, &result);
    value(&results, "before", 0.875);
    value(&results, "charge", 1.0);
    value(&results, "after", 2.0);
    value(&results, "lazy", 1.0);
    value(&results, "independent", 3.0);
    singular(&results, "held");
    let without_held = deck(
        ".meas tran before EQN {charge}\n.meas tran charge INTEG I(V1)\n.meas tran after EQN {charge+1}\n.meas tran lazy EQN {IF(TIME==.5,7,charge)}",
    );
    let traces = evaluate_tran_equation_measurements(&without_held, &result).unwrap();
    let trace = |name: &str| {
        traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case(name))
            .unwrap()
    };
    assert!(!trace("after").valid[4] && trace("after").values[4].is_nan());
    assert!(trace("after").valid[5]);
    assert!(!trace("before").valid[5] && trace("before").values[5].is_nan());
    assert!(trace("before").valid[6]);
    assert!(trace("lazy").valid.iter().all(|valid| *valid));
    assert_eq!(trace("lazy").values[4], 7.0);
    let unrelated_failure = deck(
        ".meas tran bad EQN {NO_SUCH_FN(2)}\n.meas tran charge INTEG I(V1)\n.meas tran good AVG V(out)",
    );
    let measured_results = evaluate_tran_measurements(&unrelated_failure, &result);
    assert!(!measured(&measured_results, "bad").passed);
    value(&measured_results, "charge", 1.0);
    value(&measured_results, "good", 3.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_primitive_boundaries_cancel_across_owners_without_range_loss() {
    for order in [1, 4, u32::MAX] {
        let mut result = fixture(0.5, order, 1e300);
        result.current_impulses.as_mut().unwrap()[1].derivatives = vec![CurrentImpulseDerivative {
            time: 0.5,
            order,
            coefficient: -5e299,
        }];
        let netlist = deck(
            ".meas tran cancelled INTEG {1e100*I(V1)+2e100*I(V2)} TO=.5\n.meas tran cancelled_start AVG {I(V1)+2*I(V2)} FROM=.5\n.meas tran nonzero INTEG {I(V1)+I(V2)} TO=.5",
        );
        let results = evaluate_tran_measurements(&netlist, &result);
        value(&results, "cancelled", 2.5e100);
        value(&results, "cancelled_start", 5.0);
        singular(&results, "nonzero");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_primitives_keep_charge_and_derivative_parts_separate() {
    let mut result = fixture(0.5, 3, -1e-20);
    result.current_impulses.as_mut().unwrap()[0].points = vec![
        rspice_core::CurrentImpulsePoint {
            time: 0.0,
            charge_coulombs: 99.0,
        },
        rspice_core::CurrentImpulsePoint {
            time: 0.5,
            charge_coulombs: 0.25,
        },
        rspice_core::CurrentImpulsePoint {
            time: 1.0,
            charge_coulombs: -0.125,
        },
    ];
    let netlist = deck(
        ".meas tran charge INTEG I(V1)\n.meas tran mean AVG I(V1)\n.meas tran stop INTEG I(V1) TO=.5\n.meas tran after INTEG I(V1) FROM=.625\n.meas tran resumed AVG I(V1) FROM=.625",
    );
    let results = evaluate_tran_measurements(&netlist, &result);
    value(&results, "charge", 1.125);
    value(&results, "mean", 1.125);
    singular(&results, "stop");
    value(&results, "after", 0.25);
    value(&results, "resumed", 2.0 / 3.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn interval_consumers_never_drop_an_off_grid_primitive() {
    let netlist = deck(
        ".meas tran charge INTEG I(V1)\n.meas tran alias EQN {charge}\n.meas tran nested INTEG charge\n.meas tran nested_alias AVG alias\n.meas tran peak MAX charge\n.meas tran rms RMS charge\n.meas tran before AVG charge TO=.25\n.meas tran alias_event FIND alias AT=.3125\n.meas tran unused EQN {IF(0,charge,7)}\n.meas tran unused_avg AVG unused",
    );
    let results = evaluate_tran_measurements(&netlist, &fixture(0.3125, 1, -2.0));
    value(&results, "charge", 1.0);
    value(&results, "before", 0.125);
    value(&results, "unused_avg", 7.0);
    singular(&results, "alias_event");
    for name in ["nested", "nested_alias", "peak", "rms"] {
        let result = measured(&results, name);
        assert!(!result.passed && result.value.is_none(), "{result:?}");
        assert!(
            result
                .error
                .as_ref()
                .unwrap()
                .contains("generalized interval-operator composition"),
            "{result:?}"
        );
    }
}
