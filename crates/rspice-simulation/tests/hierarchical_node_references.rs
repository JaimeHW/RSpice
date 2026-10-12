//! A subcircuit-internal node referenced from the top level by its flattened
//! name is one signal on the headless path too: the path the mixed-signal
//! corpus drives, where a duplicate identity used to fail every analysis
//! ("returned duplicate signal identities").
//!
//! The deck spells the reference three ways against mixed-case instances, so
//! the canonical name (`xOut.xIn.MID`) matches none of them byte for byte.

use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::NoAbort;
use rspice_simulation::analysis_preparation::analysis_spec_to_config;
use rspice_simulation::execution::{
    HeadlessRunInput, HeadlessTaskRequest, PreparedRunAuthorization, prepare_headless_run,
};
use rspice_simulation::prepared_dependency::{ExecutionArtifactKind, required_artifact_kinds};
use rspice_simulation::results::SimulationResult;
use rspice_simulation::runner::SimulationRunner;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const CIRCUIT: &str = "\
* hierarchical node references
.subckt inner a b
r1 a mid 1k
r2 mid b 1k
c1 mid b 1n
.ends inner
.subckt outer p q
xIn p q inner
.ends outer
v1 top 0 dc 2 ac 1
xOut top 0 outer
rload xout.xin.mid 0 2k
rcaps XOUT.XIN.MID 0 2k
rmixed Xout.Xin.Mid 0 2k
.meas tran lower find v(xout.xin.mid) at=1.5u
.meas tran upper find v(XOUT.XIN.MID) at=1.5u
.meas tran mixed find v(Xout.Xin.Mid) at=1.5u
";

const CARDS: &str = ".op\n.ac lin 1 100k 100k\n.tran 10n 2u\n";

/// With the three loads on it, the internal node sits at 4/7 V.
const MID_DC: f64 = 4.0 / 7.0;

fn origin() -> std::path::PathBuf {
    std::env::temp_dir()
        .join("rspice-hierarchical-references")
        .join("circuit.cir")
}

/// The deck's cards as the explicit task graph a study would carry.
fn tasks() -> Vec<HeadlessTaskRequest> {
    let deck = format!("{CIRCUIT}{CARDS}.end\n");
    let queue = rspice_simulation::manual_deck::build_manual_deck_queue(27.0, &deck)
        .unwrap_or_else(|errors| panic!("cards lower: {errors:?}"));
    let mut tasks = queue
        .into_iter()
        .map(|mut queued| {
            queued.analysis_line = analysis_spec_to_config(&queued.spec)
                .expect("card renders")
                .to_spice();
            HeadlessTaskRequest {
                instance_id: AnalysisInstanceId::new(),
                label: queued.spec.run_type().display_name().to_owned(),
                dependencies: Vec::new(),
                analysis: queued,
            }
        })
        .collect::<Vec<_>>();
    let operating_points = tasks
        .iter()
        .filter(|task| {
            matches!(
                task.analysis.spec,
                AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
            )
        })
        .map(|task| task.instance_id)
        .collect::<Vec<_>>();
    for task in &mut tasks {
        let needs_seed = required_artifact_kinds(&task.analysis.spec, &task.analysis.spec_options)
            .contains(&ExecutionArtifactKind::DcOperatingPointSeed);
        if needs_seed
            && let [producer] = operating_points.as_slice()
            && *producer != task.instance_id
        {
            task.dependencies = vec![*producer];
        }
    }
    tasks
}

fn run() -> Vec<SimulationResult> {
    let path = origin();
    let snapshot = prepare_headless_run(HeadlessRunInput::new(CIRCUIT, &path, tasks()), &NoAbort)
        .unwrap_or_else(|error| panic!("headless run prepares: {error:?}"));
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .expect("run is authorized");
    let mut artifacts = HashMap::new();
    let mut results = Vec::new();
    let mut runner = SimulationRunner::new();
    let mut pending = dispatch.into_tasks();
    while let Some(task) = pending.pop_front() {
        let identity = task.instance_id();
        let task = task
            .resolve_dependency_artifacts(&artifacts)
            .expect("dependencies resolve");
        let producer = task.artifact_producer().expect("artifact producer");
        runner.start_prepared(task, false).expect("task starts");
        let deadline = Instant::now() + Duration::from_secs(120);
        let result = loop {
            if let Some(result) = runner.poll_result() {
                break result.unwrap_or_else(|error| panic!("task runs: {error}"));
            }
            assert!(Instant::now() < deadline, "headless task did not finish");
            std::thread::sleep(Duration::from_millis(2));
        };
        if let Some(artifact) = producer.capture(&result, &pending).expect("capture") {
            artifacts.insert(identity, artifact);
        }
        results.push(result);
    }
    results
}

/// The one key in `keys` naming the internal node; it must be the canonical
/// spelling, and no two keys may differ only in case.
fn only_internal_signal<'a>(keys: impl Iterator<Item = &'a String>, analysis: &str) -> &'a str {
    let keys = keys.collect::<Vec<_>>();
    let mut folded = keys
        .iter()
        .map(|key| key.to_ascii_lowercase())
        .collect::<Vec<_>>();
    folded.sort();
    folded.dedup();
    assert_eq!(
        folded.len(),
        keys.len(),
        "{analysis}: no two signals may differ only in case: {keys:?}"
    );
    let matches = keys
        .iter()
        .filter(|key| key.to_ascii_lowercase().contains("xout.xin.mid"))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "{analysis}: the internal node is one signal: {keys:?}"
    );
    assert!(
        matches[0].contains("xOut.xIn.MID"),
        "{analysis}: reported under the spelling expansion gives it: {}",
        matches[0]
    );
    matches[0]
}

#[test]
fn every_analysis_reports_one_connected_internal_node() {
    let results = run();
    assert_eq!(results.len(), 3, "op, ac and tran each produce a result");
    for result in &results {
        match result {
            SimulationResult::DcOp(op) => {
                let key = only_internal_signal(op.node_voltages.keys(), "op");
                let voltage = op.node_voltages[key];
                assert!(
                    (voltage - MID_DC).abs() < 1e-9,
                    "op: the loads reach the internal node: {voltage}"
                );
            }
            SimulationResult::Ac { waveforms, .. } => {
                let key = only_internal_signal(waveforms.keys(), "ac");
                let waveform = &waveforms[key];
                let magnitude =
                    waveform.y_values[0].hypot(waveform.y_imag.as_ref().expect("complex")[0]);
                // |R / ((1k + R) + j w R C 1k)| with R = 400 and C = 1n.
                let omega = 2.0 * std::f64::consts::PI * 1.0e5;
                let expected =
                    400.0 / (1400.0_f64.powi(2) + (omega * 400.0 * 1.0e-9 * 1.0e3).powi(2)).sqrt();
                assert!(
                    (magnitude - expected).abs() < 1e-9,
                    "ac: |v(mid)| {magnitude} vs {expected}"
                );
            }
            SimulationResult::Transient {
                waveforms,
                measurements,
                ..
            } => {
                let key = only_internal_signal(waveforms.keys(), "tran");
                let waveform = &waveforms[key];
                let last = *waveform.y_values.last().expect("samples");
                assert!(
                    (last - MID_DC).abs() < 1e-3,
                    "tran: the loads reach the internal node: {last}"
                );
                let value = |name: &str| {
                    let measurement = measurements
                        .iter()
                        .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
                        .unwrap_or_else(|| panic!("{name} measured: {measurements:?}"));
                    assert!(
                        measurement.error.is_none(),
                        "{name}: {:?}",
                        measurement.error
                    );
                    measurement.value.expect("measured value")
                };
                let lower = value("lower");
                assert!(
                    (lower - MID_DC).abs() < 1e-3,
                    "the measurement reads the connected node: {lower}"
                );
                for name in ["upper", "mixed"] {
                    assert_eq!(value(name), lower, "{name} measures the same signal");
                }
            }
            other => panic!("unexpected result {:?}", std::mem::discriminant(other)),
        }
    }
}
