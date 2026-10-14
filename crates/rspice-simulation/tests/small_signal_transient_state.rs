//! A headless task graph runs a small-signal card around a transient state.
//!
//! The card names the state with `ACTIMES=` (the transient it depends on
//! runs to the time) or `CHECKPOINT=` (a file in the circuit's directory,
//! read once at preparation and carried to the worker sealed). Both name the
//! same accepted state of a diode charging a capacitor through a resistor,
//! so both linearize alike, and neither linearizes like the operating point.

use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::NoAbort;
use rspice_simulation::analysis_preparation::analysis_spec_to_config;
use rspice_simulation::execution::{
    HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest, PreparedRunAuthorization,
    prepare_headless_run,
};
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation::results::SimulationResult;
use rspice_simulation::runner::SimulationRunner;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const CIRCUIT: &str = "\
* diode-RC charging
v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1
r1 in a 10k
d1 a out dmod
c1 out 0 1n
r2 out 0 1meg
.model dmod d is=1e-14 n=1.2 cjo=5p
";

const TRAN: &str = ".tran 0.1u 20u 0 0.1u";
const AC: &str = ".ac dec 2 1k 1meg";
const CHECKPOINT: &str = "diode_state.rspice-tran";

struct Directory(PathBuf);

impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The circuit, and the state its transient reaches at 20 us written to a
/// checkpoint file beside it, in a directory of the test's own.
fn directory(test: &str) -> Directory {
    let directory = Directory(std::env::temp_dir().join(format!(
        "rspice-headless-transient-state-{test}-{}",
        std::process::id()
    )));
    std::fs::create_dir_all(&directory.0).expect("create the circuit's directory");
    std::fs::write(directory.0.join("circuit.cir"), CIRCUIT).expect("write the circuit");
    let writer = rspice_core::Netlist::parse(&format!("{CIRCUIT}{TRAN}\n.end\n"))
        .expect("the writer deck parses");
    let Some(rspice_core::netlist::AnalysisCommand::Tran {
        step,
        stop,
        start,
        max_step,
        ..
    }) = writer.analyses.first()
    else {
        panic!("the writer deck has a .TRAN card");
    };
    let max_step =
        rspice_core::execution::resolve_transient_maximum_step(*step, *stop, *start, *max_step)
            .expect("the window resolves");
    let (_, checkpoint) = rspice_core::Engine::default()
        .run_tran_checkpointed(&writer, *stop, max_step)
        .expect("the transient checkpoints");
    checkpoint
        .save(&directory.0.join(CHECKPOINT))
        .expect("the checkpoint saves");
    directory
}

/// The typed task a card lowers to, with `line` as the card it carries.
fn task(card: &str, line: &str, dependencies: Vec<AnalysisInstanceId>) -> HeadlessTaskRequest {
    let deck = format!("{CIRCUIT}{card}\n.end\n");
    let mut queue = rspice_simulation::manual_deck::build_manual_deck_queue(27.0, &deck)
        .unwrap_or_else(|errors| panic!("{card} lowers: {errors:?}"));
    assert_eq!(queue.len(), 1, "{card}");
    let queued = queue.remove(0);
    HeadlessTaskRequest {
        instance_id: AnalysisInstanceId::new(),
        label: queued.spec.run_type().display_name().to_owned(),
        dependencies,
        analysis: QueuedAnalysis {
            analysis_line: line.to_owned(),
            ..queued
        },
    }
}

fn run(
    origin: &Path,
    tasks: Vec<HeadlessTaskRequest>,
) -> HashMap<AnalysisInstanceId, SimulationResult> {
    let mut input = HeadlessRunInput::new(CIRCUIT, origin, tasks);
    input.resolver = HeadlessSourceResolver::Native {
        include_search_paths: Vec::new(),
    };
    let snapshot = prepare_headless_run(input, &NoAbort)
        .unwrap_or_else(|error| panic!("the headless run prepares: {error}"));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the run is authorized");
    let mut artifacts = HashMap::new();
    let mut results = HashMap::new();
    let mut runner = SimulationRunner::new();
    let mut pending = dispatch.into_tasks();
    while let Some(task) = pending.pop_front() {
        let identity = task.instance_id();
        let task = task
            .resolve_dependency_artifacts(&artifacts)
            .expect("dependencies resolve");
        let producer = task.artifact_producer().expect("artifact producer");
        runner.start_prepared(task, false).expect("the task starts");
        let deadline = Instant::now() + Duration::from_secs(120);
        let result = loop {
            if let Some(result) = runner.poll_result() {
                break result.unwrap_or_else(|error| panic!("the task runs: {error}"));
            }
            assert!(
                Instant::now() < deadline,
                "the headless task did not finish"
            );
            std::thread::sleep(Duration::from_millis(2));
        };
        if let Some(artifact) = producer.capture(&result, &pending).expect("capture") {
            artifacts.insert(identity, artifact);
        }
        results.insert(identity, result);
    }
    results
}

/// The `v(out)` phasors of an AC result.
fn v_out(result: &SimulationResult) -> Vec<(f64, f64)> {
    let SimulationResult::Ac { waveforms, .. } = result else {
        panic!("an AC result: {result:?}");
    };
    let waveform = waveforms
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("v(out)"))
        .map(|(_, waveform)| waveform)
        .expect("v(out)");
    let imaginary = waveform.y_imag.as_ref().expect("a complex waveform");
    waveform
        .y_values
        .iter()
        .copied()
        .zip(imaginary.iter().copied())
        .collect()
}

#[test]
fn a_headless_card_runs_around_a_live_state_and_a_checkpoints_alike() {
    let directory = directory("alike");
    let origin = directory.0.join("circuit.cir");
    let transient = task(TRAN, &analysis_spec_line(TRAN), Vec::new());
    let parent = transient.instance_id;
    let live = task(AC, &format!("{AC} actimes=20u"), vec![parent]);
    let read = task(
        AC,
        &format!("{AC} checkpoint=\"{CHECKPOINT}\""),
        vec![parent],
    );
    let operating_point = task(AC, AC, Vec::new());
    let ids = [
        live.instance_id,
        read.instance_id,
        operating_point.instance_id,
    ];
    let results = run(&origin, vec![transient, live, read, operating_point]);
    let [live, read, operating_point] = ids.map(|id| v_out(&results[&id]));

    assert_eq!(live.len(), read.len());
    for ((live_re, live_im), (read_re, read_im)) in live.iter().zip(&read) {
        let scale = live_re.hypot(*live_im);
        assert!(
            (live_re - read_re).hypot(live_im - read_im) <= 1e-9 * scale,
            "the checkpoint's state ({read_re}, {read_im}) against the live state ({live_re}, \
             {live_im})"
        );
    }
    assert_ne!(
        live, operating_point,
        "the state at 20 us is not the operating point"
    );
}

#[test]
fn a_sealed_source_bundle_carries_no_checkpoint_file() {
    let directory = directory("sealed");
    let origin = directory.0.join("circuit.cir");
    let transient = task(TRAN, &analysis_spec_line(TRAN), Vec::new());
    let read = task(
        AC,
        &format!("{AC} checkpoint=\"{CHECKPOINT}\""),
        vec![transient.instance_id],
    );
    let error = prepare_headless_run(
        HeadlessRunInput::new(CIRCUIT, &origin, vec![transient, read]),
        &NoAbort,
    )
    .expect_err("a sealed bundle reads no checkpoint file");
    assert!(
        error.to_string().contains("sealed source bundle"),
        "{error}"
    );
}

/// The card a study document carries for `card`'s typed request.
fn analysis_spec_line(card: &str) -> String {
    let queued = task(card, card, Vec::new());
    let AnalysisSpec::Transient { .. } = queued.analysis.spec else {
        panic!("{card} is a transient");
    };
    analysis_spec_to_config(&queued.analysis.spec)
        .expect("the card renders")
        .to_spice()
}
