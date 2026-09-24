//! Published marker identities must survive deletion, scoped save, and reload.

use super::*;
use crate::state::{ComponentType, Point};
use crate::workbench::documents::result_document::{AnalysisPresentationKey, marker_anchor_for};
use crate::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, active_document, active_document_is_dirty,
    confirm_revert_active_document, generated_netlist_input_digest, has_unsaved_changes,
    prepare_revert_active_document, save_native, saved_snapshot_authorizes_continuation,
};

fn retained_results() -> (AppState, AnalysisPresentationKey) {
    let mut state = AppState::default();
    let key = install_retained_results(&mut state);
    (state, key)
}

fn install_retained_results(state: &mut AppState) -> AnalysisPresentationKey {
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
    seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.simulation.runs.push(run);
    assert!(state.simulation.select_run(0));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    key
}

fn unique_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "rspice-{label}-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ))
}

fn remove_project_artifacts(path: &std::path::Path) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}

fn add_marker(state: &mut AppState, analysis: AnalysisPresentationKey) -> u32 {
    state
        .ui
        .results
        .add_marker(
            analysis,
            marker_anchor_for(analysis, "V(out)"),
            "V(out)".to_owned(),
            0.5,
        )
        .unwrap()
}

#[test]
fn removed_marker_id_is_not_reused_after_scoped_save_and_reopen() {
    for scope in [SaveScope::ActiveDocument, SaveScope::AllDocuments] {
        for remove_all in [false, true] {
            let path = unique_path("marker-allocation-history");
            let (mut state, analysis) = retained_results();
            let first = add_marker(&mut state, analysis);
            let highest = add_marker(&mut state, analysis);
            save_native(
                &mut state,
                SaveScope::AllDocuments,
                &path,
                DestinationAuthority::UserSelected,
            )
            .unwrap();
            state.ui.results.remove_marker(highest);
            if remove_all {
                state.ui.results.remove_marker(first);
            }
            state
                .schematic
                .with_undo("Unrelated drawing edit", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::new(2, 3));
                });
            assert!(active_document_is_dirty(&state));
            save_native(&mut state, scope, &path, DestinationAuthority::Canonical).unwrap();
            assert!(!active_document_is_dirty(&state));
            assert_eq!(
                has_unsaved_changes(&state),
                scope == SaveScope::ActiveDocument
            );

            let written = crate::io::load_project_file(&path).unwrap();
            let expected_components = usize::from(scope == SaveScope::AllDocuments);
            assert_eq!(
                written.workspace.schematic_buffers[&state.workspace.active_key()]
                    .components
                    .len(),
                expected_components
            );
            let mut reopened = AppState::default();
            assert!(load_project_from_path(&mut reopened, &path));
            assert!(!has_unsaved_changes(&reopened));
            let next = add_marker(&mut reopened, analysis);
            assert!(
                next > highest,
                "a published marker label was reused after deletion and reopen"
            );
            assert_eq!(
                reopened.ui.results.markers.len(),
                if remove_all { 1 } else { 2 }
            );
            remove_project_artifacts(&path);
        }
    }
}

#[test]
fn allocator_only_edits_are_dirty_and_revert_does_not_reuse_live_ids() {
    let path = unique_path("marker-allocation-revert");
    let (mut state, analysis) = retained_results();
    let published = add_marker(&mut state, analysis);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .unwrap();
    let saved_document = active_document(&state);
    let solver_input = generated_netlist_input_digest(&state).unwrap();
    let abandoned = add_marker(&mut state, analysis);
    state.ui.results.remove_marker(abandoned);
    assert!(
        active_document_is_dirty(&state),
        "allocation history needs a durable Results owner"
    );
    assert!(has_unsaved_changes(&state));
    assert_eq!(
        generated_netlist_input_digest(&state).unwrap(),
        solver_input
    );
    assert!(!saved_snapshot_authorizes_continuation(
        &state,
        SaveScope::ActiveDocument,
        &saved_document
    ));
    assert!(!saved_snapshot_authorizes_continuation(
        &state,
        SaveScope::AllDocuments,
        &saved_document
    ));
    let review = prepare_revert_active_document(&state).unwrap();
    confirm_revert_active_document(&mut state, &review).unwrap();
    assert!(
        !has_unsaved_changes(&state),
        "revert restores the accepted allocation history"
    );
    assert_eq!(state.ui.results.markers[0].id, published);
    assert!(
        add_marker(&mut state, analysis) > abandoned,
        "an abandoned live ID cannot retarget an open editor"
    );
    remove_project_artifacts(&path);
}

#[test]
fn saving_another_document_does_not_publish_results_allocation_history() {
    let path = unique_path("marker-history-save-scope");
    let (mut state, analysis) = retained_results();
    let published = add_marker(&mut state, analysis);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .unwrap();
    let pending = add_marker(&mut state, analysis);
    state.ui.results.remove_marker(pending);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    state.schematic.with_undo("Drawing edit", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(2, 3));
    });
    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .unwrap();
    assert!(!active_document_is_dirty(&state));
    assert!(has_unsaved_changes(&state));
    let written = crate::io::load_project_file(&path).unwrap();
    assert_eq!(
        written.result_presentation.marker_id_high_water,
        Some(published)
    );
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    assert!(active_document_is_dirty(&state));
    save_native(
        &mut state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .unwrap();
    let written = crate::io::load_project_file(&path).unwrap();
    assert_eq!(
        written.result_presentation.marker_id_high_water,
        Some(pending)
    );
    assert_eq!(written.result_presentation.markers.len(), 1);
    assert!(!has_unsaved_changes(&state));
    remove_project_artifacts(&path);
}

#[test]
fn clearing_results_or_replacing_a_drawing_preserves_project_marker_history() {
    for replace_drawing in [false, true] {
        let path = unique_path("marker-history-design-reset");
        let (mut state, analysis) = retained_results();
        let published = add_marker(&mut state, analysis);
        let project_id = state.workspace.project.id();
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        state
            .ui
            .results
            .reconcile_retained_datasets(&state.simulation);
        if replace_drawing {
            assert!(
                crate::workbench::workflows::file_workflow::apply_loaded_schematic(
                    &mut state,
                    crate::state::SchematicState::default(),
                    crate::workbench::workflows::file_workflow::SchematicLoadOrigin::BrowserImport(
                        "replacement.rsp"
                    )
                )
            );
        } else {
            state.clear_simulation_results();
        }
        state
            .ui
            .results
            .reconcile_retained_datasets(&state.simulation);
        assert_eq!(state.workspace.project.id(), project_id);
        assert!(state.ui.results.markers.is_empty());
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::Canonical,
        )
        .unwrap();
        let mut reopened = AppState::default();
        assert!(load_project_from_path(&mut reopened, &path));
        assert!(!has_unsaved_changes(&reopened));
        let analysis = install_retained_results(&mut reopened);
        assert!(add_marker(&mut reopened, analysis) > published);

        create_new_project(&mut reopened);
        assert_ne!(reopened.workspace.project.id(), project_id);
        let analysis = install_retained_results(&mut reopened);
        assert_eq!(add_marker(&mut reopened, analysis), 1);
        remove_project_artifacts(&path);
    }
}

#[test]
fn legacy_marker_history_is_adopted_without_dirtying_an_unchanged_project() {
    let path = unique_path("marker-history-legacy");
    let (mut state, analysis) = retained_results();
    let highest = add_marker(&mut state, analysis);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .unwrap();
    let mut legacy: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert!(
        legacy
            .as_object_mut()
            .unwrap()
            .remove("result_marker_id_high_water")
            .is_some()
    );
    std::fs::write(&path, serde_json::to_vec(&legacy).unwrap()).unwrap();
    let written = crate::io::load_project_file(&path).unwrap();
    assert_eq!(written.result_presentation.marker_id_high_water, None);
    let mut reopened = AppState::default();
    assert!(load_project_from_path(&mut reopened, &path));
    assert!(!has_unsaved_changes(&reopened));
    assert!(add_marker(&mut reopened, analysis) > highest);
    assert!(has_unsaved_changes(&reopened));
    remove_project_artifacts(&path);
}

#[test]
fn removing_the_last_possible_marker_does_not_reset_exhaustion_after_reopen() {
    let path = unique_path("marker-history-exhausted");
    let (mut state, analysis) = retained_results();
    add_marker(&mut state, analysis);
    state.ui.results.markers[0].id = u32::MAX;
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .unwrap();
    // Install the published maximum through the real restoration path.
    assert!(load_project_from_path(&mut state, &path));
    state.ui.results.remove_marker(u32::MAX);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::Canonical,
    )
    .unwrap();
    let mut reopened = AppState::default();
    assert!(load_project_from_path(&mut reopened, &path));
    assert!(!has_unsaved_changes(&reopened));
    assert!(
        reopened
            .ui
            .results
            .add_marker(
                analysis,
                marker_anchor_for(analysis, "V(out)"),
                "V(out)".to_owned(),
                0.5,
            )
            .unwrap_err()
            .contains("exhausted")
    );
    assert!(reopened.ui.results.markers.is_empty());
    assert!(!has_unsaved_changes(&reopened));
    remove_project_artifacts(&path);
}
