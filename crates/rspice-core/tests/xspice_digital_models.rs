//! Native XSPICE digital code models pinned against ngspice code-model semantics.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;
use std::fs;
use std::path::{Path, PathBuf};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

fn transient_node_series<'a>(result: &'a TransientResult, node: &str) -> &'a [f64] {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", result.node_names));
    &result.voltages[idx]
}

fn max_between(times: &[f64], values: &[f64], t_start: f64, t_end: f64) -> f64 {
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| (time >= t_start && time <= t_end).then_some(value))
        .fold(f64::NEG_INFINITY, f64::max)
}

fn min_between(times: &[f64], values: &[f64], t_start: f64, t_end: f64) -> f64 {
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| (time >= t_start && time <= t_end).then_some(value))
        .fold(f64::INFINITY, f64::min)
}

fn value_near_time(times: &[f64], values: &[f64], target: f64) -> f64 {
    let nearest_delta = times
        .iter()
        .map(|time| (time - target).abs())
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_else(|| panic!("no samples for target time {target:e}"));
    assert!(
        nearest_delta <= 1.0e-15,
        "expected sample near {target:e}, nearest delta was {nearest_delta:e}"
    );
    times
        .iter()
        .zip(values)
        .filter_map(|(&time, &value)| {
            ((time - target).abs() <= nearest_delta + f64::EPSILON).then_some(value)
        })
        .next_back()
        .expect("nearest sample exists")
}

fn digital_tokens(result: &TransientResult, node: &str) -> Vec<(f64, String)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("digital trace {node} missing from {:?}", result.node_names))
        .iter()
        .map(|point| (point.time, point.value.to_ngspice_token()))
        .collect()
}

fn parse_file(path: &Path) -> Netlist {
    Netlist::parse_file(path).unwrap_or_else(|err| panic!("deck parses: {err}"))
}

fn run_temp_deck(
    prefix: &str,
    stimulus: &str,
    deck: &str,
    tstop: f64,
    max_step: f64,
) -> TransientResult {
    let dir = unique_temp_dir(prefix);
    fs::create_dir_all(&dir).expect("create temp d_source fixture dir");
    fs::write(dir.join("stim.stim"), stimulus).expect("write d_source stimulus");
    let deck_path = dir.join("deck.cir");
    fs::write(&deck_path, deck).expect("write d_source deck");
    let netlist = parse_file(&deck_path);
    let result = Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .expect("transient solves");
    let _ = fs::remove_dir_all(dir);
    result
}

#[test]
fn d_source_drives_dac_bridge_from_stimulus_file_like_ngspice() {
    let result = run_temp_deck(
        "rspice-d-source",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_source to dac_bridge oracle
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=0 t_fall=0)
rload out 0 1k
.end
",
        3.0e-9,
        100.0e-12,
    );
    let out = transient_node_series(&result, "out");

    assert!(
        out.first().copied().unwrap_or(f64::NAN).abs() < 0.1,
        "d_source t=0 vector should initialize dac low like ngspice, got {:?}",
        out.first()
    );
    let high_window_max = max_between(&result.time, out, 1.15e-9, 1.9e-9);
    assert!(
        high_window_max > 4.9,
        "d_source 1ns event should drive dac high like ngspice, max={high_window_max}"
    );
    let low_window_min = min_between(&result.time, out, 2.15e-9, 3.0e-9);
    assert!(
        low_window_min < 0.1,
        "d_source 2ns event should drive dac low like ngspice, min={low_window_min}"
    );
}

#[test]
fn d_source_records_digital_event_trace() {
    let result = run_temp_deck(
        "rspice-d-source-trace",
        "0 0s\n1n 1s\n2n Uu\n",
        "\
* d_source digital trace oracle
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.end
",
        2.5e-9,
        100.0e-12,
    );

    let trace = digital_tokens(&result, "d");
    assert!(
        trace
            .iter()
            .any(|(time, token)| *time == 0.0 && token == "0s"),
        "missing t=0 0s digital event in {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 1.0e-9).abs() <= 1.0e-18 && token == "1s"),
        "missing t=1ns 1s digital event in {trace:?}"
    );
    assert!(
        trace
            .iter()
            .any(|(time, token)| (*time - 2.0e-9).abs() <= 1.0e-18 && token == "Uu"),
        "missing t=2ns Uu digital event in {trace:?}"
    );
}

#[test]
fn d_source_reaches_dac_bridge_independent_of_instance_order() {
    let result = run_temp_deck(
        "rspice-d-source-reversed",
        "0 0s\n1n 1s\n2n 0s\n",
        "\
* d_source should not depend on instance order
a_dac [d] [out] dac
a_src [d] src
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=0 t_fall=0)
rload out 0 1k
.end
",
        2.0e-9,
        100.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let at_event = value_near_time(&result.time, out, 1.0e-9);
    assert!(
        at_event > 4.9,
        "reversed instance order should propagate the 1ns d_source event before stamping, got {at_event}"
    );
}

#[test]
fn dac_bridge_initial_high_uses_target_not_default_ramp_from_zero() {
    let result = run_temp_deck(
        "rspice-d-source-initial-high",
        "0 1s\n",
        "\
* dac_bridge initial high should resolve immediately
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5)
rload out 0 1k
.end
",
        200.0e-12,
        20.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let early_min = min_between(&result.time, out, 0.0, 200.0e-12);
    assert!(
        early_min > 4.9,
        "initial high d_source row should not ramp from 0 V during the first transient window, min={early_min}"
    );
}

#[test]
fn dac_bridge_interrupted_ramp_starts_new_ramp_at_event_value() {
    let result = run_temp_deck(
        "rspice-d-source-interrupted-ramp",
        "0 0s\n250p 1s\n500p 0s\n",
        "\
* dac_bridge interrupted ramp continuity
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"stim.stim\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1n t_fall=1n)
rload out 0 1k
.end
",
        700.0e-12,
        50.0e-12,
    );
    let out = transient_node_series(&result, "out");
    let at_fall_event = value_near_time(&result.time, out, 500.0e-12);
    assert!(
        at_fall_event > 1.2 && at_fall_event < 1.3,
        "falling event should start from the rising ramp value at 500ps, got {at_fall_event}"
    );
}
