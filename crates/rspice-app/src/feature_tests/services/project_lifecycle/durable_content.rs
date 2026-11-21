//! Authored annotations and editor policy participate in scoped persistence.

use super::*;
use rspice_app_results::workbench::documents::result_document::{
    AnalysisPresentationKey, MarkerPlacement, WavePanePresentationKey, place_marker,
    waves::source_waveform_anchor, working_view,
};

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn cleared_run_sequence_is_dirty_and_survives_scoped_and_all_saves() {
    for scope in [SaveScope::ActiveDocument, SaveScope::AllDocuments] {
        let mut state = AppState::default();
        state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Results);
        let path = unique_path("cleared-run-sequence");
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        let solver_input = generated_netlist_input_digest(&state).unwrap();
        assert_eq!(state.model.simulation.start_run().id, 1);
        state.model.simulation.clear_runs();
        assert!(state.model.simulation.retained.runs.is_empty());
        assert!(active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 1);
        assert_eq!(
            generated_netlist_input_digest(&state).unwrap(),
            solver_input
        );

        state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 4));
        save_native(&mut state, scope, &path, DestinationAuthority::Canonical).unwrap();
        assert!(!active_document_is_dirty(&state));
        assert_eq!(
            has_unsaved_changes(&state),
            scope == SaveScope::ActiveDocument
        );
        let saved = rspice_app_services::io::load_project_file(&path).unwrap();
        assert_eq!(saved.file.simulation_results.next_run_id, 1);
        let mut restored =
            rspice_app_services::io::simulation_state_from_results(saved.file.simulation_results)
                .unwrap();
        assert!(restored.retained.runs.is_empty());
        assert_eq!(restored.start_run().id, 2);
        remove_project_artifacts(&path);
    }
}

pub(super) fn retained_results() -> (AppState, AnalysisPresentationKey) {
    let mut state = AppState::default();
    let mut run = rspice_app_state::state::SimulationRun::new(1);
    run.add_analysis(
        rspice_app_state::state::AnalysisResult::new(
            1,
            rspice_app_state::state::AnalysisType::Transient,
            "Tran",
        )
        .with_waveforms(vec![rspice_app_state::state::RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![1.0, 2.0],
            "#ffffff",
        )]),
    );
    run.restore_provenance(rspice_app_state::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(rspice_app_state::state::SimulationRunLifecycle::Completed)
        .unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.model.simulation.retained.runs.push(run);
    assert!(state.model.simulation.select_run(0));
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
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

    /// What the reader does in the quick view, kept by the run's working
    /// view once the quick view has drawn.
    fn add(self, state: &mut AppState, key: AnalysisPresentationKey) {
        quick_frame(state);
        match self {
            Self::Marker => mark(state, 0.5),
            Self::LogAxis => {
                state
                    .results
                    .results
                    .session
                    .log_y_panes
                    .insert(WavePanePresentationKey {
                        analysis: key,
                        unit: "V".to_owned(),
                    });
            }
            Self::Expression => {
                state
                    .results
                    .results
                    .add_expression_trace(&state.model.simulation, key, "V(out)*2".to_owned())
                    .unwrap();
            }
        }
        quick_frame(state);
    }
}

/// The quick view's frame around nothing: the shown run's working view is
/// made or followed, drawn, and kept.
fn quick_frame(state: &mut AppState) {
    let mut parts = state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
}

fn mark(state: &mut AppState, x: f64) {
    let (analysis, anchor) =
        source_waveform_anchor(&mut state.results_parts(), "V(out)").expect("V(out) is drawn");
    place_marker(
        &mut state.results_parts(),
        MarkerPlacement {
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x,
            samples: &vec![0.0, 1.0].into(),
        },
    )
    .expect("the working view keeps the marker");
}

/// What the project keeps of what the reader made of its runs: its views.
fn annotations(project: &ProjectSnapshot) -> serde_json::Value {
    serde_json::to_value(&project.file.workspace.visualization_documents).unwrap()
}

#[test]
fn every_result_annotation_is_dirty_revertible_and_independent_of_solver_inputs() {
    for annotation in Annotation::ALL {
        let (mut state, key) = retained_results();
        quick_frame(&mut state);
        let baseline = snapshot(&state).unwrap();
        let solver_input = generated_netlist_input_digest(&state).unwrap();
        state
            .platform
            .project_lifecycle
            .accept_project(baseline.clone(), None);
        annotation.add(&mut state, key);
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
            serde_json::to_value(&edited.file.simulation_results).unwrap(),
            serde_json::to_value(&baseline.file.simulation_results).unwrap()
        );
        let token = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &token).unwrap();
        assert_eq!(
            snapshot(&state).unwrap().file.simulation_results,
            baseline.file.simulation_results
        );
        assert_eq!(
            annotations(&snapshot(&state).unwrap()),
            annotations(&baseline)
        );
        assert!(!has_unsaved_changes(&state));

        // Revert also restores accepted annotations after they are removed.
        state
            .platform
            .project_lifecycle
            .accept_project(edited.clone(), None);
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
        quick_frame(&mut state);
        let path = unique_path("result-annotations");
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        annotation.add(&mut state, key);
        state
            .model
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
        let saved = rspice_app_services::io::load_project_file(&path).unwrap();
        assert_eq!(annotations(&saved), annotations(&edited), "{annotation:?}");
        assert!(
            saved.file.workspace.schematic_buffers
                [&state.model.workspace.content.active_view.key()]
                .document()
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
        annotation.add(&mut state, key);
        state.results.results.session.log_y_panes.clear();
        quick_frame(&mut state);
        mark(&mut state, 0.75);
        let post_save = prepare_post_save_registry(
            &state,
            &saved.file,
            SaveScope::ActiveDocument,
            SnapshotContent::Current,
        )
        .unwrap();
        assert!(post_save.is_dirty(&ProjectDocumentId::ResultHistory));

        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::Canonical,
        )
        .unwrap();
        let all_saved = rspice_app_services::io::load_project_file(&path).unwrap();
        assert_eq!(
            annotations(&all_saved),
            annotations(&snapshot(&state).unwrap())
        );
        assert_eq!(
            all_saved.file.workspace.schematic_buffers
                [&state.model.workspace.content.active_view.key()]
                .document()
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
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    state.model.schematic.with_undo("Place probe", |schematic| {
        schematic.document_mut_for_test().probes.push(
            rspice_app_state::state::SchematicProbe::new(99, Point::new(0, 0), "P99", None)
                .unwrap(),
        );
    });
    assert!(has_unsaved_changes(&state));
    assert!(active_document_is_dirty(&state));
    assert_eq!(dirty_document_count(&state), 1);
    assert!(state.model.schematic.undo());
    assert!(
        !has_unsaved_changes(&state),
        "undo returns to accepted content"
    );
    assert!(state.model.schematic.redo());
    assert!(has_unsaved_changes(&state));
    let token = prepare_revert_active_document(&state).unwrap();
    confirm_revert_active_document(&mut state, &token).unwrap();
    assert!(state.model.schematic.document().probes.is_empty());
    assert!(!has_unsaved_changes(&state));
}

#[test]
fn grid_and_document_policy_are_owned_by_the_schematic_document() {
    for edit_policy in [false, true] {
        let mut state = AppState::default();
        let baseline = snapshot(&state).unwrap();
        state
            .platform
            .project_lifecycle
            .accept_project(baseline, None);
        if edit_policy {
            state
                .model
                .schematic
                .document_mut_for_test()
                .document_policy
                .net_naming = rspice_app_state::state::NetNamingPolicy::SpiceCompatibleRelaxed;
        } else {
            state.model.schematic.document_mut_for_test().grid_size += 1;
        }
        assert!(has_unsaved_changes(&state));
        assert!(active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 1);
        let token = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &token).unwrap();
        assert!(!has_unsaved_changes(&state));
    }
}
