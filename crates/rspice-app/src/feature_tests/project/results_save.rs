//! A save whose stored run's results go to a folder on another volume copies them before it
//! writes the project: the project stays unsaved until the file is written, a cancel leaves the
//! file as it was, and results deleted outside RSpice reopen missing, by name.

use std::path::Path;
use std::time::{Duration, Instant};

use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::workflows::project_workflow::SaveRequestOutcome;
use rspice_app_services::io::result_segments::{
    hold_placements_for_tests, place_by_copy_for_tests,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    SaveScope, has_unsaved_changes_fresh,
};
use rspice_formats::project_results::segment_folder;
use rspice_project::lifecycle::SaveContinuationEvent;

const DECK: &str = "results kept beside the project
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 10n
.tran 0.1u 300u
.end
";

/// What the save continues with once its results are kept, polled as frames would.
fn finished(state: &mut AppState) -> SaveContinuationEvent {
    let ctx = egui::Context::default();
    let until = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(event) = state.poll_results_save(&ctx) {
            return event;
        }
        assert!(Instant::now() < until, "the results were kept within 30 s");
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn files_in(folder: &Path) -> Vec<String> {
    let mut names: Vec<_> = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

#[test]
fn a_cancelled_save_copying_its_results_leaves_the_project_file_and_its_unsaved_changes() {
    let mut state: AppState =
        rspice_app_runs::simulation::controller::test_execution::run_manual_batch_session(DECK);
    state.open_project_session();
    let root = std::env::temp_dir().join(format!("rspice-results-save-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let file = root.join("kept.rspiceproj");

    // The project as it was saved before its run.
    let simulation = std::mem::take(&mut state.model.simulation);
    assert!(state.save_project_to_path(&file));
    state.model.simulation = simulation;
    let before = std::fs::read(&file).unwrap();
    assert!(has_unsaved_changes_fresh(&state), "the run is not saved");

    // Saved to a folder on another volume, the run's segment is copied first.
    place_by_copy_for_tests(true);
    hold_placements_for_tests(true);
    let SaveRequestOutcome::CanonicalPending(transaction) =
        state.save_project_scope(SaveScope::AllDocuments)
    else {
        panic!("the save waits for its results to be copied");
    };
    assert!(
        has_unsaved_changes_fresh(&state),
        "the project stays unsaved while its results are copied"
    );
    assert_eq!(std::fs::read(&file).unwrap(), before);

    state.cancel_results_save();
    assert_eq!(
        finished(&mut state),
        SaveContinuationEvent::Cancelled(transaction)
    );
    assert!(
        has_unsaved_changes_fresh(&state),
        "a cancelled save leaves the project unsaved"
    );
    assert_eq!(
        std::fs::read(&file).unwrap(),
        before,
        "a cancelled save leaves the project file byte for byte"
    );
    let folder = segment_folder(&file);
    assert_eq!(
        files_in(&folder),
        Vec::<String>::new(),
        "nothing of the copy is left"
    );

    // Saved again, the copy ends, and only then is the file written and the project saved.
    hold_placements_for_tests(false);
    let SaveRequestOutcome::CanonicalPending(transaction) =
        state.save_project_scope(SaveScope::AllDocuments)
    else {
        panic!("the save waits for its results to be copied");
    };
    assert_eq!(
        finished(&mut state),
        SaveContinuationEvent::Saved(transaction)
    );
    place_by_copy_for_tests(false);
    assert!(!has_unsaved_changes_fresh(&state));
    let results = state
        .platform
        .project_lifecycle
        .result_cache
        .capture(&state.model.simulation);
    let digest = results.stored_segments()[0].digest;
    assert_eq!(files_in(&folder), [format!("{digest}.rsr")]);
    let saved = std::fs::read_to_string(&file).unwrap();
    assert!(
        saved.contains(&digest.to_string()),
        "the file names the segment"
    );

    // Deleted outside RSpice while no session reads them, the results reopen missing, and the
    // project says whose they were.
    let label = results.runs[0].label.clone();
    drop((results, state));
    std::fs::remove_dir_all(&folder).unwrap();
    let mut reopened = AppState::default();
    assert!(reopened.load_project_from_path(&file));
    let missing = format!("The results of {label} are missing");
    assert!(
        reopened
            .model
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with(&missing)),
        "{missing}: {:?}",
        reopened
            .model
            .log_buffer
            .entries()
            .map(|entry| entry.message.clone())
            .collect::<Vec<_>>()
    );

    drop(reopened);
    let _ = std::fs::remove_dir_all(&root);
}
