//! Authored annotations and editor policy participate in scoped persistence.

use super::*;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsState, WavePanePresentationKey, marker_anchor_for,
};

fn retained_results() -> (AppState, AnalysisPresentationKey) {
    let mut state = AppState::default();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "Tran")
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![1.0, 2.0],
                "#ffffff",
            )]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.simulation.runs.push(run);
    assert!(state.simulation.select_run(0));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    (state, key)
}

#[derive(Clone, Copy, Debug)]
enum Annotation {
    Marker,
    LogAxis,
    Expression,
}

impl Annotation {
    const ALL: [Self; 3] = [Self::Marker, Self::LogAxis, Self::Expression];

    fn add(
        self,
        results: &mut ResultsState,
        simulation: &crate::state::SimulationState,
        key: AnalysisPresentationKey,
    ) {
        match self {
            Self::Marker => {
                results.add_marker(
                    key,
                    marker_anchor_for(key, "V(out)"),
                    "V(out)".to_owned(),
                    0.5,
                );
            }
            Self::LogAxis => {
                results.log_y_panes.insert(WavePanePresentationKey {
                    analysis: key,
                    unit: "V".to_owned(),
                });
            }
            Self::Expression => {
                results
                    .add_expression_trace(simulation, key, "V(out)*2".to_owned())
                    .unwrap();
            }
        }
    }
}

fn annotations(project: &ProjectFile) -> serde_json::Value {
    serde_json::to_value((
        &project.result_presentation.markers,
        &project.result_presentation.log_y_panes,
        &project.result_presentation.expression_groups,
    ))
    .unwrap()
}

#[test]
fn result_presentation_retains_flat_project_wire_format_and_legacy_defaults() {
    let (mut state, key) = retained_results();
    for annotation in Annotation::ALL {
        annotation.add(&mut state.ui.results, &state.simulation, key);
    }
    let project = snapshot(&state).unwrap();
    let wire = serde_json::to_value(&project).unwrap();
    assert!(wire.get("result_presentation").is_none());
    for name in [
        "result_markers",
        "result_log_y_panes",
        "result_expression_groups",
    ] {
        assert_eq!(wire[name].as_array().unwrap().len(), 1, "{name}");
        // Versions predating each field restore the other fields unchanged.
        let mut legacy = wire.clone();
        legacy.as_object_mut().unwrap().remove(name);
        let restored: ProjectFile = serde_json::from_value(legacy.clone()).unwrap();
        restored.validate().unwrap();
        assert_eq!(serde_json::to_value(restored).unwrap(), legacy);
    }
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert_eq!(annotations(&restored), annotations(&project));
    assert_eq!(serde_json::to_value(restored).unwrap(), wire);
}

#[test]
fn result_log_axis_presentation_has_canonical_order_after_restore() {
    let (mut state, key) = retained_results();
    let units: Vec<_> = (0..16).map(|index| format!("unit-{index:02}")).collect();
    for unit in units.iter().rev() {
        state
            .ui
            .results
            .log_y_panes
            .insert(WavePanePresentationKey {
                analysis: key,
                unit: unit.clone(),
            });
    }
    let presentation = state.ui.results.project_presentation(&state.simulation);
    assert_eq!(
        presentation
            .log_y_panes
            .iter()
            .map(|pane| &pane.unit)
            .collect::<Vec<_>>(),
        units.iter().collect::<Vec<_>>()
    );
    let expected = serde_json::to_value(&presentation).unwrap();
    crate::workbench::documents::result_document::restore_presentation(&mut state, presentation);
    assert_eq!(
        serde_json::to_value(state.ui.results.project_presentation(&state.simulation)).unwrap(),
        expected
    );
}

#[test]
fn every_result_annotation_is_dirty_revertible_and_independent_of_solver_inputs() {
    for annotation in Annotation::ALL {
        let (mut state, key) = retained_results();
        let baseline = snapshot(&state).unwrap();
        let solver_input = generated_netlist_input_digest(&state).unwrap();
        state.project_lifecycle.accepted = Some(AcceptedProject {
            baseline: baseline.clone(),
            binding: None,
        });
        annotation.add(&mut state.ui.results, &state.simulation, key);
        assert!(has_unsaved_changes(&state), "{annotation:?}");
        assert!(active_document_is_dirty(&state), "{annotation:?}");
        assert_eq!(dirty_document_count(&state), 1);
        assert_eq!(
            generated_netlist_input_digest(&state).unwrap(),
            solver_input
        );

        let edited = snapshot(&state).unwrap();
        assert_ne!(annotations(&edited), annotations(&baseline));
        assert_eq!(
            serde_json::to_value(&edited.simulation_results).unwrap(),
            serde_json::to_value(&baseline.simulation_results).unwrap()
        );
        let token = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &token).unwrap();
        assert_eq!(
            snapshot(&state).unwrap().simulation_results,
            baseline.simulation_results
        );
        assert_eq!(
            annotations(&snapshot(&state).unwrap()),
            annotations(&baseline)
        );
        assert!(!has_unsaved_changes(&state));

        // Revert also restores accepted annotations after they are removed.
        state.project_lifecycle.accepted = Some(AcceptedProject {
            baseline: edited.clone(),
            binding: None,
        });
        revert_document(&mut state, ProjectDocumentId::ResultHistory).unwrap();
        assert_eq!(
            annotations(&snapshot(&state).unwrap()),
            annotations(&edited)
        );
        assert!(!has_unsaved_changes(&state));
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn result_annotation_save_publishes_only_results_and_retains_later_edits() {
    for annotation in Annotation::ALL {
        let (mut state, key) = retained_results();
        let path = unique_path("result-annotations");
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        annotation.add(&mut state.ui.results, &state.simulation, key);
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 4));
        let edited = snapshot(&state).unwrap();
        save_native(
            &mut state,
            SaveScope::ActiveDocument,
            &path,
            DestinationAuthority::Canonical,
        )
        .unwrap();
        let saved = crate::io::load_project_file(&path).unwrap();
        assert_eq!(annotations(&saved), annotations(&edited), "{annotation:?}");
        assert!(
            saved.workspace.schematic_buffers[&state.workspace.active_view.key()]
                .components
                .is_empty()
        );
        assert!(!active_document_is_dirty(&state));
        assert!(
            has_unsaved_changes(&state),
            "unrelated drawing is still a draft"
        );

        // Save acceptance compares against the actual published snapshot.
        // A later annotation must remain dirty even if an earlier write finishes.
        annotation.add(&mut state.ui.results, &state.simulation, key);
        state.ui.results.log_y_panes.clear();
        state.ui.results.add_marker(
            key,
            marker_anchor_for(key, "V(out)"),
            "V(out)".to_owned(),
            0.75,
        );
        let post_save =
            prepare_post_save_registry(&state, &saved, SaveScope::ActiveDocument).unwrap();
        assert!(post_save.is_dirty(&ProjectDocumentId::ResultHistory));

        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::Canonical,
        )
        .unwrap();
        let all_saved = crate::io::load_project_file(&path).unwrap();
        assert_eq!(
            annotations(&all_saved),
            annotations(&snapshot(&state).unwrap())
        );
        assert_eq!(
            all_saved.workspace.schematic_buffers[&state.workspace.active_view.key()]
                .components
                .len(),
            1
        );
        assert!(!has_unsaved_changes(&state));
        remove_project_artifacts(&path);
    }
}

#[test]
fn unbound_probe_edits_participate_in_dirty_undo_redo_and_revert() {
    let mut state = AppState::default();
    let baseline = snapshot(&state).unwrap();
    state.project_lifecycle.accepted = Some(AcceptedProject {
        baseline,
        binding: None,
    });
    state.schematic.with_undo("Place probe", |schematic| {
        schematic
            .probes
            .push(crate::state::SchematicProbe::new(99, Point::new(0, 0), "P99", None).unwrap());
    });
    assert!(has_unsaved_changes(&state));
    assert!(active_document_is_dirty(&state));
    assert_eq!(dirty_document_count(&state), 1);
    assert!(state.schematic.undo());
    assert!(
        !has_unsaved_changes(&state),
        "undo returns to accepted content"
    );
    assert!(state.schematic.redo());
    assert!(has_unsaved_changes(&state));
    let token = prepare_revert_active_document(&state).unwrap();
    confirm_revert_active_document(&mut state, &token).unwrap();
    assert!(state.schematic.probes.is_empty());
    assert!(!has_unsaved_changes(&state));
}

#[test]
fn grid_and_document_policy_are_owned_by_the_schematic_document() {
    for edit_policy in [false, true] {
        let mut state = AppState::default();
        let baseline = snapshot(&state).unwrap();
        state.project_lifecycle.accepted = Some(AcceptedProject {
            baseline,
            binding: None,
        });
        if edit_policy {
            state.schematic.document_policy.net_naming =
                crate::state::NetNamingPolicy::SpiceCompatibleRelaxed;
        } else {
            state.schematic.grid_size += 1;
        }
        assert!(has_unsaved_changes(&state));
        assert!(active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 1);
        let token = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &token).unwrap();
        assert!(!has_unsaved_changes(&state));
    }
}
