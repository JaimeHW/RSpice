//! Capture and verify numerical fingerprints of the generated Verilog-A built-ins.
//!
//! `capture` writes one fixture per model; `verify` re-evaluates and compares
//! against them; `audit` runs the independent derivative oracle and reports what
//! it could and could not confirm.
//!
//! The three exist as separate verbs because they answer different questions and
//! are trusted differently. `verify` says the backend still computes what it
//! used to — necessary during a rewrite, but it can only ever confirm that a
//! wrong answer stayed wrong. `audit` says the derivative is actually the
//! derivative of the current, which is the property the goldens themselves were
//! never able to establish.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Args;
use rspice_core::device::veriloga_generated::builtins;
use rspice_core::device::veriloga_harness::fixture::{GoldenCase, GoldenFixture, GoldenPoint};
use rspice_core::device::veriloga_harness::golden::GoldenHarness;
use serde::Serialize;

use crate::error::BenchError;

/// Bias points captured per model per case.
const DEFAULT_PROBE_POINTS: usize = 6;

/// Tolerances a replay must meet, matching the backend-rewrite gates.
const PRIMAL_TOLERANCE: f64 = 1.0e-12;
const JACOBIAN_TOLERANCE: f64 = 1.0e-9;
const NOISE_TOLERANCE: f64 = 1.0e-10;

/// Relative agreement demanded of the derivative oracle where the numerical
/// difference converged.
const AUDIT_BASE_TOLERANCE: f64 = 1.0e-5;

/// A fixture that records the capturing backend's defect rather than the model.
///
/// Listing one is a claim that the *replay* is right and the *baseline* is
/// wrong, which is the opposite of what this command is for, so each entry has
/// to say how that was established by something other than the replay itself.
/// The alternative — re-capturing the fixture — would make `verify` compare a
/// backend against itself for that model, and silently retire the only
/// cross-backend check there is.
///
/// Checked in both directions. A model that starts deviating is a failure, and
/// so is a listed model that stops: an allowlist nobody prunes stops describing
/// anything.
struct FixtureDeviation {
    model: &'static str,
    /// Relative Jacobian disagreement to tolerate, rounded up from the measured
    /// value.
    jacobian: f64,
    why: &'static str,
}

const FIXTURE_DEVIATIONS: &[FixtureDeviation] = &[FixtureDeviation {
    model: "DIODE_CMC",
    jacobian: 5.0e-3,
    why: "the model writes `I(a, aik) <+ $simparam(\"gmin\", 0.0) * V(a, aik)`, and the backend \
          that captured this fixture folded that call to its literal fallback at generation time \
          — its generated source contains no `simparam` and no `gmin` anywhere, so the term was \
          dropped. The canonical backend emits `ctx.simparam_or(\"gmin\", ..)` and reads the \
          simulator's value, which is `constants::GMIN` = 1e-12; four entries of the (a, aik) \
          block therefore differ by exactly 1e-12, with the signs of a parallel conductance. The \
          independent derivative oracle confirms the canonical Jacobian against the model's own \
          currents at 8.5e-7, so the replay is the correct side",
}];

fn fixture_deviation(model: &str) -> Option<&'static FixtureDeviation> {
    FIXTURE_DEVIATIONS
        .iter()
        .find(|deviation| deviation.model == model)
}

#[derive(Args, Debug)]
pub struct VerilogAGoldenArgs {
    #[command(subcommand)]
    command: GoldenCommand,
}

#[derive(clap::Subcommand, Debug)]
enum GoldenCommand {
    /// Write a fingerprint fixture per model.
    Capture(CaptureArgs),
    /// Re-evaluate every model and compare against the checked-in fixtures.
    Verify(VerifyArgs),
    /// Check stamped Jacobians against a numerical derivative of the stamped currents.
    Audit(AuditArgs),
}

#[derive(Args, Debug)]
struct CaptureArgs {
    /// Directory to write fixtures into.
    #[arg(long, default_value = "benchmarks/reference/veriloga-golden")]
    out: PathBuf,
    /// Restrict to these models; empty means every compiled-in built-in.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Bias points per case.
    #[arg(long, default_value_t = DEFAULT_PROBE_POINTS)]
    points: usize,
}

#[derive(Args, Debug)]
struct VerifyArgs {
    /// Directory holding the fixtures.
    #[arg(long, default_value = "benchmarks/reference/veriloga-golden")]
    fixtures: PathBuf,
    /// Restrict to these models; empty means every fixture present.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Require bit-identical replay instead of the gate tolerances.
    #[arg(long)]
    exact: bool,
}

#[derive(Args, Debug)]
struct AuditArgs {
    /// Restrict to these models; empty means every compiled-in built-in.
    #[arg(long, value_delimiter = ',')]
    models: Vec<String>,
    /// Bias points per model.
    #[arg(long, default_value_t = DEFAULT_PROBE_POINTS)]
    points: usize,
    /// Write a JSON report here.
    #[arg(long)]
    out: Option<PathBuf>,
}

pub fn run(args: &VerilogAGoldenArgs) -> Result<ExitCode, BenchError> {
    match &args.command {
        GoldenCommand::Capture(capture) => run_capture(capture),
        GoldenCommand::Verify(verify) => run_verify(verify),
        GoldenCommand::Audit(audit) => run_audit(audit),
    }
}

fn selected_models(requested: &[String]) -> Result<Vec<&'static str>, BenchError> {
    let available = builtins::builtin_names();
    if requested.is_empty() {
        return Ok(available.to_vec());
    }
    let mut selected = Vec::with_capacity(requested.len());
    for name in requested {
        let found = available
            .iter()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .copied()
            .ok_or_else(|| BenchError::GeneratedStamp {
                message: format!("'{name}' is not a compiled-in generated built-in"),
            })?;
        selected.push(found);
    }
    Ok(selected)
}

/// Fixture file name for a model.
///
/// Model names come from the registry and are already identifier-shaped, but
/// they are folded to lower case so a case-insensitive file system cannot make
/// two models collide silently.
fn fixture_path(root: &Path, model_name: &str) -> PathBuf {
    root.join(format!("{}.txt", model_name.to_ascii_lowercase()))
}

fn build_fixture(model_name: &'static str, points: usize) -> Result<GoldenFixture, BenchError> {
    let mut harness =
        GoldenHarness::new(model_name, &[]).map_err(|error| BenchError::GeneratedStamp {
            message: error.to_string(),
        })?;

    let mut captured = Vec::new();
    for point in harness.probe_points(points) {
        let record = harness
            .evaluate(&point)
            .map_err(|error| BenchError::GeneratedStamp {
                message: error.to_string(),
            })?;
        captured.push(GoldenPoint {
            unknowns: point,
            record,
        });
    }

    Ok(GoldenFixture {
        model_name: model_name.to_string(),
        node_count: harness.node_count(),
        branch_count: harness.branch_count(),
        cases: vec![GoldenCase {
            options: Vec::new(),
            points: captured,
        }],
    })
}

fn run_capture(args: &CaptureArgs) -> Result<ExitCode, BenchError> {
    let models = selected_models(&args.models)?;
    fs::create_dir_all(&args.out).map_err(|error| BenchError::GeneratedStamp {
        message: format!("creating {}: {error}", args.out.display()),
    })?;

    let mut written = 0usize;
    let mut failed = Vec::new();
    for model_name in models {
        match build_fixture(model_name, args.points) {
            Ok(fixture) => {
                let path = fixture_path(&args.out, model_name);
                fs::write(&path, fixture.render()).map_err(|error| {
                    BenchError::GeneratedStamp {
                        message: format!("writing {}: {error}", path.display()),
                    }
                })?;
                written += 1;
            }
            Err(error) => failed.push(format!("{model_name}: {error}")),
        }
    }

    println!("captured {written} fixtures into {}", args.out.display());
    for failure in &failed {
        eprintln!("skipped: {failure}");
    }
    // A model that cannot be captured has no baseline, which is exactly the
    // situation the rewrite must not start from.
    Ok(if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[derive(Debug, Default)]
struct Deviation {
    worst_primal: f64,
    worst_jacobian: f64,
    worst_noise: f64,
    structural: Vec<String>,
}

fn relative(left: f64, right: f64) -> f64 {
    if left == right {
        return 0.0;
    }
    let scale = left.abs().max(right.abs());
    if scale == 0.0 { 0.0 } else { (left - right).abs() / scale }
}

fn compare_point(
    label: &str,
    expected: &GoldenPoint,
    actual: &GoldenPoint,
    deviation: &mut Deviation,
) {
    for (index, (want, got)) in expected
        .record
        .rhs
        .iter()
        .zip(actual.record.rhs.iter())
        .enumerate()
    {
        // A value that used to be finite and now is not never shows up as a
        // large relative error, so it is checked separately rather than being
        // folded into the maximum.
        if want.is_finite() != got.is_finite() {
            deviation
                .structural
                .push(format!("{label}: rhs[{index}] finiteness changed"));
        }
        deviation.worst_primal = deviation.worst_primal.max(relative(*want, *got));
    }

    for (index, (want, got)) in expected
        .record
        .jacobian
        .iter()
        .zip(actual.record.jacobian.iter())
        .enumerate()
    {
        if (*want == 0.0) != (*got == 0.0) {
            deviation
                .structural
                .push(format!("{label}: jacobian[{index}] appeared or vanished"));
        }
        deviation.worst_jacobian = deviation.worst_jacobian.max(relative(*want, *got));
    }

    for (index, (want, got)) in expected
        .record
        .capacitance
        .iter()
        .zip(actual.record.capacitance.iter())
        .enumerate()
    {
        if (*want == 0.0) != (*got == 0.0) {
            deviation
                .structural
                .push(format!("{label}: capacitance[{index}] appeared or vanished"));
        }
        deviation.worst_jacobian = deviation.worst_jacobian.max(relative(*want, *got));
    }

    if expected.record.noise.len() != actual.record.noise.len() {
        deviation.structural.push(format!(
            "{label}: noise source count {} -> {}",
            expected.record.noise.len(),
            actual.record.noise.len()
        ));
    }
    for (want, got) in expected.record.noise.iter().zip(actual.record.noise.iter()) {
        if want.mechanism != got.mechanism {
            deviation.structural.push(format!(
                "{label}: noise mechanism '{}' -> '{}'",
                want.mechanism, got.mechanism
            ));
        }
        if want.active != got.active {
            deviation.structural.push(format!(
                "{label}: noise '{}' active {} -> {}",
                want.mechanism, want.active, got.active
            ));
        }
        deviation.worst_noise = deviation.worst_noise.max(relative(want.psd, got.psd));
    }
}

fn run_verify(args: &VerifyArgs) -> Result<ExitCode, BenchError> {
    let models = selected_models(&args.models)?;
    let mut failures = Vec::new();
    let mut verified = 0usize;

    for model_name in models {
        let path = fixture_path(&args.fixtures, model_name);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                failures.push(format!("{model_name}: reading {}: {error}", path.display()));
                continue;
            }
        };
        let expected = match GoldenFixture::parse(&text) {
            Ok(fixture) => fixture,
            Err(error) => {
                failures.push(format!("{model_name}: {}: {error}", path.display()));
                continue;
            }
        };

        let actual = match build_fixture(model_name, expected_point_count(&expected)) {
            Ok(fixture) => fixture,
            Err(error) => {
                failures.push(format!("{model_name}: {error}"));
                continue;
            }
        };

        if expected.node_count != actual.node_count || expected.branch_count != actual.branch_count
        {
            failures.push(format!(
                "{model_name}: topology changed: {}+{} unknowns captured, {}+{} now",
                expected.node_count,
                expected.branch_count,
                actual.node_count,
                actual.branch_count
            ));
            continue;
        }

        let mut deviation = Deviation::default();
        for (case_index, (want_case, got_case)) in
            expected.cases.iter().zip(actual.cases.iter()).enumerate()
        {
            for (point_index, (want, got)) in
                want_case.points.iter().zip(got_case.points.iter()).enumerate()
            {
                compare_point(
                    &format!("case {case_index} point {point_index}"),
                    want,
                    got,
                    &mut deviation,
                );
            }
        }

        let known = fixture_deviation(model_name);
        let (primal_limit, jacobian_limit, noise_limit) = if args.exact {
            (0.0, 0.0, 0.0)
        } else {
            (
                PRIMAL_TOLERANCE,
                known.map_or(JACOBIAN_TOLERANCE, |deviation| deviation.jacobian),
                NOISE_TOLERANCE,
            )
        };

        let mut model_failures = deviation.structural.clone();
        // The other direction: a fixture that stops recording its defect means
        // it has been re-captured, or the backend has changed under it, and
        // either way the entry is now describing nothing.
        if let Some(known) = known
            && !args.exact
            && deviation.worst_jacobian <= JACOBIAN_TOLERANCE
        {
            model_failures.push(format!(
                "listed as a fixture deviation but now agrees to {:.0e}; drop the entry ({})",
                JACOBIAN_TOLERANCE, known.why
            ));
        }
        if deviation.worst_primal > primal_limit {
            model_failures.push(format!(
                "primal deviation {:.3e} exceeds {primal_limit:.0e}",
                deviation.worst_primal
            ));
        }
        if deviation.worst_jacobian > jacobian_limit {
            model_failures.push(format!(
                "jacobian deviation {:.3e} exceeds {jacobian_limit:.0e}",
                deviation.worst_jacobian
            ));
        }
        if deviation.worst_noise > noise_limit {
            model_failures.push(format!(
                "noise deviation {:.3e} exceeds {noise_limit:.0e}",
                deviation.worst_noise
            ));
        }

        if model_failures.is_empty() {
            verified += 1;
        } else {
            for failure in model_failures.iter().take(5) {
                failures.push(format!("{model_name}: {failure}"));
            }
        }
    }

    println!("verified {verified} models against {}", args.fixtures.display());
    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    Ok(if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn expected_point_count(fixture: &GoldenFixture) -> usize {
    fixture
        .cases
        .first()
        .map_or(DEFAULT_PROBE_POINTS, |case| case.points.len())
}

#[derive(Serialize)]
struct AuditReport {
    models: Vec<AuditModelReport>,
}

#[derive(Serialize)]
struct AuditModelReport {
    model_name: &'static str,
    entries: usize,
    tightly_verified_fraction: f64,
    unverified_significant: usize,
    worst_relative_error: f64,
}

fn run_audit(args: &AuditArgs) -> Result<ExitCode, BenchError> {
    let models = selected_models(&args.models)?;
    let mut reports = Vec::new();
    let mut failures = Vec::new();

    println!(
        "{:<26} {:>8} {:>10} {:>12} {:>12}",
        "model", "entries", "verified", "gaps", "worst"
    );

    for model_name in models {
        let mut harness =
            GoldenHarness::new(model_name, &[]).map_err(|error| BenchError::GeneratedStamp {
                message: error.to_string(),
            })?;

        let mut entries = 0usize;
        let mut tight = 0usize;
        let mut gaps = 0usize;
        let mut worst = 0.0_f64;

        for (index, point) in harness.probe_points(args.points).into_iter().enumerate() {
            let audit = match harness.jacobian_audit(&point) {
                Ok(audit) => audit,
                Err(error) => {
                    failures.push(format!("{model_name}: point {index}: {error}"));
                    continue;
                }
            };
            entries += audit.entries.len();
            tight += audit.checked().count();
            gaps += audit.unverified_significant().len();
            worst = worst.max(audit.worst_relative_error());

            for failure in audit.failures(AUDIT_BASE_TOLERANCE).into_iter().take(3) {
                failures.push(format!(
                    "{model_name}: point {index}: d(row {})/d(col {}) stamped {:e}, numeric {:e}, relative error {:.3e}",
                    failure.row, failure.col, failure.stamped, failure.numeric, failure.relative_error
                ));
            }
        }

        let fraction = if entries == 0 {
            1.0
        } else {
            tight as f64 / entries as f64
        };
        println!(
            "{model_name:<26} {entries:>8} {:>9.1}% {gaps:>12} {worst:>12.2e}",
            fraction * 100.0
        );
        reports.push(AuditModelReport {
            model_name,
            entries,
            tightly_verified_fraction: fraction,
            unverified_significant: gaps,
            worst_relative_error: worst,
        });
    }

    if let Some(path) = &args.out {
        let json = serde_json::to_string_pretty(&AuditReport { models: reports }).map_err(
            |error| BenchError::GeneratedStamp {
                message: format!("serializing audit report: {error}"),
            },
        )?;
        fs::write(path, json).map_err(|error| BenchError::GeneratedStamp {
            message: format!("writing {}: {error}", path.display()),
        })?;
    }

    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    Ok(if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_difference_is_symmetric_and_zero_on_equality() {
        assert_eq!(relative(1.0, 1.0), 0.0);
        assert_eq!(relative(0.0, 0.0), 0.0);
        assert_eq!(relative(1.0, 2.0), relative(2.0, 1.0));
        assert_eq!(relative(1.0, 2.0), 0.5);
    }

    #[test]
    fn identical_infinities_do_not_report_a_deviation() {
        // `inf - inf` is NaN, which would compare false against every
        // tolerance and silently pass. Equality short-circuits it instead.
        assert_eq!(relative(f64::INFINITY, f64::INFINITY), 0.0);
    }

    #[test]
    fn fixture_paths_are_case_folded() {
        let root = Path::new("fixtures");
        assert_eq!(
            fixture_path(root, "PSP104VA"),
            fixture_path(root, "psp104va")
        );
    }
}
