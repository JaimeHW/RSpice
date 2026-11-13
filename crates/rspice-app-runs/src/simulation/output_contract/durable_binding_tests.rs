//! Real solver and project-boundary coverage for captured physical sources.

use super::binding_tests::{HIERARCHY, check_value, close, dc, execute};
use super::fixtures::{ac, output, run};
use super::*;
use crate::io::project_io::ProjectSimulationResults;
use crate::state::{OutputSelectionMode, SavedOutputBoundSource, SimulationRun, SimulationState};

fn tran() -> AnalysisSpec {
    AnalysisSpec::Transient {
        stop_time: 1e-5,
        step_time: 1e-6,
        start_time: 0.0,
        max_timestep: Some(1e-6),
        uic: false,
    }
}

fn stored(run: SimulationRun) -> ProjectSimulationResults {
    let mut state = SimulationState::default();
    state.retained.next_run_id = run.id;
    state.retained.runs = vec![run].into();
    let stored = crate::io::capture_simulation_results(&state);
    stored.validate().unwrap();
    stored
}

#[test]
fn task_ground_and_formal_port_bindings_survive_reload_without_deck_history() {
    let deck = HIERARCHY
        .replace("X1 in bias divider", "X1 in GND! divider")
        .replace(".end\n", ".PREPROCESS REPLACEGROUND TRUE\n.end\n");
    for spec in [AnalysisSpec::dc_op(), dc(), tran(), ac()] {
        let nested = matches!(spec, AnalysisSpec::DcSweep { .. });
        let complex = matches!(spec, AnalysisSpec::Ac { .. });
        for kind in [
            SavedOutputKind::RawVoltageOrCurrent,
            SavedOutputKind::DerivedExpression,
        ] {
            for deferred in [false, true] {
                let outputs = ["V(/X1/a)", "V(/X1/b)", "V(/GND!)"].map(|expression| {
                    let mut output = output(kind, &format!("Saved {expression}"), expression);
                    if deferred {
                        output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                    }
                    output
                });
                let persisted = stored(execute(&deck, spec.clone(), &outputs));
                assert!(persisted.executed_decks.is_empty());
                let loaded: ProjectSimulationResults =
                    serde_json::from_slice(&serde_json::to_vec(&persisted).unwrap()).unwrap();
                let mut state = crate::io::simulation_state_from_results(loaded).unwrap();
                let analysis = &mut state.retained.runs[0].analyses[0];
                for index in 0..outputs.len() {
                    if deferred {
                        materialize_deferred_saved_output(analysis, index).unwrap();
                    }
                    let receipt = &analysis.saved_output_receipts[index];
                    let names = receipt.status.materialized_waveforms().collect::<Vec<_>>();
                    assert_eq!(names.len(), if nested { 2 } else { 1 }, "{receipt:?}");
                    for (name, _) in names {
                        let wave = analysis
                            .waveforms
                            .iter()
                            .find(|wave| wave.name == name)
                            .unwrap();
                        for (&x, &y) in wave.x.iter().zip(wave.y.iter()) {
                            close(
                                y,
                                if index != 0 {
                                    0.0
                                } else if nested {
                                    x
                                } else if complex {
                                    1.0
                                } else {
                                    4.0
                                },
                            );
                        }
                        if complex && kind == SavedOutputKind::RawVoltageOrCurrent {
                            assert!(wave.complex.is_some());
                        }
                    }
                }
                analysis.validate_retained_evidence().unwrap();
            }
        }
    }
}

#[test]
fn literal_ground_like_nodes_win_before_scope_aliases() {
    for node in ["GND!", "GROUND", "/0", "/top/GND", "/X1/a"] {
        let deck = format!("Literal ground-like node\nV1 {node} 0 7\nR1 {node} 0 1k\n.end\n");
        for kind in [
            SavedOutputKind::RawVoltageOrCurrent,
            SavedOutputKind::DerivedExpression,
        ] {
            let run = execute(
                &deck,
                AnalysisSpec::dc_op(),
                &[output(kind, "Literal node", &format!("V({node})"))],
            );
            check_value(&run, 0, 7.0);
        }
    }
}

#[test]
fn authored_transient_and_ac_labels_cannot_invent_deferred_physical_sources() {
    let deck = "Physical source identity\nV1 in 0 DC 4 AC 2\nR1 in 0 1k\n.end\n";
    for spec in [tran(), ac()] {
        let complex = matches!(spec, AnalysisSpec::Ac { .. });
        for kind in [
            SavedOutputKind::RawVoltageOrCurrent,
            SavedOutputKind::DerivedExpression,
        ] {
            for mode in [
                OutputSelectionMode::ExplicitOnly,
                OutputSelectionMode::SaveAll,
            ] {
                for (label, query) in [
                    ("V(absent)", "V(absent)"),
                    ("absent", "V(/absent)"),
                    ("I(absent)", "I(absent)"),
                ] {
                    let alias = output(kind, label, "V(in)");
                    let mut absent = output(kind, "Missing physical source", query);
                    absent.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                    let mut present = output(kind, "Valid physical source", "V(in)");
                    present.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                    let executed = run(deck, spec.clone(), &[alias, absent, present], mode);
                    for reload in [false, true] {
                        let persisted = stored(executed.clone());
                        let mut state = if reload {
                            crate::io::simulation_state_from_results(
                                serde_json::from_slice::<ProjectSimulationResults>(
                                    &serde_json::to_vec(&persisted).unwrap(),
                                )
                                .unwrap(),
                            )
                            .unwrap()
                        } else {
                            crate::io::simulation_state_from_results(persisted).unwrap()
                        };
                        let before =
                            serde_json::to_vec(&crate::io::capture_simulation_results(&state))
                                .unwrap();
                        let version = state.view.data_version;
                        assert!(
                            state
                                .materialize_deferred_saved_output(
                                    executed.run_id,
                                    executed.analyses[0].id,
                                    1
                                )
                                .is_err()
                        );
                        assert_eq!(state.view.data_version, version);
                        assert_eq!(
                            serde_json::to_vec(&crate::io::capture_simulation_results(&state))
                                .unwrap(),
                            before
                        );
                        state
                            .materialize_deferred_saved_output(
                                executed.run_id,
                                executed.analyses[0].id,
                                2,
                            )
                            .unwrap();
                        check_value(&state.retained.runs[0], 2, if complex { 2.0 } else { 4.0 });
                    }
                }
            }
        }
    }
}

#[test]
fn saved_source_bindings_are_validated_and_authenticated() {
    let mut output = output(
        SavedOutputKind::RawVoltageOrCurrent,
        "Deferred voltage",
        "V(in)",
    );
    output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
    let persisted = stored(execute(HIERARCHY, tran(), &[output]));
    for replacement in [
        None,
        Some(SavedOutputBoundSource::Ground),
        Some(SavedOutputBoundSource::Missing),
    ] {
        let mut changed = persisted.clone();
        let bindings = &mut changed.runs[0].analyses[0].saved_output_receipts[0].source_bindings;
        if let Some(replacement) = replacement {
            bindings
                .as_mut()
                .unwrap()
                .references
                .insert("v(in)".to_owned(), replacement);
        } else {
            *bindings = None;
        }
        assert!(changed.validate().is_err());
    }
    let mut json = serde_json::to_value(&persisted).unwrap();
    json["runs"][0]["analyses"][0]["saved_output_receipts"][0]["source_bindings"] =
        serde_json::Value::Null;
    assert!(serde_json::from_value::<ProjectSimulationResults>(json).is_err());
}
