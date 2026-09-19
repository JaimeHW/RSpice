//! What a sweep owes an element whose value the deck spelled as an expression.
//!
//! A sweep that names a device value sets that device's value. The spelling the
//! deck used for that one field — a literal, a parameter expression, or an
//! expression that reads circuit state — is overridden for the swept point, so
//! the two points of a two-point sweep differ by exactly the number the sweep
//! chose. Before this module the override was written into the element's
//! numeric field alone: the authored expression stayed beside it and won
//! wherever it is read, which made `.STEP` on a state-reading resistor, on any
//! passive inside a subcircuit body, and on an instance parameter a subcircuit
//! body deferred produce two identical points and no diagnostic.
//!
//! Every expectation here is a closed form written out in the test — a
//! resistive divider, the first-order RC and RL transfer magnitudes, and the
//! level-1 sheet-resistance law `R = RSH * L / W`. None is a number this
//! engine once printed.

use super::*;
use crate::netlist::DcSweepSpec;

/// A divider whose lower leg is a compound parameter expression.
const EXPRESSION_DIVIDER: &str = "Expression valued divider\n\
.param r=1k\n\
V1 in 0 10\n\
R1 in out 1k\n\
R2 out 0 {2*r}\n\
.end\n";

/// A divider whose upper leg genuinely reads circuit state, so the builder can
/// only realize it as a behavioural element: `R1 = 1k*(1+0.1*V(in))`.
const STATE_READING_DIVIDER: &str = "State reading divider\n\
V1 in 0 10\n\
R1 in out {1k*(1+0.1*V(in))}\n\
R2 out 0 1k\n\
.end\n";

/// Two legs valued from one design parameter, with a literal third leg so the
/// answer is not a ratio that cancels the parameter out.
const SHARED_PARAMETER_DIVIDER: &str = "Shared design parameter\n\
.param r=1k\n\
V1 in 0 10\n\
R1 in out {r}\n\
R2 out 0 {2*r}\n\
R3 out 0 6k\n\
.end\n";

/// A source whose DC value is spelled as a parameter expression.
const EXPRESSION_SOURCE: &str = "Expression valued source\n\
.param vdd=10\n\
V1 in 0 {vdd}\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

/// An RC low pass whose capacitor is spelled as an expression inside the body,
/// where the parser keeps the spelling instead of folding it.
const SUBCIRCUIT_LOW_PASS: &str = "Subcircuit low pass\n\
V1 in 0 AC 1\n\
X1 in out LP c=1n\n\
.subckt LP a b c=1n\n\
R1 a b 1k\n\
C1 b 0 {2*c}\n\
.ends\n\
.end\n";

/// The RL high pass twin of [`SUBCIRCUIT_LOW_PASS`].
const SUBCIRCUIT_HIGH_PASS: &str = "Subcircuit high pass\n\
V1 in 0 AC 1\n\
X1 in out HP l=1m\n\
.subckt HP a b l=1m\n\
R1 a b 1k\n\
L1 b 0 {2*l}\n\
.ends\n\
.end\n";

/// A divider whose upper leg lives in a subcircuit body and is spelled there.
const SUBCIRCUIT_DIVIDER: &str = "Subcircuit divider\n\
.param r=1k\n\
V1 in 0 10\n\
X1 in out DIV r=1k\n\
R3 out 0 1k\n\
.subckt DIV a b r=1k\n\
R1 a b {2*r}\n\
.ends\n\
.end\n";

/// A model-valued resistor whose width the body defers as an expression.
const DEFERRED_INSTANCE_PARAMETER: &str = "Deferred instance parameter\n\
.param w=2u\n\
V1 in 0 10\n\
X1 in out DIV w=2u\n\
R3 out 0 1k\n\
.subckt DIV a b w=2u\n\
R1 a b RMOD W={2*w} L=1u\n\
.ends\n\
.model RMOD R(RSH=100)\n\
.end\n";

/// A square sheet resistor, so `R = RSH * L / W` makes the model parameter the
/// resistance itself.
const MODEL_SHEET_RESISTOR: &str = "Model sheet resistor\n\
V1 in 0 10\n\
R1 in out RMOD L=1u W=1u\n\
R2 out 0 1k\n\
.model RMOD R(RSH=100)\n\
.end\n";

/// An ideal transconductor driving a load resistor: `V(out) = GM*M*V(in)*R`.
const TRANSCONDUCTOR: &str = "Transconductor\n\
V1 in 0 1\n\
G1 0 out in 0 1m\n\
R1 out 0 1k\n\
.end\n";

/// A behavioural source, whose value is an expression and nothing else.
const BEHAVIORAL_SOURCE: &str = "Behavioral source\n\
V2 ref 0 10\n\
B1 in 0 V={1+0.1*V(ref)}\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

/// Asserts a relative agreement and prints both numbers when it fails, so a
/// regression reads as a measurement rather than as a boolean.
#[track_caller]
fn close(measured: Value, expected: Value, relative: Value, what: &str) {
    let tolerance = expected.abs().max(1.0) * relative;
    assert!(
        (measured - expected).abs() <= tolerance,
        "{what}: measured {measured:e}, closed form {expected:e}, tolerance {tolerance:e}"
    );
}

#[track_caller]
fn voltage(result: &SimulationResult, node: &str) -> Value {
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

/// The two points of a device sweep, as `(swept value, V(node))`.
#[track_caller]
fn stepped_voltages(
    deck: &str,
    device: &str,
    param: Option<&str>,
    node: &str,
    values: [Value; 2],
) -> [(Value, Value); 2] {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let points = Engine::default()
        .run_step_device(&netlist, device, param, &values, &NoAbort)
        .expect("the device sweep runs");
    assert_eq!(points.len(), 2, "a two-point sweep publishes two points");
    [
        (points[0].0, voltage(&points[0].1, node)),
        (points[1].0, voltage(&points[1].1, node)),
    ]
}

/// `|V(out)|` at one frequency for each point of a device sweep.
#[track_caller]
fn stepped_ac_magnitudes(
    deck: &str,
    device: &str,
    param: &str,
    node: &str,
    values: [Value; 2],
    frequency: Value,
) -> [Value; 2] {
    let engine = Engine::default();
    let netlist = Netlist::parse(deck).expect("deck parses");
    let command = StepCommand {
        target: StepTarget::Device,
        name: device.to_string(),
        param_name: Some(param.to_string()),
        sweep: StepSweep::List(values.to_vec()),
    };
    let stepped = engine
        .step_netlists_for_command(&netlist, &command, &values)
        .expect("the sweep materializes one netlist per point");
    assert_eq!(stepped.len(), 2);
    let magnitude = |netlist: &Netlist| {
        let sweep = engine.run_ac(netlist, &[frequency]).expect("AC runs");
        let point = &sweep[0];
        let index = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("missing AC node {node}"));
        point.voltages[index].norm()
    };
    [magnitude(&stepped[0].1), magnitude(&stepped[1].1)]
}

/// `10*R2/(R1+R2)` with `R1 = 1k`, whatever the deck spelled `R2` with.
#[test]
fn stepping_a_resistor_spelled_as_an_expression_moves_the_divider() {
    let netlist = Netlist::parse(EXPRESSION_DIVIDER).expect("deck parses");
    let nominal = Engine::default()
        .run_dc_op(&netlist)
        .expect("the authored deck solves");
    close(
        voltage(&nominal, "out"),
        10.0 * 2000.0 / 3000.0,
        1e-9,
        "the authored spelling still governs an unswept run",
    );

    for (swept, measured) in
        stepped_voltages(EXPRESSION_DIVIDER, "R2", Some("R"), "out", [1.0e3, 4.0e3])
    {
        close(
            measured,
            10.0 * swept / (1.0e3 + swept),
            1e-9,
            &format!("V(out) at R2 = {swept:e}"),
        );
    }
}

/// The stepped point is the linear resistor of the stepped value; the unswept
/// run is still the behavioural element, whose resistance `1k*(1+0.1*V(in))`
/// can only be `2k` because `V(in)` is pinned at 10 V.
#[test]
fn stepping_a_state_reading_resistor_replaces_its_expression_for_that_point() {
    let netlist = Netlist::parse(STATE_READING_DIVIDER).expect("deck parses");
    let nominal = Engine::default()
        .run_dc_op(&netlist)
        .expect("the behavioural deck solves");
    close(
        voltage(&nominal, "out"),
        10.0 * 1000.0 / 3000.0,
        1e-8,
        "an unswept state-reading resistor is still R = 1k*(1+0.1*V(in)) = 2k",
    );

    for (swept, measured) in stepped_voltages(
        STATE_READING_DIVIDER,
        "R1",
        Some("R"),
        "out",
        [1.0e3, 4.0e3],
    ) {
        close(
            measured,
            10.0 * 1000.0 / (swept + 1000.0),
            1e-8,
            &format!("V(out) at the stepped R1 = {swept:e}"),
        );
    }
}

/// Stepping the design parameter is unchanged: every expression that reads it
/// re-evaluates, so both legs move together.
#[test]
fn stepping_a_parameter_still_re_evaluates_the_expressions_that_read_it() {
    let netlist = Netlist::parse(SHARED_PARAMETER_DIVIDER).expect("deck parses");
    let points = Engine::default()
        .run_step_with_abort(&netlist, "r", &[1.0e3, 2.0e3], &NoAbort)
        .expect("the parameter sweep runs");
    assert_eq!(points.len(), 2);

    for (swept, result) in &points {
        // R1 = r, R2 = 2r, R3 = 6k: V(out) = 10*(2r||6k)/(r + 2r||6k).
        let lower = 1.0 / (1.0 / (2.0 * swept) + 1.0 / 6.0e3);
        close(
            voltage(result, "out"),
            10.0 * lower / (swept + lower),
            1e-9,
            &format!("V(out) at r = {swept:e}"),
        );
    }
}

/// A `.DC` sweep of a source whose DC value the deck spelled as an expression
/// sweeps the source, not the expression.
#[test]
fn a_dc_sweep_of_an_expression_valued_source_sweeps_it() {
    let netlist = Netlist::parse(EXPRESSION_SOURCE).expect("deck parses");
    let points = Engine::default()
        .run_dc_sweep_spec_with_report_and_abort(
            &netlist,
            "V1",
            &DcSweepSpec::linear(2.0, 8.0, 3.0),
            &NoAbort,
        )
        .expect("the source sweep runs");

    let swept: Vec<Value> = points.iter().map(|point| point.sweep_value).collect();
    assert_eq!(swept, [2.0, 5.0, 8.0]);
    for point in &points {
        close(
            voltage(&point.result, "out"),
            point.sweep_value / 2.0,
            1e-9,
            &format!("an equal divider halves V1 = {:e}", point.sweep_value),
        );
    }
}

/// The RC corner and the RL corner move as the stepped value computes them.
/// Both reactances are spelled inside a subcircuit body, where the parser keeps
/// the expression rather than folding it into a number.
#[test]
fn a_stepped_capacitor_and_inductor_spelled_as_expressions_take_the_stepped_value() {
    let frequency = 1.0e5;
    let omega = 2.0 * std::f64::consts::PI * frequency;
    let resistance = 1.0e3;

    let capacitances = [2.0e-9, 8.0e-9];
    let measured = stepped_ac_magnitudes(
        SUBCIRCUIT_LOW_PASS,
        "X1:C1",
        "C",
        "out",
        capacitances,
        frequency,
    );
    for (capacitance, measured) in capacitances.into_iter().zip(measured) {
        // |H| = 1/sqrt(1 + (wRC)^2).
        let pole = omega * resistance * capacitance;
        close(
            measured,
            1.0 / (1.0 + pole * pole).sqrt(),
            1e-9,
            &format!("|V(out)| at C1 = {capacitance:e}"),
        );
    }

    let inductances = [2.0e-3, 8.0e-3];
    let measured = stepped_ac_magnitudes(
        SUBCIRCUIT_HIGH_PASS,
        "X1:L1",
        "L",
        "out",
        inductances,
        frequency,
    );
    for (inductance, measured) in inductances.into_iter().zip(measured) {
        // |H| = wL/sqrt(R^2 + (wL)^2).
        let reactance = omega * inductance;
        close(
            measured,
            reactance / (resistance * resistance + reactance * reactance).sqrt(),
            1e-9,
            &format!("|V(out)| at L1 = {inductance:e}"),
        );
    }
}

/// A passive inside a subcircuit body keeps the swept value through hierarchy
/// elaboration, which otherwise resolves the body's expression over it.
#[test]
fn a_stepped_subcircuit_resistor_keeps_the_stepped_value_through_flattening() {
    let netlist = Netlist::parse(SUBCIRCUIT_DIVIDER).expect("deck parses");
    let nominal = Engine::default()
        .run_dc_op(&netlist)
        .expect("the authored deck solves");
    close(
        voltage(&nominal, "out"),
        10.0 * 1000.0 / 3000.0,
        1e-9,
        "the body spelling governs an unswept run",
    );

    for (swept, measured) in stepped_voltages(
        SUBCIRCUIT_DIVIDER,
        "X1:R1",
        Some("R"),
        "out",
        [1.0e3, 4.0e3],
    ) {
        close(
            measured,
            10.0 * 1000.0 / (swept + 1000.0),
            1e-9,
            &format!("V(out) at the stepped X1:R1 = {swept:e}"),
        );
    }
}

/// An instance parameter the body deferred as an expression is overridden the
/// same way, so the swept width reaches the sheet-resistance law.
#[test]
fn a_stepped_instance_parameter_overrides_the_expression_the_body_deferred() {
    // R = RSH * L / W, with RSH = 100 ohm/square and L = 1 um.
    let resistance = |width: Value| 100.0 * 1.0e-6 / width;
    let netlist = Netlist::parse(DEFERRED_INSTANCE_PARAMETER).expect("deck parses");
    let nominal = Engine::default()
        .run_dc_op(&netlist)
        .expect("the authored deck solves");
    close(
        voltage(&nominal, "out"),
        10.0 * 1000.0 / (resistance(4.0e-6) + 1000.0),
        1e-9,
        "the deferred W = 2*w = 4 um governs an unswept run",
    );

    for (swept, measured) in stepped_voltages(
        DEFERRED_INSTANCE_PARAMETER,
        "X1:R1",
        Some("W"),
        "out",
        [2.0e-6, 8.0e-6],
    ) {
        close(
            measured,
            10.0 * 1000.0 / (resistance(swept) + 1000.0),
            1e-9,
            &format!("V(out) at the stepped W = {swept:e}"),
        );
    }
}

/// A swept model parameter replaces the expression the card spelled it with,
/// which subcircuit model scoping would otherwise resolve back over it.
#[test]
fn a_stepped_model_parameter_overrides_the_expression_the_card_spelled_it_with() {
    // A square sheet resistor: R = RSH * L / W = RSH.
    for (swept, measured) in stepped_voltages(
        MODEL_SHEET_RESISTOR,
        "RMOD",
        Some("RSH"),
        "out",
        [100.0, 400.0],
    ) {
        close(
            measured,
            10.0 * 1000.0 / (swept + 1000.0),
            1e-9,
            &format!("V(out) at RSH = {swept:e}"),
        );
    }

    let mut netlist = Netlist::parse(MODEL_SHEET_RESISTOR).expect("deck parses");
    let model = netlist
        .models
        .iter_mut()
        .find(|model| model.name.eq_ignore_ascii_case("RMOD"))
        .expect("the model card parses");
    model.expr_params.push(("RSH".into(), "2*rs".into()));
    Engine::apply_model_step_value(model, "rsh", 400.0);
    assert!(
        model.expr_params.is_empty(),
        "the authored spelling must not survive the swept value: {:?}",
        model.expr_params
    );
    assert!(
        model
            .params
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("RSH") && *value == 400.0),
        "the swept value must be the card's RSH: {:?}",
        model.params
    );
}

/// An unnamed VCCS target names the device's value, which is GM. `M` stays
/// reachable by name.
#[test]
fn a_bare_vccs_sweep_names_its_transconductance_not_its_multiplicity() {
    // V(out) = GM * M * V(in) * R, with V(in) = 1 V and R = 1 kohm.
    for (swept, measured) in stepped_voltages(TRANSCONDUCTOR, "G1", None, "out", [1.0e-3, 4.0e-3]) {
        close(
            measured,
            swept * 1.0 * 1.0e3,
            1e-9,
            &format!("V(out) at the bare GM = {swept:e}"),
        );
    }
    for (swept, measured) in
        stepped_voltages(TRANSCONDUCTOR, "G1", Some("GM"), "out", [1.0e-3, 4.0e-3])
    {
        close(
            measured,
            swept * 1.0 * 1.0e3,
            1e-9,
            &format!("V(out) at the named GM = {swept:e}"),
        );
    }
    for (swept, measured) in stepped_voltages(TRANSCONDUCTOR, "G1", Some("M"), "out", [1.0, 2.0]) {
        close(
            measured,
            1.0e-3 * swept * 1.0 * 1.0e3,
            1e-9,
            &format!("V(out) at M = {swept:e}"),
        );
    }
}

/// A behavioural source stores no number for its value, so a sweep that names
/// it is refused by name instead of landing on a neighbouring field.
#[test]
fn sweeping_a_behavioral_source_value_refuses_by_name() {
    let netlist = Netlist::parse(BEHAVIORAL_SOURCE).expect("deck parses");
    let engine = Engine::default();
    for parameter in [None, Some("VALUE")] {
        let error = engine
            .run_step_device(&netlist, "B1", parameter, &[1.0, 2.0], &NoAbort)
            .expect_err("a behavioural source's value is not a sweepable field");
        let message = error.to_string();
        assert!(
            message.contains("B1") && message.contains("is an expression"),
            "the refusal must name the target and say why: {message}"
        );
    }

    // A field the element does store is still sweepable, and a field it does
    // not have is still refused with the name of the fields it has.
    for parameter in ["M", "TC1", "TC2"] {
        engine
            .run_step_device(&netlist, "B1", Some(parameter), &[1.0, 2.0], &NoAbort)
            .unwrap_or_else(|error| panic!("sweeping {parameter} must stay supported: {error}"));
    }
    let error = engine
        .run_step_device(&netlist, "B1", Some("GM"), &[1.0, 2.0], &NoAbort)
        .expect_err("a behavioural source has no GM");
    assert!(
        error.to_string().contains("use M, TC1, or TC2"),
        "the refusal must name the fields the element has: {error}"
    );
}

/// Point `k+1` is built from the authored deck, not from point `k`: a two
/// variable sweep whose points each override a different element must publish
/// the divider of its own coordinate pair.
#[test]
fn the_next_point_starts_from_the_authored_deck() {
    const TWO_VARIABLE_SWEEP: &str = "Two variable sweep\n\
.param r=1k\n\
V1 in 0 10\n\
R1 in out {2*r}\n\
R2 out 0 {3*r}\n\
.STEP R1 R LIST 1k 4k\n\
.STEP R2 R LIST 2k 8k\n\
.end\n";

    let engine = Engine::default();
    let netlist = Netlist::parse(TWO_VARIABLE_SWEEP).expect("deck parses");
    let steps: Vec<StepCommand> = netlist
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            crate::netlist::AnalysisCommand::Step(step) => Some(step.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(steps.len(), 2, "both .STEP cards parse");

    let plan = engine
        .plan_step_commands(&netlist, &steps, StepPlanLimits::new(16, 8, 64, 4096))
        .expect("the Cartesian plan is valid");
    let runs = engine
        .materialize_step_plan_with_abort(&plan, &NoAbort)
        .expect("every run materializes");
    assert_eq!(runs.len(), 4, "two values on each of two variables");

    let mut seen = Vec::new();
    for run in &runs {
        let (upper, lower) = match run.step_values() {
            [upper, lower] => (*upper, *lower),
            other => panic!("each run carries one coordinate per variable, got {other:?}"),
        };
        let result = engine
            .run_dc_op(run.netlist())
            .expect("each planned run solves");
        close(
            voltage(&result, "out"),
            10.0 * lower / (upper + lower),
            1e-9,
            &format!("V(out) at R1 = {upper:e}, R2 = {lower:e}"),
        );
        seen.push((upper, lower));
    }
    seen.sort_by(|a, b| a.partial_cmp(b).expect("finite coordinates"));
    assert_eq!(
        seen,
        [
            (1.0e3, 2.0e3),
            (1.0e3, 8.0e3),
            (4.0e3, 2.0e3),
            (4.0e3, 8.0e3)
        ],
        "the plan visits every coordinate pair exactly once"
    );
}
