//! Probe identity must survive every spelling accepted by authoring.
//! These circuits use distinct internal voltages and branch currents, so a
//! missing or incorrectly bound signal cannot pass as a physical zero.

use super::fixtures::{output, run};
use super::*;
use crate::state::{OutputSelectionMode, SimulationRun};

pub(super) const HIERARCHY: &str = "Scoped probes\nV1 in 0 DC 4 AC 1\nV2 bias 0 DC -1 AC 0\nX1 in bias divider\nX2 bias in divider\n.subckt divider a b\nR1 a mid 1k\nR2 mid b 1k\nVprobe sensed mid 0\nR3 sensed b 1k\n.ends\n.end\n";

pub(super) fn dc() -> AnalysisSpec {
    AnalysisSpec::DcSweep {
        source_name: "V1".to_owned(),
        start: 1.0,
        stop: 3.0,
        step: 1.0,
        source2: Some("V2".to_owned()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: Some(1.0),
        hysteresis: false,
    }
}

pub(super) fn ac() -> AnalysisSpec {
    AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 10.0,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
    }
}

pub(super) fn execute(deck: &str, spec: AnalysisSpec, outputs: &[SavedOutput]) -> SimulationRun {
    let line = match spec {
        AnalysisSpec::DcOp { .. } => ".op",
        AnalysisSpec::Ac { .. } => ".ac lin 3 1 10",
        AnalysisSpec::Transient { .. } => ".tran 1u 10u",
        _ => ".dc V1 1 3 1 V2 0 1 1",
    };
    run(
        deck,
        "Probe bindings",
        spec,
        line,
        outputs,
        OutputSelectionMode::ExplicitOnly,
    )
}

pub(super) fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

pub(super) fn check_value(run: &SimulationRun, index: usize, expected: f64) {
    let analysis = &run.analyses[0];
    let receipt = &analysis.saved_output_receipts[index];
    let names = receipt.status.materialized_waveforms().collect::<Vec<_>>();
    assert_eq!(
        names.len(),
        1,
        "{}: {:?}",
        receipt.source_expression,
        receipt.status
    );
    let waveform = analysis
        .waveforms
        .iter()
        .find(|w| w.name == names[0].0)
        .unwrap();
    assert!(!waveform.y.is_empty());
    for &value in waveform.y.iter() {
        close(value, expected);
    }
}

fn expected(quantity: usize, source: f64, bias: f64) -> f64 {
    match quantity {
        0 => (source + 2.0 * bias) / 3.0,
        1 => (bias + 2.0 * source) / 3.0,
        2 => (bias - source) / 3000.0,
        3 => (source - bias) / 3.0,
        _ => unreachable!(),
    }
}

fn scoped_outputs(spec: AnalysisSpec, derived: bool) {
    let nested = matches!(spec, AnalysisSpec::DcSweep { .. });
    let complex = matches!(spec, AnalysisSpec::Ac { .. });
    let members = if nested { 2 } else { 1 };
    let mut outputs = Vec::new();
    for (first, second, branch) in [
        ("X1.mid", "X2.mid", "X1.Vprobe"),
        ("X1:mid", "X2:mid", "X1:Vprobe"),
        ("/X1/mid", "/X2/mid", "/X1/Vprobe"),
        ("/top/X1/mid", "/top/X2/mid", "/top/X1/Vprobe"),
    ] {
        let expressions = [
            format!("V({first})"),
            format!("V({second})"),
            format!("I({branch})"),
            if derived {
                format!("V({first}) - V(bias)")
            } else {
                format!("V({first},bias)")
            },
        ];
        for expression in expressions {
            outputs.push(output(
                if derived {
                    SavedOutputKind::DerivedExpression
                } else {
                    SavedOutputKind::RawVoltageOrCurrent
                },
                &format!("Bound {}", outputs.len()),
                &expression,
            ));
        }
    }
    let run = execute(HIERARCHY, spec, &outputs);
    run.validate_provenance().unwrap();
    assert!(run.success);
    let analysis = &run.analyses[0];
    assert_eq!(analysis.saved_output_receipts.len(), outputs.len());
    let mut missing = Vec::new();
    for (index, receipt) in analysis.saved_output_receipts.iter().enumerate() {
        let names = receipt.status.materialized_waveforms().collect::<Vec<_>>();
        if names.len() != members {
            missing.push(format!(
                "{}: {:?}",
                receipt.source_expression, receipt.status
            ));
            continue;
        }
        for (member, (name, _)) in names.iter().enumerate() {
            let wave = analysis
                .waveforms
                .iter()
                .find(|wave| wave.name == *name)
                .unwrap();
            if !derived {
                assert_eq!(
                    wave.unit.as_deref(),
                    Some(if index % 4 == 2 { "A" } else { "V" })
                );
            }
            for (point, (&x, &y)) in wave.x.iter().zip(wave.y.iter()).enumerate() {
                let source = if nested {
                    x
                } else if complex {
                    1.0
                } else {
                    4.0
                };
                let bias = if nested {
                    member as f64
                } else if complex {
                    0.0
                } else {
                    -1.0
                };
                let value = expected(index % 4, source, bias);
                close(y, if complex { value.abs() } else { value });
                if complex {
                    let phase = wave.complex.as_ref().unwrap();
                    close(phase.real[point], value);
                    close(phase.imag[point], 0.0);
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "accepted scoped probes did not bind: {missing:#?}"
    );
}

#[test]
fn scoped_raw_operating_point_probes_resolve_all_authored_spellings() {
    scoped_outputs(AnalysisSpec::dc_op(), false);
}

#[test]
fn scoped_raw_dc_probes_resolve_every_member() {
    scoped_outputs(dc(), false);
}

#[test]
fn scoped_raw_ac_probes_preserve_the_bound_phasor() {
    scoped_outputs(ac(), false);
}

#[test]
fn scoped_transient_probes_resolve_raw_and_calculator_outputs() {
    for derived in [false, true] {
        scoped_outputs(
            AnalysisSpec::Transient {
                stop_time: 1e-5,
                step_time: 1e-6,
                start_time: 0.0,
                max_timestep: Some(1e-6),
                uic: false,
            },
            derived,
        );
    }
}

#[test]
fn scoped_calculator_operating_point_probes_resolve_all_authored_spellings() {
    scoped_outputs(AnalysisSpec::dc_op(), true);
}

#[test]
fn scoped_calculator_dc_probes_resolve_every_member() {
    scoped_outputs(dc(), true);
}

#[test]
fn root_scope_spellings_resolve_the_same_voltage() {
    let outputs = ["V(in)", "V(/in)", "V(/top/in)"]
        .map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(HIERARCHY, AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    for index in 0..outputs.len() {
        check_value(&run, index, 4.0);
    }
}

#[test]
fn a_ground_like_ordinary_node_keeps_its_solved_value() {
    let deck = "Ordinary ground-like name\nV1 GROUND 0 7\nR1 GROUND 0 1k\n.end\n";
    assert!(
        !rspice_core::Netlist::parse(deck)
            .unwrap()
            .ground_policy()
            .is_ground("GROUND")
    );
    let run = execute(
        deck,
        AnalysisSpec::dc_op(),
        &[output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Ordinary",
            "V(GROUND)",
        )],
    );
    check_value(&run, 0, 7.0);
}

#[test]
fn calculator_probe_arguments_keep_literal_node_punctuation_and_large_numeric_names() {
    let nodes = ["out+", "out-", "n$bias", "n#1", "1e999", "00", "-1"];
    let mut deck = "Literal probe nodes\n".to_owned();
    for (index, node) in nodes.iter().enumerate() {
        deck.push_str(&format!(
            "V{} {node} 0 {}\nR{} {node} 0 1k\n",
            index + 1,
            index + 1,
            index + 1
        ));
    }
    deck.push_str(".end\n");
    let raw = nodes.map(|node| {
        output(
            SavedOutputKind::RawVoltageOrCurrent,
            node,
            &format!("V({node})"),
        )
    });
    let control = execute(&deck, AnalysisSpec::dc_op(), &raw);
    for index in 0..nodes.len() {
        check_value(&control, index, (index + 1) as f64);
    }
    let derived = nodes.map(|node| {
        output(
            SavedOutputKind::DerivedExpression,
            node,
            &format!("2 * V({node})"),
        )
    });
    let run = execute(&deck, AnalysisSpec::dc_op(), &derived);
    for index in 0..nodes.len() {
        check_value(&run, index, 2.0 * (index + 1) as f64);
    }
}

#[test]
fn literal_dotted_node_names_and_absent_scopes_do_not_become_ground_or_other_nodes() {
    let deck = "Literal dotted nodes\nV1 0.0 0 5\nV2 1 0 8\nR1 0.0 0 1k\nR2 1 0 1k\n.end\n";
    let outputs = ["V(0.0)", "V(1)", "V(/missing/1)"]
        .map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(deck, AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    check_value(&run, 0, 5.0);
    check_value(&run, 1, 8.0);
    assert!(matches!(
        run.analyses[0].saved_output_receipts[2].status,
        SavedOutputMaterializationStatus::Unavailable { .. }
    ));
}

#[test]
fn literal_slash_nodes_take_precedence_over_hierarchy_aliases() {
    let deck = HIERARCHY.replace(
        ".end\n",
        "V3 /X1/mid 0 DC 9 AC 2\nRliteral /X1/mid 0 1k\n.end\n",
    );
    for spec in [AnalysisSpec::dc_op(), dc(), ac()] {
        let complex = matches!(spec, AnalysisSpec::Ac { .. });
        let nested = matches!(spec, AnalysisSpec::DcSweep { .. });
        for kind in [
            SavedOutputKind::RawVoltageOrCurrent,
            SavedOutputKind::DerivedExpression,
        ] {
            let outputs = ["V(/X1/mid)", "V(X1.mid)"].map(|expr| output(kind, expr, expr));
            let run = execute(&deck, spec.clone(), &outputs);
            run.validate_provenance().unwrap();
            for (index, receipt) in run.analyses[0].saved_output_receipts.iter().enumerate() {
                let names = receipt.status.materialized_waveforms().collect::<Vec<_>>();
                assert_eq!(
                    names.len(),
                    if nested { 2 } else { 1 },
                    "{:?}",
                    receipt.status
                );
                for (member, (name, _)) in names.iter().enumerate() {
                    let wave = run.analyses[0]
                        .waveforms
                        .iter()
                        .find(|w| w.name == *name)
                        .unwrap();
                    for (&x, &value) in wave.x.iter().zip(wave.y.iter()) {
                        let expected = if index == 0 {
                            if complex { 2.0 } else { 9.0 }
                        } else if nested {
                            expected(0, x, member as f64)
                        } else {
                            expected(
                                0,
                                if complex { 1.0 } else { 4.0 },
                                if complex { 0.0 } else { -1.0 },
                            )
                        };
                        close(value, expected);
                    }
                }
            }
        }
    }
}

#[test]
fn scoped_deferred_probes_resolve_after_project_reload() {
    for spec in [AnalysisSpec::dc_op(), dc(), ac()] {
        let complex = matches!(spec, AnalysisSpec::Ac { .. });
        let nested = matches!(spec, AnalysisSpec::DcSweep { .. });
        let outputs = ["V(/top/X1/mid,bias)", "I(X1:Vprobe)"].map(|expr| {
            let mut output = output(SavedOutputKind::RawVoltageOrCurrent, expr, expr);
            output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
            output
        });
        let run = execute(HIERARCHY, spec, &outputs);
        run.validate_provenance().unwrap();
        let mut state = crate::state::SimulationState::default();
        state.next_run_id = run.id;
        state.runs = vec![run].into();
        let snapshot = crate::io::project_io::ProjectSimulationResults::from_state(&state);
        snapshot.validate().unwrap();
        let restored: crate::io::project_io::ProjectSimulationResults =
            serde_json::from_slice(&serde_json::to_vec(&snapshot).unwrap()).unwrap();
        restored.apply_to_state(&mut state).unwrap();
        let mut analysis = state.runs[0].analyses[0].clone();
        for index in 0..outputs.len() {
            assert_eq!(
                analysis.saved_output_receipts[index].status,
                SavedOutputMaterializationStatus::Deferred
            );
            materialize_deferred_saved_output(&mut analysis, index).unwrap();
            analysis.validate_retained_evidence().unwrap();
            let names = analysis.saved_output_receipts[index]
                .status
                .materialized_waveforms()
                .collect::<Vec<_>>();
            assert_eq!(names.len(), if nested { 2 } else { 1 });
            for (member, (name, _)) in names.iter().enumerate() {
                let wave = analysis.waveforms.iter().find(|w| w.name == *name).unwrap();
                assert_eq!(
                    wave.unit.as_deref(),
                    Some(if index == 0 { "V" } else { "A" })
                );
                for (point, (&x, &y)) in wave.x.iter().zip(wave.y.iter()).enumerate() {
                    let value = expected(
                        if index == 0 { 3 } else { 2 },
                        if nested {
                            x
                        } else if complex {
                            1.0
                        } else {
                            4.0
                        },
                        if nested {
                            member as f64
                        } else if complex {
                            0.0
                        } else {
                            -1.0
                        },
                    );
                    close(y, if complex { value.abs() } else { value });
                    if complex {
                        let phasor = wave.complex.as_ref().unwrap();
                        close(phasor.real[point], value);
                        close(phasor.imag[point], 0.0);
                    }
                }
            }
        }
    }
}

#[test]
fn raw_scope_lookup_preserves_quantity_namespaces_and_does_not_rewrite_retained_names() {
    let waveforms = [
        WaveformData::new("V(X1.out)", vec![0.0], vec![2.0], "#ffffff").with_unit("V"),
        WaveformData::new("I(X1.out)", vec![0.0], vec![3.0], "#ffffff").with_unit("A"),
    ];
    for (expr, expected) in [("V(/X1/out)", 2.0), ("I(X1:out)", 3.0)] {
        let resolved = resolve_raw_probe(expr, &waveforms, "result", false).unwrap();
        assert_eq!(resolved.y.as_slice(), &[expected]);
    }
    assert!(resolve_raw_probe("I(/X1/out)", &waveforms[..1], "result", false).is_err());
    assert!(resolve_raw_probe("V(/X1/out)", &waveforms[1..], "result", false).is_err());
    let mut literal = waveforms[0].clone();
    literal.name = "V(X1:out)".to_owned();
    assert!(resolve_raw_probe("V(/X1/out)", &[literal], "result", false).is_err());
}

#[test]
fn deferred_op_outputs_use_physical_tables_after_renaming_and_project_reload() {
    use crate::io::project_io::ProjectSimulationResults;
    use crate::state::SimulationState;

    // V1 is both a voltage-source instance and a distinct 7 V circuit node.
    let deck = "OP identity\nV1 in 0 4\nR1 in 0 1k\nV2 V1 0 7\nR2 V1 0 1k\n.end\n";
    for (alias, source, query, expected) in [
        ("V(absent)", "V(in)", "V(absent)", None),
        ("V(absent)", "V(in)", "V(/top/absent)", None),
        ("I(absent)", "I(V1)", "I(absent)", None),
        ("I(absent)", "I(V1)", "I(/top/absent)", None),
        ("absent", "V(in)", "V(absent)", None),
        ("V(V1)", "V(in)", "V(V1)", Some(7.0)),
        ("I(V1)", "V(in)", "I(V1)", Some(-0.004)),
    ] {
        for kind in [
            SavedOutputKind::RawVoltageOrCurrent,
            SavedOutputKind::DerivedExpression,
        ] {
            for selection in [
                OutputSelectionMode::ExplicitOnly,
                OutputSelectionMode::SaveAll,
            ] {
                let alias = output(kind, alias, source);
                let mut deferred = output(kind, "Deferred physical signal", query);
                deferred.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                let run = run(
                    deck,
                    "OP identity",
                    AnalysisSpec::dc_op(),
                    ".op",
                    &[alias, deferred],
                    selection,
                );
                run.validate_provenance().unwrap();
                check_value(&run, 0, if source.starts_with('I') { -0.004 } else { 4.0 });
                for reload in [false, true] {
                    let mut state = SimulationState::default();
                    state.next_run_id = run.id;
                    state.runs = vec![run.clone()].into();
                    if reload {
                        let snapshot = ProjectSimulationResults::from_state(&state);
                        snapshot.validate().unwrap();
                        let restored: ProjectSimulationResults =
                            serde_json::from_slice(&serde_json::to_vec(&snapshot).unwrap())
                                .unwrap();
                        restored.apply_to_state(&mut state).unwrap();
                    }
                    assert!(state.select_run(0));
                    let before =
                        serde_json::to_vec(&ProjectSimulationResults::from_state(&state)).unwrap();
                    let visible_before = state.waveforms.clone();
                    let version_before = state.data_version;
                    let result =
                        state.materialize_deferred_saved_output(run.run_id, run.analyses[0].id, 1);
                    if let Some(expected) = expected {
                        result.unwrap();
                        check_value(&state.runs[0], 1, expected);
                        let visible = state
                            .waveforms
                            .iter()
                            .find(|wave| wave.name == "Deferred physical signal")
                            .unwrap();
                        close(visible.y[0], expected);
                    } else {
                        assert!(result.is_err(), "{kind:?}: {query}, reload={reload}");
                        assert_eq!(state.data_version, version_before);
                        assert_eq!(state.waveforms, visible_before);
                        assert_eq!(
                            serde_json::to_vec(&ProjectSimulationResults::from_state(&state))
                                .unwrap(),
                            before
                        );
                    }
                    state.runs[0].validate_provenance().unwrap();
                    ProjectSimulationResults::from_state(&state)
                        .validate()
                        .unwrap();
                }
            }
        }
    }
}

#[test]
fn ngspice_ground_aliases_bind_against_the_decks_policy() {
    let deck = "Ground alias\nV1 in GND 4\nR1 in GND 1k\n.end\n";
    assert!(
        rspice_core::Netlist::parse(deck)
            .unwrap()
            .ground_policy()
            .is_ground("GND")
    );
    let outputs = ["V(in,GND)", "V(GND,in)", "V(GND)"]
        .map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(deck, AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    for (index, value) in [4.0, -4.0, 0.0].into_iter().enumerate() {
        check_value(&run, index, value);
    }
}

#[test]
fn preprocess_ground_aliases_bind_against_the_decks_policy() {
    let deck =
        "Replaced ground\n.PREPROCESS REPLACEGROUND TRUE\nV1 in GROUND 4\nR1 in GROUND 1k\n.end\n";
    assert!(
        rspice_core::Netlist::parse(deck)
            .unwrap()
            .ground_policy()
            .is_ground("GROUND")
    );
    let outputs = ["V(in,GROUND)", "V(GROUND,in)", "V(GROUND)"]
        .map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(deck, AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    for (index, value) in [4.0, -4.0, 0.0].into_iter().enumerate() {
        check_value(&run, index, value);
    }
}

#[test]
fn replacement_ground_bang_alias_is_accepted_and_bound() {
    let deck = "Bang ground\n.PREPROCESS REPLACEGROUND TRUE\nV1 in GND! 4\nR1 in GND! 1k\n.end\n";
    assert!(
        rspice_core::Netlist::parse(deck)
            .unwrap()
            .ground_policy()
            .is_ground("GND!")
    );
    let run = execute(
        deck,
        AnalysisSpec::dc_op(),
        &[output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Bang ground",
            "V(in,GND!)",
        )],
    );
    check_value(&run, 0, 4.0);
}

#[test]
fn formal_port_probes_bind_to_the_instantiating_nodes() {
    let outputs = ["V(X1.a)", "V(X1.b)", "V(/top/X1/a)", "V(/X1/b)"]
        .map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(HIERARCHY, AnalysisSpec::dc_op(), &outputs);
    run.validate_provenance().unwrap();
    for (index, expected) in [4.0, -1.0, 4.0, -1.0].into_iter().enumerate() {
        check_value(&run, index, expected);
    }
}

#[test]
fn scoped_canonical_ground_uses_the_owning_analysis_axis() {
    let outputs =
        ["V(/0)", "V(/top/0)"].map(|expr| output(SavedOutputKind::RawVoltageOrCurrent, expr, expr));
    let run = execute(HIERARCHY, AnalysisSpec::dc_op(), &outputs);
    for index in 0..outputs.len() {
        check_value(&run, index, 0.0);
    }
}
