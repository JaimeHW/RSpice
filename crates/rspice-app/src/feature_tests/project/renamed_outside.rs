//! A project renamed or moved outside RSpice. Closed, it opens by its new name with every run
//! and document it was saved with. Open, the next save finds its file gone from where this
//! window holds it: the save writes nothing at the old name, says so, and the runs stay in the
//! session for Save As to keep.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::workflows::project_workflow::SaveRequestOutcome;
use rspice_app_results::workbench::documents::result_document::create_document;
use rspice_app_runs::simulation::controller::test_execution::{
    run_manual_batch_session, run_manual_deck_in,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    SaveScope, has_unsaved_changes_fresh,
};
use rspice_app_state::product::{DatasetId, ObjectRevision, ResultDocumentId};
use rspice_formats::project_results::segment_folder;
use rspice_project::lifecycle::SaveContinuationEvent;
use rspice_vocabulary::diagnostics::LogSeverity;

/// The deck of `run`, its resistor stepped by the run. Each test names a capacitance of its
/// own: a kept segment is found by the digest of its samples, so a deck another test runs
/// alongside would read as this one's.
fn deck(run: u32, capacitance: &str) -> String {
    format!(
        "run {run} of a project renamed outside RSpice\nV1 in 0 PULSE(0 1 0 1u 1u 50u 100u)\nR1 \
         in out {run}k\nC1 out 0 {capacitance}\n.tran 0.1u 300u\n.end\n"
    )
}

/// A scratch folder removed when the test lets go of it.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!("rspice-{name}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).expect("a scratch folder");
        Self(root)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Every file under `folder`, by name, with its bytes. A file held locked, as the open
/// project's lease is, is listed by name.
fn files_under(folder: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    let mut files = Vec::new();
    let mut pending = vec![folder.to_path_buf()];
    while let Some(folder) = pending.pop() {
        for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push((path.display().to_string(), std::fs::read(&path).ok()));
            }
        }
    }
    files.sort();
    files
}

/// Each run's dataset with the bits of every sample it reads, in the order the runs are kept.
fn runs_read(state: &AppState) -> Vec<(DatasetId, Vec<Vec<u64>>)> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| {
            let samples = run
                .analyses
                .iter()
                .flat_map(|analysis| &analysis.waveforms)
                .flat_map(|waveform| [&waveform.x, &waveform.y])
                .map(|values| values.samples(..).map(f64::to_bits).collect())
                .collect();
            (run.dataset_id, samples)
        })
        .collect()
}

/// Each result document the project keeps: its identity, title and revision.
fn documents(state: &AppState) -> Vec<(ResultDocumentId, String, ObjectRevision)> {
    state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .map(|document| {
            (
                document.id(),
                document.title().to_owned(),
                document.revision(),
            )
        })
        .collect()
}

/// A session of two runs and a result document over the first, saved at `file`.
fn saved_project(file: &Path, capacitance: &str) -> AppState {
    let mut state: AppState = run_manual_batch_session(&deck(1, capacitance));
    state.open_project_session();
    let _ = run_manual_deck_in(&mut state, &deck(2, capacitance));
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    create_document::open(&mut state.results_parts());
    state.results.create_result_document.layout_id = "single-pane".to_owned();
    create_document::commit(&mut state.results_parts()).expect("the document is created");
    assert!(state.save_project_to_path(file), "the project saves");
    state
}

/// Save the open project to its own file, as the Save command does, waiting for the results it
/// keeps beside the file. Returns the words the save failed with, if it did.
fn save(state: &mut AppState) -> Result<(), String> {
    match state.save_project_scope(SaveScope::AllDocuments) {
        SaveRequestOutcome::CanonicalComplete => Ok(()),
        SaveRequestOutcome::CanonicalPending(transaction) => {
            let ctx = egui::Context::default();
            let until = Instant::now() + Duration::from_secs(30);
            loop {
                match state.poll_results_save(&ctx) {
                    Some(SaveContinuationEvent::Saved(saved)) if saved == transaction => {
                        return Ok(());
                    }
                    Some(SaveContinuationEvent::Failed(_, message)) => return Err(message),
                    Some(other) => panic!("the save ended {other:?}"),
                    None => {}
                }
                assert!(Instant::now() < until, "the results were kept within 30 s");
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        SaveRequestOutcome::Failed(message) => Err(message),
        other => panic!("the save went {other:?}"),
    }
}

/// Open the project at `file` in a new session.
fn opened(file: &Path) -> AppState {
    let mut state = AppState::default();
    assert!(
        state.load_project_from_path(file),
        "{} opens",
        file.display()
    );
    state
}

/// Renamed while closed, the project opens by its new name with every run and document; saved
/// there, it keeps its runs beside its new name and no longer needs the old name's folder; and
/// moved with its folder, it opens from where it went.
#[test]
fn a_project_renamed_while_closed_opens_by_its_new_name_with_every_run_and_document() {
    let scratch = Scratch::new("renamed-closed");
    let folder = scratch.0.join("amp");
    std::fs::create_dir_all(&folder).unwrap();
    let file = folder.join("amp.rspiceproj");
    let state = saved_project(&file, "33n");
    let (runs, documents_saved) = (runs_read(&state), documents(&state));
    assert_eq!(runs.len(), 2);
    assert_eq!(documents_saved.len(), 1);
    drop(state);

    // Renamed.
    let renamed = folder.join("amp-final.rspiceproj");
    std::fs::rename(&file, &renamed).unwrap();
    let mut state = opened(&renamed);
    assert_eq!(runs_read(&state), runs, "every run reads as it was saved");
    assert_eq!(documents(&state), documents_saved, "every document is kept");

    // Saved by its new name, its runs are kept beside that name.
    save(&mut state).expect("the renamed project saves");
    drop(state);
    std::fs::remove_dir_all(segment_folder(&file)).unwrap();
    let state = opened(&renamed);
    assert_eq!(runs_read(&state), runs, "the runs are kept by the new name");
    assert_eq!(documents(&state), documents_saved);
    drop(state);

    // Moved with its folder.
    let moved = scratch.0.join("moved");
    std::fs::rename(&folder, &moved).unwrap();
    let state = opened(&moved.join("amp-final.rspiceproj"));
    assert_eq!(
        runs_read(&state),
        runs,
        "every run reads from the moved folder"
    );
    assert_eq!(documents(&state), documents_saved);
}

/// Renamed or moved while open, the project's next save writes nothing where its file was and
/// says why; the session keeps every run, unsaved, and Save As keeps them where the reader picks.
#[test]
fn a_project_renamed_while_open_is_not_written_at_its_old_name_and_loses_no_run() {
    let scratch = Scratch::new("renamed-open");
    let folder = scratch.0.join("amp");
    std::fs::create_dir_all(&folder).unwrap();
    let file = folder.join("amp.rspiceproj");
    let mut state = saved_project(&file, "47n");
    // A run made since the save, which only the session holds.
    let _ = run_manual_deck_in(&mut state, &deck(3, "47n"));
    let runs = runs_read(&state);
    assert_eq!(runs.len(), 3);

    for (gone, to) in [
        // Renamed in its folder.
        (file.clone(), folder.join("amp-final.rspiceproj")),
        // Moved to another folder. A folder holding an open project cannot be moved on
        // Windows, whose open lease is a file held open in it.
        (
            folder.join("amp-final.rspiceproj"),
            scratch.0.join("elsewhere.rspiceproj"),
        ),
    ] {
        std::fs::rename(&gone, &to).unwrap();
        let on_disk = files_under(&scratch.0);
        let log_before = state.model.log_buffer.entries().count();

        let words = save(&mut state).expect_err("the save is refused");
        assert!(
            words.contains("is no longer there")
                && words.contains("renamed, moved or deleted outside RSpice"),
            "{words}"
        );
        assert_eq!(
            files_under(&scratch.0),
            on_disk,
            "nothing is written, at the old name or beside it"
        );
        assert!(!file.exists() && !gone.exists());
        let logged: Vec<_> = state
            .model
            .log_buffer
            .entries()
            .skip(log_before)
            .filter(|entry| entry.severity == LogSeverity::Error)
            .map(|entry| entry.message.clone())
            .collect();
        assert!(
            logged.iter().any(|line| line.contains(&words)),
            "the Task log says why: {logged:?}"
        );
        assert!(has_unsaved_changes_fresh(&state), "the work stays unsaved");
        assert_eq!(runs_read(&state), runs, "no run is lost");
    }

    // Save As keeps every run where the reader picks.
    let kept = scratch.0.join("kept").join("amp-kept.rspiceproj");
    std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
    assert!(state.save_project_as_to_path(&kept));
    if state.platform.results_save.is_some() {
        let ctx = egui::Context::default();
        let until = Instant::now() + Duration::from_secs(30);
        while !matches!(
            state.poll_results_save(&ctx),
            Some(SaveContinuationEvent::Saved(_))
        ) {
            assert!(Instant::now() < until, "the results were kept within 30 s");
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    drop(state);
    let state = opened(&kept);
    assert_eq!(runs_read(&state), runs, "Save As keeps every run");
}
