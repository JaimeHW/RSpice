//! A volume that fills while a run writes its results: the operating system refuses a write of
//! the run's segment (`ENOSPC`), injected under the store's writer, never by filling a disk. The
//! run stops and says so, its segment ends as the store promises, and the earlier runs and the
//! project are as they were.

use std::path::{Path, PathBuf};

use crate::workbench::app_state::AppState;
use rspice_app_runs::simulation::controller::test_execution::{
    run_manual_batch_session, run_manual_deck_in,
};
use rspice_app_state::state::{SimulationRun, SimulationRunLifecycle};
use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticLevel;
use rspice_app_state::workbench::app_state::diagnostic_store::run_notices::{
    STORAGE_FULL_CODE, storage_stop_message,
};
use rspice_results::store_reader::StoreReader;
use rspice_simulation::results_store::write_faults::{self, WriteFault};
use rspice_vocabulary::diagnostics::LogSeverity;

/// Its samples are its own: a kept segment is found by the digest of its samples, so a deck
/// another test runs alongside would read as this one's.
const EARLIER: &str = "results written before the volume filled
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 22n
.tran 0.1u 300u
.end
";

/// Two hundred thousand rows of four columns, several MiB of segment, far past the fault. Its
/// title is `marker`, which arms the fault for this run and no other test's.
fn filled_deck(marker: &str, capacitance: &str) -> String {
    format!(
        "{marker}\nV1 in 0 PULSE(0 1 0 1u 1u 50u 100u)\nR1 in out 1k\nC1 out 0 {capacitance}\n\
         .tran 1n 200u 0 1n\n.end\n"
    )
}

/// Where the volume fills: a quarter of a MiB into the run's segment.
const FILLS_AT: u64 = 256 << 10;

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

/// The bits of every sample `run` holds, signal by signal.
fn sample_bits(run: &SimulationRun) -> Vec<(String, Vec<u64>, Vec<u64>)> {
    run.analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .map(|waveform| {
            let bits = |values: &rspice_results::waveform::WaveformValues| {
                values.samples(..).map(f64::to_bits).collect::<Vec<_>>()
            };
            (waveform.name.clone(), bits(&waveform.x), bits(&waveform.y))
        })
        .collect()
}

/// The segment a stored run's samples are read from.
fn segment_of(run: &SimulationRun) -> PathBuf {
    let waveform = &run.analyses[0].waveforms[0];
    let (source, _) = waveform
        .x
        .stored_window()
        .expect("a solved transient is read from its store");
    PathBuf::from(source.store())
}

/// The run whose results filled the volume, after an earlier run was saved with its project:
/// the session, the project file, the earlier run and the run that filled it.
fn filled_after_a_saved_run(
    scratch: &Scratch,
    (marker, capacitance): (&str, &str),
    fault: WriteFault,
) -> (AppState, PathBuf, SimulationRun, SimulationRun) {
    let mut state: AppState = run_manual_batch_session(EARLIER);
    state.open_project_session();
    let file = scratch.0.join("filled.rspiceproj");
    assert!(state.save_project_to_path(&file), "the project saves");
    let saved = files_under(&scratch.0);
    let earlier = state.model.simulation.retained.runs[0].clone();

    let armed = write_faults::arm(marker, fault);
    let filled = run_manual_deck_in(&mut state, &filled_deck(marker, capacitance));
    drop(armed);
    assert_eq!(
        files_under(&scratch.0),
        saved,
        "a run writes nothing beside the project"
    );
    (state, file, earlier, filled)
}

/// The rows the run wrote before the volume refused one stand as its result, its segment ends
/// with `END` saying the volume filled, the run, its log, its findings and the status bar say so
/// in words, and the earlier run and the saved project are as they were: the project reopens
/// with the earlier run, and saved again it reopens with both, the stopped one still saying how
/// its results storage stopped it.
#[test]
fn a_volume_that_fills_during_a_run_stops_it_in_words_and_leaves_the_project_whole() {
    let scratch = Scratch::new("full-disk");
    let (mut state, file, earlier, filled) = filled_after_a_saved_run(
        &scratch,
        ("volume-fills-under-this-run", "10n"),
        WriteFault {
            past: FILLS_AT,
            closing_fits: true,
        },
    );
    let earlier_bits = sample_bits(&earlier);

    // The run stops where the volume filled, keeping the rows it wrote.
    assert_eq!(filled.lifecycle, SimulationRunLifecycle::Completed);
    let analysis = &filled.analyses[0];
    assert!(analysis.success && analysis.has_data(), "{analysis:?}");
    let stop = filled
        .results_storage_stop()
        .expect("the run says the volume stopped it");
    assert!(stop.budget.is_none(), "a full volume, not a budget");
    let kept = analysis.waveforms[0].x.len();
    assert_eq!(stop.rows, kept as u64);
    assert!(
        kept > 0,
        "the rows written before the volume filled are kept"
    );
    let (_, last) = analysis.waveforms[0].x.ends().expect("rows");
    assert!(
        last < 200e-6 && stop.at == Some(last),
        "stopped at {last}, said {:?}",
        stop.at
    );

    // Its segment ends with END, saying the volume filled.
    let segment = StoreReader::open_file(&segment_of(&filled)).expect("the segment opens");
    assert!(segment.digest().is_some(), "END is written");
    let status = segment
        .with_reader(|reader| Ok(format!("{:?}", reader.state())))
        .unwrap();
    assert!(status.contains("DiskFull"), "{status}");
    assert_eq!(segment.rows(), kept as u64);

    // In words: the run's log, its finding, and the status bar.
    let words = storage_stop_message(stop);
    assert!(
        words.starts_with("Stopped at t = ")
            && words.contains(" filled; ")
            && words.contains(&format!("; {kept} points kept (")),
        "{words}"
    );
    let logged: Vec<&str> = filled
        .log
        .lines()
        .iter()
        .filter(|line| line.severity == LogSeverity::Error)
        .map(|line| line.message.as_str())
        .collect();
    assert_eq!(logged, [format!("{}: {words}", stop.analysis)]);
    assert_eq!(
        state.model.simulation.execution.status,
        "Stopped: results storage full"
    );
    let findings: Vec<_> = state
        .model
        .diagnostics
        .published_run_records(filled.id)
        .iter()
        .filter(|record| record.canonical.code.as_ref() == STORAGE_FULL_CODE)
        .map(|record| (record.level, record.message.clone()))
        .collect();
    assert_eq!(findings, [(DiagnosticLevel::Error, words.clone())]);

    // The earlier run reads as it did.
    let after = state
        .model
        .simulation
        .retained
        .run_by_dataset_id(earlier.dataset_id)
        .expect("the earlier run is kept");
    assert_eq!(sample_bits(after), earlier_bits);

    // The project as saved reopens with the earlier run.
    let mut reopened = AppState::default();
    assert!(
        reopened.load_project_from_path(&file),
        "the project reopens"
    );
    let runs = &reopened.model.simulation.retained.runs;
    assert_eq!(runs.len(), 1);
    assert_eq!(sample_bits(&runs[0]), earlier_bits);
    drop(reopened);

    // Saved again, it reopens with both runs, the stopped one as the rows it kept.
    let filled_bits = sample_bits(&filled);
    assert!(state.save_project_to_path(&file), "the project saves");
    drop(state);
    let mut reopened = AppState::default();
    assert!(
        reopened.load_project_from_path(&file),
        "the project reopens"
    );
    let retained = &reopened.model.simulation.retained;
    assert_eq!(retained.runs.len(), 2);
    assert_eq!(
        sample_bits(retained.run_by_dataset_id(earlier.dataset_id).unwrap()),
        earlier_bits
    );
    let stopped = retained.run_by_dataset_id(filled.dataset_id).unwrap();
    assert_eq!(sample_bits(stopped), filled_bits);
    let logged: Vec<&str> = stopped
        .log
        .lines()
        .iter()
        .filter(|line| line.severity == LogSeverity::Error)
        .map(|line| line.message.as_str())
        .collect();
    assert_eq!(
        logged,
        [format!("{}: {words}", stop.analysis)],
        "its log still says why"
    );
    assert_eq!(
        stopped.results_storage_stop(),
        Some(stop),
        "it reopens saying its results storage stopped it"
    );
}

/// A volume with not even room to close the segment, filled by another writer past the reserve
/// the store keeps for `END`: the run still stops in words with the rows it wrote, kept again in a
/// segment of their own, and the earlier run and the saved project are as they were.
#[test]
fn a_volume_with_no_room_to_close_the_segment_still_stops_the_run_in_words() {
    let scratch = Scratch::new("full-disk-closing");
    let (state, file, earlier, filled) = filled_after_a_saved_run(
        &scratch,
        ("volume-fills-to-its-last-byte", "15n"),
        WriteFault {
            past: FILLS_AT,
            closing_fits: false,
        },
    );

    assert_eq!(filled.lifecycle, SimulationRunLifecycle::Completed);
    let analysis = &filled.analyses[0];
    assert!(analysis.success && analysis.has_data(), "{analysis:?}");
    let stop = filled
        .results_storage_stop()
        .expect("the run says the volume stopped it");
    assert!(stop.budget.is_none(), "a full volume, not a budget");
    let kept = analysis.waveforms[0].x.len();
    assert!(kept > 0 && stop.rows == kept as u64, "{stop:?}");
    let words = storage_stop_message(stop);
    let logged: Vec<&str> = filled
        .log
        .lines()
        .iter()
        .filter(|line| line.severity == LogSeverity::Error)
        .map(|line| line.message.as_str())
        .collect();
    assert_eq!(logged, [format!("{}: {words}", stop.analysis)]);
    assert_eq!(
        state.model.simulation.execution.status,
        "Stopped: results storage full"
    );

    // Its own segment could not be closed, so its rows are kept again in a segment of their own,
    // which ends with END on every row the run wrote.
    let path = segment_of(&filled);
    let segment = StoreReader::open_file(&path).expect("the segment opens");
    assert!(segment.digest().is_some(), "{}", path.display());
    assert_eq!(segment.rows(), kept as u64);
    assert!(
        sample_bits(&filled)
            .iter()
            .all(|(_, x, y)| x.len() == kept && y.len() == kept),
        "the kept rows read"
    );

    let mut reopened = AppState::default();
    assert!(
        reopened.load_project_from_path(&file),
        "the project reopens"
    );
    let runs = &reopened.model.simulation.retained.runs;
    assert_eq!(runs.len(), 1);
    assert_eq!(sample_bits(&runs[0]), sample_bits(&earlier));
}
