//! Every subject a save writes moves the dirty answer held between frames:
//! an edit in any of them reads unsaved, and taking it back reads saved. A
//! held answer that missed an edit cannot let a close skip its review.

use super::*;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
    SimulationRunProvenance,
};
use rspice_app_state::workbench::app_state::model::InPlaceLayout;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::app::dialogs::confirmation_state::ProjectReviewRequest;
use rspice_results::result_presentation::{SignalSet, SignalSetMember};

fn captures() -> usize {
    rspice_project::registry::PROJECT_CAPTURES.with(std::cell::Cell::get)
}

/// The project saved as it stands, with the design workspace in front.
fn saved() -> AppState {
    let mut state = AppState::default();
    state.workbench.activate(Workspace::Design);
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    assert!(!has_unsaved_changes(&state));
    state
}

/// `edit` reads unsaved and `undo` saved again, both from the answer held
/// between frames, which captures the edit once, and from a decision's own
/// capture.
fn edited_then_undone(
    subject: &str,
    edit: impl FnOnce(&mut AppState),
    undo: impl FnOnce(&mut AppState),
) {
    let mut state = saved();
    edit(&mut state);
    let before = captures();
    assert!(
        has_unsaved_changes(&state),
        "{subject}: the edit reads saved"
    );
    assert!(has_unsaved_changes(&state), "{subject}");
    assert_eq!(
        captures() - before,
        1,
        "{subject}: one capture for the edit"
    );
    assert!(has_unsaved_changes_fresh(&state), "{subject}");
    undo(&mut state);
    assert!(
        !has_unsaved_changes(&state),
        "{subject}: taking the edit back reads unsaved"
    );
    assert!(!has_unsaved_changes_fresh(&state), "{subject}");
}

fn completed_run() -> SimulationRun {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffffff"),
        ]),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run
}

#[test]
fn an_edit_in_each_saved_subject_reads_unsaved_until_it_is_taken_back() {
    edited_then_undone(
        "the project descriptor",
        |state| {
            state
                .model
                .workspace
                .content
                .project
                .description
                .push_str(" edited");
        },
        |state| {
            let description = &mut state.model.workspace.content.project.description;
            description.truncate(description.len() - " edited".len());
        },
    );
    edited_then_undone(
        "the schematic in the editor",
        |state| {
            assert!(
                state
                    .model
                    .schematic
                    .with_undo("Place resistor", |schematic| {
                        schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
                    })
            );
        },
        |state| assert!(state.model.schematic.undo()),
    );
    edited_then_undone(
        "the design libraries",
        |state| {
            state
                .model
                .library_manager
                .add_library(rspice_design::library::Library::new("bench_parts"));
        },
        |state| {
            assert!(
                state
                    .model
                    .library_manager
                    .remove_library("bench_parts")
                    .is_some()
            );
        },
    );
    edited_then_undone(
        "the simulation plan",
        |state| state.model.sim_setup.reference_pvt.temperature_celsius += 50.0,
        |state| state.model.sim_setup.reference_pvt.temperature_celsius -= 50.0,
    );
    edited_then_undone(
        "the model catalog",
        |state| {
            state
                .model
                .model_library_manager
                .add_library(rspice_model_library::ModelLibrary::new("bench_models"));
        },
        |state| {
            assert!(
                state
                    .model
                    .model_library_manager
                    .remove_library("bench_models")
                    .is_some()
            );
        },
    );
    edited_then_undone(
        "the retained results",
        |state| state.model.simulation.retained.runs.push(completed_run()),
        |state| assert!(state.model.simulation.retained.runs.pop().is_some()),
    );
    edited_then_undone(
        "the Results presentation",
        |state| {
            state.results.results.signal_sets.push(SignalSet {
                name: "bench".into(),
                members: vec![SignalSetMember {
                    analysis: AnalysisType::Transient,
                    name: "V(out)".into(),
                    artifact: false,
                }],
            });
        },
        |state| assert!(state.results.results.signal_sets.pop().is_some()),
    );
}

#[test]
fn a_symbol_edited_in_place_is_captured_again() {
    let mut state = saved();
    let before = captures();
    state.model.symbol_in_place_layout = Some(InPlaceLayout {
        symbol: CellViewRef::new("bench", "amp", "symbol"),
        parent: state.model.workspace.content.active_view.clone(),
        symbol_was_open: false,
    });
    assert!(!has_unsaved_changes(&state));
    assert_eq!(captures() - before, 1);
    state.model.symbol_in_place_layout = None;
    assert!(!has_unsaved_changes(&state));
    assert_eq!(captures() - before, 2);
}

#[test]
fn a_held_answer_that_missed_an_edit_cannot_close_the_project_without_review() {
    let mut state = saved();
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
        assert!(!state.closing_loses_nothing());
        assert!(state.request_close_project());
    });
    assert!(matches!(
        state.workbench.project_review_dialog.request,
        Some(ProjectReviewRequest::CloseProject(_))
    ));
    assert!(state.platform.project_lifecycle.is_open());
}
