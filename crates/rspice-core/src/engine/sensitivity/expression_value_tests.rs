//! What a `.SENS` study owes an element whose value the deck spelled as an
//! expression.
//!
//! The sensitivity of an output to element `X` is `d(out)/d(value(X))` taken
//! at the value the run used, however the deck wrote that value down. So the
//! contract measured here is an equality between two decks: a divider whose
//! lower resistor is `2k` and a divider whose lower resistor is `{2*r}` with
//! `.param r=1k` publish the same row — same name, same nominal, the same
//! derivative to the last bit — and perturbing that row moves that element
//! alone, never the parameter inside its expression.
//!
//! Every oracle here is a closed form written out in the test: the two-resistor
//! divider and its parallel load, the chain rule joining `PARAM:R` to the
//! device rows it drives, an ideal VCVS, a resistor multiplicity, and the RC
//! low-pass transfer function with its magnitude derivative. No expectation is
//! a number this engine once printed.
//!
//! Every device row below is a refined finite difference, which is what every
//! device row has always been; the design rows `PARAM:*` take the exact linear
//! path, and each test that reads one says so.

use super::*;
use crate::analysis::AcSensitivityOutput;

/// `V(out) = 10*R2/(R1+R2)` written in literals.
const LITERAL_DIVIDER: &str = "Literal divider\n\
V1 in 0 10\n\
R1 in out 1k\n\
R2 out 0 2k\n\
.end\n";

/// The same circuit, with both resistors valued through one design parameter
/// and the lower one spelled as a compound expression.
const EXPRESSION_DIVIDER: &str = "Expression valued divider\n\
.param r=1k\n\
R1 in out {r}\n\
R2 out 0 {2*r}\n\
V1 in 0 10\n\
.end\n";

/// `R1 = 2r`, `R2 = 3r` and a literal `R3` across `R2`, so the design row is
/// not the accidental zero a ratiometric divider produces.
const SHARED_PARAMETER_DIVIDER: &str = "Shared design parameter\n\
.param r=1k\n\
R1 in out {2*r}\n\
R2 out 0 {3*r}\n\
R3 out 0 6k\n\
V1 in 0 10\n\
.end\n";

/// The literal twin of [`SHARED_PARAMETER_DIVIDER`], element for element.
const SHARED_PARAMETER_LITERALS: &str = "Shared design parameter\n\
R1 in out 2k\n\
R2 out 0 3k\n\
R3 out 0 6k\n\
V1 in 0 10\n\
.end\n";

/// The ratiometric divider written the way a parameterized design writes it:
/// `R2` is `{2*r}` inline, not a dependent `.param`. `V(out) = 10*2r/3r` does
/// not depend on `r` at all, so `PARAM:R` — which is total — must be the exact
/// cancellation `1*dV/dR1 + 2*dV/dR2 = 0` of two nonzero device rows.
const RATIOMETRIC_DIVIDER: &str = "Ratiometric divider\n\
.param r=1k\n\
V1 in 0 10\n\
R1 in out {r}\n\
R2 out 0 {2*r}\n\
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

/// Whether this deck and parameter qualify for the exact linear path rather
/// than the refinement driver.
fn takes_the_exact_path(
    engine: &Engine,
    netlist: &Netlist,
    output: &AcSensitivityOutput,
    name: &str,
    value: Value,
) -> bool {
    engine
        .linear_parameter_sensitivity(
            netlist,
            output,
            name,
            value,
            None,
            &mut 0,
            &crate::abort_signal::NoAbort,
        )
        .expect("the qualification probe itself must not fail")
        .is_some()
}

/// Both decks build the same circuit, so both must publish the same rows. The
/// expression deck used to publish two of the three: `R2` was dropped outright
/// because the parsed element carried no resolved value for the collector to
/// read.
#[test]
fn an_expression_valued_resistor_reports_the_row_a_literal_one_reports() {
    let engine = Engine::default();
    let study = |deck: &str| {
        engine
            .run_sensitivity_dc_complete(&Netlist::parse(deck).expect("deck parses"), probe(2), &[])
            .expect("the unfiltered DC study runs")
    };
    let literal = study(LITERAL_DIVIDER);
    let expression = study(EXPRESSION_DIVIDER);

    assert_eq!(row_names(&literal), ["R1", "R2", "V1"]);
    assert_eq!(row_names(&expression), ["R1", "R2", "V1"]);
    assert_eq!(literal.output_value, expression.output_value);
    close(literal.output_value, 20.0 / 3.0, 1e-9, "V(out) = 10*2k/3k");

    // The closed forms first, then the equality of the two spellings.
    for (name, expected) in [
        ("R1", -10.0 * 2000.0 / 9.0e6),
        ("R2", 10.0 * 1000.0 / 9.0e6),
        ("V1", 2.0 / 3.0),
    ] {
        let literal_row = literal.get(name).expect("literal row");
        let expression_row = expression.get(name).expect("expression row");
        close(
            expression_row.absolute,
            expected,
            1e-8,
            &format!("dV(out)/d{name}"),
        );
        assert_eq!(
            expression_row.nominal_value, literal_row.nominal_value,
            "{name} nominal value differs between the two spellings"
        );
        assert_eq!(
            expression_row.absolute, literal_row.absolute,
            "{name} derivative differs between the two spellings"
        );
        assert_eq!(
            expression_row.normalized, literal_row.normalized,
            "{name} normalized sensitivity differs between the two spellings"
        );
        assert_eq!(expression_row.element_type, literal_row.element_type);
    }
}

/// A device target perturbs the element, not the parameter its value reads.
/// `R1` and `R2` are both defined from `r`; moving `R1` leaves `R2` where the
/// nominal run put it, which is why each device row equals its literal twin
/// exactly while the design row `PARAM:R` — which moves both at once — is a
/// different number entirely.
#[test]
fn perturbing_an_expression_valued_element_leaves_its_siblings_at_nominal() {
    let engine = Engine::default();
    let parameterized = Netlist::parse(SHARED_PARAMETER_DIVIDER).expect("deck parses");
    let literals = Netlist::parse(SHARED_PARAMETER_LITERALS).expect("literal deck parses");

    let expression = engine
        .run_sensitivity_dc_complete(&parameterized, probe(2), &filters(&["R*", "PARAM:*"]))
        .expect("the union study runs");
    let literal = engine
        .run_sensitivity_dc_complete(&literals, probe(2), &filters(&["R*"]))
        .expect("the literal study runs");
    assert_eq!(row_names(&expression), ["PARAM:R", "R1", "R2", "R3"]);
    assert_eq!(row_names(&literal), ["R1", "R2", "R3"]);
    close(expression.output_value, 5.0, 1e-9, "V(out) = 10*2k/4k");
    assert_eq!(expression.output_value, literal.output_value);

    // R2 || R3 = 2k, so V(out) = 10*2k/(2k+2k) and
    // dV/dR1 = -10*Rb/(R1+Rb)^2, dV/dRb = +10*R1/(R1+Rb)^2,
    // dRb/dR2 = (R3/(R2+R3))^2 = 4/9, dRb/dR3 = (R2/(R2+R3))^2 = 1/9.
    let branch = 10.0 * 2000.0 / 16.0e6;
    for (name, expected) in [
        ("R1", -branch),
        ("R2", branch * 4.0 / 9.0),
        ("R3", branch / 9.0),
    ] {
        let row = expression.get(name).expect("expression row");
        close(row.absolute, expected, 1e-8, &format!("dV(out)/d{name}"));
        assert_eq!(
            row.absolute,
            literal.get(name).expect("literal row").absolute,
            "{name} moved a sibling: its row differs from the literal deck's"
        );
    }

    assert!(
        takes_the_exact_path(&engine, &parameterized, &probe(2), "R", 1000.0),
        "every element is a qualified linear kind: PARAM:R must be exact"
    );
    let design = expression.get("PARAM:R").expect("design row").absolute;
    let r1 = expression.get("R1").expect("R1 row").absolute;
    let r2 = expression.get("R2").expect("R2 row").absolute;
    // dR1/dr = 2 and dR2/dr = 3; R3 does not read r at all.
    close(
        design,
        2.0 * (-branch) + 3.0 * (branch * 4.0 / 9.0),
        1e-8,
        "PARAM:R = 2*dV/dR1 + 3*dV/dR2",
    );
    close(
        design,
        2.0 * r1 + 3.0 * r2,
        1e-8,
        "the chain rule, measured",
    );
    // 2*dV/dR1 + 3*dV/dR2 is a third of dV/dR1 away from it: the total
    // derivative and the device row are different measurements of this deck.
    assert!(
        (design - r1).abs() > r1.abs() * 0.1,
        "the design row must not be mistaken for the device row: \
         PARAM:R {design:e}, R1 {r1:e}"
    );
}

/// The fixture this lane restores. `V(out) = 10*2r/3r` is independent of `r`,
/// so the total derivative `PARAM:R` is the exact cancellation of two nonzero
/// device rows — and both of those rows exist only because the element values
/// are now read through their expressions.
///
/// `PARAM:R` takes the exact path; `R1` and `R2` are refined finite
/// differences.
#[test]
fn the_ratiometric_divider_obeys_the_chain_rule_through_its_expressions() {
    let engine = Engine::default();
    let netlist = Netlist::parse(RATIOMETRIC_DIVIDER).expect("deck parses");
    assert!(
        takes_the_exact_path(&engine, &netlist, &probe(2), "R", 1000.0),
        "the ratiometric divider is qualified linear: PARAM:R must be exact"
    );

    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &filters(&["R*", "PARAM:*"]))
        .expect("the union study runs");
    assert_eq!(row_names(&result), ["PARAM:R", "R1", "R2"]);
    close(result.output_value, 20.0 / 3.0, 1e-9, "V(out) = 10*2r/3r");

    let r1 = result.get("R1").expect("R1 row").absolute;
    let r2 = result.get("R2").expect("R2 row").absolute;
    close(r1, -10.0 * 2000.0 / 9.0e6, 1e-8, "dV(out)/dR1");
    close(r2, 10.0 * 1000.0 / 9.0e6, 1e-8, "dV(out)/dR2");
    assert_eq!(result.get("R2").expect("R2 row").nominal_value, 2000.0);

    // dR1/dr = 1, dR2/dr = 2, and the closed-form sum is exactly zero.
    let design = result.get("PARAM:R").expect("design row").absolute;
    let bound = r1.abs() * 1e-9;
    assert!(
        design.abs() <= bound,
        "PARAM:R = 1*dV/dR1 + 2*dV/dR2 = 0 in closed form, measured {design:e} \
         against a bound of {bound:e} (device row {r1:e})"
    );
    let recombined = r1 + 2.0 * r2;
    assert!(
        recombined.abs() <= r1.abs() * 1e-8,
        "the device rows themselves must cancel through the chain rule: \
         R1 {r1:e} + 2*R2 {r2:e} = {recombined:e}"
    );
}

/// The other two fields a parameterized deck writes as expressions. Both are
/// evaluated where they are parsed rather than deferred onto the element, so
/// both already had rows; this measures that they are the rows the closed form
/// asks for, and keeps that cell of the matrix honest.
///
/// `V(out) = gain * V(mid)` with `V(mid) = V1/2`, so
/// `dV(out)/dV1 = gain/2` and `dV(out)/d(gain) = V(mid)`.
#[test]
fn an_expression_valued_source_and_gain_report_their_rows() {
    let netlist = Netlist::parse(
        "Expression valued source and gain\n\
.param vdd=1 k=3\n\
V1 in 0 {2*vdd}\n\
R1 in mid 1k\n\
R2 mid 0 1k\n\
E1 out 0 mid 0 {2*k}\n\
RL out 0 1k\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(3), &filters(&["V1", "E1"]))
        .expect("the study runs");
    assert_eq!(row_names(&result), ["E1", "V1"]);
    close(result.output_value, 6.0, 1e-9, "V(out) = 2*k * vdd");

    let source = result.get("V1").expect("source row");
    let gain = result.get("E1").expect("gain row");
    assert_eq!(source.nominal_value, 2.0);
    assert_eq!(gain.nominal_value, 6.0);
    close(source.absolute, 3.0, 1e-8, "dV(out)/dV1 = gain/2");
    close(gain.absolute, 1.0, 1e-8, "dV(out)/d(E1 gain) = V(mid)");
}

/// An instance parameter written as an expression, on the simplest device
/// whose instance parameter has a closed form: a resistor multiplicity, where
/// the electrical value is `R/M` exactly (`apply_resistor_instance_scaling`).
///
/// With `R2 = 4k` and `M = 2`, `R2` contributes `2k` to the divider, so
/// `dV(out)/dM = dV/dR_eff * (-R2/M^2)` and `dV(out)/dR2 = dV/dR_eff / M`.
#[test]
fn an_expression_valued_instance_parameter_reports_its_row() {
    let netlist = Netlist::parse(
        "Expression valued multiplicity\n\
.param m0=1\n\
V1 in 0 10\n\
R1 in out 1k\n\
R2 out 0 4k M={2*m0}\n\
.end\n",
    )
    .expect("deck parses");
    let engine = Engine::default();
    let result = engine
        .run_sensitivity_dc_complete(&netlist, probe(2), &[])
        .expect("the unfiltered study runs");
    assert_eq!(row_names(&result), ["R1", "R2", "R2_M", "V1"]);
    close(result.output_value, 20.0 / 3.0, 1e-9, "V(out) = 10*2k/3k");

    let effective = 10.0 * 1000.0 / 9.0e6; // dV(out)/d(R2/M)
    let multiplicity = result.get("R2_M").expect("multiplicity row");
    assert_eq!(multiplicity.nominal_value, 2.0);
    close(
        multiplicity.absolute,
        effective * -(4000.0 / 4.0),
        1e-6,
        "dV(out)/dM = dV/dR_eff * (-R2/M^2)",
    );
    close(
        result.get("R2").expect("R2 row").absolute,
        effective / 2.0,
        1e-8,
        "dV(out)/dR2 = dV/dR_eff / M",
    );
    close(
        result.get("R1").expect("R1 row").absolute,
        -10.0 * 2000.0 / 9.0e6,
        1e-8,
        "dV(out)/dR1",
    );
}

/// The frequency-domain half of the contract, on a first-order low pass whose
/// resistor and capacitor are both spelled as compound expressions.
///
/// `H = 1/(1 + jwRC)`; with `x = wRC`, `dH/dR = -jwC/(1+jx)^2`,
/// `dH/dC = -jwR/(1+jx)^2`, `d|H|/dR = -w^2*R*C^2*(1+x^2)^(-3/2)` and
/// `d|H|/dC = -w^2*R^2*C*(1+x^2)^(-3/2)`. Both rows are refined finite
/// differences, held to 1e-5 of each quantity's own sweep-wide maximum.
#[test]
fn an_expression_valued_rc_low_pass_follows_its_closed_form() {
    const R: Value = 2.0e3;
    const C: Value = 1.0e-9;

    let netlist = Netlist::parse(
        "Expression valued low pass\n\
.param r0=1k c0=500p\n\
V1 in 0 AC 1\n\
R1 in out {2*r0}\n\
C1 out 0 {2*c0}\n\
.end\n",
    )
    .expect("deck parses");
    let corner = 1.0 / (std::f64::consts::TAU * R * C);
    let frequencies = [corner / 10.0, corner, corner * 10.0];
    let engine = Engine::default();
    let result = engine
        .run_sensitivity_ac_complete(&netlist, probe(2), &frequencies, &filters(&["R1", "C1"]))
        .expect("the AC study runs");
    assert_eq!(
        result
            .sensitivities
            .iter()
            .map(|trace| trace.vector_name.clone())
            .collect::<Vec<_>>(),
        ["C1", "R1"]
    );
    assert_eq!(result.get("R1").expect("R1 trace").nominal_value, R);
    assert_eq!(result.get("C1").expect("C1 trace").nominal_value, C);

    // Closed forms, point by point, before any measurement is read.
    let mut expected: Vec<[Complex64; 4]> = Vec::with_capacity(frequencies.len());
    for (index, &frequency) in frequencies.iter().enumerate() {
        let omega = std::f64::consts::TAU * frequency;
        let x = omega * R * C;
        let denominator = Complex64::new(1.0, x);
        let transfer = denominator.inv();
        let roll = (1.0 + x * x).powf(-1.5);
        expected.push([
            Complex64::new(0.0, -omega * C) / (denominator * denominator),
            Complex64::new(0.0, -omega * R) / (denominator * denominator),
            Complex64::new(-omega * omega * R * C * C * roll, 0.0),
            Complex64::new(-omega * omega * R * R * C * roll, 0.0),
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

    for (name, complex_column, magnitude_column) in [("R1", 0, 2), ("C1", 1, 3)] {
        let trace = result.get(name).expect("trace");
        for (index, frequency) in frequencies.iter().enumerate() {
            let at = |quantity: &str, measured: Value, want: Value, column: usize| {
                let tolerance = scale(column) * 1e-5;
                assert!(
                    (measured - want).abs() <= tolerance,
                    "{name} {quantity} at {frequency:e} Hz: measured {measured:e}, \
                     closed form {want:e}, tolerance {tolerance:e}"
                );
            };
            at(
                "d(Re H)/dp",
                trace.absolute[index].re,
                expected[index][complex_column].re,
                complex_column,
            );
            at(
                "d(Im H)/dp",
                trace.absolute[index].im,
                expected[index][complex_column].im,
                complex_column,
            );
            at(
                "d|H|/dp",
                trace.magnitude[index].value().expect("magnitude"),
                expected[index][magnitude_column].re,
                magnitude_column,
            );
        }
    }
}
