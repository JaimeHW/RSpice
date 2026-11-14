//! A mixed deck the CLI runs runs on the application's prepared manual-deck
//! route with the CLI's numbers.
//!
//! The CLI compiles a deck's Verilog files as one unit, reads a source's
//! local `include headers beside it, and reads a source line written in an
//! included netlist file relative to that file. The Netlist & Script page
//! prepares the same deck through
//! `rspice_simulation::execution::preparation::build_prepared_manual_deck`,
//! which seals those sources by content first. Each deck here runs through
//! `rspice run` and through that route; the time axis and every node
//! voltage and branch current are bit-identical, and every digital event
//! falls at the same time with the same value.

mod common;

use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::time::Duration;

use common::{passed, run_to_json, test_dir};
use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::IncludeProcessor;
use rspice_simulation::analysis_preparation::AnalysisInputs;
use rspice_simulation::error::SimulationError;
use rspice_simulation::execution::preparation::{
    ManualSourceInputs, RunPreparationInputs, build_prepared_manual_deck,
};
use rspice_simulation::execution::{
    HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest, PreparedRunAuthorization,
    prepare_headless_run,
};
use rspice_simulation::manual_deck::build_manual_deck_queue_with_runtimes;
use rspice_simulation::netlist_gen::{NetlistSourceData, NetlistSourceHost, TableRoute};
use rspice_simulation::netlist_preparation::{IncludeSearchChain, seal_deck_hdl_sources};
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation::project_technology::ProjectTechnologyInputs;
use rspice_simulation::results::SimulationResult;
use rspice_vocabulary::product::AnalysisInstanceId;

/// A manual deck reads no schematic source tables; asking for one is a bug.
struct NoSchematicFiles;

impl NetlistSourceHost for NoSchematicFiles {
    fn route_retaining(
        &self,
        _: &str,
        _: Option<&Path>,
        _: Option<&rspice_design::stimulus_library::definition::RetainedPwlFile>,
    ) -> (TableRoute, Option<String>) {
        panic!("a manual deck asked for a schematic source table")
    }
    fn is_file(&self, _: &str) -> std::io::Result<bool> {
        panic!("a manual deck asked for schematic file metadata")
    }
    fn engine_refusal(&self, _: &str) -> Option<String> {
        panic!("a manual deck asked for a schematic source file")
    }
    fn named_file_matches(
        &self,
        _: &str,
        _: &rspice_design::stimulus_library::definition::RetainedPwlFile,
    ) -> Option<bool> {
        panic!("a manual deck asked for a retained-file comparison")
    }
}

/// The deck at `origin` prepared as the Netlist & Script page prepares a
/// saved deck, with no project around it, and run task by task.
fn prepared_manual_run(origin: &Path) -> Vec<SimulationResult> {
    run_tasks(prepared_manual_tasks(origin))
}

/// The tasks of the deck at `origin` prepared as the Netlist & Script page
/// prepares a saved deck.
fn prepared_manual_tasks(
    origin: &Path,
) -> VecDeque<rspice_simulation::execution::AuthorizedTaskDispatch> {
    let snapshot = prepared_manual_snapshot(origin)
        .unwrap_or_else(|error| panic!("{} prepares: {error}", origin.display()));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the snapshot is authorized");
    dispatch.into_tasks()
}

/// The deck at `origin` prepared as the Netlist & Script page prepares a
/// saved deck, with no project around it.
fn prepared_manual_snapshot(
    origin: &Path,
) -> Result<
    rspice_simulation::execution::PreparedRunSnapshot,
    rspice_simulation::execution::PreparationError,
> {
    let source = std::fs::read_to_string(origin).expect("read the deck");
    let inputs: RunPreparationInputs<'_, (), ()> = RunPreparationInputs {
        analysis: AnalysisInputs {
            sim_setup: &Default::default(),
            schematic: &Default::default(),
            selected_components: &Default::default(),
            selection_occurrence: None,
            runs: &[],
            active_run: None,
            project_revision: Default::default(),
            plan_payloads: &[],
        },
        technology: ProjectTechnologyInputs {
            project: &Default::default(),
            sim_setup: &Default::default(),
            models: &Default::default(),
            resolutions: &Default::default(),
            registry: &Default::default(),
            layouts: &Default::default(),
        },
        libraries: &Default::default(),
        project_sources: &Default::default(),
        design_management: &Default::default(),
        configuration_sets: &Default::default(),
        source_data: NetlistSourceData::new(&NoSchematicFiles),
        schematic_file: None,
        schematic_topology_revision: 0,
        imported_checkpoints: &Default::default(),
        imported_transient_checkpoints: &Default::default(),
        display_waveform_cache_samples: 0,
        max_parallel_workers: None,
        library_folders: &[],
        compilation: None,
        design_checks: Default::default(),
    };
    build_prepared_manual_deck(
        &inputs,
        ManualSourceInputs {
            source: &source,
            descriptor: None,
            generated_artifact: None,
            owned: false,
            origin: Some(origin),
        },
    )
}

/// The deck at `origin` prepared on the headless route the conformance
/// runner and `rspice study` run: its HDL sealed, each line an included file
/// writes sealed where it stands, its one `.tran` card the task and the rest
/// the circuit.
fn headless_run(origin: &Path) -> Vec<SimulationResult> {
    run_tasks(headless_tasks(origin))
}

/// The tasks of the deck at `origin` prepared on the headless route.
fn headless_tasks(origin: &Path) -> VecDeque<rspice_simulation::execution::AuthorizedTaskDispatch> {
    let source = std::fs::read_to_string(origin).expect("read the deck");
    let (cards, circuit): (Vec<&str>, Vec<&str>) = source
        .lines()
        .partition(|line| line.trim_start().to_ascii_lowercase().starts_with(".tran"));
    let [card] = cards.as_slice() else {
        panic!("one .tran card, saw {cards:?}");
    };
    let circuit = circuit.join("\n") + "\n";
    let sealed = seal_deck_hdl_sources(&circuit, Some(origin), &IncludeSearchChain::default())
        .unwrap_or_else(|refusal| panic!("the deck's sources seal: {refusal:?}"));
    let expanded = IncludeProcessor::new(origin.parent().expect("a deck directory"))
        .expand_content_rewriting(&sealed.source, origin, &mut |location, _| {
            sealed.included_line(location)
        })
        .expect("the deck's includes expand");
    let body: Vec<&str> = expanded
        .lines()
        .filter(|line| !line.trim().eq_ignore_ascii_case(".end"))
        .collect();
    let queue = build_manual_deck_queue_with_runtimes(
        27.0,
        &format!("{}\n{card}\n.end\n", body.join("\n")),
        &sealed.runtimes,
    )
    .unwrap_or_else(|error| panic!("the card lowers: {error:?}"));
    let tasks = queue
        .into_iter()
        .map(|queued| HeadlessTaskRequest {
            instance_id: AnalysisInstanceId::new(),
            label: "Transient".to_owned(),
            dependencies: Vec::new(),
            analysis: QueuedAnalysis {
                spec: queued.spec,
                config: None,
                spec_options: queued.spec_options,
                analysis_line: (*card).to_owned(),
                numeric_override: None,
            },
        })
        .collect();
    let mut input = HeadlessRunInput::new(&sealed.source, origin, tasks);
    input.resolver = HeadlessSourceResolver::Native {
        include_search_paths: Vec::new(),
    };
    input.veriloga_runtimes = sealed.runtimes;
    input.included_hdl_lines = sealed.included;
    let snapshot = prepare_headless_run(input, &NoAbort)
        .unwrap_or_else(|error| panic!("{} prepares headless: {error}", origin.display()));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the snapshot is authorized");
    dispatch.into_tasks()
}

/// Run dispatched tasks one by one, threading each producer's artifact.
fn run_tasks(
    pending: VecDeque<rspice_simulation::execution::AuthorizedTaskDispatch>,
) -> Vec<SimulationResult> {
    try_run_tasks(pending).unwrap_or_else(|error| panic!("the task solves: {error}"))
}

/// [`run_tasks`], to the first task's error.
fn try_run_tasks(
    mut pending: VecDeque<rspice_simulation::execution::AuthorizedTaskDispatch>,
) -> Result<Vec<SimulationResult>, Box<SimulationError>> {
    let mut artifacts = HashMap::new();
    let mut runner = rspice_simulation::runner::SimulationRunner::new();
    let mut results = Vec::new();
    while let Some(task) = pending.pop_front() {
        let instance = task.instance_id();
        let resolved = task
            .resolve_dependency_artifacts(&artifacts)
            .expect("dependency artifacts resolve");
        let producer = resolved.artifact_producer().expect("an artifact producer");
        runner
            .start_prepared(resolved, false)
            .unwrap_or_else(|error| panic!("the task starts: {error}"));
        let outcome = loop {
            if let Some(outcome) = runner.poll_result() {
                break outcome;
            }
            runner.drain_engine_log();
            std::thread::sleep(Duration::from_millis(5));
        };
        let simulated = outcome.map_err(Box::new)?;
        if let Some(artifact) = producer
            .capture(&simulated, &pending)
            .expect("the artifact is captured")
        {
            artifacts.insert(instance, artifact);
        }
        results.push(simulated);
    }
    Ok(results)
}

/// `rspice run <deck>` to a JSON result document.
fn cli_run(origin: &Path) -> serde_json::Value {
    let output = origin.with_extension("json");
    passed(&run_to_json(origin, &output));
    let text = std::fs::read_to_string(&output).expect("read the CLI result");
    serde_json::from_str(&text).expect("the CLI result is JSON")
}

fn reals(values: &serde_json::Value) -> Vec<f64> {
    values
        .as_array()
        .unwrap_or_else(|| panic!("an array of reals: {values}"))
        .iter()
        .map(|value| value.as_f64().expect("a real"))
        .collect()
}

fn same_bits(what: &str, cli: &[f64], prepared: &[f64]) {
    assert_eq!(cli.len(), prepared.len(), "{what}: sample count");
    for (index, (want, got)) in cli.iter().zip(prepared).enumerate() {
        assert_eq!(
            want.to_bits(),
            got.to_bits(),
            "{what}[{index}]: CLI {want:e}, prepared route {got:e}"
        );
    }
}

/// The XSPICE event code of a CLI digital point's state and strength: the
/// code the prepared route records for the same value.
fn event_code(point: &serde_json::Value) -> u8 {
    use rspice_core::execution::result_document::{DigitalStateTag, DigitalStrengthTag};
    let state: DigitalStateTag =
        serde_json::from_value(point["state"].clone()).expect("a state tag");
    let strength: DigitalStrengthTag =
        serde_json::from_value(point["strength"].clone()).expect("a strength tag");
    rspice_core::xspice::DigitalValue {
        state: state.into(),
        strength: strength.into(),
    }
    .event_code()
}

/// The CLI's transient and the prepared route's agree: the same time axis,
/// every voltage and current bit for bit, and every digital event at the
/// same time with the same value. Returns the digital events of `node`.
fn assert_same_transient(deck: &Path, node: &str) -> Vec<(f64, String)> {
    assert_same_transient_on(deck, node, prepared_manual_run)
}

/// The CLI's transient of `deck` against `route`'s, bit for bit; the events
/// of digital `node`, as the CLI states them.
fn assert_same_transient_on(
    deck: &Path,
    node: &str,
    route: fn(&Path) -> Vec<SimulationResult>,
) -> Vec<(f64, String)> {
    let cli = cli_run(deck);
    let prepared = route(deck);
    let [
        SimulationResult::Transient {
            time,
            waveforms,
            events,
            ..
        },
    ] = prepared.as_slice()
    else {
        panic!("one transient, saw {} results", prepared.len());
    };
    same_bits("time", &reals(&cli["axes"][0]["values"]["values"]), time);
    let signals = cli["signals"].as_array().expect("the CLI's signals");
    assert!(!signals.is_empty());
    for signal in signals {
        let name = signal["descriptor"]["canonicalName"]
            .as_str()
            .expect("a signal name");
        // The prepared route names a node voltage by its node alone.
        let key = name
            .strip_prefix("v(")
            .and_then(|node| node.strip_suffix(')'))
            .unwrap_or(name);
        let waveform = waveforms
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
            .map(|(_, waveform)| waveform)
            .unwrap_or_else(|| {
                panic!(
                    "the prepared route has no {name}: {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                )
            });
        same_bits(
            name,
            &reals(&signal["values"]["samples"]),
            &waveform.y_values,
        );
    }
    let traces = cli["payload"]["digitalTraces"]
        .as_array()
        .expect("the CLI's digital traces");
    assert_eq!(traces.len(), events.digital.len(), "digital traces");
    let mut seen = Vec::new();
    for trace in traces {
        let name = trace["nodeName"].as_str().expect("a node name");
        let history = events
            .digital
            .iter()
            .find(|history| history.node_name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("the prepared route has no digital {name}"));
        let points = trace["points"].as_array().expect("points");
        let cli_events: Vec<(u64, u8)> = points
            .iter()
            .map(|point| {
                let time = point["time"].as_f64().expect("a time");
                (time.to_bits(), event_code(point))
            })
            .collect();
        let prepared_events: Vec<(u64, u8)> = history
            .points
            .iter()
            .map(|point| (point.time_s.to_bits(), point.value_code))
            .collect();
        assert_eq!(cli_events, prepared_events, "digital {name}");
        if name.eq_ignore_ascii_case(node) {
            seen = points
                .iter()
                .map(|point| {
                    let time = point["time"].as_f64().expect("a time");
                    (time, point["state"].as_str().expect("a state").to_owned())
                })
                .collect();
        }
    }
    seen
}

const LEAF: &str = "`timescale 1ns/1ps\nmodule leaf(y);\n  output y;\n  reg y;\n  \
                    initial y = 1'b0;\n  always #5 y = ~y;\nendmodule\n";
const TOP: &str = "`timescale 1ns/1ps\nmodule top(q);\n  output q;\n  wire q;\n  \
                   leaf u1(.y(q));\nendmodule\n";
const CIRCUIT: &str = "X1 q top\nR1 q 0 1k\n.tran 0.1n 20n\n.end\n";

fn write(dir: &Path, name: &str, text: &str) {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("create the directory");
    std::fs::write(path, text).expect("write the file");
}

/// The clock toggles every 5 ns from 0 at time zero.
fn assert_clock(events: &[(f64, String)]) {
    let expected = [
        (0.0, "zero"),
        (5e-9, "one"),
        (10e-9, "zero"),
        (15e-9, "one"),
        (20e-9, "zero"),
    ];
    assert_eq!(events.len(), expected.len(), "{events:?}");
    for ((time, state), (want_time, want_state)) in events.iter().zip(expected) {
        assert!((time - want_time).abs() < 1e-18, "{events:?}");
        assert_eq!(state, want_state, "{events:?}");
    }
}

/// `top` in one file instantiates `leaf` from another: one compilation
/// unit on both routes.
#[test]
fn a_module_of_one_file_instantiating_one_of_another_gives_the_clis_numbers() {
    let dir = test_dir("prepared-unit");
    write(&dir, "rtl/top.v", TOP);
    write(&dir, "lib/leaf.v", LEAF);
    write(
        &dir,
        "deck.cir",
        &format!("two-file unit\n.hdl \"rtl/top.v\"\n.hdl \"lib/leaf.v\"\n{CIRCUIT}"),
    );
    assert_clock(&assert_same_transient(&dir.join("deck.cir"), "q"));
}

/// `leaf.v` reads its half period from a header beside it, which the
/// prepared route seals with it.
#[test]
fn a_local_include_header_gives_the_clis_numbers() {
    let dir = test_dir("prepared-header");
    write(&dir, "models/half.vh", "`define HALF 5\n");
    write(
        &dir,
        "models/leaf.v",
        &format!(
            "`include \"half.vh\"\n{}",
            LEAF.replace("#5", "#`HALF").replace("leaf", "clk")
        ),
    );
    write(
        &dir,
        "deck.cir",
        "header\n.hdl \"models/leaf.v\"\nX1 q clk\nR1 q 0 1k\n.tran 0.1n 20n\n.end\n",
    );
    assert_clock(&assert_same_transient(&dir.join("deck.cir"), "q"));
}

/// The sources are named in an included file, relative to it.
#[test]
fn source_lines_of_an_included_file_give_the_clis_numbers() {
    let dir = test_dir("prepared-included");
    write(&dir, "rtl/top.v", TOP);
    write(&dir, "lib/leaf.v", LEAF);
    write(
        &dir,
        "models/clock.inc",
        "* the clock's sources, named relative to this file\n.hdl \"../rtl/top.v\"\n\
         .hdl \"../lib/leaf.v\"\n",
    );
    write(
        &dir,
        "deck.cir",
        &format!("included sources\n.include \"models/clock.inc\"\n{CIRCUIT}"),
    );
    assert_clock(&assert_same_transient(&dir.join("deck.cir"), "q"));
    // The headless route seals the included file's lines where they stand,
    // as the prepared manual-deck route does.
    assert_clock(&assert_same_transient_on(
        &dir.join("deck.cir"),
        "q",
        headless_run,
    ));
}

/// A two-terminal resistor; an instance with three nodes is refused when
/// the circuit elaborates, at the line that wrote the instance.
const VRES: &str = "`include \"disciplines.vams\"\nmodule vres(p, n);\n  inout p, n;\n  \
                    electrical p, n;\n  analog I(p, n) <+ V(p, n) / 1k;\nendmodule\n";

/// A refusal of an instance an included file writes names that file and
/// line on the prepared routes, as the CLI names it, one file deep and two.
#[test]
fn a_refusal_in_an_included_file_names_the_file_and_line_the_cli_names() {
    let dir = test_dir("prepared-included-refusal");
    write(&dir, "vres.va", VRES);
    write(
        &dir,
        "sub/one.inc",
        "* one file deep\nR1 a 0 1k\nX1 a 0 0 vres\n",
    );
    write(
        &dir,
        "sub/two.inc",
        "* two files deep\nR2 a 0 1k\n.include \"deeper.inc\"\n",
    );
    write(&dir, "sub/deeper.inc", "* the instance\nX2 a 0 0 vres\n");
    for (deck, include, file, line) in [
        ("one.cir", "sub/one.inc", "sub/one.inc", 3),
        ("two.cir", "sub/two.inc", "sub/deeper.inc", 2),
    ] {
        write(
            &dir,
            deck,
            &format!(
                "included refusal\n.hdl \"vres.va\"\nV1 a 0 1\n.include \"{include}\"\n\
                 .tran 1n 2n\n.end\n"
            ),
        );
        let origin = dir.join(deck);
        let cli = common::run_rspice(&["run", origin.to_str().expect("a UTF-8 path")]);
        assert!(
            !cli.status.success(),
            "{deck}: the CLI refuses the instance"
        );
        let cli = String::from_utf8_lossy(&cli.stderr);
        let site = rspice_core::netlist::NetlistSourceLocation::in_file(
            dir.join(file).canonicalize().expect("the included file"),
            line,
        )
        .to_string();
        assert!(cli.contains(&site), "{deck}: the CLI names {site}: {cli}");
        for (route, tasks) in [
            ("prepared", prepared_manual_tasks(&origin)),
            ("headless", headless_tasks(&origin)),
        ] {
            let error = try_run_tasks(tasks).expect_err("the route refuses the instance");
            let SimulationError::Elaboration { location, .. } = &*error else {
                panic!("{deck} {route}: an elaboration refusal, got {error:?}");
            };
            assert_eq!(location.as_deref(), Some(site.as_str()), "{deck} {route}");
            assert!(
                cli.contains(&error.to_string()),
                "{deck} {route}: the CLI said {cli}; the route said {error}"
            );
        }
    }
}

/// A syntax error in an included file names that file and line on the
/// prepared manual-deck route, as the CLI names it, one file deep and two.
#[test]
fn a_syntax_error_in_an_included_file_names_the_file_and_line_the_cli_names() {
    let dir = test_dir("prepared-included-syntax");
    write(
        &dir,
        "sub/one.inc",
        "* one file deep\nR1 a 0 1k\nR2 a 0 zz=\n",
    );
    write(
        &dir,
        "sub/two.inc",
        "* two files deep\nR3 a 0 1k\n.include \"deeper.inc\"\n",
    );
    write(&dir, "sub/deeper.inc", "* the card\nR4 a 0 zz=\n");
    for (deck, include, file, line) in [
        ("one.cir", "sub/one.inc", "sub/one.inc", 3),
        ("two.cir", "sub/two.inc", "sub/deeper.inc", 2),
    ] {
        write(
            &dir,
            deck,
            &format!("included syntax\nV1 a 0 1\n.include \"{include}\"\n.op\n.end\n"),
        );
        let origin = dir.join(deck);
        let cli = common::run_rspice(&["run", origin.to_str().expect("a UTF-8 path")]);
        assert!(!cli.status.success(), "{deck}: the CLI refuses the card");
        let cli = String::from_utf8_lossy(&cli.stderr);
        let site = rspice_core::netlist::NetlistSourceLocation::in_file(
            dir.join(file).canonicalize().expect("the included file"),
            line,
        );
        let said = format!("Syntax error at line {line}: {site}: ");
        assert!(cli.contains(&said), "{deck}: the CLI says {said}: {cli}");
        let error = prepared_manual_snapshot(&origin).expect_err("the route refuses the card");
        let tail = cli[cli.find(&said).expect("found above")..].trim_end();
        assert!(
            error.message().contains(tail),
            "{deck}: the CLI said {tail}; the route said {error}"
        );
    }
}

/// The toggle peer of the `d_process` protocol: it echoes the header, then
/// answers each clocked packet (an 8-byte time and one input byte) with a
/// bit that flips on every call.
#[cfg(unix)]
const TOGGLE_PEER: &str = "#!/bin/sh\nhead -c 3\nq=0\nwhile :; do\n  \
                           n=$(head -c 9 | wc -c)\n  [ $n -eq 9 ] || exit 0\n  \
                           q=$((1 - q))\n  if [ $q -eq 1 ]; then printf '\\001'; \
                           else printf '\\000'; fi\ndone\n";

/// A peer of the same protocol that answers every packet with 0: it never
/// toggles, so a run shows which of the two peers it started.
#[cfg(unix)]
const QUIET_PEER: &str = "#!/bin/sh\nhead -c 3\nwhile :; do\n  \
                          n=$(head -c 9 | wc -c)\n  [ $n -eq 9 ] || exit 0\n  \
                          printf '\\000'\ndone\n";

#[cfg(unix)]
fn write_peer(dir: &Path, name: &str, script: &str) {
    use std::os::unix::fs::PermissionsExt;

    write(dir, name, script);
    std::fs::set_permissions(dir.join(name), std::fs::Permissions::from_mode(0o755))
        .expect("make the peer executable");
}

/// A deck whose `d_process` starts `program`: a 1 MHz clock clocks it, and
/// its output drives an RC load.
#[cfg(unix)]
fn process_deck(program: &str) -> String {
    format!(
        "d_process toggle peer\nvc cntl 0 0.5\naosc cntl dosc osc1\n\
         .model osc1 d_osc(cntl_array=[0 1] freq_array=[1e6 1e6] duty_cycle=0.5 init_phase=0 \
         rise_delay=1e-10 fall_delay=1e-10)\n\
         aproc [dosc] dosc null [d] proc1\n\
         .model proc1 d_process(process_file=\"{program}\" clk_delay=1e-9 reset_delay=1e-9)\n\
         abr [d] [dd] dac1\n\
         .model dac1 dac_bridge(out_low=0 out_high=1 t_rise=1n t_fall=1n)\n\
         r1 dd out 1k\nc1 out 0 1p\n.tran 10n 5u 0 100n\n.end\n"
    )
}

/// The directory this test process puts first on its `PATH`, holding the
/// toggle peer as `rspice-path-toggle` and the quiet peer as
/// `rspice-path-shadowed`. The CLI child inherits the variable, and the
/// prepared route's solve starts its program from this process.
#[cfg(unix)]
fn peers_on_path() {
    static ON_PATH: std::sync::OnceLock<common::TestDirectory> = std::sync::OnceLock::new();
    ON_PATH.get_or_init(|| {
        let dir = test_dir("prepared-process-path");
        write_peer(&dir, "rspice-path-toggle", TOGGLE_PEER);
        write_peer(&dir, "rspice-path-shadowed", QUIET_PEER);
        let path = std::env::var_os("PATH").unwrap_or_default();
        let path = std::env::join_paths(
            std::iter::once(dir.to_path_buf()).chain(std::env::split_paths(&path)),
        )
        .expect("a PATH");
        // SAFETY: the standard library takes its environment lock for every
        // read it makes, including a spawn's, and nothing in this test binary
        // reads the environment outside it.
        unsafe { std::env::set_var("PATH", path) };
        dir
    });
}

/// One toggle a clock delay after each rising edge of the 1 MHz clock.
#[cfg(unix)]
fn assert_toggles(events: &[(f64, String)]) {
    let toggles: Vec<_> = events.iter().skip(1).collect();
    assert_eq!(toggles.len(), 5, "{events:?}");
    for (edge, (time, state)) in toggles.iter().enumerate() {
        let expected = (edge as f64 + 0.5) * 1e-6 + 1e-9;
        assert!((time - expected).abs() < 1e-15, "{events:?}");
        assert_eq!(
            state,
            if edge % 2 == 0 { "one" } else { "zero" },
            "{events:?}"
        );
    }
}

/// A `d_process` names a bare program that stands beside no deck: both
/// routes look it up on `PATH` and give the same events and voltages.
#[cfg(unix)]
#[test]
fn a_bare_d_process_program_on_path_gives_the_clis_numbers() {
    peers_on_path();
    let dir = test_dir("prepared-process-bare");
    write(&dir, "deck.cir", &process_deck("rspice-path-toggle"));
    assert_toggles(&assert_same_transient(&dir.join("deck.cir"), "d"));
}

/// A file beside the deck shadows the program of the same name on `PATH`,
/// on both routes: the toggle peer beside it runs, not the quiet one.
#[cfg(unix)]
#[test]
fn a_d_process_program_beside_the_deck_shadows_one_on_path() {
    peers_on_path();
    let dir = test_dir("prepared-process-shadow");
    write_peer(&dir, "rspice-path-shadowed", TOGGLE_PEER);
    write(&dir, "deck.cir", &process_deck("rspice-path-shadowed"));
    assert_toggles(&assert_same_transient(&dir.join("deck.cir"), "d"));
}

/// A `d_process` names its program relative to the deck: both routes start
/// it from the deck's directory and give the same events and voltages.
#[cfg(unix)]
#[test]
fn a_d_process_program_beside_the_deck_gives_the_clis_numbers() {
    let dir = test_dir("prepared-process");
    write_peer(&dir, "toggle", TOGGLE_PEER);
    write(&dir, "deck.cir", &process_deck("toggle"));
    assert_toggles(&assert_same_transient(&dir.join("deck.cir"), "d"));
}
