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

/// Draw the Results navigator `frames` times; the sample work they did.
fn draw_navigator(ctx: &egui::Context, state: &mut ResultsTestHost, frames: usize) -> ColumnWork {
    use crate::workbench::docks::navigator;
    let before = ColumnWork::read();
    let mut run_details = None;
    for _ in 0..frames {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    navigator::results::results(ui, &mut state.results_parts(), &mut run_details);
                });
            },
        );
    }
    before.since()
}

/// A run of 50,000 stored samples, shown in Results.
fn stored_run(state: &mut ResultsTestHost) {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
    let x: Vec<f64> = (0..50_000).map(|row| row as f64 * 1.0e-9).collect();
    let y: Vec<f64> = x.iter().map(|time| (time * 1.0e6).sin()).collect();
    let mut run = SimulationRun::new(1);
    run.keep_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)".to_owned(), x, y, "#4f81bd"),
        ]),
    );
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
}

/// The first check of a kept segment runs as a job while the navigator
/// draws: the run that reads it says it is being checked, no frame reads a
/// sample, and the check's end leaves the run reading.
#[test]
fn the_navigator_draws_flat_frames_while_a_kept_segment_is_checked() {
    use rspice_results::store_reader::{SegmentDigest, UncheckedSegment, defer_check};
    let mut state = ResultsTestHost::default();
    stored_run(&mut state);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let (source, _) = state.model.simulation.retained.runs[0].analyses[0].waveforms[0]
        .y
        .stored_window()
        .expect("the run keeps its samples in a segment");
    let digest = source.identity().expect("its segment states its digest").0;
    let path = std::path::PathBuf::from(source.store());

    hold_jobs_for_tests(true);
    defer_check(UncheckedSegment {
        digest: SegmentDigest(digest),
        path,
        replace_from: None,
    });
    let checks = &mut state.platform.segment_checks;
    crate::io::result_segments::start_segment_checks(checks);
    hold_jobs_for_tests(false);
    assert_eq!(state.platform.segment_checks.len(), 1);
    draw_navigator(&ctx, &mut state, 1);
    assert_eq!(
        draw_navigator(&ctx, &mut state, FRAMES),
        ColumnWork::default()
    );
    let run = &state.model.simulation.retained.runs[0];
    assert!(run.checking_results().is_some(), "the run is being checked");
    assert_eq!(run.unread_results(), None);

    let check = state.platform.segment_checks[0].clone();
    check.release();
    assert!(check.wait(std::time::Duration::from_secs(60)));
    assert!(!crate::io::result_segments::poll_segment_checks(
        &mut state.model.simulation,
        &mut state.platform.segment_checks
    ));
    let run = &state.model.simulation.retained.runs[0];
    assert_eq!(run.checking_results(), None);
    assert_eq!(run.unread_results(), None);
}

/// Packing a project with its results runs as a job: the frames drawn while
/// it works read no sample, and Cancel leaves no archive.
#[test]
fn the_navigator_draws_flat_frames_while_a_project_is_packed() {
    use crate::io::project_pack::{ProjectPackJob, start_project_pack};
    let folder =
        std::env::temp_dir().join(format!("rspice-results-job-pack-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let project = folder.join("amp.rspiceproj");
    std::fs::write(&project, "{}").unwrap();
    let mut state = ResultsTestHost::default();
    stored_run(&mut state);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    for cancel in [true, false] {
        let archive = folder.join("amp.rspicepack");
        hold_jobs_for_tests(true);
        let pack = start_project_pack(project.clone(), archive.clone());
        hold_jobs_for_tests(false);
        draw_navigator(&ctx, &mut state, 1);
        assert_eq!(
            draw_navigator(&ctx, &mut state, FRAMES),
            ColumnWork::default()
        );
        let ProjectPackJob::Packing { job, .. } = &pack else {
            panic!("a pack is written");
        };
        if cancel {
            pack.cancel();
        }
        job.release();
        assert!(pack.wait(std::time::Duration::from_secs(60)));
        assert_eq!(archive.exists(), !cancel, "cancelled: {cancel}");
    }
    std::fs::remove_dir_all(&folder).unwrap();
}

/// Rows of a kept run's segment the writer is handed at a time.
const KEPT_CHUNK_ROWS: u64 = 1 << 20;

/// Sample `row` of a kept run's signal: a sine of 64 periods over the run, with noise of a tenth
/// of its amplitude drawn from the row, so the store's codec keeps about half its bytes.
fn kept_sample(row: u64, rows: u64) -> f64 {
    let mut z = row.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let noise = (z >> 11) as f64 / (1_u64 << 53) as f64 - 0.5;
    let phase = std::f64::consts::TAU * 64.0 * row as f64 / rows as f64;
    0.1f64.mul_add(noise, phase.sin())
}

/// A transient of `rows` rows kept as a project keeps its runs, under its digest's name in
/// `folder`, and opened as a project opens them: no check has read the file yet, so the run
/// reads as being checked and a check of its own is left to run. The run's time step is 1 ns.
fn kept_unchecked_transient(
    folder: &std::path::Path,
    rows: u64,
) -> (
    crate::state::AnalysisResult,
    rspice_results::store_reader::SegmentDigest,
) {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use rspice_results::store_reader::{ColumnReference, kept_segment_name, opening_unchecked};
    use rspice_results_store::{
        Axis, FileIo, RunMeta, RunStatus, Schema, SegmentWriter, Signal, SignalKind,
    };
    let written = folder.join("written.rsr");
    let schema = Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: vec![Signal::new("V(out)", "V", SignalKind::Real)],
    };
    let io = FileIo::create(&written).expect("the segment is created");
    let mut writer =
        SegmentWriter::create(io, &schema, &RunMeta::default()).expect("the segment starts");
    let (mut x, mut y) = (Vec::new(), Vec::new());
    for first in (0..rows).step_by(KEPT_CHUNK_ROWS as usize) {
        let end = (first + KEPT_CHUNK_ROWS).min(rows);
        x.clear();
        y.clear();
        x.extend((first..end).map(|row| row as f64 * 1.0e-9));
        y.extend((first..end).map(|row| kept_sample(row, rows)));
        writer.push_rows(&x, &y).expect("the rows are written");
    }
    let (receipt, io) = writer.finish(RunStatus::Completed).expect("the run ends");
    drop(io);
    // Copied under its name, so no file of the folder is one this process has checked.
    let kept = folder.join(kept_segment_name(&receipt.digest));
    std::fs::copy(&written, &kept).expect("the segment is kept");
    std::fs::remove_file(&written).expect("the written copy goes");
    rspice_formats::project_results::keep_segments_in(folder);
    let column = |column: u64| {
        let reference = ColumnReference {
            segment: receipt.digest.to_string(),
            column,
            column_rows: rows,
            first: 0,
            rows,
            lead: None,
            ascending: column == 0,
            column_digest: None,
        };
        opening_unchecked(|| reference.values()).expect("the window names its segment")
    };
    let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new("V(out)", column(0), column(1), "#4f81bd").with_unit("V"),
    ]);
    (analysis, receipt.digest)
}

/// Draw the waveform sheet once; how long the frame took.
fn draw_waves(ctx: &egui::Context, state: &mut ResultsTestHost) -> std::time::Duration {
    use crate::workbench::documents::result_document::{prepare_viewer_state, waves};
    let started = std::time::Instant::now();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            prepare_viewer_state(&mut state.results_parts());
            egui::CentralPanel::default()
                .show(ctx, |ui| waves::show(ui, &mut state.results_parts()));
        },
    );
    started.elapsed()
}

/// The X extent of the strip the sheet drew last, if it knows one.
fn drawn_x_range(state: &ResultsTestHost) -> Option<(f64, f64)> {
    state.results.results.models.last_built()?.first()?.x_range
}

/// A reader service that keeps every read it is asked for waiting: what the window draws while
/// its service reads elsewhere, and what a frame reads itself.
struct Waiting;

impl rspice_results::waveform::ReadService for Waiting {
    fn request(
        &self,
        _run: u64,
        _values: &rspice_results::waveform::WaveformValues,
        _x: &rspice_results::waveform::WaveformValues,
        _read: rspice_results::waveform::Read,
    ) -> Option<rspice_results::waveform::ReadValue> {
        None
    }
}

/// Failures reported for reads of the file at `path`.
fn failures_of(path: &std::path::Path) -> Vec<String> {
    rspice_results::waveform::take_column_read_failures()
        .into_iter()
        .filter(|failure| std::path::Path::new(&failure.store) == path)
        .map(|failure| failure.to_string())
        .collect()
}

/// A run whose kept segment waits for its first check is opened and plotted on the window's
/// thread, which never waits for a check: its frames read the run as being checked at once,
/// report no failure, and leave the check to its job, which no frame starts or runs. The check's
/// end reads the run again, and the sheet draws it whole.
#[test]
fn a_plotted_run_waits_for_no_check_of_its_kept_segment() {
    use rspice_results::store_reader::{check_progress, kept_segment_name};
    const ROWS: u64 = 1 << 22;
    rspice_results::store_reader::never_wait_for_checks_here();
    let folder =
        std::env::temp_dir().join(format!("rspice-results-unchecked-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let (analysis, digest) = kept_unchecked_transient(&folder, ROWS);
    let kept = folder.join(kept_segment_name(&digest));
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    hold_jobs_for_tests(true);
    let mut checks = Vec::new();
    crate::io::result_segments::start_segment_checks(&mut checks);
    hold_jobs_for_tests(false);
    assert_eq!(checks.len(), 1, "the open left one check");
    let work = ColumnWork::read();
    rspice_results::waveform::read_through(&Waiting, || {
        for _ in 0..FRAMES {
            draw_waves(&ctx, &mut state);
        }
    });
    assert_eq!(work.since(), ColumnWork::default());
    assert_eq!(
        check_progress(digest.0),
        Some((0, 0)),
        "no frame started or ran the check"
    );
    assert!(!checks[0].is_finished());
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the run is shown");
    assert!(run.checking_results().is_some(), "the run is being checked");
    assert_eq!(run.unread_results(), None);
    assert_eq!(drawn_x_range(&state), None, "nothing is drawn of it yet");
    assert_eq!(failures_of(&kept), Vec::<String>::new());

    checks[0].release();
    assert!(checks[0].wait(std::time::Duration::from_secs(120)));
    assert!(!crate::io::result_segments::poll_segment_checks(
        &mut state.model.simulation,
        &mut checks
    ));
    rspice_results::waveform::read_through(&Waiting, || {
        for _ in 0..2 {
            draw_waves(&ctx, &mut state);
        }
    });
    assert_eq!(
        drawn_x_range(&state),
        Some((0.0, (ROWS - 1) as f64 * 1.0e-9)),
        "the check's end draws the run"
    );
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the run is shown");
    assert_eq!(run.checking_results(), None);
    let y = &run.analyses[0].waveforms[0].y;
    assert_eq!(
        y.read_range((ROWS - 1) as usize..ROWS as usize).unwrap()[0],
        kept_sample(ROWS - 1, ROWS)
    );
    assert_eq!(failures_of(&kept), Vec::<String>::new());
    drop(state);
    std::fs::remove_dir_all(&folder).unwrap();
}

/// The run above at the size a project open was measured at (a segment of about 377 MB on
/// disk), drawn through the app's reader service while its check runs as a job, as the window
/// draws it: no frame waits for the check, and the sheet draws the run once it ends.
#[test]
#[ignore = "writes a segment of about 377 MB; run by hand"]
fn a_plotted_run_of_377_mb_draws_flat_frames_while_its_kept_segment_is_checked() {
    use std::io::Write as _;
    use std::time::{Duration, Instant};
    const ROWS: u64 = 47_000_000;
    rspice_results::store_reader::never_wait_for_checks_here();
    let folder = std::env::temp_dir().join(format!(
        "rspice-results-unchecked-377-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let started = Instant::now();
    let (analysis, digest) = kept_unchecked_transient(&folder, ROWS);
    let written = started.elapsed();
    let kept = folder.join(rspice_results::store_reader::kept_segment_name(&digest));
    let bytes = std::fs::metadata(&kept).unwrap().len();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let reader: &'static crate::state::run_reader::RunReader = Box::leak(Box::new(
        crate::state::run_reader::RunReader::new(crate::state::run_reader::DEFAULT_BUDGET),
    ));

    let mut checks = Vec::new();
    let opened = Instant::now();
    crate::io::result_segments::start_segment_checks(&mut checks);
    let mut frames = Vec::new();
    let mut checked = None;
    let mut drawn = None;
    rspice_results::waveform::read_through(reader, || {
        while drawn.is_none() && opened.elapsed() < Duration::from_secs(600) {
            let running = crate::io::result_segments::poll_segment_checks(
                &mut state.model.simulation,
                &mut checks,
            );
            if !running && checked.is_none() {
                checked = Some(opened.elapsed());
            }
            frames.push(draw_waves(&ctx, &mut state));
            if checked.is_some() && drawn_x_range(&state).is_some() {
                reader.settle();
                frames.push(draw_waves(&ctx, &mut state));
                drawn = Some(opened.elapsed());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    });
    let checked = checked.expect("the check ended");
    let drawn = drawn.expect("the sheet drew the run");
    let mut sorted = frames.clone();
    sorted.sort_unstable();
    let worst = *sorted.last().unwrap();
    let millis = |duration: Duration| duration.as_secs_f64() * 1e3;
    writeln!(
        std::io::stdout().lock(),
        "{ROWS} rows, {bytes} bytes on disk, written in {:.1} s | checked {:.0} ms after the \
         open, drawn after {:.0} ms | {} frames, median {:.1} ms, worst {:.1} ms",
        written.as_secs_f64(),
        millis(checked),
        millis(drawn),
        frames.len(),
        millis(sorted[sorted.len() / 2]),
        millis(worst),
    )
    .unwrap();
    assert!(
        worst * 4 < checked,
        "a frame of {worst:?} waited on a check of {checked:?}"
    );
    assert_eq!(
        drawn_x_range(&state),
        Some((0.0, (ROWS - 1) as f64 * 1.0e-9))
    );
    assert_eq!(failures_of(&kept), Vec::<String>::new());
    drop(state);
    std::fs::remove_dir_all(&folder).unwrap();
}
