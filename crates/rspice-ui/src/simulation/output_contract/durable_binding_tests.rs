//! Real solver and project-boundary coverage for captured physical sources.

use super::binding_tests::{HIERARCHY, ac, check_value, close, dc, execute};
use super::fixtures::{output, run};
use super::*;
use crate::io::project_io::{PersistedField, ProjectSimulationResults};
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
    state.next_run_id = run.id;
    state.runs = vec![run].into();
    let stored = ProjectSimulationResults::from_state(&state);
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
                let outputs = ["V(/top/X1/a)", "V(/X1/b)", "V(/top/GND!)"].map(|expression| {
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
                let mut state = loaded.into_simulation_state().unwrap();
                let analysis = &mut state.runs[0].analyses[0];
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
                    ("absent", "V(/top/absent)"),
                    ("I(absent)", "I(absent)"),
                ] {
                    let alias = output(kind, label, "V(in)");
                    let mut absent = output(kind, "Missing physical source", query);
                    absent.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                    let mut present = output(kind, "Valid physical source", "V(in)");
                    present.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
                    let executed = run(
                        deck,
                        "Source identity",
                        spec.clone(),
                        if complex {
                            ".ac lin 3 1 10"
                        } else {
                            ".tran 1u 10u"
                        },
                        &[alias, absent, present],
                        mode,
                    );
                    for reload in [false, true] {
                        let persisted = stored(executed.clone());
                        let mut state = if reload {
                            serde_json::from_slice::<ProjectSimulationResults>(
                                &serde_json::to_vec(&persisted).unwrap(),
                            )
                            .unwrap()
                            .into_simulation_state()
                            .unwrap()
                        } else {
                            persisted.into_simulation_state().unwrap()
                        };
                        let before =
                            serde_json::to_vec(&ProjectSimulationResults::from_state(&state))
                                .unwrap();
                        let version = state.data_version;
                        assert!(
                            state
                                .materialize_deferred_saved_output(
                                    executed.run_id,
                                    executed.analyses[0].id,
                                    1
                                )
                                .is_err()
                        );
                        assert_eq!(state.data_version, version);
                        assert_eq!(
                            serde_json::to_vec(&ProjectSimulationResults::from_state(&state))
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
                        check_value(&state.runs[0], 2, if complex { 2.0 } else { 4.0 });
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
    for schema in 1..24 {
        let mut injected = persisted.clone();
        injected.schema_version = schema;
        assert!(
            injected
                .migrate_to_current(crate::product::ProjectId::new())
                .unwrap_err()
                .contains("source bindings")
        );
    }
}

#[test]
fn schema_21_through_23_authenticate_original_digests_without_inventing_bindings() {
    for schema in [21, 22, 23] {
        for spec in [AnalysisSpec::dc_op(), tran()] {
            let op = matches!(spec, AnalysisSpec::DcOp { .. });
            let mut output = output(
                SavedOutputKind::RawVoltageOrCurrent,
                "Historical voltage",
                "V(in)",
            );
            output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
            let mut executed = execute(HIERARCHY, spec, &[output]);
            executed.analyses[0].saved_output_receipts[0].source_bindings = None;
            let original = executed.analyses[0].legacy_v12_result_data_digest();
            let dataset = executed.legacy_v12_dataset_content_digest();
            let mut old = stored(executed);
            old.schema_version = schema;
            old.runs[0].analyses[0].result_data_digest = PersistedField::Value(original);
            old.runs[0].dataset_content_digest = PersistedField::Value(dataset);
            for corrupt_dataset in [false, true] {
                let mut corrupt = old.clone();
                let altered = PersistedField::Value(ContentDigest::from_bytes([9; 32]));
                if corrupt_dataset {
                    corrupt.runs[0].dataset_content_digest = altered;
                } else {
                    corrupt.runs[0].analyses[0].result_data_digest = altered;
                }
                let before = corrupt.clone();
                assert!(
                    corrupt
                        .migrate_to_current(crate::product::ProjectId::new())
                        .unwrap_err()
                        .contains("digest")
                );
                assert_eq!(corrupt, before);
            }
            old.migrate_to_current(crate::product::ProjectId::new())
                .unwrap();
            assert_eq!(old.schema_version, 26);
            let mut state = old.into_simulation_state().unwrap();
            let analysis = &mut state.runs[0].analyses[0];
            assert!(analysis.saved_output_receipts[0].source_bindings.is_none());
            assert_ne!(analysis.result_data_digest(), original);
            let result = materialize_deferred_saved_output(analysis, 0);
            if op {
                result.unwrap();
            } else {
                assert!(result.unwrap_err().contains("historical output"));
            }
        }
    }
}
