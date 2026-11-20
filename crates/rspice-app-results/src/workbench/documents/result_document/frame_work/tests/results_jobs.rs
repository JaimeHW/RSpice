//! Long Results work runs as a job off the UI thread. The frames drawn while
//! it works read its progress and nothing it works on, and Cancel leaves
//! nothing behind.

use rspice_results::waveform::column_work::ColumnWork;

use crate::workbench::documents::result_document::import::ResultImportJob;
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::results_job::hold_jobs_for_tests;
use crate::workbench::workflows::result_import_workflow::{
    discard_result_import_draft, finish_result_import_job, import_result_dataset_from_path,
    show_result_import_dialog, start_result_import_commit,
};

/// Frames drawn while a job is held, and after it ends.
const FRAMES: usize = 10;

/// An AC sweep of `rows` frequencies and three magnitudes, written as CSV
/// into its own folder: generated, so its every value is known.
fn ac_sweep_csv(rows: usize) -> (std::path::PathBuf, std::path::PathBuf) {
    let folder =
        std::env::temp_dir().join(format!("rspice-results-job-{}-{rows}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("sweep.csv");
    let mut text = String::from("frequency,V(a),V(b),V(c)\n");
    for row in 0..rows {
        let frequency = 1.0 + row as f64;
        text.push_str(&format!(
            "{frequency},{},{},{}\n",
            1.0 / frequency,
            2.0 / frequency,
            3.0 / frequency
        ));
    }
    std::fs::write(&path, text).unwrap();
    (folder, path)
}

/// Draw the import dialog `frames` times; the sample work they did.
fn draw(ctx: &egui::Context, state: &mut ResultsTestHost, frames: usize) -> ColumnWork {
    let before = ColumnWork::read();
    for _ in 0..frames {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| show_result_import_dialog(ctx, &mut state.results_parts()),
        );
    }
    before.since()
}

/// The import's job, while the dialog runs one.
fn import_job(state: &mut ResultsTestHost) -> ResultImportJob {
    state
        .results_parts()
        .result_import
        .job
        .clone()
        .expect("the dialog runs a job")
}

/// Let a held import job work.
fn release(job: &ResultImportJob) {
    match job {
        ResultImportJob::Reading(job) => job.release(),
        ResultImportJob::Committing(job) => job.release(),
    }
}

/// The segment files that hold the staged draft's samples.
fn staged_segments(state: &mut ResultsTestHost) -> Vec<std::path::PathBuf> {
    let mut paths: Vec<_> = state
        .results_parts()
        .result_import
        .waveforms
        .iter()
        .flat_map(|waveform| [&waveform.x, &waveform.y])
        .filter_map(|values| values.stored_window())
        .map(|(source, _)| std::path::PathBuf::from(source.store()))
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// Reading, parsing and keeping a source runs while the dialog draws its
/// progress; the review pages then draw from what the job read, and the
/// commit runs as a job too. No frame reads a sample.
#[test]
fn the_import_dialog_draws_flat_frames_through_its_jobs() {
    let (folder, path) = ac_sweep_csv(50_000);
    let mut state = ResultsTestHost::default();
    state.platform.project_lifecycle.authority.open_session();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    hold_jobs_for_tests(true);
    assert!(import_result_dataset_from_path(
        &mut state.results_parts(),
        &path
    ));
    let reading = import_job(&mut state);
    assert_eq!(draw(&ctx, &mut state, FRAMES), ColumnWork::default());
    assert!(
        state.results_parts().result_import.job.is_some(),
        "the dialog waits on the read"
    );
    release(&reading);
    finish_result_import_job(&mut state.results_parts());
    assert!(state.results_parts().result_import.waveforms.len() == 3);
    assert!(
        !staged_segments(&mut state).is_empty(),
        "the staged samples are kept in segments"
    );

    use crate::workbench::documents::result_document::import::ResultImportStage;
    for stage in [
        ResultImportStage::Detect,
        ResultImportStage::Map,
        ResultImportStage::Validate,
    ] {
        state.results_parts().result_import.stage = stage;
        draw(&ctx, &mut state, 1);
        assert_eq!(
            draw(&ctx, &mut state, FRAMES),
            ColumnWork::default(),
            "{stage:?}"
        );
    }

    start_result_import_commit(&mut state.results_parts()).expect("the draft commits");
    let committing = import_job(&mut state);
    assert_eq!(draw(&ctx, &mut state, FRAMES), ColumnWork::default());
    assert!(state.model.simulation.retained.runs.is_empty());
    release(&committing);
    finish_result_import_job(&mut state.results_parts());
    hold_jobs_for_tests(false);
    std::fs::remove_dir_all(&folder).unwrap();

    assert!(!state.results_parts().result_import.open);
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the import is shown");
    assert_eq!(run.analyses[0].waveforms.len(), 3);
    assert_eq!(run.analyses[0].waveforms[2].y.len(), 50_000);
    assert_eq!(
        run.analyses[0].waveforms[2]
            .y
            .read_range(49_999..50_000)
            .unwrap()[0],
        3.0 / 50_000.0
    );
}

/// Cancel during the read, or on the review it led to, leaves no draft, no
/// run and no segment.
#[test]
fn cancelling_an_import_leaves_nothing_behind() {
    let (folder, path) = ac_sweep_csv(20_000);
    let mut state = ResultsTestHost::default();
    state.platform.project_lifecycle.authority.open_session();

    hold_jobs_for_tests(true);
    assert!(import_result_dataset_from_path(
        &mut state.results_parts(),
        &path
    ));
    let ResultImportJob::Reading(reading) = import_job(&mut state) else {
        panic!("the import reads first");
    };
    discard_result_import_draft(&mut state.results_parts());
    reading.release();
    assert!(reading.wait(std::time::Duration::from_secs(60)));
    assert!(matches!(
        reading.take_outcome(),
        Some(crate::workbench::results_job::JobOutcome::Cancelled)
    ));
    assert!(!state.results_parts().result_import.open);

    assert!(import_result_dataset_from_path(
        &mut state.results_parts(),
        &path
    ));
    release(&import_job(&mut state));
    finish_result_import_job(&mut state.results_parts());
    hold_jobs_for_tests(false);
    let segments = staged_segments(&mut state);
    assert!(!segments.is_empty());
    assert!(segments.iter().all(|segment| segment.is_file()));
    discard_result_import_draft(&mut state.results_parts());
    std::fs::remove_dir_all(&folder).unwrap();

    assert!(
        segments.iter().all(|segment| !segment.exists()),
        "{segments:?}"
    );
    assert!(state.model.simulation.retained.runs.is_empty());
}
