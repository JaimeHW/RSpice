//! A headless task graph runs a small-signal card around a transient state.
//!
//! The card names the state with `ACTIMES=` (the transient it depends on
//! runs to the time) or `CHECKPOINT=` (a file in the circuit's directory,
//! read once at preparation and carried to the worker sealed). Both name the
//! same accepted state of a diode charging a capacitor through a resistor,
//! so both linearize alike, and neither linearizes like the operating point.
//!
//! Every small-signal kind runs headless, and each one's result equals what
//! `rspice run` computes for the same deck: the engine's state entry, which
//! the command line calls with the deck's own engine.

use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::NoAbort;
use rspice_core::engine::SmallSignalCardResult;
use rspice_simulation::analysis_preparation::analysis_spec_to_config;
use rspice_simulation::execution::{
    HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest, PreparedRunAuthorization,
    prepare_headless_run,
};
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation::results::{SimulationResult, TransferFunctionScalar};
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

/// The same charging circuit with an inverting loop closed around `out`
/// through the probe `vp`, and a distortion tone on its source, so that every
/// small-signal kind has something to read; the diode's conductance at the
/// state sets them all.
const LOOP_CIRCUIT: &str = "\
* diode-RC charging inside a loop
v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1 distof1 0.01
r1 in a 10k
d1 a out dmod
c1 out 0 1n
r2 out 0 1meg
e1 eo 0 out 0 -10
vp eo x 0
r3 x out 100k
.model dmod d is=1e-14 n=1.2 cjo=5p
";

/// The charging circuit driven through a port its deck declares. A `.SP`
/// card's own `PORT<k>=` planes add terminations the transient never had, so
/// a card around a state reads the deck's ports, which every elaboration of
/// the deck carries.
const PORT_CIRCUIT: &str = "\
* diode-RC charging driven through a port
v1 in 0 pulse(0 1 0 100n 100n 1 2) ac 1 portnum=1 z0=50
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

/// `circuit`, and the state its transient reaches at 20 us written to a
/// checkpoint file beside it, in a directory of the test's own.
fn directory_for(circuit: &str, test: &str) -> Directory {
    let directory = Directory(std::env::temp_dir().join(format!(
        "rspice-headless-transient-state-{test}-{}",
        std::process::id()
    )));
    std::fs::create_dir_all(&directory.0).expect("create the circuit's directory");
    std::fs::write(directory.0.join("circuit.cir"), circuit).expect("write the circuit");
    let writer = rspice_core::Netlist::parse(&format!("{circuit}{TRAN}\n.end\n"))
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

fn directory(test: &str) -> Directory {
    directory_for(CIRCUIT, test)
}

/// The typed task a card in `circuit` lowers to, with `line` as the card it
/// carries.
fn task_in(
    circuit: &str,
    card: &str,
    line: &str,
    dependencies: Vec<AnalysisInstanceId>,
) -> HeadlessTaskRequest {
    let deck = format!("{circuit}{card}\n.end\n");
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

fn task(card: &str, line: &str, dependencies: Vec<AnalysisInstanceId>) -> HeadlessTaskRequest {
    task_in(CIRCUIT, card, line, dependencies)
}

fn run_in(
    circuit: &str,
    origin: &Path,
    tasks: Vec<HeadlessTaskRequest>,
) -> HashMap<AnalysisInstanceId, SimulationResult> {
    let mut input = HeadlessRunInput::new(circuit, origin, tasks);
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

fn run(
    origin: &Path,
    tasks: Vec<HeadlessTaskRequest>,
) -> HashMap<AnalysisInstanceId, SimulationResult> {
    run_in(CIRCUIT, origin, tasks)
}

/// The `v(out)` phasors of an AC result.
fn v_out(result: &SimulationResult) -> Vec<(f64, f64)> {
    let SimulationResult::Ac { waveforms, .. } = result else {
        panic!("an AC result: {result:?}");
    };
    complex_waveform(waveforms, "v(out)")
}

/// One complex waveform of a frequency-domain result, by name.
fn complex_waveform(
    waveforms: &HashMap<String, rspice_simulation::results::WaveformData>,
    name: &str,
) -> Vec<(f64, f64)> {
    let waveform = waveforms
        .iter()
        .find(|(retained, _)| retained.eq_ignore_ascii_case(name))
        .map(|(_, waveform)| waveform)
        .unwrap_or_else(|| panic!("{name} among {:?}", waveforms.keys().collect::<Vec<_>>()));
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

/// `card` in the loop circuit around the state at 20 us, headless, keyed by
/// the state's spelling (`actimes=20u` or the checkpoint's), and as
/// `rspice run` computes it.
fn at_state(test: &str, card: &str) -> (Vec<SimulationResult>, SmallSignalCardResult) {
    at_state_in(LOOP_CIRCUIT, test, card)
}

/// [`at_state`] in `circuit`.
fn at_state_in(
    circuit: &str,
    test: &str,
    card: &str,
) -> (Vec<SimulationResult>, SmallSignalCardResult) {
    let directory = directory_for(circuit, test);
    let origin = directory.0.join("circuit.cir");
    let transient = task_in(circuit, TRAN, &analysis_spec_line(TRAN), Vec::new());
    let parent = transient.instance_id;
    let stated = format!("{card} actimes=20u");
    let live = task_in(circuit, card, &stated, vec![parent]);
    let read = task_in(
        circuit,
        card,
        &format!("{card} checkpoint=\"{CHECKPOINT}\""),
        vec![parent],
    );
    let ids = [live.instance_id, read.instance_id];
    let mut results = run_in(circuit, &origin, vec![transient, live, read]);
    let headless = ids
        .iter()
        .map(|id| results.remove(id).expect("each state task has a result"))
        .collect();
    (headless, on_the_command_line(circuit, &stated))
}

/// The one result `rspice run` computes for `stated`, a card around one
/// state, in `circuit` with the transient it names: the engine's state
/// entry, at the time the deck spells.
fn on_the_command_line(circuit: &str, stated: &str) -> SmallSignalCardResult {
    let deck = rspice_core::Netlist::parse(&format!("{circuit}{TRAN}\n{stated}\n.end\n"))
        .expect("the deck parses");
    let [request] = deck.small_signal_states.as_slice() else {
        panic!("{stated}: one card around a state");
    };
    let rspice_core::netlist::SmallSignalStateSource::TransientTimes(times) = &request.source
    else {
        panic!("{stated}: a card at a time");
    };
    let mut expected = rspice_core::Engine::default()
        .run_small_signal_at_transient_times(&deck, request.analysis_index, times, &NoAbort)
        .unwrap_or_else(|error| panic!("{stated}: the engine runs the state: {error}"));
    assert_eq!(expected.len(), 1, "{stated}: one state");
    let expected = expected.remove(0);
    assert!(
        (expected.state_time - times[0]).abs() <= 1e-15,
        "{stated}: the state's time"
    );
    expected.result
}

fn assert_close(what: &str, actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-9 * expected.abs().max(1e-30),
        "{what}: {actual} headless against {expected}"
    );
}

fn assert_phasors_close(what: &str, actual: &[(f64, f64)], expected: &[(f64, f64)]) {
    assert_eq!(actual.len(), expected.len(), "{what}: point count");
    for ((actual_re, actual_im), (expected_re, expected_im)) in actual.iter().zip(expected) {
        let scale = expected_re.hypot(*expected_im).max(1e-30);
        assert!(
            (actual_re - expected_re).hypot(actual_im - expected_im) <= 1e-9 * scale,
            "{what}: ({actual_re}, {actual_im}) headless against ({expected_re}, {expected_im})"
        );
    }
}

#[test]
fn a_headless_tf_card_at_a_state_equals_the_command_line() {
    let (headless, expected) = at_state("tf", ".tf v(out) v1");
    let SmallSignalCardResult::TransferFunction(expected) = expected else {
        panic!("a .TF result: {expected:?}");
    };
    for result in &headless {
        let SimulationResult::TransferFunction {
            gain,
            input_resistance,
            output_resistance,
            ..
        } = result
        else {
            panic!("a transfer-function result: {result:?}");
        };
        for (what, actual, expected) in [
            ("gain", gain, expected.gain),
            (
                "input resistance",
                input_resistance,
                expected.input_impedance,
            ),
            (
                "output resistance",
                output_resistance,
                expected.output_impedance,
            ),
        ] {
            let Some(TransferFunctionScalar::Finite(actual)) = actual else {
                panic!("{what}: {actual:?}");
            };
            assert_close(what, *actual, expected);
        }
    }
}

#[test]
fn a_headless_sp_card_at_a_state_equals_the_command_line() {
    let (headless, expected) = at_state_in(PORT_CIRCUIT, "sp", ".sp dec 2 1k 1meg");
    let SmallSignalCardResult::SParameters(expected) = expected else {
        panic!("an .SP result: {expected:?}");
    };
    let expected: Vec<(f64, f64)> = expected
        .scattering
        .data
        .iter()
        .map(|point| {
            let s11 = point.get(1, 1);
            (s11.re, s11.im)
        })
        .collect();
    for result in &headless {
        let SimulationResult::Ac { waveforms, .. } = result else {
            panic!("an S-parameter result: {result:?}");
        };
        assert_phasors_close("S11", &complex_waveform(waveforms, "S11"), &expected);
    }
}

#[test]
fn a_headless_stb_card_at_a_state_equals_the_command_line() {
    let (headless, expected) = at_state("stb", ".stb dec 2 1k 1meg probe=vp");
    let SmallSignalCardResult::Stability(expected) = expected else {
        panic!("an .STB result: {expected:?}");
    };
    let expected: Vec<(f64, f64)> = expected
        .result
        .bode_points
        .iter()
        .map(|point| (point.loop_gain.re, point.loop_gain.im))
        .collect();
    for result in &headless {
        let SimulationResult::Stb { response, .. } = result else {
            panic!("a stability result: {result:?}");
        };
        let actual: Vec<(f64, f64)> = response
            .bode_points
            .iter()
            .map(|point| (point.loop_gain.re, point.loop_gain.im))
            .collect();
        assert_phasors_close("loop gain", &actual, &expected);
    }
}

#[test]
fn a_headless_pz_card_at_a_state_equals_the_command_line() {
    let (headless, expected) = at_state("pz", ".pz in 0 out 0 vol pz");
    let SmallSignalCardResult::PoleZero(expected) = expected else {
        panic!("a .PZ result: {expected:?}");
    };
    let roots = |roots: &[num_complex::Complex64]| -> Vec<(f64, f64)> {
        roots.iter().map(|root| (root.re, root.im)).collect()
    };
    assert!(!expected.poles.is_empty(), "the circuit has poles");
    for result in &headless {
        let SimulationResult::PoleZero {
            poles, zeros, gain, ..
        } = result
        else {
            panic!("a pole-zero result: {result:?}");
        };
        assert_phasors_close("poles", poles, &roots(&expected.poles));
        assert_phasors_close("zeros", zeros, &roots(&expected.zeros));
        match (gain, expected.dc_gain) {
            (Some(actual), Some(expected)) => assert_close("DC gain", *actual, expected),
            (None, None) => {}
            other => panic!("DC gain: {other:?}"),
        }
    }
}

#[test]
fn a_headless_sens_card_at_a_state_equals_the_command_line() {
    // A .SENS card reruns the state's transient once per perturbation, and a
    // checkpoint holds the unperturbed state only: the engine refuses that
    // form, so this card runs around the live state alone.
    let card = ".sens v(out) r1 ac lin 1 100k 100k";
    let directory = directory_for(LOOP_CIRCUIT, "sens");
    let origin = directory.0.join("circuit.cir");
    let transient = task_in(LOOP_CIRCUIT, TRAN, &analysis_spec_line(TRAN), Vec::new());
    let stated = format!("{card} actimes=20u");
    let live = task_in(LOOP_CIRCUIT, card, &stated, vec![transient.instance_id]);
    let id = live.instance_id;
    let results = run_in(LOOP_CIRCUIT, &origin, vec![transient, live]);
    let expected = on_the_command_line(LOOP_CIRCUIT, &stated);
    let SmallSignalCardResult::Sensitivity(expected) = expected else {
        panic!("a .SENS result: {expected:?}");
    };
    let rspice_core::engine::SensitivityCardResult::Ac(expected) = *expected else {
        panic!("an AC sensitivity");
    };
    let SimulationResult::SensitivityStudy { evidence } = &results[&id] else {
        panic!("a sensitivity result: {:?}", results[&id]);
    };
    assert!(!evidence.rows.is_empty(), "the study has rows");
    assert_eq!(evidence.rows.len(), expected.sensitivities.len());
    for row in &evidence.rows {
        let trace = expected
            .sensitivities
            .iter()
            .find(|trace| trace.vector_name == row.parameter)
            .unwrap_or_else(|| panic!("{} among the engine's rows", row.parameter));
        assert_eq!(row.raw.len(), trace.magnitude.len(), "{}", row.parameter);
        for (actual, expected) in row.raw.iter().zip(&trace.magnitude) {
            match (actual, expected) {
                (
                    rspice_core::analysis::sensitivity::SensitivityValue::Available(actual),
                    rspice_core::analysis::sensitivity::SensitivityValue::Available(expected),
                ) => assert_close(&row.parameter, *actual, *expected),
                other => panic!("{}: {other:?}", row.parameter),
            }
        }
    }
}

#[test]
fn a_headless_disto_card_at_a_state_equals_the_command_line() {
    let (headless, expected) = at_state("disto", ".disto dec 2 1k 1meg");
    let SmallSignalCardResult::Distortion(expected) = expected else {
        panic!("a .DISTO result: {expected:?}");
    };
    let out = expected.points[0]
        .fundamental_f1
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("node out");
    let fundamental: Vec<(f64, f64)> = expected
        .points
        .iter()
        .map(|point| {
            let value = point.fundamental_f1.voltages[out];
            (value.re, value.im)
        })
        .collect();
    let second = rspice_core::analysis::DistortionProduct::SecondHarmonic;
    let ratio: Vec<(f64, f64)> = expected
        .points
        .iter()
        .map(|point| {
            let product = point.product(second).expect("a second harmonic");
            let value = product.response.voltages[out] / point.fundamental_f1.voltages[out];
            (value.re, value.im)
        })
        .collect();
    for result in &headless {
        let SimulationResult::Ac { waveforms, .. } = result else {
            panic!("a distortion result: {result:?}");
        };
        assert_phasors_close(
            "V(out) F1",
            &complex_waveform(waveforms, "V(out) F1"),
            &fundamental,
        );
        assert_phasors_close(
            "V(out) HD2/F1",
            &complex_waveform(waveforms, &format!("V(out) {}/F1", second.label())),
            &ratio,
        );
    }
}
