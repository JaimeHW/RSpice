use super::*;
use crate::calculator::ast::CalculatorExpr;
use crate::calculator::evaluator::evaluate;
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::{CalcValue, ComplexValue, RealValue};
use crate::spice_value::parse_spice_value_checked;

fn parse(text: &str) -> CalculatorExpr {
    try_parse(text, parse_spice_value_checked).unwrap_or_else(|error| panic!("{text}: {error}"))
}

fn wave(name: &str, x: &[f64], y: &[f64]) -> RetainedWaveform {
    RetainedWaveform::new(name, x.to_vec(), y.to_vec()).with_unit("V")
}

/// Evaluate `text` over `own`, whose X is in `unit`, with `baseline` pinned
/// and also retained as Run 2.
fn evaluate_over(
    text: &str,
    own: &[RetainedWaveform],
    unit: Option<&str>,
    baseline: &[RetainedWaveform],
    baseline_unit: Option<&str>,
) -> Result<CalcValue, EvaluationError> {
    let datasets = RunDatasets {
        own,
        coordinate_unit: unit,
        runs: vec![NamedRun {
            names: vec![BASELINE.to_owned(), "Run 2".to_owned()],
            waveforms: baseline,
            coordinate_unit: baseline_unit,
        }],
    };
    evaluate(
        &parse(text),
        &WaveformsContext::new(own).with_datasets(&datasets),
    )
}

fn real_samples(value: CalcValue) -> (Vec<f64>, Vec<f64>) {
    match value {
        CalcValue::Real(RealValue::Waveform(x, y)) => (x, y),
        other => panic!("expected a real waveform, got {other:?}"),
    }
}

fn close(left: &[f64], right: &[f64]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| (left - right).abs() <= 1e-12 * right.abs().max(1.0))
}

#[test]
fn a_signal_names_the_run_it_is_read_from() {
    let of = |signal: &str, dataset: &str| CalculatorExpr::WaveformRef {
        signal: signal.to_owned(),
        dataset: Some(dataset.to_owned()),
    };
    assert_eq!(parse("V(out)@baseline"), of("V(out)", "baseline"));
    assert_eq!(parse("V(out)@\"Run 3\""), of("V(out)", "Run 3"));
    assert_eq!(parse("I(R1) @ baseline"), of("I(R1)", "baseline"));
    assert_eq!(parse("out@baseline"), of("out", "baseline"));
    assert_eq!(
        parse("V(out) - V(out)@baseline"),
        CalculatorExpr::binary(
            crate::calculator::ast::BinaryOp::Sub,
            CalculatorExpr::wave("V(out)"),
            of("V(out)", "baseline"),
        )
    );
    let error = try_parse("V(out)@", parse_spice_value_checked)
        .expect_err("@ names a run")
        .to_string();
    assert!(error.contains("expected a run after '@'"), "{error}");
}

#[test]
fn the_difference_from_an_identical_run_is_zero() {
    let x = [0.0, 1.0e-3, 2.0e-3, 3.0e-3];
    let y = [0.0, 0.7, -0.2, 1.1];
    let own = [wave("V(out)", &x, &y)];
    let baseline = [wave("V(out)", &x, &y)];
    let value = evaluate_over(
        "V(out) - V(out)@baseline",
        &own,
        Some("s"),
        &baseline,
        Some("s"),
    )
    .expect("the runs share an axis");
    let (axis, difference) = real_samples(value);
    assert!(close(&axis, &x));
    assert!(
        difference.iter().all(|value| *value == 0.0),
        "{difference:?}"
    );
}

/// A baseline sampled at other times is read onto the result's own times,
/// and a known offset is what the difference shows; the result's times past
/// the baseline's end are missing, not extrapolated.
#[test]
fn the_difference_from_a_shifted_run_is_its_offset() {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let own = [wave("V(out)", &x, &[0.0, 2.0, 4.0, 6.0, 8.0])];
    let baseline_x = [0.0, 0.5, 1.5, 3.5];
    let baseline_y = baseline_x.map(|x| 2.0 * x - 0.25);
    let baseline = [wave("V(out)", &baseline_x, &baseline_y)];
    let value = evaluate_over(
        "V(out) - V(out)@\"Run 2\"",
        &own,
        Some("s"),
        &baseline,
        Some("s"),
    )
    .expect("the runs overlap");
    let (axis, difference) = real_samples(value);
    assert!(close(&axis, &x));
    assert!(close(&difference[..4], &[0.25; 4]), "{difference:?}");
    assert!(difference[4].is_nan(), "{difference:?}");
}

#[test]
fn a_run_that_sweeps_another_quantity_is_refused_with_its_reason() {
    let own = [wave("V(out)", &[0.0, 1.0, 2.0], &[0.0, 1.0, 2.0])];
    let baseline = [wave("V(out)", &[1.0, 10.0, 100.0], &[0.0, 1.0, 2.0])];
    let error = evaluate_over(
        "V(out) - V(out)@baseline",
        &own,
        Some("s"),
        &baseline,
        Some("Hz"),
    )
    .expect_err("time and frequency are not one axis")
    .to_string();
    assert!(
        error.contains("V(out)@baseline sweeps X in Hz and this result sweeps it in s"),
        "{error}"
    );
}

#[test]
fn a_run_with_no_x_in_common_is_refused() {
    let own = [wave("V(out)", &[5.0, 6.0, 7.0], &[0.0, 1.0, 2.0])];
    let baseline = [wave("V(out)", &[0.0, 1.0, 2.0], &[0.0, 1.0, 2.0])];
    let error = evaluate_over("V(out)@baseline", &own, Some("s"), &baseline, Some("s"))
        .expect_err("the runs share no X")
        .to_string();
    assert!(error.contains("which no X of this result"), "{error}");
}

#[test]
fn a_signal_in_another_unit_is_refused() {
    let own = [wave("V(out)", &[0.0, 1.0, 2.0], &[0.0, 1.0, 2.0])];
    let baseline = [
        RetainedWaveform::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]).with_unit("A"),
    ];
    let error = evaluate_over(
        "V(out) - V(out)@baseline",
        &own,
        Some("s"),
        &baseline,
        Some("s"),
    )
    .expect_err("volts and amps do not subtract")
    .to_string();
    assert!(
        error.contains("V(out) is in V here and in A in baseline"),
        "{error}"
    );
}

#[test]
fn an_unpinned_baseline_says_how_to_pin_one() {
    let own = [wave("V(out)", &[0.0, 1.0], &[0.0, 1.0])];
    let datasets = RunDatasets::<RetainedWaveform> {
        own: &own,
        coordinate_unit: Some("s"),
        runs: Vec::new(),
    };
    let error = evaluate(
        &parse("V(out)@baseline"),
        &WaveformsContext::new(&own).with_datasets(&datasets),
    )
    .expect_err("no baseline is pinned")
    .to_string();
    assert!(error.contains("no baseline is pinned here"), "{error}");
}

/// On a decade sweep the baseline is read between its points along log X, so
/// a response that is straight in log X stays straight.
#[test]
fn a_decade_sweep_is_read_along_log_x() {
    let own_x = [1.0, 10f64.sqrt(), 10.0, 1000f64.sqrt(), 100.0];
    let own = [wave("V(out)", &own_x, &own_x.map(f64::log10))];
    let baseline_x = [1.0, 10.0, 100.0];
    let baseline = [wave("V(out)", &baseline_x, &baseline_x.map(f64::log10))];
    let value = evaluate_over(
        "V(out) - V(out)@baseline",
        &own,
        Some("Hz"),
        &baseline,
        Some("Hz"),
    )
    .expect("the sweeps overlap");
    let (_, difference) = real_samples(value);
    assert!(close(&difference, &[0.0; 5]), "{difference:?}");
}

/// Complex samples are subtracted as complex numbers: two responses of one
/// magnitude and different phase differ.
#[test]
fn complex_samples_are_subtracted_before_they_are_projected() {
    let x = [1.0, 10.0, 100.0];
    let complex = |name: &str, real: [f64; 3], imag: [f64; 3]| {
        let magnitude = real
            .iter()
            .zip(&imag)
            .map(|(real, imag)| real.hypot(*imag))
            .collect::<Vec<_>>();
        RetainedWaveform::new(name, x.to_vec(), magnitude)
            .with_unit("V")
            .with_complex_components(name, real.to_vec(), imag.to_vec())
    };
    let own = [complex("V(out)", [0.0, 0.0, 0.0], [1.0, 1.0, 1.0])];
    let baseline = [complex("V(out)", [1.0, 1.0, 1.0], [0.0, 0.0, 0.0])];
    let value = evaluate_over(
        "V(out) - V(out)@baseline",
        &own,
        Some("Hz"),
        &baseline,
        Some("Hz"),
    )
    .expect("the sweeps share an axis");
    let CalcValue::Complex(ComplexValue::Waveform(_, samples)) = value else {
        panic!("a complex difference stays complex: {value:?}");
    };
    assert!(
        samples
            .iter()
            .all(|sample| (sample.re + 1.0).abs() < 1e-12 && (sample.im - 1.0).abs() < 1e-12),
        "{samples:?}"
    );
}

#[test]
fn a_difference_from_another_run_is_told_from_other_expressions() {
    for (text, difference) in [
        ("V(out) - V(out)@baseline", true),
        ("V(out)@\"Run 2\" - V(out)", true),
        ("V(out)@baseline", false),
        ("V(out) - V(in)", false),
        ("V(out) + V(out)@baseline", false),
    ] {
        assert_eq!(is_run_difference(&parse(text)), difference, "{text}");
    }
}
