//! Sealing code-model data files into prepared runs.

use super::*;
use crate::execution::{
    HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest, PreparedRunAuthorization,
    PreparedRunSnapshot, prepare_headless_run,
};
use crate::execution_options::SpecExecutionOptions;
use crate::preparation::QueuedAnalysis;
use crate::results::SimulationResult;
use crate::runner::SimulationRunner;
use rspice_core::NoAbort;
use rspice_core::xspice::DigitalValue;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_vocabulary::product::AnalysisInstanceId;
use std::time::{Duration, Instant};

/// A directory of its own for one test's deck and data files.
struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let directory =
            std::env::temp_dir().join(format!("rspice-data-files-{}", AnalysisInstanceId::new()));
        std::fs::create_dir(&directory).unwrap();
        Self(directory)
    }

    fn write(&self, name: &str, contents: &str) {
        std::fs::write(self.0.join(name), contents).unwrap();
    }

    fn deck(&self) -> PathBuf {
        self.0.join("circuit.cir")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn transient(stop_time: f64) -> HeadlessTaskRequest {
    let spec = AnalysisSpec::Transient {
        stop_time,
        step_time: stop_time / 100.0,
        start_time: 0.0,
        max_timestep: Some(stop_time / 100.0),
        uic: false,
    };
    HeadlessTaskRequest {
        instance_id: AnalysisInstanceId::new(),
        label: "Transient".into(),
        dependencies: Vec::new(),
        analysis: QueuedAnalysis {
            spec,
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line: format!(".tran {} {stop_time}", stop_time / 100.0),
            numeric_override: None,
        },
    }
}

/// Prepare `source` as the native deck `circuit.cir` of `fixture`, as
/// `rspice study` prepares a study's circuit.
fn prepare(
    fixture: &Fixture,
    source: &str,
    task: &HeadlessTaskRequest,
) -> Result<PreparedRunSnapshot, crate::execution::HeadlessPreparationError> {
    fixture.write("circuit.cir", source);
    let deck = fixture.deck();
    let mut input = HeadlessRunInput::new(source, &deck, vec![task.clone()]);
    input.resolver = HeadlessSourceResolver::Native {
        include_search_paths: Vec::new(),
    };
    prepare_headless_run(input, &NoAbort)
}

fn execute(snapshot: PreparedRunSnapshot) -> SimulationResult {
    solve(snapshot).unwrap()
}

fn solve(snapshot: PreparedRunSnapshot) -> Result<SimulationResult, crate::error::SimulationError> {
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap();
    let mut pending = dispatch.into_tasks();
    let task = pending.pop_front().unwrap();
    let mut runner = SimulationRunner::new();
    runner
        .start_prepared(
            task.resolve_dependency_artifacts(&Default::default())
                .unwrap(),
            false,
        )
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if let Some(result) = runner.poll_result() {
            return result;
        }
        assert!(Instant::now() < deadline, "prepared task did not finish");
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// Each change of digital node `node`: its time, bit for bit, and its value.
fn changes(result: &SimulationResult, node: &str) -> Vec<(u64, u8)> {
    let SimulationResult::Transient { events, .. } = result else {
        panic!("transient result")
    };
    events
        .digital
        .iter()
        .find(|history| history.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("no history of {node}: {:?}", events.digital))
        .points
        .iter()
        .map(|point| (point.time_s.to_bits(), point.value_code))
        .collect()
}

fn level(bit: bool) -> u8 {
    if bit {
        DigitalValue::one().event_code()
    } else {
        DigitalValue::zero().event_code()
    }
}

/// A deck whose `d_source` reads `file`. Tests run side by side in one
/// process, where a path is registered for one set of bytes at a time, so
/// each names its own.
fn d_source(file: &str) -> String {
    format!(
        "d_source stimulus\nV1 in 0 1\nR1 in 0 1k\na_src [d] src\n\
         .model src d_source (input_file=\"{file}\")\n.end\n"
    )
}

/// The rows of the stimulus file reach the event net through a prepared run,
/// and it is the bytes preparation read that it replays: the file is
/// rewritten before the run solves.
#[test]
fn a_d_source_replays_the_file_preparation_sealed() {
    let fixture = Fixture::new();
    fixture.write("replayed.txt", "0 0s\n1n 1s\n2n 0s\n");
    let task = transient(3e-9);
    let snapshot = prepare(&fixture, &d_source("replayed.txt"), &task).unwrap();
    assert_eq!(
        snapshot.metadata().sealed_data_files,
        vec![std::fs::canonicalize(fixture.0.join("replayed.txt")).unwrap()]
    );
    fixture.write("replayed.txt", "0 1s\n");
    let result = execute(snapshot);
    assert_eq!(
        changes(&result, "d"),
        [(0.0f64, false), (1e-9, true), (2e-9, false)]
            .map(|(time, bit)| (time.to_bits(), level(bit)))
    );
}

/// A `d_vcd` replays its dump onto the event net at exactly the times the
/// VCD reader gives its ticks, through a prepared run.
#[test]
fn a_d_vcd_replays_its_dump_through_a_prepared_run() {
    let fixture = Fixture::new();
    let mut vcd = String::from(
        "$timescale 1ps $end\n$scope module tb $end\n$var wire 1 ! clk $end\n$upscope $end\n\
         $enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n",
    );
    for tick in 1..=4u64 {
        vcd.push_str(&format!("#{}\n{}!\n", tick * 500, tick % 2));
    }
    fixture.write("clk.vcd", &vcd);
    let document = rspice_core::io::parse_vcd_reader(vcd.as_bytes()).unwrap();
    let expected: Vec<(u64, u8)> = document.signals[0]
        .changes
        .iter()
        .enumerate()
        .map(|(index, change)| {
            let time = document.timescale.seconds_at_tick(change.tick).unwrap();
            (time.to_bits(), level(index % 2 == 1))
        })
        .collect();
    let source = "d_vcd stimulus\nV1 in 0 1\nR1 in 0 1k\na_stim [clk] vstim\n\
                  .model vstim d_vcd (file=\"clk.vcd\" signals=\"tb.clk\")\n.end\n";
    let snapshot = prepare(&fixture, source, &transient(3e-9)).unwrap();
    std::fs::remove_file(fixture.0.join("clk.vcd")).unwrap();
    assert_eq!(changes(&execute(snapshot), "clk"), expected);
}

/// The file's bytes are part of the run's identity: the same bytes prepare
/// the same run, and an edited file a different one.
#[test]
fn an_edited_data_file_is_a_different_run() {
    let fixture = Fixture::new();
    let task = transient(3e-9);
    let source = d_source("edited.txt");
    fixture.write("edited.txt", "0 0s\n1n 1s\n");
    let first = prepare(&fixture, &source, &task).unwrap();
    let again = prepare(&fixture, &source, &task).unwrap();
    fixture.write("edited.txt", "0 0s\n1n 0s\n");
    let edited = prepare(&fixture, &source, &task).unwrap();
    assert_eq!(first.digest(), again.digest());
    assert_ne!(first.digest(), edited.digest());
    assert_eq!(first.executable_netlist(), edited.executable_netlist());
}

/// A file the deck names but preparation cannot read is refused there,
/// naming the instance, the parameter and the path.
#[test]
fn a_missing_data_file_is_refused_at_preparation() {
    let fixture = Fixture::new();
    let error = prepare(&fixture, &d_source("missing.txt"), &transient(3e-9))
        .expect_err("a missing stimulus file is refused")
        .to_string();
    for named in ["'A_SRC'", "input_file=\"missing.txt\"", "cannot be read"] {
        assert!(error.contains(named), "{named}: {error}");
    }
}

/// The schema default names a file too, though the deck spells none.
#[test]
fn a_defaulted_data_file_is_sealed_as_well() {
    let fixture = Fixture::new();
    let source = "default stimulus\nV1 in 0 1\nR1 in 0 1k\na_src [d] src\n\
                  .model src d_source\n.end\n";
    let error = prepare(&fixture, source, &transient(3e-9))
        .expect_err("the default source.txt is missing")
        .to_string();
    assert!(error.contains("source.txt"), "{error}");
    fixture.write("source.txt", "0 1s\n");
    let snapshot = prepare(&fixture, source, &transient(3e-9)).unwrap();
    assert_eq!(
        changes(&execute(snapshot), "d"),
        [(0.0f64.to_bits(), level(true))]
    );
}

/// Preparing a run holds its files only while a check builds a deck, and a
/// plain transient builds none: a run whose file another running simulation
/// holds with other bytes prepares, and only its solve is refused until that
/// simulation lets the path go.
#[test]
fn a_run_solving_other_bytes_does_not_refuse_preparation() {
    let fixture = Fixture::new();
    let path = "contended.txt";
    let solving = PreparedDataFiles {
        files: vec![SealedDataFile {
            path: path.into(),
            contents: "0 1s\n".into(),
            digest: file_digest("0 1s\n"),
            read_from: None,
        }]
        .into(),
    };
    let held = solving.hold(ResourceLimits::default()).unwrap();
    fixture.write(path, "0 0s\n1n 1s\n");
    let task = transient(3e-9);
    let refused = prepare(&fixture, &d_source(path), &task)
        .expect("preparation builds no deck, so holds no file");
    let error = solve(refused)
        .expect_err("the solve needs the path another simulation holds")
        .to_string();
    assert!(
        error.contains("held by another running simulation"),
        "{error}"
    );
    drop(held);
    let snapshot = prepare(&fixture, &d_source(path), &task).unwrap();
    assert_eq!(
        changes(&execute(snapshot), "d"),
        [(0.0f64, false), (1e-9, true)].map(|(time, bit)| (time.to_bits(), level(bit)))
    );
}

/// Carried files are checked against their digests, and two runs in one
/// process share a path only while they agree on its bytes.
#[test]
fn held_files_are_shared_only_by_runs_that_agree_on_their_bytes() {
    let sealed = |contents: &str| PreparedDataFiles {
        files: vec![SealedDataFile {
            path: format!("rspice-held-{}.txt", std::process::id()),
            contents: contents.to_owned(),
            digest: file_digest(contents),
            read_from: None,
        }]
        .into(),
    };
    let limits = ResourceLimits::default();
    let first = sealed("0 0s\n");
    let held = first.hold(limits).unwrap();
    let again = first.hold(limits).unwrap();
    assert!(sealed("0 1s\n").hold(limits).is_err());
    drop(held);
    assert!(sealed("0 1s\n").hold(limits).is_err());
    drop(again);
    drop(sealed("0 1s\n").hold(limits).unwrap());

    let mut tampered = sealed("0 0s\n");
    Arc::get_mut(&mut tampered.files).unwrap()[0].contents = "0 1s\n".into();
    assert!(tampered.validate(limits).is_err());
    let round_trip: PreparedDataFiles =
        serde_json::from_str(&serde_json::to_string(&first).unwrap()).unwrap();
    assert_eq!(round_trip, first);
}
