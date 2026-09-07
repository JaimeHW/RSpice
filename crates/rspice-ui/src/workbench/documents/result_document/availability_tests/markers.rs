//! Marker identity and payload failures must never corrupt saved annotations.

use super::*;
use crate::io::{ProjectExecutionContext, ProjectFile, ProjectSimulationResults};

fn marker_project(state: &AppState) -> ProjectFile {
    ProjectFile::new_with_execution_context(
        state.workspace.clone(),
        state.library_manager.clone(),
        ProjectSimulationResults::from_state(&state.simulation),
        ProjectExecutionContext::from_state(
            state.workspace.project.id(),
            &state.sim_setup,
            &state.model_library_manager,
        )
        .unwrap(),
    )
    .with_result_presentation(state.ui.results.project_presentation(&state.simulation))
}

fn marker_state(id: u32) -> (AppState, AnalysisPresentationKey) {
    let mut state = transient_state();
    let run = &mut state.simulation.runs[0];
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    assert!(state.simulation.select_run(0));
    let analysis = active_analysis_key(&state);
    state.ui.results.adopt_markers(
        vec![ResultMarker {
            id,
            analysis,
            anchor: marker_anchor_for(analysis, "V(out)"),
            trace_name: "V(out)".to_owned(),
            x: 0.5,
            kind: MarkerKind::Peak,
            note: "Retained annotation".to_owned(),
        }],
        0,
    );
    (state, analysis)
}

fn place_quick(
    state: &mut AppState,
    analysis: AnalysisPresentationKey,
    x: f64,
) -> Option<MarkerSelector> {
    place_marker(
        state,
        MarkerPlacement {
            analysis,
            anchor: marker_anchor_for(analysis, "V(out)"),
            trace_name: "V(out)".to_owned(),
            x,
            samples: &[0.0, 1.0],
        },
    )
}

#[test]
fn restored_maximum_marker_id_cannot_wrap_or_retarget_existing_annotations() {
    let (mut state, analysis) = marker_state(u32::MAX);
    let project = marker_project(&state);
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    restore_presentation(&mut state, restored.result_presentation);
    assert_eq!(place_quick(&mut state, analysis, 0.75), None);
    assert_eq!(state.ui.results.markers.len(), 1);
    assert_eq!(state.ui.results.markers[0].id, u32::MAX);
    assert_eq!(state.ui.results.markers[0].note, "Retained annotation");
    assert!(
        state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("marker") && entry.message.contains("exhausted"))
    );
    commit_marker_edit(
        &mut state,
        MarkerSelector::Quick(u32::MAX),
        "Edited",
        MarkerKind::Spec,
    )
    .unwrap();
    assert_eq!(state.ui.results.markers[0].note, "Edited");
    state.ui.results.remove_marker(u32::MAX);
    assert!(state.ui.results.markers.is_empty());
    assert_eq!(
        place_quick(&mut state, analysis, 0.75),
        None,
        "deleted identities must not be reused"
    );
}

#[test]
fn nonfinite_marker_coordinates_are_refused_before_project_publication() {
    let (state, _) = marker_state(7);
    let mut project = marker_project(&state);
    for coordinate in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        project.result_presentation.markers[0].x = coordinate;
        assert!(
            crate::io::project_io::serialize_project_file(&project).is_err(),
            "a nonfinite marker would serialize as null and make the project unreadable"
        );
    }
}

#[test]
fn marker_anchor_must_name_the_same_analysis_as_the_marker() {
    let (state, _) = marker_state(7);
    let mut project = marker_project(&state);
    project.result_presentation.markers[0].anchor.analysis = AnalysisPresentationKey::new(
        crate::product::DatasetId::new(),
        &state.simulation.runs[0].analyses[0],
    );
    assert!(
        project.validate().is_err(),
        "one marker cannot identify two different datasets"
    );
}

#[test]
fn duplicate_marker_ids_are_refused_before_project_publication() {
    let (state, _) = marker_state(7);
    let mut project = marker_project(&state);
    let mut duplicate = project.result_presentation.markers[0].clone();
    duplicate.note = "Another annotation".to_owned();
    duplicate.x = 0.75;
    project.result_presentation.markers.push(duplicate);
    assert!(
        crate::io::project_io::serialize_project_file(&project).is_err(),
        "duplicate IDs alias edit and delete operations"
    );
}

#[test]
fn an_edit_cannot_succeed_after_its_quick_marker_has_been_removed() {
    let (mut state, _) = marker_state(7);
    state.ui.results.remove_marker(7);
    assert!(
        commit_marker_edit(
            &mut state,
            MarkerSelector::Quick(7),
            "Later edit",
            MarkerKind::Peak
        )
        .is_err()
    );
    assert!(state.ui.results.markers.is_empty());
}

#[test]
fn last_marker_identity_can_be_allocated_once_and_remains_serializable() {
    let (mut state, analysis) = marker_state(u32::MAX - 1);
    assert_eq!(
        place_quick(&mut state, analysis, 0.75),
        Some(MarkerSelector::Quick(u32::MAX))
    );
    let before = serde_json::to_value(&state.ui.results.markers).unwrap();
    assert_eq!(place_quick(&mut state, analysis, 0.9), None);
    assert_eq!(
        serde_json::to_value(&state.ui.results.markers).unwrap(),
        before
    );
    let project = marker_project(&state);
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert_eq!(
        serde_json::to_value(restored.result_presentation.markers).unwrap(),
        before
    );
}

#[test]
fn invalid_marker_placement_does_not_consume_an_identity_or_mutate_annotations() {
    let (mut state, analysis) = marker_state(7);
    let before = serde_json::to_value(&state.ui.results.markers).unwrap();
    for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(place_quick(&mut state, analysis, x), None);
        assert!(
            state
                .ui
                .results
                .add_marker(
                    analysis,
                    marker_anchor_for(analysis, "V(out)"),
                    "V(out)".to_owned(),
                    x
                )
                .is_err()
        );
    }
    let foreign = AnalysisPresentationKey::new(
        crate::product::DatasetId::new(),
        &state.simulation.runs[0].analyses[0],
    );
    assert!(
        state
            .ui
            .results
            .add_marker(
                analysis,
                marker_anchor_for(foreign, "V(out)"),
                "V(out)".to_owned(),
                0.75
            )
            .is_err()
    );
    assert_eq!(
        serde_json::to_value(&state.ui.results.markers).unwrap(),
        before
    );
    assert_eq!(
        place_quick(&mut state, analysis, 0.75),
        Some(MarkerSelector::Quick(8))
    );

    // Until every legacy caller uses the marker owner, a directly inserted
    // identity must also be respected by the allocator.
    let mut inserted = state.ui.results.markers[0].clone();
    inserted.id = 100;
    state.ui.results.markers.push(inserted);
    assert_eq!(
        place_quick(&mut state, analysis, 0.9),
        Some(MarkerSelector::Quick(101))
    );
}

#[test]
fn duplicate_ids_in_older_projects_are_repaired_without_losing_annotations() {
    let (mut state, _) = marker_state(7);
    let mut project = marker_project(&state);
    let template = project.result_presentation.markers[0].clone();
    project.result_presentation.marker_id_high_water = None;
    project.result_presentation.markers = [0, 1, 2, 7, u32::MAX, 7, 0, u32::MAX]
        .into_iter()
        .enumerate()
        .map(|(index, id)| {
            let mut marker = template.clone();
            marker.id = id;
            marker.note = format!("Annotation {index}");
            marker.x = index as f64 * 0.125;
            marker
        })
        .collect();
    // Simulate bytes from older writers, before publication enforced unique IDs.
    let text = serde_json::to_string(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert_eq!(
        restored
            .result_presentation
            .markers
            .iter()
            .map(|marker| marker.id)
            .collect::<Vec<_>>(),
        [0, 1, 2, 7, u32::MAX, 3, 4, 5]
    );
    assert!(
        restored
            .workspace_migration_warning
            .as_deref()
            .unwrap()
            .contains("duplicate IDs")
    );
    for (original, repaired) in project
        .result_presentation
        .markers
        .iter()
        .zip(&restored.result_presentation.markers)
    {
        let mut expected = original.clone();
        expected.id = repaired.id;
        assert_eq!(
            serde_json::to_value(repaired).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
    }
    let repeated = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert_eq!(
        serde_json::to_value(&restored.result_presentation.markers).unwrap(),
        serde_json::to_value(repeated.result_presentation.markers).unwrap()
    );
    let normalized_text = crate::io::project_io::serialize_project_file(&restored).unwrap();
    let normalized = crate::io::project_io::load_project_text(&normalized_text, None).unwrap();
    assert_eq!(
        serde_json::to_value(&restored.result_presentation.markers).unwrap(),
        serde_json::to_value(normalized.result_presentation.markers).unwrap()
    );

    restore_presentation(&mut state, restored.result_presentation);
    commit_marker_edit(
        &mut state,
        MarkerSelector::Quick(3),
        "Only this annotation",
        MarkerKind::Spec,
    )
    .unwrap();
    assert_eq!(state.ui.results.markers[5].note, "Only this annotation");
    assert_eq!(state.ui.results.markers[3].note, "Annotation 3");
    state.ui.results.remove_marker(3);
    assert_eq!(state.ui.results.markers.len(), 7);
    assert!(state.ui.results.markers.iter().any(|marker| marker.id == 7));
}
