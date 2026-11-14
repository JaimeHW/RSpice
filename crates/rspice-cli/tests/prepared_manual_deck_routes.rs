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
use rspice_simulation::analysis_preparation::AnalysisInputs;
use rspice_simulation::execution::PreparedRunAuthorization;
use rspice_simulation::execution::preparation::{
    ManualSourceInputs, RunPreparationInputs, build_prepared_manual_deck,
};
use rspice_simulation::netlist_gen::{NetlistSourceData, NetlistSourceHost, TableRoute};
use rspice_simulation::project_technology::ProjectTechnologyInputs;
use rspice_simulation::results::SimulationResult;

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
    let snapshot = build_prepared_manual_deck(
        &inputs,
        ManualSourceInputs {
            source: &source,
            descriptor: None,
            generated_artifact: None,
            owned: false,
            origin: Some(origin),
        },
    )
    .unwrap_or_else(|error| panic!("{} prepares: {error}", origin.display()));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("the snapshot is authorized");
    let mut pending: VecDeque<_> = dispatch.into_tasks();
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
        let simulated = outcome.unwrap_or_else(|error| panic!("the task solves: {error}"));
        if let Some(artifact) = producer
            .capture(&simulated, &pending)
            .expect("the artifact is captured")
        {
            artifacts.insert(instance, artifact);
        }
        results.push(simulated);
    }
    results
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

/// The result document's state and strength tags of a digital event's
/// value code, as the CLI writes them.
fn state_and_strength(code: u8) -> (String, String) {
    use rspice_core::execution::result_document::{DigitalStateTag, DigitalStrengthTag};
    let value = rspice_core::xspice::DigitalValue::from_event_code(code)
        .unwrap_or_else(|| panic!("{code} is not an event code"));
    let tag = |value: serde_json::Value| value.as_str().expect("a tag").to_owned();
    (
        tag(serde_json::to_value(DigitalStateTag::from(value.state)).expect("a state tag")),
        tag(
            serde_json::to_value(DigitalStrengthTag::from(value.strength)).expect("a strength tag"),
        ),
    )
}

/// The CLI's transient and the prepared route's agree: the same time axis,
/// every voltage and current bit for bit, and every digital event at the
/// same time with the same value. Returns the digital events of `node`.
fn assert_same_transient(deck: &Path, node: &str) -> Vec<(f64, String)> {
    let cli = cli_run(deck);
    let prepared = prepared_manual_run(deck);
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
        let cli_events: Vec<(u64, String, String)> = trace["points"]
            .as_array()
            .expect("points")
            .iter()
            .map(|point| {
                let text = |key: &str| point[key].as_str().expect("a tag").to_owned();
                let time = point["time"].as_f64().expect("a time");
                (time.to_bits(), text("state"), text("strength"))
            })
            .collect();
        let prepared_events: Vec<(u64, String, String)> = history
            .points
            .iter()
            .map(|point| {
                let (state, strength) = state_and_strength(point.value_code);
                (point.time_s.to_bits(), state, strength)
            })
            .collect();
        assert_eq!(cli_events, prepared_events, "digital {name}");
        if name.eq_ignore_ascii_case(node) {
            seen = cli_events
                .into_iter()
                .map(|(time, state, _)| (f64::from_bits(time), state))
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
}
