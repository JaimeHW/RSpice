//! Build-time policy for resistor model levels.
//!
//! Xyce `R LEVEL=2` is a thermal/semiconductor resistor family. RSpice supports
//! the validated DC electrical subset natively and fails closed for the
//! self-consistent thermal stateful form until that topology is implemented.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist, StepCommand, StepSweep, StepTarget};
use rspice_core::solver::SimulationResult;

fn op_voltage(deck: &str, node: &str) -> Result<f64, String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .map(|op| result_voltage(&op, node))
        .map_err(|err| err.to_string())
}

fn result_voltage(result: &SimulationResult, node: &str) -> f64 {
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node} in {:?}", result.node_names));
    result.node_voltages[idx]
}

fn step_command(netlist: &Netlist) -> &StepCommand {
    netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .expect(".STEP command captured")
}

#[test]
fn xyce_resistor_level2_rsh_geometry_runs_as_native_electrical_subset() {
    let deck = "* Xyce R LEVEL=2 semiconductor resistor DC subset\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1000u w=1u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 RSH=1)\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out").expect("R LEVEL=2 RSH geometry runs natively");
    assert!(
        (vout - 0.5).abs() < 1e-12,
        "1k LEVEL=2 resistor into 1k load should divide to 0.5 V, got {vout}"
    );
}

#[test]
fn xyce_resistor_level2_uses_one_sided_narrow_and_model_r_multiplier() {
    let deck = "* Xyce R LEVEL=2 uses RSH*(L-NARROW)/(W-NARROW) times model R\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=5u w=3u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 RSH=100 NARROW=1u R=2)\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out").expect("R LEVEL=2 one-sided NARROW geometry runs natively");
    let expected = 1000.0 / (1000.0 + 400.0);
    assert!(
        (vout - expected).abs() < 1e-12,
        "LEVEL=2 should resolve 100*(5u-1u)/(3u-1u)*2 = 400 ohm, got V(out)={vout}"
    );
}

#[test]
fn xyce_resistor_level2_uses_default_width_defw_when_w_is_omitted() {
    let deck = "* Xyce R LEVEL=2 default width comes from DEFW=10e-6\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1000u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 RSH=1)\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out").expect("R LEVEL=2 default DEFW width runs natively");
    let expected = 1000.0 / (1000.0 + 100.0);
    assert!(
        (vout - expected).abs() < 1e-12,
        "LEVEL=2 should default W to DEFW=10e-6 and resolve 100 ohm, got V(out)={vout}"
    );
}

#[test]
fn xyce_resistor_level2_instance_r_still_runs_with_model_multiplier() {
    let deck = "* Xyce R LEVEL=2 explicit instance R with model multiplier\n\
                v1 in 0 dc 1\n\
                r1 in out rmod r=500\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 R=2)\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out").expect("R LEVEL=2 explicit instance R runs natively");
    assert!(
        (vout - 0.5).abs() < 1e-12,
        "instance R=500 with model R multiplier 2 should resolve 1k, got V(out)={vout}"
    );
}

#[test]
fn xyce_resistor_level2_explicit_r_participates_in_bare_device_step() {
    let deck = "* Xyce-style bare .STEP over modeled R LEVEL=2 instance value\n\
                v1 in 0 dc 1\n\
                r3 in out rmod r=500\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2)\n\
                .step r3 500 1000 500\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("bare .STEP resistor deck parses");
    let step = step_command(&netlist);
    assert_eq!(step.target, StepTarget::Device);
    assert!(step.name.eq_ignore_ascii_case("r3"));
    assert!(step.param_name.is_none());
    match &step.sweep {
        StepSweep::Linear { start, stop, step } => {
            assert_eq!((*start, *stop, *step), (500.0, 1000.0, 500.0));
        }
        ref other => panic!("bare resistor .STEP should parse as linear sweep, got {other:?}"),
    }

    let stepped = Engine::default()
        .run_step_command(&netlist, step, &[500.0, 1000.0])
        .expect("bare .STEP R3 should run modeled LEVEL=2 resistor OPs");
    assert_eq!(stepped.len(), 2);

    for ((value, result), expected_resistance) in stepped.iter().zip([500.0, 1000.0]) {
        assert!(
            (*value - expected_resistance).abs() < 1e-12,
            "step result should preserve requested resistor value, got {value}"
        );
        let expected_vout = 1000.0 / (1000.0 + expected_resistance);
        let vout = result_voltage(result, "out");
        assert!(
            (vout - expected_vout).abs() < 1e-12,
            ".STEP R3={expected_resistance} should solve V(out)={expected_vout}, got {vout}"
        );
    }
}

#[test]
fn resistor_level1_geometry_still_uses_plain_rsh_path() {
    let deck = "* ordinary level-1 resistor geometry remains supported\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1000u w=1u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=1 RSH=1)\n\
                .op\n\
                .end\n";

    op_voltage(deck, "out").expect("R LEVEL=1 RSH geometry remains a plain resistor");
}

#[test]
fn xyce_resistor_level2_self_heating_form_fails_closed_until_state_is_native() {
    let deck = "* Xyce R LEVEL=2 self-consistent thermal form needs a state variable\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1u a=1u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)\n\
                .op\n\
                .end\n";

    let message = op_voltage(deck, "out")
        .expect_err("self-consistent thermal LEVEL=2 form must fail until state is native");
    assert!(
        message.contains("LEVEL=2")
            && message.contains("self")
            && message.contains("thermal")
            && message.contains("native"),
        "error should identify unsupported stateful thermal LEVEL=2 form: {message}"
    );
}
