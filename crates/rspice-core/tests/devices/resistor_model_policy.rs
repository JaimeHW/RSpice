//! Build-time policy for resistor model levels.
//!
//! Xyce `R LEVEL=2` is a thermal/semiconductor resistor family. RSpice supports
//! the validated DC electrical subset and the self-consistent electrothermal
//! transient state natively, while unsupported parameter combinations still
//! fail closed during model resolution.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
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

fn branch_current(result: &SimulationResult, branch: &str) -> f64 {
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing branch {branch} in {:?}", result.branch_names))
}

#[test]
fn xyce_numeric_resistor_models_remain_lexically_scoped_across_sibling_subcircuits() {
    let deck = "* sibling subcircuits own distinct local RMOD cards\n\
                XACTIVE 2 0 ACTIVE\n\
                VIN 1 0 5\n\
                VMON 1 2 0\n\
                .subckt UNUSED a b\n\
                R1 a b RMOD L=1\n\
                .model RMOD R (RSH=1 DEFW=1)\n\
                .ends\n\
                .subckt ACTIVE a b\n\
                R1 a b RMOD L=1\n\
                .model RMOD R (RSH=.031 NARROW=0 DEFW=1)\n\
                .ends\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("scoped resistor-model deck parses");
    let model_names = netlist
        .models
        .iter()
        .map(|model| model.name.to_ascii_uppercase())
        .collect::<Vec<_>>();
    assert_eq!(model_names, ["UNUSED::RMOD", "ACTIVE::RMOD"]);

    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let parameters = engine
        .resolved_resistor_parameters(&netlist, "XACTIVE.R1")
        .expect("scoped resistor parameters resolve")
        .expect("flattened active resistor exists");
    assert_eq!(parameters.reported_resistance.to_bits(), 0.031f64.to_bits());
    assert_eq!(parameters.resistance.to_bits(), 0.031f64.to_bits());

    let result = engine
        .run_dc_op(&netlist)
        .expect("active scoped resistor circuit solves");
    let expected_current = 5.0 / 0.031;
    let actual_current = branch_current(&result, "VMON");
    assert!(
        (actual_current - expected_current).abs() <= expected_current * 1.0e-12,
        "active resistor must use ACTIVE::RMOD (0.031 ohm), got {actual_current} A"
    );
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
fn zero_and_xyce_near_zero_resistors_expose_solved_branch_current() {
    for (resistance, expected_current) in [("0", 1.0e-3), ("1.0e-101", 1.0e-3)] {
        let deck = format!(
            "* branch-form zero resistor\n\
             v1 a 0 dc 1\n\
             r1 a b {resistance}\n\
             r2 b 0 1k\n\
             .op\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("zero/near-zero branch-form resistor OP solves");

        let current = branch_current(&result, "r1");
        assert!(
            (current - expected_current).abs() < 1e-12,
            "R1={resistance} should carry {expected_current} A, got {current}"
        );

        let va = result_voltage(&result, "a");
        let vb = result_voltage(&result, "b");
        assert!(
            (va - vb).abs() <= 1e-12,
            "R1={resistance} should enforce Va ~= Vb, got Va={va}, Vb={vb}"
        );
    }
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
fn xyce_colon_resistor_instance_parameter_step_updates_multiplier() {
    let deck = "* Xyce-style .STEP over a resistor instance parameter\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1000u w=1u\n\
                rload out 0 1k\n\
                .model rmod R (RSH=1)\n\
                .step r1:m 1 2 1\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("colon device-parameter .STEP deck parses");
    let step = step_command(&netlist);
    assert_eq!(step.target, StepTarget::Device);
    assert!(step.name.eq_ignore_ascii_case("r1"));
    assert!(
        step.param_name
            .as_deref()
            .is_some_and(|param| param.eq_ignore_ascii_case("m"))
    );

    let stepped = Engine::default()
        .run_step_command(&netlist, step, &[1.0, 2.0])
        .expect(".STEP R1:M should run modeled resistor OPs");
    assert_eq!(stepped.len(), 2);

    for ((value, result), expected_resistance) in stepped.iter().zip([1000.0, 500.0]) {
        let expected_vout = 1000.0 / (1000.0 + expected_resistance);
        let vout = result_voltage(result, "out");
        assert!(
            (vout - expected_vout).abs() < 1e-12,
            ".STEP R1:M={value} should solve V(out)={expected_vout}, got {vout}"
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

/// `R LEVEL=2` self-heating does not act on the operating point.
///
/// This used to be a fail-closed guard, on the theory that a resistor with a
/// `HEATCAPACITY` might need a self-consistent steady-state temperature before
/// `.op` could be answered. `be8f87fc6` made the transient thermal state
/// native, so the deck resolves; the open question was whether the value it
/// resolves to is right.
///
/// Xyce's own `THERMAL_RESISTOR` suite settles it. `linear.cir` drives copper
/// whose resistivity is a `table(temp+273.15, ...)`, so its resistance moves
/// with device temperature and the operating point cannot hide a temperature
/// error. At `t = 0` its reference output prints
///
/// ```text
/// R1:R = 1.70105000e-04    R1:TEMP = 2.70000037e+01
/// ```
///
/// and `1.70105e-4` is `table(300.15) * L/A` to every printed digit — the
/// resistivity at TNOM = 27 C exactly, not at the 27.0000037 C the same row
/// reports for the device. Xyce enters the operating point at nominal
/// temperature and lets the thermal state evolve only through the transient.
///
/// So the isothermal divider is the correct `.op` answer here: R =
/// RESISTIVITY*L/A = 1 ohm against RLOAD = 1k.
#[test]
fn xyce_resistor_level2_self_heating_does_not_move_the_operating_point() {
    let deck = "* Xyce R LEVEL=2 thermal form resolves isothermally at .op\n\
                v1 in 0 dc 1\n\
                r1 in out rmod l=1u a=1u\n\
                rload out 0 1k\n\
                .model rmod R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out").expect("LEVEL=2 thermal resistor resolves at .op");
    let expected = 1000.0 / 1001.0;
    assert!(
        (vout - expected).abs() < 1e-12,
        "LEVEL=2 self-heating must not perturb the operating point: \
         got V(out)={vout}, isothermal divider is {expected}"
    );
}

/// The same rule against Xyce's own numbers rather than against reasoning.
///
/// R1 of `tests/xyce/Netlists/THERMAL_RESISTOR/linear.cir`, reduced to the one
/// branch that carries it. Its resistivity is temperature-dependent, so if the
/// operating point applied self-heating the current would move. Xyce's
/// `linear.cir.prn` row 0 gives `I(R1) = 2.93936098e+04`.
#[test]
fn xyce_thermal_resistor_operating_point_matches_the_linear_oracle() {
    let deck = "* Xyce THERMAL_RESISTOR linear.cir, R1 branch only\n\
                v1 1 0 5\n\
                r1 1 0 copper l=0.1 a=1e-5\n\
                .model copper r (level=2\n\
                + resistivity={table(temp+273.15, 0, 0.5e-9, 100, 3e-9, 1000, 6.6e-8)}\n\
                + heatcapacity={8.92e+3*table(temp+273.15, 0, 1, 1000, 1500)})\n\
                .op\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("table-valued LEVEL=2 model parses");
    let op = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("table-valued LEVEL=2 resistor resolves at .op");

    // Xyce prints the source current with the opposite sign convention.
    let got = branch_current(&op, "v1").abs();
    let expected = 2.93936098e4;
    let relative = (got - expected).abs() / expected;
    assert!(
        relative <= 1.0e-5,
        "I(R1) at the operating point must match Xyce's linear.cir row 0: \
         got {got:.8e}, expected {expected:.8e}, relative {relative:.2e}"
    );
}
