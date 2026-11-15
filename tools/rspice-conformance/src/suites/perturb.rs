//! Tolerance perturbation: proof that a corpus comparison can see a wrong
//! number.
//!
//! A reference suite that passes says the simulator agrees with its oracle
//! to within each case's tolerance. It does not say the tolerance is tight
//! enough to notice anything, or that the comparison consults a number at
//! all. This module asks that second question directly. With
//! [`PERTURB_ENV`] set to `e` (for example `0.01`), every comparison primitive
//! of the circuit suites scales RSpice's value by `1 + e` before comparing it
//! with the reference; a complex value is also rotated by `e` radians so a
//! phase-only check moves too. Every case that passes unperturbed must then
//! fail. One that still passes is *blind* to an error of that size, because
//! its tolerance admits it or because its verdict never reads a perturbed
//! value, and the suites' full-corpus tests report every such case, with the
//! tolerance it was held to, instead of asserting their usual census.
//!
//! The hook sits on the RSpice operand of each value comparison, never on the
//! reference, and not on a coordinate a suite checks under its own axis
//! tolerance (a time, frequency, index or step value), so a perturbed run
//! measures the margin each value tolerance leaves. The variable is read
//! once per process; the per-case child processes inherit it. Every
//! comparison happens on the native side, so the mixed corpus's
//! browser-worker route is perturbed like the others.

use std::sync::OnceLock;

use num_complex::Complex64;

/// The opt-in environment variable: the relative error `e` to apply.
pub const PERTURB_ENV: &str = "RSPICE_CONFORMANCE_PERTURB";

/// A relative error applied to RSpice's side of every comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perturbation {
    epsilon: f64,
}

impl Perturbation {
    /// A perturbation of relative size `epsilon`: finite, nonzero, and
    /// smaller than one in magnitude, so a scaled value keeps its sign.
    pub fn new(epsilon: f64) -> Result<Self, String> {
        if !epsilon.is_finite() || epsilon == 0.0 || epsilon.abs() >= 1.0 {
            return Err(format!(
                "{PERTURB_ENV} must be a finite relative error with 0 < |e| < 1, got {epsilon}"
            ));
        }
        Ok(Self { epsilon })
    }

    /// Parse the variable's text, for example `0.01` or `1e-3`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let trimmed = text.trim();
        let epsilon = trimmed
            .parse::<f64>()
            .map_err(|error| format!("{PERTURB_ENV}={trimmed:?} is not a number: {error}"))?;
        Self::new(epsilon)
    }

    /// The relative error `e`.
    pub fn epsilon(self) -> f64 {
        self.epsilon
    }

    /// `value · (1 + e)`.
    pub fn real(self, value: f64) -> f64 {
        value * (1.0 + self.epsilon)
    }

    /// `value · (1 + e) · exp(i·e)`: the magnitude scaled like a real value
    /// and the phase moved by `e` radians.
    pub fn complex(self, value: Complex64) -> Complex64 {
        value * Complex64::from_polar(1.0 + self.epsilon, self.epsilon)
    }
}

/// The perturbation this process runs under, if [`PERTURB_ENV`] asks for
/// one. An unset or empty variable means none.
///
/// # Panics
///
/// On a value [`Perturbation::parse`] rejects: a perturbed run that silently
/// ran unperturbed would report every case as caught.
pub fn active() -> Option<Perturbation> {
    #[cfg(test)]
    if let Some(scoped) = SCOPED.with(std::cell::Cell::get) {
        return scoped;
    }
    static ACTIVE: OnceLock<Option<Perturbation>> = OnceLock::new();
    *ACTIVE.get_or_init(|| {
        from_env_value(std::env::var_os(PERTURB_ENV).as_deref()).unwrap_or_else(|error| {
            panic!("{error}");
        })
    })
}

#[cfg(test)]
thread_local! {
    /// A unit test's own perturbation, which takes the place of the
    /// environment on that test's thread alone.
    static SCOPED: std::cell::Cell<Option<Option<Perturbation>>> =
        const { std::cell::Cell::new(None) };
}

/// Run `body` with `perturbation` in place of the environment's on this
/// thread, so a unit test can drive a comparison primitive at a chosen `e`.
#[cfg(test)]
pub(crate) fn scoped<R>(perturbation: Option<Perturbation>, body: impl FnOnce() -> R) -> R {
    struct Restore(Option<Option<Perturbation>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            SCOPED.with(|scoped| scoped.set(self.0));
        }
    }
    let _restore = Restore(SCOPED.with(|scoped| scoped.replace(Some(perturbation))));
    body()
}

/// The perturbation a raw environment value asks for.
fn from_env_value(raw: Option<&std::ffi::OsStr>) -> Result<Option<Perturbation>, String> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    let text = raw
        .to_str()
        .ok_or_else(|| format!("{PERTURB_ENV} is not valid Unicode"))?;
    if text.trim().is_empty() {
        return Ok(None);
    }
    Perturbation::parse(text).map(Some)
}

/// RSpice's real value as a comparison should see it.
pub fn actual(value: f64) -> f64 {
    active().map_or(value, |perturbation| perturbation.real(value))
}

/// RSpice's complex value as a comparison should see it.
pub fn actual_complex(value: Complex64) -> Complex64 {
    active().map_or(value, |perturbation| perturbation.complex(value))
}

/// A case that passed under perturbation, and the tolerance it was held to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlindCase {
    pub case: String,
    pub tolerance: String,
}

impl BlindCase {
    pub fn new(case: impl Into<String>, tolerance: impl Into<String>) -> Self {
        Self {
            case: case.into(),
            tolerance: tolerance.into(),
        }
    }
}

/// The verdict of a perturbed corpus run over `checked` cases that compare
/// numbers: `Ok` with a one-line summary when every one of them failed, else
/// `Err` naming each case that still passed, sorted, one per line.
pub fn blind_verdict(
    suite: &str,
    perturbation: Perturbation,
    checked: usize,
    blind: &[BlindCase],
) -> Result<String, String> {
    let e = perturbation.epsilon();
    if checked == 0 {
        return Err(format!(
            "{suite}: perturbed by {e:e} but no case compared a number; a census that checks nothing proves nothing"
        ));
    }
    if blind.is_empty() {
        return Ok(format!(
            "{suite}: all {checked} checked case(s) caught a {e:e} relative error"
        ));
    }
    let mut lines: Vec<String> = blind
        .iter()
        .map(|case| format!("  BLIND {}  [{}]", case.case, case.tolerance))
        .collect();
    lines.sort();
    Err(format!(
        "{suite}: {} of {checked} checked case(s) still pass with RSpice's values moved by {e:e}; their tolerance cannot see an error that size:\n{}",
        blind.len(),
        lines.join("\n")
    ))
}

/// Print a perturbed run's summary, or panic with its blind cases.
///
/// # Panics
///
/// When [`blind_verdict`] is `Err`.
pub fn assert_no_blind_cases(
    suite: &str,
    perturbation: Perturbation,
    checked: usize,
    blind: &[BlindCase],
) {
    match blind_verdict(suite, perturbation, checked, blind) {
        Ok(summary) => println!("{summary}"),
        Err(report) => panic!("{report}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perturbation(epsilon: f64) -> Perturbation {
        Perturbation::new(epsilon).expect("valid perturbation")
    }

    #[test]
    fn parse_accepts_relative_errors_and_rejects_the_rest() {
        assert_eq!(Perturbation::parse("0.01").unwrap().epsilon(), 0.01);
        assert_eq!(Perturbation::parse(" 1e-3 ").unwrap().epsilon(), 1e-3);
        assert_eq!(Perturbation::parse("-0.01").unwrap().epsilon(), -0.01);
        for bad in ["0", "1", "-1", "2.5", "nan", "inf", "one percent", ""] {
            assert!(
                Perturbation::parse(bad).is_err(),
                "{bad:?} must be rejected"
            );
        }
    }

    #[test]
    fn an_unset_or_empty_variable_means_no_perturbation() {
        assert_eq!(from_env_value(None), Ok(None));
        assert_eq!(from_env_value(Some("".as_ref())), Ok(None));
        assert_eq!(from_env_value(Some("  ".as_ref())), Ok(None));
        assert_eq!(
            from_env_value(Some("0.001".as_ref())),
            Ok(Some(perturbation(0.001)))
        );
        assert!(from_env_value(Some("0.0".as_ref())).is_err());
    }

    #[test]
    fn real_values_scale_by_one_plus_epsilon() {
        for epsilon in [1e-3, 1e-2] {
            let p = perturbation(epsilon);
            for value in [1.0, -3.3, 1e-12, 4.7e9] {
                let moved = p.real(value);
                assert!(((moved - value) / value - epsilon).abs() < 1e-12);
            }
            assert_eq!(p.real(0.0), 0.0, "a zero cannot be moved multiplicatively");
        }
    }

    #[test]
    fn complex_values_move_in_magnitude_and_phase() {
        for epsilon in [1e-3, 1e-2] {
            let p = perturbation(epsilon);
            let value = Complex64::new(0.3, -0.4);
            let moved = p.complex(value);
            assert!((moved.norm() / value.norm() - (1.0 + epsilon)).abs() < 1e-12);
            assert!((moved.arg() - value.arg() - epsilon).abs() < 1e-12);
        }
    }

    #[test]
    fn the_hooks_follow_the_scoped_perturbation_and_restore_it() {
        let value = Complex64::new(1.0, 1.0);
        scoped(None, || {
            assert_eq!(actual(2.0), 2.0);
            assert_eq!(actual_complex(value), value);
        });
        for epsilon in [1e-3, 1e-2] {
            let p = perturbation(epsilon);
            scoped(Some(p), || {
                assert_eq!(actual(2.0), p.real(2.0));
                assert_eq!(actual_complex(value), p.complex(value));
                // An inner scope wins and gives way again when it ends.
                scoped(None, || assert_eq!(actual(2.0), 2.0));
                assert_eq!(active(), Some(p));
            });
        }
    }

    #[test]
    fn the_verdict_lists_every_blind_case_sorted() {
        let p = perturbation(0.01);
        let summary = blind_verdict("Suite", p, 3, &[]).expect("nothing blind");
        assert!(summary.contains("all 3 checked case(s)"), "{summary}");

        let report = blind_verdict(
            "Suite",
            p,
            3,
            &[
                BlindCase::new("zeta", "rel 5e-2"),
                BlindCase::new("alpha", "abs 1"),
            ],
        )
        .expect_err("two cases are blind");
        assert!(
            report.starts_with("Suite: 2 of 3 checked case(s)"),
            "{report}"
        );
        let alpha = report.find("BLIND alpha  [abs 1]").expect("alpha listed");
        let zeta = report.find("BLIND zeta  [rel 5e-2]").expect("zeta listed");
        assert!(alpha < zeta, "sorted: {report}");
    }

    #[test]
    fn a_run_that_checked_nothing_is_not_a_pass() {
        assert!(blind_verdict("Suite", perturbation(0.01), 0, &[]).is_err());
    }
}
