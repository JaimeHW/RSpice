//! Tolerance evaluation: one measured value against one expectation.
//!
//! Scalars pass iff `|a − e| ≤ abs + rel·|e|`; complex values compare
//! magnitude in dB and phase in degrees separately (`abs_db`, `abs_deg`);
//! roots match one-to-one by nearest neighbour with the scalar rule on the
//! distance; event traces match values exactly and times within `edge_abs`
//! with equal counts. Statistics pass inside a stated interval, or inside a
//! stated ratio interval against the oracle's own statistic.

use std::collections::BTreeMap;

use num_complex::Complex64;

use super::design::Quantity;
use super::events::Trace;
use super::measure::Measured;

/// What a quantity is compared against.
#[derive(Debug, Clone, PartialEq)]
pub enum Expected {
    /// Another producer's measure of the same quantity.
    Measured(Measured),
    /// A numeric interval fixed in advance (statistics).
    Interval([f64; 2]),
    /// Only the quantity's structural assertions apply (an event lattice).
    Structural,
}

/// The tolerance a quantity states.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Tolerance {
    pub abs: f64,
    pub rel: f64,
    pub abs_db: Option<f64>,
    pub abs_deg: Option<f64>,
    pub edge_abs: f64,
}

impl Tolerance {
    pub fn of(quantity: &Quantity) -> Self {
        Self {
            abs: quantity.abs.unwrap_or(0.0),
            rel: quantity.rel.unwrap_or(0.0),
            abs_db: quantity.abs_db,
            abs_deg: quantity.abs_deg,
            edge_abs: quantity.edge_abs.unwrap_or(0.0),
        }
    }

    pub fn allowed(&self, expected: f64) -> f64 {
        self.abs + self.rel * expected.abs()
    }
}

/// The comparison of one quantity against one oracle.
#[derive(Debug, Clone, PartialEq)]
pub struct Verdict {
    pub pass: bool,
    /// Worst observed deviation over its allowance; above 1 fails.
    pub severity: f64,
    /// `|a−e|`, allowed, and where — or why it could not be compared.
    pub detail: String,
}

impl Verdict {
    fn pass(severity: f64, detail: String) -> Self {
        Self {
            pass: true,
            severity,
            detail,
        }
    }

    fn fail(detail: impl Into<String>) -> Self {
        Self {
            pass: false,
            severity: f64::INFINITY,
            detail: detail.into(),
        }
    }

    fn from_severity(severity: f64, detail: String) -> Self {
        Self {
            pass: severity <= 1.0,
            severity,
            detail,
        }
    }
}

/// Compare `actual` (RSpice) with `expected` under `tolerance`.
pub fn compare(actual: &Measured, expected: &Expected, tolerance: Tolerance) -> Verdict {
    match (actual, expected) {
        (Measured::Values(actual), Expected::Measured(Measured::Values(expected))) => {
            scalars(actual, expected, tolerance)
        }
        (Measured::Complex(actual), Expected::Measured(Measured::Complex(expected))) => {
            complex(actual, expected, tolerance)
        }
        (
            Measured::Roots { poles, zeros },
            Expected::Measured(Measured::Roots {
                poles: expected_poles,
                zeros: expected_zeros,
            }),
        ) => {
            let pole = roots("pole", poles, expected_poles, tolerance);
            let zero = roots("zero", zeros, expected_zeros, tolerance);
            if pole.severity >= zero.severity {
                pole
            } else {
                zero
            }
        }
        (Measured::Events(actual), Expected::Measured(Measured::Events(expected))) => {
            events(actual, expected, tolerance)
        }
        (Measured::Values(actual), Expected::Interval([low, high])) => {
            let Some(value) = actual.first() else {
                return Verdict::fail("no value to place in the interval");
            };
            let inside = *value >= *low && *value <= *high;
            let detail = format!("{value:.6e} in [{low:.6e}, {high:.6e}]");
            if inside {
                Verdict::pass(0.0, detail)
            } else {
                Verdict::fail(format!("{value:.6e} outside [{low:.6e}, {high:.6e}]"))
            }
        }
        (_, Expected::Structural) => Verdict::pass(0.0, "structural assertions only".into()),
        (actual, expected) => Verdict::fail(format!(
            "measure shapes differ: RSpice {} vs oracle {}",
            shape(actual),
            match expected {
                Expected::Measured(measured) => shape(measured),
                Expected::Interval(_) => "interval",
                Expected::Structural => "structural",
            }
        )),
    }
}

fn shape(measured: &Measured) -> &'static str {
    match measured {
        Measured::Values(_) => "values",
        Measured::Complex(_) => "complex",
        Measured::Roots { .. } => "roots",
        Measured::Events(_) => "events",
    }
}

fn scalars(actual: &[f64], expected: &[f64], tolerance: Tolerance) -> Verdict {
    if actual.len() != expected.len() {
        return Verdict::fail(format!(
            "count differs: RSpice {} values, oracle {}",
            actual.len(),
            expected.len()
        ));
    }
    if actual.is_empty() {
        return Verdict::fail("no values to compare");
    }
    let mut worst = (0usize, 0.0f64, 0.0f64, 0.0f64);
    for (index, (a, e)) in actual.iter().zip(expected).enumerate() {
        // Equal infinities (a gain margin no sweep reaches) agree.
        let diff = if a == e { 0.0 } else { (a - e).abs() };
        let allowed = tolerance.allowed(*e);
        let severity = if allowed > 0.0 {
            diff / allowed
        } else if diff == 0.0 {
            0.0
        } else {
            f64::INFINITY
        };
        if severity > worst.1 || index == 0 {
            worst = (index, severity, diff, allowed);
        }
    }
    let (index, severity, diff, allowed) = worst;
    Verdict::from_severity(
        severity,
        format!(
            "[{index}] |a−e|={diff:.3e} allowed {allowed:.3e} (a={:.6e}, e={:.6e})",
            actual[index], expected[index]
        ),
    )
}

/// Magnitude difference in dB and wrapped phase difference in degrees.
pub fn db_deg(actual: Complex64, expected: Complex64) -> (f64, f64) {
    let db = 20.0 * (actual.norm() / expected.norm()).log10();
    let mut deg = (actual.arg() - expected.arg()).to_degrees();
    while deg > 180.0 {
        deg -= 360.0;
    }
    while deg < -180.0 {
        deg += 360.0;
    }
    (db, deg)
}

fn complex(actual: &[Complex64], expected: &[Complex64], tolerance: Tolerance) -> Verdict {
    if actual.len() != expected.len() || actual.is_empty() {
        return Verdict::fail(format!(
            "count differs: RSpice {} values, oracle {}",
            actual.len(),
            expected.len()
        ));
    }
    let (Some(abs_db), Some(abs_deg)) = (tolerance.abs_db, tolerance.abs_deg) else {
        return Verdict::fail("a complex quantity needs abs_db and abs_deg");
    };
    let mut worst = (0usize, -1.0f64, 0.0, 0.0);
    for (index, (a, e)) in actual.iter().zip(expected).enumerate() {
        let (db, deg) = db_deg(*a, *e);
        let severity = if db.is_finite() && deg.is_finite() {
            (db.abs() / abs_db.max(f64::MIN_POSITIVE))
                .max(deg.abs() / abs_deg.max(f64::MIN_POSITIVE))
        } else {
            f64::INFINITY
        };
        if severity > worst.1 {
            worst = (index, severity, db, deg);
        }
    }
    let (index, severity, db, deg) = worst;
    Verdict::from_severity(
        severity,
        format!("[{index}] Δ={db:.3e} dB, {deg:.3e}° allowed {abs_db:e} dB, {abs_deg:e}°"),
    )
}

fn roots(
    label: &str,
    actual: &[Complex64],
    expected: &[Complex64],
    tolerance: Tolerance,
) -> Verdict {
    if actual.len() != expected.len() {
        return Verdict::fail(format!(
            "{label} count differs: RSpice {}, oracle {}",
            actual.len(),
            expected.len()
        ));
    }
    let mut unmatched: Vec<Complex64> = actual.to_vec();
    let mut worst = (0.0f64, String::from("no roots"));
    for e in expected {
        let Some((position, a)) = unmatched
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| (**x - e).norm().total_cmp(&(**y - e).norm()))
            .map(|(position, a)| (position, *a))
        else {
            return Verdict::fail(format!("{label}s exhausted while matching"));
        };
        unmatched.remove(position);
        let diff = (a - e).norm();
        let allowed = tolerance.allowed(e.norm());
        let severity = if allowed > 0.0 {
            diff / allowed
        } else if diff == 0.0 {
            0.0
        } else {
            f64::INFINITY
        };
        if severity >= worst.0 {
            worst = (
                severity,
                format!("{label} |a−e|={diff:.3e} allowed {allowed:.3e} at e={e}"),
            );
        }
    }
    Verdict::from_severity(worst.0, worst.1)
}

fn events(
    actual: &BTreeMap<String, Trace>,
    expected: &BTreeMap<String, Trace>,
    tolerance: Tolerance,
) -> Verdict {
    let mut worst = Verdict::pass(0.0, "events equal".into());
    for (net, expected_trace) in expected {
        let Some(actual_trace) = actual.get(net) else {
            return Verdict::fail(format!("net {net} absent"));
        };
        if actual_trace.len() != expected_trace.len() {
            return Verdict::fail(format!(
                "net {net}: {} events vs oracle {}",
                actual_trace.len(),
                expected_trace.len()
            ));
        }
        for (index, ((at, av), (et, ev))) in actual_trace.iter().zip(expected_trace).enumerate() {
            if av != ev {
                return Verdict::fail(format!(
                    "net {net} event {index}: value {av} vs {ev} at {et:e}"
                ));
            }
            let diff = (at - et).abs();
            let severity = if tolerance.edge_abs > 0.0 {
                diff / tolerance.edge_abs
            } else if diff == 0.0 {
                0.0
            } else {
                f64::INFINITY
            };
            if severity > worst.severity {
                worst = Verdict::from_severity(
                    severity,
                    format!(
                        "net {net} event {index}: |Δt|={diff:.3e} allowed {:.3e}",
                        tolerance.edge_abs
                    ),
                );
            }
        }
    }
    worst
}

/// Every edge of every net lies on `t0 + k·period` within `edge_abs`.
pub fn lattice(traces: &BTreeMap<String, Trace>, t0: f64, period: f64, edge_abs: f64) -> Verdict {
    let mut worst = Verdict::pass(0.0, "every edge on the lattice".into());
    let mut edges = 0usize;
    for (net, trace) in traces {
        for pair in trace.windows(2) {
            if pair[0].1 == pair[1].1 {
                continue;
            }
            edges += 1;
            let time = pair[1].0;
            let k = ((time - t0) / period).round();
            let diff = (time - (t0 + k * period)).abs();
            let severity = if edge_abs > 0.0 {
                diff / edge_abs
            } else if diff == 0.0 {
                0.0
            } else {
                f64::INFINITY
            };
            if severity > worst.severity {
                worst = Verdict::from_severity(
                    severity,
                    format!(
                        "net {net} edge at {time:.6e}: {diff:.3e} off the lattice, allowed {edge_abs:.3e}"
                    ),
                );
            }
        }
    }
    if edges == 0 {
        return Verdict::fail("no edges to place on the lattice");
    }
    worst
}

/// A statistic against the oracle's own statistic, as a ratio interval.
pub fn ratio(actual: f64, expected: f64, [low, high]: [f64; 2]) -> Verdict {
    let value = actual / expected;
    let detail = format!("ratio {value:.4} (a={actual:.4e}, e={expected:.4e}) in [{low}, {high}]");
    if value.is_finite() && value >= low && value <= high {
        Verdict::pass(0.0, detail)
    } else {
        Verdict::fail(detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tol(abs: f64, rel: f64) -> Tolerance {
        Tolerance {
            abs,
            rel,
            ..Default::default()
        }
    }

    #[test]
    fn scalar_rule_is_abs_plus_rel_of_expected() {
        let expected = Expected::Measured(Measured::Values(vec![100.0, 10.0]));
        assert!(
            compare(
                &Measured::Values(vec![100.9, 10.0]),
                &expected,
                tol(0.0, 0.01)
            )
            .pass
        );
        let verdict = compare(
            &Measured::Values(vec![100.0, 10.2]),
            &expected,
            tol(0.0, 0.01),
        );
        assert!(!verdict.pass, "{verdict:?}");
        assert!(verdict.detail.starts_with("[1]"), "{}", verdict.detail);
        assert!(!compare(&Measured::Values(vec![1.0]), &expected, tol(1.0, 1.0)).pass);
        assert!(
            compare(
                &Measured::Values(vec![3.0]),
                &Expected::Measured(Measured::Values(vec![0.0])),
                tol(3.0, 0.0)
            )
            .pass
        );
    }

    #[test]
    fn complex_values_compare_in_db_and_degrees() {
        let e = Complex64::from_polar(10.0, 0.5);
        let a = Complex64::from_polar(
            10.0 * 10f64.powf(0.0005 / 20.0),
            0.5 + 0.001f64.to_radians(),
        );
        let tolerance = Tolerance {
            abs_db: Some(0.001),
            abs_deg: Some(0.01),
            ..Default::default()
        };
        let expected = Expected::Measured(Measured::Complex(vec![e]));
        assert!(compare(&Measured::Complex(vec![a]), &expected, tolerance).pass);
        assert!(!compare(&Measured::Complex(vec![-a]), &expected, tolerance).pass);
        let (_, deg) = db_deg(
            Complex64::from_polar(1.0, 3.1),
            Complex64::from_polar(1.0, -3.1),
        );
        assert!(deg.abs() < 10.0, "phase must wrap: {deg}");
    }

    #[test]
    fn roots_match_one_to_one_and_counts_must_agree() {
        let expected = Expected::Measured(Measured::Roots {
            poles: vec![Complex64::new(-1.0, 0.0), Complex64::new(-10.0, 5.0)],
            zeros: vec![],
        });
        let actual = Measured::Roots {
            poles: vec![Complex64::new(-10.0, 5.0001), Complex64::new(-1.00001, 0.0)],
            zeros: vec![],
        };
        assert!(compare(&actual, &expected, tol(0.0, 1e-4)).pass);
        let missing = Measured::Roots {
            poles: vec![Complex64::new(-1.0, 0.0)],
            zeros: vec![],
        };
        assert!(!compare(&missing, &expected, tol(1.0, 1.0)).pass);
    }

    #[test]
    fn events_need_exact_values_equal_counts_and_close_times() {
        let mut expected = BTreeMap::new();
        expected.insert(
            "q".to_owned(),
            vec![(0.0, "0".to_owned()), (1e-9, "1".to_owned())],
        );
        let mut actual = expected.clone();
        actual.get_mut("q").unwrap()[1].0 = 1.0005e-9;
        let tolerance = Tolerance {
            edge_abs: 1e-12,
            ..Default::default()
        };
        assert!(
            compare(
                &Measured::Events(actual.clone()),
                &Expected::Measured(Measured::Events(expected.clone())),
                tolerance
            )
            .pass
        );
        actual.get_mut("q").unwrap()[1].1 = "x".into();
        assert!(
            !compare(
                &Measured::Events(actual.clone()),
                &Expected::Measured(Measured::Events(expected.clone())),
                tolerance
            )
            .pass
        );
        actual.get_mut("q").unwrap().pop();
        assert!(
            !compare(
                &Measured::Events(actual),
                &Expected::Measured(Measured::Events(expected)),
                tolerance
            )
            .pass
        );
    }

    #[test]
    fn lattice_and_intervals() {
        let mut traces = BTreeMap::new();
        traces.insert(
            "dv".to_owned(),
            vec![
                (0.0, "0".to_owned()),
                (102e-9, "1".to_owned()),
                (302.5e-9, "0".to_owned()),
            ],
        );
        assert!(lattice(&traces, 102e-9, 200e-9, 2.5e-9).pass);
        assert!(!lattice(&traces, 102e-9, 200e-9, 0.1e-9).pass);
        assert!(
            compare(
                &Measured::Values(vec![1.0]),
                &Expected::Interval([0.72, 3.3]),
                Tolerance::default()
            )
            .pass
        );
        assert!(
            !compare(
                &Measured::Values(vec![3.4]),
                &Expected::Interval([0.72, 3.3]),
                Tolerance::default()
            )
            .pass
        );
        assert!(ratio(1.0, 1.2, [0.48, 2.08]).pass);
        assert!(!ratio(1.0, 0.4, [0.48, 2.08]).pass);
    }
}
