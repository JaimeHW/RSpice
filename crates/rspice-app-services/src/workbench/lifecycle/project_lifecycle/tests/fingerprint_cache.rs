//! Recurring dirty queries scan neither the accepted content nor an
//! unchanged working project: each is fingerprinted once, and the working
//! project again only when something in it changed.

use super::*;
use crate::state::result_presentation::AnalysisPresentationKey;

/// A new session holding one completed transient run, selected, with the
/// results workspace in front.
fn retained_results() -> (ServicesHost, AnalysisPresentationKey) {
    let mut state = ServicesHost::default();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "Tran")
            .with_waveforms(vec![crate::state::RetainedTrace::new(
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
    state.model.simulation.retained.runs.push(run);
    assert!(state.model.simulation.select_run(0));
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    (state, key)
}

fn fingerprint_passes() -> usize {
    registry::FINGERPRINT_PASSES.with(std::cell::Cell::get)
}

fn captures() -> usize {
    rspice_project::registry::PROJECT_CAPTURES.with(std::cell::Cell::get)
}

/// Every recurring query, and a registry refresh, once each.
fn query_round(state: &mut ServicesHost) -> bool {
    let unsaved = has_unsaved_changes(state);
    assert_eq!(active_document_is_dirty(state), unsaved);
    assert_eq!(dirty_document_count(state) > 0, unsaved);
    refresh_registry(state).unwrap();
    unsaved
}

#[test]
fn the_working_project_is_fingerprinted_once_per_change_across_dirty_queries() {
    for samples in [2, 100_000] {
        let (mut state, _) = retained_results();
        state.model.simulation.retained.runs[0].analyses[0].waveforms =
            vec![crate::state::RetainedTrace::new(
                "V(out)",
                (0..samples).map(|index| index as f64).collect::<Vec<_>>(),
                vec![1.0; samples],
                "#ffffff",
            )];
        assert!(state.model.simulation.select_run(0));
        state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Design);
        let baseline = snapshot(&state).unwrap();
        registry::FINGERPRINT_PASSES.with(|passes| passes.set(0));
        state
            .platform
            .project_lifecycle
            .accept_project(baseline, None);
        for _ in 0..3 {
            assert!(!query_round(&mut state));
        }
        assert_eq!(
            fingerprint_passes(),
            2,
            "{samples} samples: one pass of the accepted content and one of the working project"
        );

        assert!(
            state
                .model
                .schematic
                .with_undo("Place resistor", |schematic| {
                    schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
                })
        );
        for _ in 0..3 {
            assert!(query_round(&mut state));
        }
        assert!(state.model.schematic.undo());
        for _ in 0..3 {
            assert!(!query_round(&mut state));
        }
        assert_eq!(
            fingerprint_passes(),
            4,
            "{samples} samples: one more pass for the edit and one for its undo"
        );
    }
}

#[test]
fn a_hundred_idle_dirty_queries_capture_nothing() {
    let mut state = ServicesHost::default();
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    assert!(!has_unsaved_changes(&state));
    let before = captures();
    for _ in 0..100 {
        assert!(!has_unsaved_changes(&state));
        assert!(!active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 0);
        assert!(dirty_documents(&state).is_empty());
    }
    assert_eq!(captures() - before, 0);

    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 4));
    for _ in 0..100 {
        assert!(has_unsaved_changes(&state));
    }
    assert_eq!(
        captures() - before,
        1,
        "one capture for the edit, then none"
    );
}

#[test]
fn a_decision_captures_the_project_whatever_answer_is_held() {
    let mut state = ServicesHost::default();
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    assert!(!has_unsaved_changes(&state));
    let before = captures();
    assert!(!has_unsaved_changes_fresh(&state));
    assert!(!active_document_is_dirty_fresh(&state));
    assert!(dirty_documents_fresh(&state).is_empty());
    assert_eq!(captures() - before, 3);
}

#[test]
fn a_stale_held_answer_reads_clean_while_a_decision_sees_the_edit() {
    let mut state = ServicesHost::default();
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 4));
    hold_accepted_answer(&state);
    without_audit(|| {
        assert!(
            !has_unsaved_changes(&state),
            "the held answer is the stale one"
        );
        assert!(has_unsaved_changes_fresh(&state));
        assert!(active_document_is_dirty_fresh(&state));
        assert_eq!(
            dirty_documents_fresh(&state),
            [ProjectDocumentId::CellView(
                state.model.workspace.content.active_view.clone()
            )]
        );
        assert!(!saved_snapshot_authorizes_continuation(
            &state,
            SaveScope::AllDocuments,
            &ProjectDocumentId::ProjectConfiguration,
        ));
    });
    // The fresh capture replaced what was held.
    assert!(has_unsaved_changes(&state));
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "the dirty answer held for this working stamp is stale")]
fn a_stale_held_answer_fails_the_audit() {
    let mut state = ServicesHost::default();
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 4));
    hold_accepted_answer(&state);
    has_unsaved_changes(&state);
}
