//! A node inside a subcircuit, referenced from another scope by its flattened
//! name (`rload xout.xin.mid 0`, `[xdig.dmid]`, `v(xout.xin.mid)`), is the
//! internal node itself: one node, one signal, reported under the spelling
//! expansion gives it everywhere else.
//!
//! SPICE node names are case-insensitive. The deck below spells the reference
//! three ways and the instances in mixed case, so the canonical name
//! (`xOut.xIn.MID`: instance path as authored, node token as folded) matches
//! none of the references byte for byte. Before the flattener gave each
//! case-insensitive class one spelling, every reference became a second,
//! disconnected node and each analysis reported the node twice.

use rspice_core::analysis::measure_signals::evaluate_tran_measurements;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Flattener, Netlist};

/// The flattened name of the internal node, as expansion spells it.
const CANONICAL: &str = "xOut.xIn.MID";

const SUBCIRCUITS: &str = "\
.subckt inner a b
r1 a mid 1k
r2 mid b 1k
c1 mid b 1n
.ends inner
.subckt outer p q
xIn p q inner
.ends outer
";

/// Three loads on the internal node, one per spelling, plus a behavioral
/// probe of it. With the loads attached the node sits at
/// `2 * (1k || 2k/3) / (1k + (1k || 2k/3)) = 4/7` V; a reference that missed
/// it would leave it at 1 V.
const LOADS: &str = "\
xOut top 0 outer
rload xout.xin.mid 0 2k
rcaps XOUT.XIN.MID 0 2k
rmixed Xout.Xin.Mid 0 2k
bsense sense 0 v=2*v(xOUT.xin.MID)
rsense sense 0 1k
";

const MID_DC: f64 = 4.0 / 7.0;

/// `|v(mid) / v(top)|` with the loads attached: `R / ((1k + R) + j w R C 1k)`
/// with `R = 1k || 2k/3 = 400` and `C = 1n`.
fn divider_gain(frequency: f64) -> f64 {
    let r: f64 = 400.0;
    let omega = 2.0 * std::f64::consts::PI * frequency;
    r / ((1.0e3 + r).powi(2) + (omega * r * 1.0e-9 * 1.0e3).powi(2)).sqrt()
}

fn deck(source: &str, analysis: &str) -> Netlist {
    let text =
        format!("* hierarchical node references\n{source}\n{SUBCIRCUITS}{LOADS}{analysis}\n.end\n");
    Netlist::parse(&text).expect("deck parses")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// The one name `names` reports for the internal node, which must be the
/// canonical spelling, and its index.
fn only_internal_node(names: &[String], analysis: &str) -> usize {
    let matches = names
        .iter()
        .enumerate()
        .filter(|(_, name)| name.eq_ignore_ascii_case(CANONICAL))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "{analysis}: the internal node must be one signal, got {names:?}"
    );
    let (index, name) = matches[0];
    assert_eq!(
        name, CANONICAL,
        "{analysis}: reported under the spelling expansion gives it"
    );
    let mut folded = names
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .collect::<Vec<_>>();
    folded.sort();
    folded.dedup();
    assert_eq!(
        folded.len(),
        names.len(),
        "{analysis}: no two signals may differ only in case: {names:?}"
    );
    index
}

fn index_of(names: &[String], wanted: &str) -> usize {
    names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(wanted))
        .unwrap_or_else(|| panic!("{wanted} missing from {names:?}"))
}

#[test]
fn flattening_gives_every_reference_the_generated_spelling() {
    let netlist = deck("v1 top 0 dc 2", ".op");
    let mut flattener = Flattener::new(&netlist.subcircuits);
    let elements = flattener.flatten(&netlist).expect("deck flattens");
    for name in ["xOut.xIn.r1", "xOut.xIn.r2", "rload", "rcaps", "rmixed"] {
        let element = elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} flattened"));
        assert!(
            element.nodes.iter().any(|node| node == CANONICAL),
            "{name} must reach {CANONICAL}: {:?}",
            element.nodes
        );
        assert!(
            !element
                .nodes
                .iter()
                .any(|node| node != CANONICAL && node.eq_ignore_ascii_case(CANONICAL)),
            "{name} keeps a case variant: {:?}",
            element.nodes
        );
    }
}

#[test]
fn operating_point_reports_one_connected_node() {
    let netlist = deck("v1 top 0 dc 2", ".op");
    let result = engine().run_dc_op(&netlist).expect("operating point");
    let mid = only_internal_node(&result.node_names, "op");
    let voltage = result.node_voltages[mid];
    assert!(
        (voltage - MID_DC).abs() < 1e-9,
        "op: the loads reach the internal node: {voltage}"
    );
    let sense = result.node_voltages[index_of(&result.node_names, "sense")];
    assert!(
        (sense - 2.0 * voltage).abs() < 1e-9,
        "op: the behavioral probe reads the same node: {sense} vs {voltage}"
    );
}

#[test]
fn dc_sweep_reports_one_connected_node() {
    let netlist = deck("v1 top 0 dc 2", ".dc v1 0 2 1");
    let points = engine()
        .run_dc_sweep(&netlist, "v1", 0.0, 2.0, 1.0)
        .expect("dc sweep");
    assert_eq!(points.len(), 3);
    for (input, result) in points {
        let mid = only_internal_node(&result.node_names, "dc");
        let expected = input * MID_DC / 2.0;
        assert!(
            (result.node_voltages[mid] - expected).abs() < 1e-9,
            "dc at {input} V: {} vs {expected}",
            result.node_voltages[mid]
        );
    }
}

#[test]
fn ac_reports_one_connected_node() {
    let netlist = deck("v1 top 0 dc 2 ac 1", ".ac lin 1 100k 100k");
    let frequency = 1.0e5;
    let points = engine().run_ac(&netlist, &[frequency]).expect("ac");
    let point = &points[0];
    let mid = only_internal_node(&point.node_names, "ac");
    let expected = divider_gain(frequency);
    let magnitude = point.voltages[mid].norm();
    assert!(
        (magnitude - expected).abs() < 1e-9,
        "ac: |v(mid)| {magnitude} vs {expected}"
    );
}

#[test]
fn noise_output_on_the_reference_resolves_to_the_internal_node() {
    let netlist = deck(
        "v1 top 0 dc 2 ac 1",
        ".noise v(xout.xin.mid) v1 lin 1 1k 1k",
    );
    let points = engine()
        .run_noise_named_with_input_source(&netlist, "XOUT.XIN.MID", None, "v1", &[1.0e3], 300.15)
        .expect("noise");
    let point = &points[0];
    let mid = only_internal_node(&point.node_names, "noise");
    let gain = point.voltages[mid].norm();
    let expected = divider_gain(1.0e3);
    assert!(
        (gain - expected).abs() < 1e-9,
        "noise: the output port is the connected node: gain {gain} vs {expected}"
    );
    assert!(point.output_noise_density > 0.0);
}

/// The C1 shape: a digital node driven inside a subcircuit and read at top
/// level by bridges, here once per spelling.
const DIGITAL: &str = "\
.subckt dig ain
aadc [ain] [dmid] adc1
.model adc1 adc_bridge(in_low=0.5 in_high=0.5)
.ends dig
xDig top dig
adac1 [xdig.dmid] [a1] dac1
adac2 [XDIG.DMID] [a2] dac1
adac3 [Xdig.Dmid] [a3] dac1
.model dac1 dac_bridge(out_low=0 out_high=1 out_undef=0.5 t_rise=1n t_fall=1n)
ra1 a1 0 1k
ra2 a2 0 1k
ra3 a3 0 1k
";

#[test]
fn transient_and_measurements_report_one_connected_node() {
    let netlist = deck(
        &format!("v1 top 0 pulse(0 2 0 1n 1n 1 2)\n{DIGITAL}"),
        ".tran 10n 2u\n\
         .meas tran lower find v(xout.xin.mid) at=1.5u\n\
         .meas tran upper find v(XOUT.XIN.MID) at=1.5u\n\
         .meas tran mixed find v(Xout.Xin.Mid) at=1.5u\n\
         .meas tran sensed find v(sense) at=1.5u",
    );
    let result = engine()
        .run_tran(&netlist, 2.0e-6, 10.0e-9)
        .expect("transient");
    let mid = only_internal_node(&result.node_names, "tran");
    let waveform = &result.voltages[mid];
    let last = *waveform.last().expect("samples");
    // Seven time constants (285.7 ns) after the 0 -> 2 V edge.
    assert!(
        (last - MID_DC).abs() < 1e-3,
        "tran: the loads reach the internal node: {last}"
    );
    for bridge in ["a1", "a2", "a3"] {
        let output = *result.voltages[index_of(&result.node_names, bridge)]
            .last()
            .expect("samples");
        assert!(
            (output - 1.0).abs() < 1e-9,
            "tran: {bridge} reads the digital node the subcircuit drives: {output}"
        );
    }

    let measurements = evaluate_tran_measurements(&netlist, &result);
    let value = |name: &str| {
        let measurement = measurements
            .iter()
            .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} measured"));
        assert!(
            measurement.error.is_none(),
            "{name}: {:?}",
            measurement.error
        );
        measurement.value.expect("measured value")
    };
    let lower = value("lower");
    for name in ["upper", "mixed"] {
        assert_eq!(value(name), lower, "{name} measures the same signal");
    }
    let index = result
        .time
        .partition_point(|time| *time < 1.5e-6)
        .min(result.time.len() - 1);
    let (t0, t1) = (result.time[index - 1], result.time[index]);
    let (v0, v1) = (waveform[index - 1], waveform[index]);
    let interpolated = v0 + (v1 - v0) * (1.5e-6 - t0) / (t1 - t0);
    assert!(
        (lower - interpolated).abs() < 1e-12,
        "the measurement reads the canonical waveform: {lower} vs {interpolated}"
    );
    assert!(
        (value("sensed") - 2.0 * lower).abs() < 1e-9,
        "the behavioral probe follows the node"
    );
}
