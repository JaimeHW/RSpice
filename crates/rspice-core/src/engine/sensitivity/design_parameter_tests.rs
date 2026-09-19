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
