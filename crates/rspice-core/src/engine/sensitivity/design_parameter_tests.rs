//! What a `.SENS` filter list selects from the one variable universe, and
//! what the design-parameter rows are worth.
//!
//! Every oracle here is a closed form written out in the test: the chain rule
//! joining a design parameter to the device fields it drives, Euler's identity
//! on a behavioural product, and the RC low-pass transfer function and its
//! magnitude, phase and normalized derivatives. No expectation is a number
//! this engine once printed.

use super::*;
use crate::analysis::AcSensitivityOutput;

/// `I(V1) = -drive/R1`: one design parameter reaching one source field.
const SOURCE_DRIVEN: &str = "Design parameter driving a source\n\
.param drive=2\n\
V1 out 0 {drive}\n\
R1 out 0 2\n\
.end\n";

/// `V(out) = 10*R2/(R1+R2)` with `R1={r}` and `R2={rr}`, `rr={2*r}`: the ratio
/// is independent of `r`, so `PARAM:R` — which is TOTAL, and moves `rr` with
/// `r` — must be the exact cancellation of two nonzero device rows, while
/// `PARAM:RR` holds `r` fixed and is the second row alone.
///
/// The second resistor is valued through a dependent parameter rather than
/// written `{2*r}` inline because an element whose value is a compound
/// expression carries no resolved value on the parsed element and therefore
/// has no device sensitivity row at all (see the report of this lane).
const RATIOMETRIC_DIVIDER: &str = "Ratiometric divider\n\
.param r=1k\n\
.param rr={2*r}\n\
V1 in 0 10\n\
R1 in out {r}\n\
R2 out 0 {rr}\n\
.end\n";

fn probe(node: usize) -> AcSensitivityOutput {
    AcSensitivityOutput::Voltage {
        positive: node,
        negative: None,
    }
}

fn filters(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

fn row_names(result: &crate::analysis::SensitivityResult) -> Vec<String> {
    let mut names = result
        .sensitivities
        .iter()
        .map(|row| row.vector_name.to_ascii_uppercase())
        .collect::<Vec<_>>();
    names.sort();
    names
}

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

/// The empty filter keeps the meaning every existing surface relies on: every
/// device and model variable, and no design parameter. Design-variable
/// sensitivity is an explicit request.
#[test]
fn a_sens_card_with_no_filter_names_no_design_parameter() {
    let engine = Engine::default();

    let netlist = Netlist::parse(SOURCE_DRIVEN).expect("deck parses");
    let result = engine
        .run_sensitivity_dc_complete(
            &netlist,
            AcSensitivityOutput::BranchCurrent("V1".into()),
            &[],
        )
        .expect("unfiltered DC sensitivity runs");
    assert_eq!(row_names(&result), ["R1", "V1"]);
    close(result.output_value, -1.0, 1e-9, "I(V1) nominal");
    close(
        result.get("R1").expect("R1 row").absolute,
        0.5,
        1e-9,
        "dI(V1)/dR1 = drive/R1^2",
    );
    close(
        result.get("V1").expect("V1 row").absolute,
        -0.5,
        1e-9,
        "dI(V1)/dV1 = -1/R1",
    );

    let netlist = Netlist::parse(RATIOMETRIC_DIVIDER).expect("deck parses");
    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &[])
        .expect("unfiltered DC sensitivity runs");
    assert_eq!(row_names(&result), ["R1", "R2", "V1"]);
    close(
        result.get("R1").expect("R1 row").absolute,
        -10.0 * 2000.0 / 9.0e6,
        1e-8,
        "dV(out)/dR1 = -V1*R2/(R1+R2)^2",
    );
    close(
        result.get("R2").expect("R2 row").absolute,
        10.0 * 1000.0 / 9.0e6,
        1e-8,
        "dV(out)/dR2 = +V1*R1/(R1+R2)^2",
    );
}

/// `PARAM:` is the one spelling that reaches the design parameters, by exact
/// name and by glob, on its own or beside a device filter.
#[test]
fn a_param_filter_selects_design_parameters_by_name_and_by_glob() {
    let netlist = Netlist::parse(SOURCE_DRIVEN).expect("deck parses");
    let engine = Engine::default();
    for selection in [
        &["PARAM:DRIVE"][..],
        &["PARAM:D*"],
        &["PARAM:*"],
        &["param:drive"],
    ] {
        let result = engine
            .run_sensitivity_dc_complete(&netlist, probe(1), &filters(selection))
            .unwrap_or_else(|error| panic!("{selection:?} must select DRIVE: {error}"));
        assert_eq!(row_names(&result), ["PARAM:DRIVE"], "filter {selection:?}");
        let row = result.get("PARAM:DRIVE").expect("design row");
        assert_eq!(row.element, "DRIVE");
        assert_eq!(row.parameter, "DRIVE");
        assert_eq!(row.element_type, ElementType::DesignParameter);
        assert_eq!(row.nominal_value, 2.0);
    }

    let mixed = engine
        .run_sensitivity_dc_complete(&netlist, probe(1), &filters(&["R1", "PARAM:*"]))
        .expect("a mixed filter list selects from both namespaces");
    assert_eq!(row_names(&mixed), ["PARAM:DRIVE", "R1"]);
}

/// A bare glob never crosses into the design-parameter namespace, however
/// suggestively the parameter is named. `.SENS V(out) R*` on a deck with a
/// `.param rload` answers exactly what it answered before this existed, and a
/// filter spelled like the parameter itself selects nothing at all rather than
/// quietly reaching it.
#[test]
fn a_bare_glob_never_selects_a_design_parameter() {
    let netlist = Netlist::parse(
        "Parameter named like a device\n\
.param rload=2\n\
V1 out 0 1\n\
R1 out 0 {rload}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    for selection in [&["R*"][..], &["*"], &["R1:*"]] {
        let result = engine
            .run_sensitivity_dc_complete(&netlist, probe(1), &filters(selection))
            .unwrap_or_else(|error| panic!("{selection:?} must still select devices: {error}"));
        assert!(
            result
                .sensitivities
                .iter()
                .all(|row| row.element_type != ElementType::DesignParameter),
            "filter {selection:?} leaked into the design parameters: {:?}",
            row_names(&result)
        );
    }
    // The parameter's own bare name belongs to no namespace this filter list
    // addresses, so it is dead rather than a way in.
    let message = engine
        .run_sensitivity_dc_complete(&netlist, probe(1), &filters(&["RLOAD"]))
        .expect_err("a bare parameter name must not reach the design parameters")
        .to_string();
    assert!(
        message.contains("no DC parameter matched filter(s) RLOAD"),
        "unexpected refusal: {message}"
    );
    let design = engine
        .run_sensitivity_dc_complete(&netlist, probe(1), &filters(&["PARAM:RLOAD"]))
        .expect("the design parameter is still reachable by its own spelling");
    assert_eq!(row_names(&design), ["PARAM:RLOAD"]);
}

/// The one collision the `PARAM:` rule can produce is an owner literally named
/// `PARAM`, whose `owner:parameter` filter would read as the design-parameter
/// namespace. It is refused by name before anything is solved.
#[test]
fn a_model_named_param_is_refused_beside_a_param_filter() {
    let netlist = Netlist::parse(
        "Model named PARAM\n\
.param sheet=100\n\
V1 in 0 1\n\
R1 in out PARAM L=10u W=1u\n\
R2 out 0 1k\n\
.model PARAM R RSH={sheet}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    let message = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["PARAM:*"]))
        .expect_err("an ambiguous owner must not be resolved silently")
        .to_string();
    assert!(
        message.contains("names the design parameters")
            && message.contains("model 'PARAM'")
            && message.contains("renamed"),
        "unexpected refusal: {message}"
    );
    // `PARAM:RSH` is the model's own parameter here, and it is refused for the
    // same reason: one spelling cannot mean two things in one filter list.
    let ambiguous = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["PARAM:RSH"]))
        .expect_err("the model's own parameter spelling is ambiguous too")
        .to_string();
    assert!(
        ambiguous.contains("names the design parameters"),
        "{ambiguous}"
    );
    // Without a `PARAM:` filter there is no ambiguity, and the model row keeps
    // the name the engine has always given it.
    let devices = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &[])
        .expect("the unfiltered study is unaffected");
    assert!(
        devices.get("PARAM:RSH").is_some(),
        "{:?}",
        row_names(&devices)
    );
}

/// `PARAM:` with nothing after it selects nothing and means nothing; it is a
/// truncated filter, not an empty selection.
#[test]
fn an_empty_param_filter_is_refused() {
    let netlist = Netlist::parse(SOURCE_DRIVEN).expect("deck parses");
    let engine = Engine::default();
    for (domain, message) in [
        (
            "DC",
            engine
                .run_sensitivity_dc_complete(&netlist, probe(1), &filters(&["PARAM:"]))
                .expect_err("a truncated filter must be refused")
                .to_string(),
        ),
        (
            "AC",
            engine
                .run_sensitivity_ac_complete(&netlist, probe(1), &[1.0e3], &filters(&["PARAM:"]))
                .expect_err("a truncated filter must be refused")
                .to_string(),
        ),
    ] {
        assert!(
            message.contains(&format!("{domain} sensitivity cannot run"))
                && message.contains("the filter 'PARAM:' names no design parameter")
                && message.contains("PARAM:*"),
            "unexpected refusal: {message}"
        );
    }
}

/// A filter item that selects nothing is a study that would silently answer a
/// narrower question. Every dead item is named, not only the all-dead case.
#[test]
fn a_filter_that_selects_nothing_is_refused_by_name() {
    let netlist = Netlist::parse(RATIOMETRIC_DIVIDER).expect("deck parses");
    let engine = Engine::default();

    let message = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["R1", "GIAN", "R9*"]))
        .expect_err("a live filter must not excuse the dead ones")
        .to_string();
    assert!(
        message.ends_with("DC sensitivity cannot run: no DC parameter matched filter(s) GIAN, R9*"),
        "unexpected refusal: {message}"
    );

    let message = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["PARAM:GIAN"]))
        .expect_err("a dead design-parameter filter is refused the same way")
        .to_string();
    assert!(
        message
            .ends_with("DC sensitivity cannot run: no DC parameter matched filter(s) PARAM:GIAN"),
        "unexpected refusal: {message}"
    );

    let message = engine
        .run_sensitivity_ac_complete(&netlist, probe(2), &[1.0e3], &filters(&["R1", "GIAN"]))
        .expect_err("the AC entry refuses the same list")
        .to_string();
    assert!(
        message.ends_with("AC sensitivity cannot run: no parameter matched filter(s) GIAN"),
        "unexpected refusal: {message}"
    );

    // A list in which every item selects something still runs.
    engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["R*", "PARAM:R"]))
        .expect("a wholly live filter list runs");
}

/// Whether this deck and parameter qualify for the exact path — captured
/// expression derivatives contracted with one transpose solve — rather than
/// the refinement driver. Every oracle below states which path its rows took.
fn takes_the_exact_path(
    engine: &Engine,
    netlist: &Netlist,
    output: &AcSensitivityOutput,
    name: &str,
    value: Value,
    frequencies: Option<&[Value]>,
) -> bool {
    engine
        .linear_parameter_sensitivity(
            netlist,
            output,
            name,
            value,
            frequencies,
            &mut 0,
            &crate::abort_signal::NoAbort,
        )
        .expect("the qualification probe itself must not fail")
        .is_some()
}

/// `I(V1) = -drive/R1`. The design parameter reaches the circuit through one
/// source field, so the chain rule has a single term and the design row must
/// equal the source row times `d(V1)/d(drive) = 1`.
///
/// Every element is a qualified linear kind and `drive` is bound to the source
/// value, so the design row takes the EXACT path; the device rows are refined
/// finite differences.
#[test]
fn the_chain_rule_joins_a_design_parameter_to_the_source_it_drives() {
    let netlist = Netlist::parse(SOURCE_DRIVEN).expect("deck parses");
    let engine = Engine::default();
    let output = AcSensitivityOutput::BranchCurrent("V1".into());
    assert!(
        takes_the_exact_path(&engine, &netlist, &output, "DRIVE", 2.0, None),
        "this deck is qualified linear: the design row must be exact"
    );

    let result = engine
        .run_sensitivity_dc_complete(&netlist, output, &filters(&["*", "PARAM:*"]))
        .expect("the union study runs");
    assert_eq!(row_names(&result), ["PARAM:DRIVE", "R1", "V1"]);
    close(result.output_value, -1.0, 1e-9, "I(V1) = -drive/R1");

    let design = result.get("PARAM:DRIVE").expect("design row").absolute;
    let source = result.get("V1").expect("V1 row").absolute;
    let resistor = result.get("R1").expect("R1 row").absolute;
    close(design, -0.5, 1e-9, "dI(V1)/d(drive) = -1/R1 (exact path)");
    close(source, -0.5, 1e-9, "dI(V1)/dV1 = -1/R1 (refined)");
    close(resistor, 0.5, 1e-9, "dI(V1)/dR1 = drive/R1^2 (refined)");
    // The chain rule with one term: d(out)/d(drive) = d(out)/dV1 * dV1/d(drive).
    close(design, source * 1.0, 1e-9, "PARAM:DRIVE = V1 * 1");
}

/// A design parameter that drives several elements is the SUM over all of
/// them, and it drives every parameter defined from it as well.
///
/// `R1={r}` with `R2={rr}`, `rr={2*r}` keeps `V(out) = 10*R2/(R1+R2)`
/// independent of `r`, so `PARAM:R` is the exact cancellation of two nonzero
/// device rows: a design row recombined from finite-difference device rows
/// could not deliver it, and a study that quietly returned "no response"
/// would be wrong. `PARAM:RR` holds `r` fixed and is the second row alone —
/// the total-versus-partial rule, measured.
///
/// Both decks are qualified linear (every element value resolves to a number
/// with a captured direction), so both design rows take the EXACT path; the
/// device rows are refined finite differences.
#[test]
fn the_chain_rule_sums_over_every_element_a_design_parameter_drives() {
    let netlist = Netlist::parse(RATIOMETRIC_DIVIDER).expect("deck parses");
    let engine = Engine::default();
    assert!(
        takes_the_exact_path(&engine, &netlist, &probe(2), "R", 1000.0, None),
        "the ratiometric divider is qualified linear: PARAM:R must be exact"
    );
    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["R*", "PARAM:*"]))
        .expect("the union study runs");
    close(result.output_value, 20.0 / 3.0, 1e-9, "V(out) = 10*2r/3r");
    assert_eq!(row_names(&result), ["PARAM:R", "PARAM:RR", "R1", "R2"]);
    let r1 = result.get("R1").expect("R1 row").absolute;
    let r2 = result.get("R2").expect("R2 row").absolute;
    close(r1, -10.0 * 2000.0 / 9.0e6, 1e-8, "dV/dR1");
    close(r2, 10.0 * 1000.0 / 9.0e6, 1e-8, "dV/dR2");
    let design = result.get("PARAM:R").expect("design row").absolute;
    // dR1/dr = 1, dR2/dr = 2, and the closed-form sum is exactly zero.
    let bound = r1.abs() * 1e-9;
    assert!(
        design.abs() <= bound,
        "PARAM:R = 1*dV/dR1 + 2*dV/dR2 = 0 in closed form, measured {design:e} \
         against a bound of {bound:e} (device row {r1:e})"
    );
    // PARAM:RR holds r fixed, so only R2 moves with it.
    close(
        result.get("PARAM:RR").expect("dependent row").absolute,
        10.0 * 1000.0 / 9.0e6,
        1e-8,
        "PARAM:RR = dV/dR2 with r held fixed",
    );

    let netlist = Netlist::parse(
        "Differential design parameter\n\
.param drive=2\n\
.param quarter={drive/4}\n\
V1 in 0 AC 1\n\
E1 a 0 in 0 {drive}\n\
E2 b 0 in 0 {quarter}\n\
.end\n",
    )
    .expect("deck parses");
    let differential = AcSensitivityOutput::Voltage {
        positive: 2,
        negative: Some(3),
    };
    assert!(
        takes_the_exact_path(
            &engine,
            &netlist,
            &differential,
            "DRIVE",
            2.0,
            Some(&[1.0e3])
        ),
        "two gain-valued VCVS sources are qualified linear: PARAM:DRIVE is exact"
    );
    let result = engine
        .run_sensitivity_ac_complete(
            &netlist,
            differential,
            &[1.0e3],
            &filters(&["E1", "E2", "PARAM:DRIVE"]),
        )
        .expect("the differential union study runs");
    close(result.output_values[0].re, 1.5, 1e-9, "V(a,b) = 2 - 0.5");
    let gain = |name: &str| result.get(name).expect("gain row").absolute[0];
    close(gain("E1").re, 1.0, 1e-5, "dV(a,b)/d(E1 gain) = V(in)");
    close(gain("E2").re, -1.0, 1e-5, "dV(a,b)/d(E2 gain) = -V(in)");
    let design = result.get("PARAM:DRIVE").expect("design row").absolute[0];
    close(design.re, 0.75, 1e-9, "PARAM:DRIVE = 1*(+1) + 1/4*(-1)");
    assert!(
        design.im.abs() <= 1e-9,
        "a real-gain chain has no quadrature derivative, got {design}"
    );
    close(
        design.re,
        gain("E1").re + 0.25 * gain("E2").re,
        1e-5,
        "the chain rule sums the two gains this parameter drives",
    );
}

/// A design parameter inside a behavioural expression reaches the circuit
/// through no target field at all: no device row carries it, which is why the
/// design rows are computed rather than recombined from device rows.
///
/// A behavioural source is not a qualified linear kind, so every row here is a
/// refined finite difference.
#[test]
fn a_design_parameter_inside_a_behavioural_expression_has_no_device_twin() {
    let netlist = Netlist::parse(
        "Behavioural design parameters\n\
.param gain=2 scale=3\n\
V1 in 0 DC 1 AC 1\n\
B1 out 0 V={gain*scale*V(in)}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    assert!(
        !takes_the_exact_path(&engine, &netlist, &probe(2), "GAIN", 2.0, None),
        "a behavioural source is not a qualified linear kind"
    );

    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["*", "PARAM:*"]))
        .expect("the union DC study runs");
    close(result.output_value, 6.0, 1e-9, "V(out) = gain*scale*V(in)");
    let row = |name: &str| result.get(name).expect("row").absolute;
    close(
        row("PARAM:GAIN"),
        3.0,
        1e-5,
        "dV(out)/d(gain) = scale*V(in)",
    );
    close(
        row("PARAM:SCALE"),
        2.0,
        1e-5,
        "dV(out)/d(scale) = gain*V(in)",
    );
    close(row("B1_M"), 6.0, 1e-5, "dV(out)/dM = gain*scale*V(in)");
    close(
        row("B1_TC1"),
        0.0,
        1e-9,
        "an isothermal study has no TC1 slope",
    );
    close(
        row("B1_TC2"),
        0.0,
        1e-9,
        "an isothermal study has no TC2 slope",
    );
    close(row("V1"), 6.0, 1e-5, "dV(out)/dV1 = gain*scale");
    // Euler's identity for a degree-one homogeneous product: each factor times
    // its own derivative returns the output.
    for (name, nominal) in [("PARAM:GAIN", 2.0), ("PARAM:SCALE", 3.0), ("B1_M", 1.0)] {
        close(
            nominal * row(name),
            result.output_value,
            1e-5,
            &format!("Euler: {name} * d(out)/d{name} = V(out)"),
        );
    }

    let ac = engine
        .run_sensitivity_ac_complete(&netlist, probe(2), &[1.0e3], &filters(&["PARAM:*"]))
        .expect("the AC study of the same deck runs");
    close(ac.output_values[0].re, 6.0, 1e-9, "AC V(out)");
    close(
        ac.get("PARAM:GAIN").expect("AC design row").absolute[0].re,
        3.0,
        1e-5,
        "AC dV(out)/d(gain)",
    );
    close(
        ac.get("PARAM:SCALE").expect("AC design row").absolute[0].re,
        2.0,
        1e-5,
        "AC dV(out)/d(scale)",
    );
}

/// A design parameter behind a model card and the model parameter it sets are
/// the same derivative when the binding is the identity: `RSH={sheet}` gives
/// `d(RSH)/d(sheet) = 1`, so `PARAM:SHEET` must equal `RMOD:RSH`, and both
/// must equal the divider's closed form scaled by the sheet geometry.
///
/// A modelled resistor is not a qualified linear kind, so both rows are
/// refined finite differences.
#[test]
fn a_design_parameter_behind_a_model_card_matches_the_model_parameter_row() {
    let netlist = Netlist::parse(
        "Model sensitivity\n\
.param sheet=100\n\
V1 in 0 AC 1\n\
R1 in out RMOD L=10u W=1u\n\
R2 out 0 1k\n\
.model RMOD R RSH={sheet}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    assert!(
        !takes_the_exact_path(&engine, &netlist, &probe(2), "SHEET", 100.0, Some(&[1.0e3])),
        "a modelled resistor is not a qualified linear kind"
    );
    let result = engine
        .run_sensitivity_ac_complete(
            &netlist,
            probe(2),
            &[1.0e3],
            &filters(&["RMOD:*", "PARAM:*"]),
        )
        .expect("the union model study runs");
    // R1 = RSH*L/W = 1k, so V(out) = 0.5 and dV/dRSH = -R2/(R1+R2)^2 * (L/W).
    close(result.output_values[0].re, 0.5, 1e-9, "V(out) = R2/(R1+R2)");
    let model = result.get("RMOD:RSH").expect("model row").absolute[0].re;
    let design = result.get("PARAM:SHEET").expect("design row").absolute[0].re;
    close(model, -2.5e-3, 1e-5, "dV(out)/d(RSH) = -R2/(R1+R2)^2 * L/W");
    close(design, -2.5e-3, 1e-5, "dV(out)/d(sheet) = dV/d(RSH) * 1");
    close(design, model, 1e-5, "PARAM:SHEET = RMOD:RSH");
}

/// The full frequency-domain contract on a first-order low pass, against the
/// closed form of every quantity the study publishes.
///
/// `H = 1/(1 + jwRC)`; with `x = wRC`,
/// `dH/dR = -jwC/(1+jx)^2`, `dH/dC = -jwR/(1+jx)^2`,
/// `d|H|/dR = -w^2*R*C^2*(1+x^2)^(-3/2)`, `d|H|/dC = -w^2*R^2*C*(1+x^2)^(-3/2)`,
/// `d(arg H)/dR = -wC/(1+x^2)`, `d(arg H)/dC = -wR/(1+x^2)`, and both
/// normalized derivatives are `-jx/(1+jx)`. `C1={c}` gives `dC/dc = 1`, so
/// `PARAM:C` must be `C1`.
///
/// The deck is qualified linear, so `PARAM:C` takes the EXACT path while the
/// device rows `R1` and `C1` are refined finite differences. Each quantity is
/// held to 1e-5 of its own sweep-wide maximum for a refined row and 1e-9 for
/// the exact one — a true relative bound where the quantity is at its scale,
/// and an absolute bound five (or nine) decades below it where the quantity
/// passes through zero.
#[test]
fn an_rc_low_pass_sensitivity_follows_its_closed_form_across_the_sweep() {
    const R: Value = 1.0e3;
    const C: Value = 1.0e-9;
    let corner = 1.0 / (std::f64::consts::TAU * R * C);

    let netlist = Netlist::parse(
        "Parametric low pass\n\
.param c=1n\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
C1 out 0 {c}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    let frequencies = super::super::sp::card_frequency_grid(
        crate::netlist::FreqVariation::Dec,
        5,
        corner / 100.0,
        corner * 100.0,
        &crate::abort_signal::NoAbort,
    )
    .expect("the decade grid builds");
    assert_eq!(frequencies.len(), 21, "DEC 5 over four decades");
    assert!(
        takes_the_exact_path(&engine, &netlist, &probe(2), "C", C, Some(&frequencies)),
        "the RC deck is qualified linear: PARAM:C must take the exact path"
    );

    let result = engine
        .run_sensitivity_ac_complete(
            &netlist,
            probe(2),
            &frequencies,
            &filters(&["R1", "C1", "PARAM:*"]),
        )
        .expect("the RC union study runs");
    assert_eq!(
        result
            .sensitivities
            .iter()
            .map(|trace| trace.vector_name.clone())
            .collect::<Vec<_>>(),
        ["C1", "PARAM:C", "R1"]
    );

    // Closed forms, point by point, before any measurement is read.
    let mut expected: Vec<[Complex64; 6]> = Vec::with_capacity(frequencies.len());
    for (index, &frequency) in frequencies.iter().enumerate() {
        let omega = std::f64::consts::TAU * frequency;
        let x = omega * R * C;
        let denominator = Complex64::new(1.0, x);
        let transfer = denominator.inv();
        let roll = (1.0 + x * x).powf(-1.5);
        expected.push([
            // dH/dR, dH/dC
            Complex64::new(0.0, -omega * C) / (denominator * denominator),
            Complex64::new(0.0, -omega * R) / (denominator * denominator),
            // d|H|/dR, d|H|/dC
            Complex64::new(-omega * omega * R * C * C * roll, 0.0),
            Complex64::new(-omega * omega * R * R * C * roll, 0.0),
            // d(arg H)/dR, d(arg H)/dC
            Complex64::new(-omega * C / (1.0 + x * x), 0.0),
            Complex64::new(-omega * R / (1.0 + x * x), 0.0),
        ]);
        close(result.output_values[index].re, transfer.re, 1e-9, "Re H");
        close(result.output_values[index].im, transfer.im, 1e-9, "Im H");
    }
    let scale = |column: usize| {
        expected
            .iter()
            .map(|point| point[column].norm())
            .fold(0.0_f64, f64::max)
    };

    let resistor = result.get("R1").expect("R1 trace");
    let capacitor = result.get("C1").expect("C1 trace");
    let design = result.get("PARAM:C").expect("design trace");
    assert_eq!(design.nominal_value, C);
    for (index, frequency) in frequencies.iter().enumerate() {
        let omega = std::f64::consts::TAU * frequency;
        let x = omega * R * C;
        let normalized = Complex64::new(0.0, -x) / Complex64::new(1.0, x);
        for (trace, columns, relative) in [
            (resistor, [0, 2, 4], 1e-5),
            (capacitor, [1, 3, 5], 1e-5),
            // PARAM:C takes the exact path: dC/dc = 1, so it answers the
            // capacitor's own closed forms to LU roundoff.
            (design, [1, 3, 5], 1e-9),
        ] {
            let at = |quantity: &str, measured: Value, want: Value, column: usize| {
                let tolerance = scale(column) * relative;
                assert!(
                    (measured - want).abs() <= tolerance,
                    "{} {quantity} at {frequency:e} Hz: measured {measured:e}, \
                     closed form {want:e}, tolerance {tolerance:e}",
                    trace.vector_name
                );
            };
            at(
                "d(Re H)/dp",
                trace.absolute[index].re,
                expected[index][columns[0]].re,
                columns[0],
            );
            at(
                "d(Im H)/dp",
                trace.absolute[index].im,
                expected[index][columns[0]].im,
                columns[0],
            );
            at(
                "d|H|/dp",
                trace.magnitude[index].value().expect("magnitude"),
                expected[index][columns[1]].re,
                columns[1],
            );
            at(
                "d(arg H)/dp",
                trace.phase[index].value().expect("phase"),
                expected[index][columns[2]].re,
                columns[2],
            );
            let measured = trace.normalized[index].value().expect("normalized");
            for (quantity, measured, want) in [
                ("Re normalized", measured.re, normalized.re),
                ("Im normalized", measured.im, normalized.im),
            ] {
                assert!(
                    (measured - want).abs() <= relative,
                    "{} {quantity} at {frequency:e} Hz: measured {measured:e}, \
                     closed form {want:e}",
                    trace.vector_name
                );
            }
        }
        // dC/dc = 1, so the design trace is the capacitor trace.
        let (measured, want) = (design.absolute[index], capacitor.absolute[index]);
        let tolerance = scale(1) * 1e-5;
        assert!(
            (measured - want).norm() <= tolerance,
            "PARAM:C at {frequency:e} Hz: measured {measured}, C1 trace {want}, \
             tolerance {tolerance:e}"
        );
    }
}

/// The probe a design row is differentiated at and the probe a device row is
/// differentiated at are the same node, although one study runs on the
/// authored hierarchy and the other on the flattened copy. A subcircuit
/// resistor valued `{r}` gives both rows the same closed form, so a probe that
/// addressed different nodes in the two netlists could not agree.
///
/// A design parameter read inside a subcircuit body still qualifies for the
/// exact path, so the design row is exact and the flattened device row is a
/// refined finite difference; the two are compared at the refined bound.
#[test]
fn a_hierarchical_deck_reports_design_and_device_rows_at_the_same_probe() {
    let netlist = Netlist::parse(
        "Hierarchical design parameter\n\
.param r=1k\n\
V1 in 0 10\n\
XDIV in out DIVIDER\n\
.subckt DIVIDER input output\n\
RTOP input output {r}\n\
RBOT output 0 1k\n\
.ends\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    assert!(
        takes_the_exact_path(&engine, &netlist, &probe(2), "R", 1000.0, None),
        "a parameter read inside a subcircuit body keeps its captured direction"
    );
    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["*RTOP", "PARAM:*"]))
        .expect("the hierarchical union study runs");
    close(result.output_value, 5.0, 1e-9, "V(out) = 10*RBOT/(r+RBOT)");
    let device = result
        .sensitivities
        .iter()
        .find(|row| row.vector_name.to_ascii_uppercase().ends_with("RTOP"))
        .expect("the flattened subcircuit resistor");
    let design = result.get("PARAM:R").expect("design row");
    close(device.absolute, -2.5e-3, 1e-5, "dV(out)/d(RTOP) (refined)");
    close(
        design.absolute,
        -2.5e-3,
        1e-9,
        "dV(out)/dr = dV/d(RTOP) * 1 (exact path)",
    );
    close(
        design.absolute,
        device.absolute,
        1e-5,
        "the two netlists resolve V(out) to the same node",
    );
}

/// Design rows draw on the study's own run budget, not a private one: the
/// complete entry seeds the counter with its nominal solve and every
/// perturbation a design row replays is charged to the same limit.
#[test]
fn design_parameter_rows_spend_the_study_budget() {
    let netlist = Netlist::parse(
        "Budgeted design parameter\n\
.param gain=2\n\
V1 in 0 DC 1\n\
B1 out 0 V={gain*V(in)}\n\
.end\n",
    )
    .expect("deck parses");
    let mut config = crate::engine::SimulationConfig::default();
    config.resource_limits.max_batch_runs = 1;
    let error = Engine::new(config)
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["PARAM:*"]))
        .expect_err("one run buys the nominal solve and nothing else");
    assert!(
        matches!(error, SimulationError::ResourceLimit(_)),
        "unexpected error: {error}"
    );
    assert!(
        error.to_string().contains("batch_runs"),
        "the refusal must name the budget it exceeded: {error}"
    );

    let generous = Engine::default()
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["PARAM:*"]))
        .expect("the same study runs against the default budget");
    close(
        generous.get("PARAM:GAIN").expect("design row").absolute,
        1.0,
        1e-5,
        "dV(out)/d(gain) = V(in)",
    );
}

/// What a netlist with no retained source does for a `PARAM:` filter.
///
/// Every shipping frontend parses text, so every netlist a user reaches this
/// through has a source. An engine-API caller can hand over a netlist whose
/// source was dropped; the parameter replay then sets the parameter value
/// without re-elaborating the deck, so the expressions that already read it
/// keep their resolved values. The row is published with the derivative that
/// netlist actually has, not omitted and not silently taken from a deck the
/// caller did not provide.
#[test]
fn a_design_parameter_on_a_deck_with_no_retained_source_is_reported_as_it_resolves() {
    let mut netlist = Netlist::parse(SOURCE_DRIVEN).expect("deck parses");
    netlist.source_text = None;
    netlist.source_path = None;
    let engine = Engine::default();
    let output = AcSensitivityOutput::BranchCurrent("V1".into());
    assert!(
        !takes_the_exact_path(&engine, &netlist, &output, "DRIVE", 2.0, None),
        "the exact path needs the retained source it replays"
    );
    let result = engine
        .run_sensitivity_dc_complete(&netlist, output, &filters(&["PARAM:*"]))
        .expect("the design row is still published");
    let row = result.get("PARAM:DRIVE").expect("design row");
    assert_eq!(
        row.absolute, 0.0,
        "without a retained source the replay cannot re-elaborate the source \
         expression, so the response is flat; measured {}",
        row.absolute
    );
}
