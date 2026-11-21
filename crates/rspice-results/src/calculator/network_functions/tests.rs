use super::*;
use crate::calculator::evaluator::{UnitContext, evaluate, infer_unit};
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::waveform::RetainedWaveform;

fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn c(re: f64, im: f64) -> Complex64 {
    Complex64::new(re, im)
}

fn polar(magnitude: f64, degrees: f64) -> Complex64 {
    Complex64::from_polar(magnitude, degrees.to_radians())
}

fn scalar(value: Complex64) -> CalcValue {
    CalcValue::Complex(ComplexValue::Scalar(value))
}

fn number(value: f64) -> CalcValue {
    CalcValue::Real(RealValue::Scalar(value))
}

fn coefficients(s: [Complex64; 4]) -> Vec<CalcValue> {
    s.into_iter().map(scalar).collect()
}

fn real_of(name: &str, args: Vec<CalcValue>) -> f64 {
    match dispatch(name, args).unwrap() {
        CalcValue::Real(RealValue::Scalar(value)) => value,
        other => panic!("{name} returned {other:?}"),
    }
}

fn complex_of(name: &str, args: Vec<CalcValue>) -> Complex64 {
    match dispatch(name, args).unwrap() {
        CalcValue::Complex(ComplexValue::Scalar(value)) => value,
        other => panic!("{name} returned {other:?}"),
    }
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

fn close_complex(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() <= 1.0e-12 * expected.norm().max(1.0),
        "{actual} != {expected}"
    );
}

/// A matched attenuator of voltage ratio `a` is unconditionally stable with
/// K = (1 + a⁴)/2a², μ = 1/a², and every gain, MAG included, equal to its
/// insertion gain a².
#[test]
fn a_matched_attenuator_is_stable_and_every_gain_is_its_loss() {
    let a = 0.5;
    let s = [c(0.0, 0.0), c(a, 0.0), c(a, 0.0), c(0.0, 0.0)];
    close(
        real_of("kf", coefficients(s)),
        (1.0 + a.powi(4)) / (2.0 * a * a),
    );
    close(real_of("mu", coefficients(s)), 1.0 / (a * a));
    close(real_of("mu_prime", coefficients(s)), 1.0 / (a * a));
    close(real_of("b1f", coefficients(s)), 1.0 - a.powi(4));
    for gain in ["gt", "ga", "gp", "gmax"] {
        close(real_of(gain, coefficients(s)), a * a);
    }
    close(real_of("gmsg", coefficients(s)), 1.0);
}

/// A unilateral amplifier (S12 = 0): GT at the port terminations is |S21|²,
/// GA and GP divide by the mismatch at the open port, and MAG is the
/// unilateral |S21|²/((1 − |S11|²)(1 − |S22|²)).
#[test]
fn a_unilateral_amplifier_reads_the_textbook_gains() {
    let (s11, s21, s22) = (polar(0.6, -60.0), polar(4.0, 120.0), polar(0.5, -30.0));
    let s = [s11, c(0.0, 0.0), s21, s22];
    let forward = s21.norm_sqr();
    close(real_of("gt", coefficients(s)), forward);
    close(
        real_of("ga", coefficients(s)),
        forward / (1.0 - s22.norm_sqr()),
    );
    close(
        real_of("gp", coefficients(s)),
        forward / (1.0 - s11.norm_sqr()),
    );
    close(
        real_of("gmax", coefficients(s)),
        forward / ((1.0 - s11.norm_sqr()) * (1.0 - s22.norm_sqr())),
    );
    // A conjugate-matched source and load reach MAG.
    let mut matched = coefficients(s);
    matched.extend([scalar(s11.conj()), scalar(s22.conj())]);
    close(real_of("gt", matched), real_of("gmax", coefficients(s)));
    // K has no reverse transmission to divide by: a hole, not a number.
    assert!(real_of("kf", coefficients(s)).is_nan());
}

/// A lossless reciprocal two-port, a series reactance, sits exactly on the
/// stability boundary: K = 1 and μ = 1.
#[test]
fn a_lossless_two_port_sits_on_the_stability_boundary() {
    let z = c(0.0, 1.0); // j50 Ω on 50 Ω
    let s11 = z / (z + 2.0);
    let s21 = 2.0 / (z + 2.0);
    let s = [s11, s21, s21, s11];
    close(real_of("kf", coefficients(s)), 1.0);
    close(real_of("mu", coefficients(s)), 1.0);
    close(real_of("b1f", coefficients(s)), 0.0);
}

/// VSWR = (1 + |Γ|)/(1 − |Γ|); a full reflection has none.
#[test]
fn vswr_reads_the_reflection_magnitude() {
    close(real_of("vswr", vec![scalar(polar(1.0 / 3.0, 75.0))]), 2.0);
    close(real_of("vswr", vec![number(0.0)]), 1.0);
    assert!(real_of("vswr", vec![number(1.0)]).is_nan());
}

/// S from a Z matrix on references `r`: S = (z − I)(z + I)⁻¹ with
/// z = D⁻¹ZD⁻¹, written out for two ports.
fn scattering(z: [[Complex64; 2]; 2], r: [f64; 2]) -> [Complex64; 4] {
    let n = |i: usize, j: usize| z[i][j] / (r[i] * r[j]).sqrt();
    let (a, b, cc, d) = (n(0, 0), n(0, 1), n(1, 0), n(1, 1));
    let det = (a + 1.0) * (d + 1.0) - b * cc;
    // (z − I)·adj(z + I) / det(z + I)
    let s11 = ((a - 1.0) * (d + 1.0) - b * cc) / det;
    let s12 = 2.0 * b / det;
    let s21 = 2.0 * cc / det;
    let s22 = ((d - 1.0) * (a + 1.0) - cc * b) / det;
    [s11, s12, s21, s22]
}

/// A tee of known impedances on unequal references reads its own Z back,
/// Y as Z's inverse, and H from Z by definition.
#[test]
fn a_tee_reads_its_impedance_admittance_and_hybrid_parameters() {
    let (za, zb, zc) = (c(30.0, 10.0), c(20.0, -15.0), c(80.0, 40.0));
    let z = [[za + zc, zc], [zc, zb + zc]];
    let r = [50.0, 75.0];
    let s = scattering(z, r);
    let call = |name: &str, i: f64, j: f64| {
        let mut args = vec![number(i), number(j)];
        args.extend(coefficients(s));
        args.extend([number(r[0]), number(r[1])]);
        complex_of(name, args)
    };
    for (i, j) in [(1, 1), (1, 2), (2, 1), (2, 2)] {
        close_complex(call("zp", i as f64, j as f64), z[i - 1][j - 1]);
    }
    let det = z[0][0] * z[1][1] - z[0][1] * z[1][0];
    close_complex(call("yp", 1.0, 1.0), z[1][1] / det);
    close_complex(call("yp", 1.0, 2.0), -z[0][1] / det);
    close_complex(call("yp", 2.0, 1.0), -z[1][0] / det);
    close_complex(call("yp", 2.0, 2.0), z[0][0] / det);
    close_complex(call("hp", 1.0, 1.0), det / z[1][1]);
    close_complex(call("hp", 1.0, 2.0), z[0][1] / z[1][1]);
    close_complex(call("hp", 2.0, 1.0), -z[1][0] / z[1][1]);
    close_complex(call("hp", 2.0, 2.0), 1.0 / z[1][1]);
}

/// A series element has no impedance matrix, so Z is a hole; its admittance
/// is ±1/Z and its hybrid matrix [Z, 1; −1, 0], on the default 50 Ω.
#[test]
fn a_series_element_has_no_impedance_matrix() {
    let zs = c(100.0, 200.0);
    let z = zs / 50.0;
    let s11 = z / (z + 2.0);
    let s21 = 2.0 / (z + 2.0);
    let call = |name: &str, i: f64, j: f64| {
        let mut args = vec![number(i), number(j)];
        args.extend(coefficients([s11, s21, s21, s11]));
        complex_of(name, args)
    };
    assert!(call("zp", 2.0, 1.0).re.is_nan());
    close_complex(call("yp", 1.0, 1.0), 1.0 / zs);
    close_complex(call("yp", 2.0, 1.0), -1.0 / zs);
    close_complex(call("hp", 1.0, 1.0), zs);
    close_complex(call("hp", 1.0, 2.0), c(1.0, 0.0));
    close_complex(call("hp", 2.0, 1.0), c(-1.0, 0.0));
    close_complex(call("hp", 2.0, 2.0), c(0.0, 0.0));
}

/// Over retained traces a function is a trace on their frequency grid, a
/// scalar argument holding at every sample; traces on different grids, an
/// out-of-range port, and a wrong count are refused.
#[test]
fn over_traces_a_function_is_a_trace_and_mismatches_are_refused() {
    let x = vec![1.0e9, 2.0e9];
    let trace = |values: [Complex64; 2]| {
        CalcValue::Complex(ComplexValue::Waveform(x.clone(), values.to_vec()))
    };
    let s21 = trace([c(2.0, 0.0), c(0.0, 3.0)]);
    let args = vec![number(0.0), number(0.0), s21, number(0.0)];
    match dispatch("gt", args).unwrap() {
        CalcValue::Real(RealValue::Waveform(axis, gains)) => {
            assert_eq!(axis, x);
            close(gains[0], 4.0);
            close(gains[1], 9.0);
        }
        other => panic!("gt returned {other:?}"),
    }
    let other_grid =
        CalcValue::Complex(ComplexValue::Waveform(vec![1.0, 2.0], vec![c(0.0, 0.0); 2]));
    assert!(matches!(
        dispatch(
            "kf",
            vec![
                trace([c(0.1, 0.0); 2]),
                other_grid,
                number(1.0),
                number(0.0)
            ]
        ),
        Err(EvaluationError::WaveformMismatch(_))
    ));
    let mut out_of_range = vec![number(3.0), number(1.0)];
    out_of_range.extend(coefficients([c(0.0, 0.0); 4]));
    assert!(matches!(
        dispatch("zp", out_of_range),
        Err(EvaluationError::TypeMismatch(_))
    ));
    assert!(matches!(
        dispatch("kf", vec![number(0.0); 3]),
        Err(EvaluationError::ArgCountMismatch {
            expected: 4,
            actual: 3,
            ..
        })
    ));
}

/// Through the parser and the retained columns an S-parameter run keeps:
/// `S21` names the complex coefficient, and the stability factor and a gain
/// in decibels evaluate on its grid with the units unit algebra states.
#[test]
fn an_expression_over_retained_coefficients_reads_its_value_and_unit() {
    let x = vec![1.0e9, 2.0e9];
    let column = |name: &str, values: [Complex64; 2]| {
        RetainedWaveform::new(
            format!("|{name}|"),
            x.clone(),
            values.iter().map(|value| value.norm()).collect::<Vec<_>>(),
        )
        .with_complex_components(
            name,
            values.iter().map(|value| value.re).collect::<Vec<_>>(),
            values.iter().map(|value| value.im).collect::<Vec<_>>(),
        )
        .with_unit("1")
    };
    let a = 0.5;
    let waveforms = [
        column("S11", [c(0.0, 0.0); 2]),
        column("S12", [c(a, 0.0); 2]),
        column("S21", [c(a, 0.0); 2]),
        column("S22", [c(0.0, 0.0); 2]),
    ];
    let context = WaveformsContext::new(&waveforms);
    let read =
        |text: &str| match evaluate(&try_parse(text, plain_number).unwrap(), &context).unwrap() {
            CalcValue::Real(RealValue::Waveform(axis, values)) => {
                assert_eq!(axis, x);
                values
            }
            other => panic!("{text} returned {other:?}"),
        };
    for k in read("kf(S11, S12, S21, S22)") {
        close(k, (1.0 + a.powi(4)) / (2.0 * a * a));
    }
    for gain in read("dB10(gt(S11, S12, S21, S22))") {
        close(gain, 20.0 * a.log10());
    }

    struct Ratios;
    impl UnitContext for Ratios {
        fn signal_unit(&self, _: &str, _: Option<&str>) -> Option<String> {
            Some("1".to_owned())
        }
        fn x_unit(&self) -> String {
            "Hz".to_owned()
        }
    }
    let unit = |text: &str| infer_unit(&try_parse(text, plain_number).unwrap(), &Ratios).unwrap();
    for ratio in [
        "kf(S11, S12, S21, S22)",
        "dB10(gmax(S11, S12, S21, S22))",
        "hp(2, 1, S11, S12, S21, S22)",
    ] {
        let unit = unit(ratio);
        assert!(
            unit.is_none() || unit.label() == "dB",
            "{ratio}: {}",
            unit.label()
        );
    }
    assert_eq!(unit("zp(2, 1, S11, S12, S21, S22)").label(), "Ω");
    assert_eq!(unit("yp(1, 1, S11, S12, S21, S22)").label(), "S");
    assert_eq!(unit("hp(1, 1, S11, S12, S21, S22)").label(), "Ω");
    assert_eq!(unit("hp(2, 2, S11, S12, S21, S22)").label(), "S");
}
