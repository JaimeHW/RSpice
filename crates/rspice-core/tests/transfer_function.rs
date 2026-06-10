//! End-to-end tests for `.TF` — the dot-command front door and the engine
//! analysis behind it.
//!
//! Every expected number below was produced by the official ngspice-46
//! release binary on the identical deck (transfer_function, input
//! impedance, output impedance), so these tests pin ngspice parity, not
//! self-consistency.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};

const TOL: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64, what: &str) {
    let scale = expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= TOL * scale,
        "{what}: got {actual}, ngspice oracle {expected}"
    );
}

#[test]
fn tf_command_parses_voltage_probe_forms() {
    let deck = "\
* tf parse
vin in 0 dc 1
r1 in out 1k
r2 out 0 2k
.tf v(out) vin
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let tf = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => Some((
                output_node.clone(),
                reference_node.clone(),
                *output_is_current,
                input_source.clone(),
            )),
            _ => None,
        })
        .expect(".tf must parse into an analysis command");
    assert_eq!(tf.0.to_uppercase(), "OUT");
    assert_eq!(tf.1, None);
    assert!(!tf.2);
    assert_eq!(tf.3.to_uppercase(), "VIN");

    let differential = "\
* tf differential probe
vin in 0 dc 1
r1 in out 1k
r2 out 0 2k
.tf v(out,in) vin
.end
";
    let netlist = Netlist::parse(differential).expect("parse");
    assert!(netlist.analyses.iter().any(|analysis| matches!(
        analysis,
        AnalysisCommand::Tf { reference_node: Some(reference), .. }
            if reference.eq_ignore_ascii_case("in")
    )));

    let current = "\
* tf current probe
vin in 0 dc 1
r1 in mid 1k
vmeas mid out dc 0
r2 out 0 2k
.tf i(vmeas) vin
.end
";
    let netlist = Netlist::parse(current).expect("parse");
    assert!(netlist.analyses.iter().any(|analysis| matches!(
        analysis,
        AnalysisCommand::Tf { output_node, output_is_current: true, .. }
            if output_node.eq_ignore_ascii_case("vmeas")
    )));
}

#[test]
fn tf_voltage_probe_matches_ngspice_oracle() {
    // ngspice-46: transfer_function = 6.666667e-01,
    // output_impedance_at_v(out) = 6.666667e+02,
    // vin#input_impedance = 3.000000e+03.
    let deck = "\
* tf divider
vin in 0 dc 1
r1 in out 1k
r2 out 0 2k
.tf v(out) vin
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_transfer_function(&netlist, "OUT", None, false, "VIN")
        .expect("tf analysis");
    assert_close(result.gain, 2.0 / 3.0, "transfer function");
    assert_close(result.input_impedance, 3000.0, "input impedance");
    assert_close(result.output_impedance, 2000.0 / 3.0, "output impedance");
}

#[test]
fn tf_current_probe_matches_ngspice_oracle() {
    // ngspice-46: transfer_function = 3.333333e-04,
    // vin#input_impedance = 3.000000e+03,
    // vmeas#output_impedance = 1.000000e+20 (infinite sentinel).
    let deck = "\
* tf transadmittance
vin in 0 dc 1
r1 in mid 1k
vmeas mid out dc 0
r2 out 0 2k
.tf i(vmeas) vin
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_transfer_function(&netlist, "VMEAS", None, true, "VIN")
        .expect("tf analysis");
    assert_close(result.gain, 1.0 / 3000.0, "transadmittance");
    assert_close(result.input_impedance, 3000.0, "input impedance");
    assert_close(
        result.output_impedance,
        1.0e20,
        "current-probe output impedance sentinel",
    );
}

#[test]
fn tf_current_source_input_matches_ngspice_oracle() {
    // ngspice-46: transfer_function = 3.000000e+03,
    // iin#input_impedance = 3.000000e+03,
    // output_impedance_at_v(in) = 3.000000e+03.
    let deck = "\
* tf current input
iin 0 in dc 0
r1 in 0 3k
.tf v(in) iin
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_transfer_function(&netlist, "IN", None, false, "IIN")
        .expect("tf analysis");
    assert_close(result.gain, 3000.0, "transimpedance");
    assert_close(result.input_impedance, 3000.0, "input impedance");
    assert_close(result.output_impedance, 3000.0, "output impedance");
}

#[test]
fn tf_rejects_non_source_inputs() {
    let deck = "\
* tf bad input
vin in 0 dc 1
r1 in out 1k
r2 out 0 2k
.tf v(out) r1
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    assert!(
        engine
            .run_transfer_function(&netlist, "OUT", None, false, "R1")
            .is_err(),
        ".tf with a resistor as input source must be rejected"
    );
}
